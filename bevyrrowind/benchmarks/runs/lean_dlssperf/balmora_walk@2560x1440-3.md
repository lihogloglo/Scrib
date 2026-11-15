# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=performance · resolution: 2560x1440 · rings: near None far None
- 4521 frames over 65.7 s · **median 14.13 ms (71 fps)** · mean 14.52 · p95 17.82 · p99 20.24 (1% low 49 fps) · worst 36.83 ms · 30 spikes over 1.5× median · 19.2 stutters/min (work 17.4/min)
- warm-up: 339 frames over 8.6 s, worst 2611.6 ms
- hitches (> 35.3 ms): **2** measured, 14 in warm-up
- timed GPU passes: 8.34 ms of a 14.52 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 14.13 ms, limit 11 ms
- **FAIL** frame_ms: 21 stutters (> 1.5x moving median of 60) in 1.09 min = 19.19/min, limit 1/min
- **FAIL** worst frame 36.83 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.743 | 2.747 | 2.793 | 1.00 | 0.186 |
| main_opaque_pass_3d | 1.662 | 1.664 | 2.042 | 1.00 | 0.076 |
| early prepass | 0.556 | 0.557 | 0.762 | 1.00 | 0.126 |
| uniform_allocation | 0.308 | 0.318 | 0.336 | 0.97 | 0.057 |
| bloom | 0.305 | 0.313 | 0.316 | 0.97 | 0.068 |
| atmosphere_luts | 0.303 | 0.312 | 0.371 | 0.97 | 0.004 |
| water_velocity | 0.282 | 0.284 | 0.353 | 0.99 | 0.002 |
| bin_unpacking | 0.255 | 0.255 | 0.272 | 1.00 | 0.040 |
| water_sspr | 0.210 | 0.210 | 0.213 | 1.00 | 0.002 |
| ssao | 0.187 | 0.192 | 0.194 | 0.97 | 0.008 |
| auxiliary_views | 0.181 | 0.181 | 0.659 | 1.00 | 0.172 |
| atmos_froxel_inject | 0.147 | 0.147 | 0.150 | 1.00 | 0.002 |
| clouds_trace | 0.140 | 0.141 | 0.233 | 1.00 | 0.001 |
| water_fft | 0.132 | 0.132 | 0.137 | 1.00 | 0.003 |
| upscaling | 0.128 | 0.128 | 0.170 | 1.00 | 0.004 |
| clouds_composite | 0.104 | 0.108 | 0.111 | 0.97 | 0.002 |
| main_transmissive_pass_3d | 0.090 | 0.090 | 0.171 | 1.00 | 0.008 |
| tonemapping | 0.088 | 0.088 | 0.092 | 1.00 | 0.004 |
| clustering | 0.063 | 0.065 | 0.069 | 0.97 | 0.020 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.422 | 0.422 | 0.13 | 0.001 |
| surface_trail | 0.050 | 0.057 | 0.057 | 0.88 | 0.002 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.041 | 0.041 | 0.048 | 1.00 | 0.005 |
| water_snapshot | 0.031 | 0.031 | 0.034 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.226 | 0.236 | 0.13 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.028 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 1.00 | 0.108 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.97 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.014 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 74.926 | 0.000 | 33303.35 | 338742.5 |
| render/thread/ms | 13.302 | 16.425 | 34.33 | 60140.3 |
| render/busy/ms | 12.733 | 15.803 | 33.54 | 57568.1 |
| main/busy/ms | 9.833 | 12.805 | 26.21 | 44453.4 |
| render/graph/ms | 7.945 | 9.945 | 18.17 | 35918.3 |
| pipelines/compile_ms | 0.850 | 0.000 | 173.58 | 3842.3 |
| pipelines/front_ms | 0.820 | 0.000 | 166.77 | 3705.9 |
| render/present/ms | 0.471 | 0.561 | 2.75 | 2129.8 |
| bigspace/total/ms | 0.444 | 0.682 | 4.47 | 2008.5 |
| terrain/update/ms | 0.290 | 0.363 | 1.51 | 1309.0 |
| controller/feed/ms | 0.267 | 0.375 | 0.73 | 1208.2 |
| stream/drive/ms | 0.257 | 0.374 | 17.18 | 1162.7 |
| terrain/select/ms | 0.257 | 0.320 | 0.58 | 1162.1 |
| far/select/ms | 0.194 | 0.275 | 0.53 | 877.8 |
| stream/tick/ms | 0.193 | 0.272 | 0.78 | 871.7 |
| bigspace/low_precision/ms | 0.161 | 0.274 | 4.20 | 728.9 |
| anim/evaluate_cpu/ms | 0.158 | 0.204 | 0.52 | 715.9 |
| anim/evaluate/ms | 0.146 | 0.307 | 3.50 | 658.8 |
| controller/move/ms | 0.136 | 0.277 | 0.56 | 613.8 |
| bigspace/high_precision/ms | 0.110 | 0.215 | 4.19 | 498.7 |
| render/acquire/ms | 0.098 | 0.122 | 3.48 | 442.4 |
| physics/insert/ms | 0.067 | 0.452 | 0.96 | 302.9 |
| physics/step/ms | 0.063 | 0.093 | 0.73 | 283.3 |
| pipelines/driver_ms | 0.030 | 0.000 | 6.82 | 136.5 |
| weather/map/ms | 0.024 | 0.034 | 0.19 | 109.8 |
| anim/apply/ms | 0.012 | 0.022 | 0.17 | 52.3 |
| controller/avoid/ms | 0.010 | 0.014 | 0.20 | 47.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.17 | 46.3 |
| script/update/ms | 0.007 | 0.009 | 0.06 | 31.7 |
| stream/despawn/ms | 0.007 | 0.000 | 1.27 | 31.1 |
| stream/spawn/ms | 0.006 | 0.000 | 1.09 | 25.9 |
| script/input/ms | 0.005 | 0.007 | 0.19 | 22.0 |
| script/ui/ms | 0.004 | 0.006 | 0.07 | 19.2 |
| controller/write/ms | 0.004 | 0.005 | 0.06 | 16.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 15.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.08 | 4.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 26 | 0 |
| anim/a2/count | count | 1 | 13 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 68 | 19 |
| controller/movers/count | count | 20 | 23 | 19 |
| crowd/c0/count | count | 26 | 59 | 12 |
| crowd/c0/joints/count | count | 1301 | 2266 | 535 |
| crowd/c0/meshes/count | count | 353 | 646 | 144 |
| crowd/c1/count | count | 146 | 157 | 149 |
| crowd/c1/joints/count | count | 5512 | 6046 | 5661 |
| crowd/c1/meshes/count | count | 1166 | 1326 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4260 | 7428 | 6179 |
| crowd/joints_uploaded/count | count | 4242 | 6643 | 6179 |
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
| frame/work/ms | ms | 12.41 | 33.54 | 11.63 |
| gpu/timed/ms | ms | 9.00 | 16.52 | 9.21 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 8 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 821 | 1416 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 821 | 1416 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 38 | 42 | 39 |
| phases/opaque3d/bins | count | 1289 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3425 | 5139 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 38 | 42 | 39 |
| phases/opaque3d_prepass/bins | count | 1289 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3425 | 5139 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 77 | 296 | 64 |
| phases/shadow/bins | count | 3801 | 5407 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9599 | 12.4 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 143 | 306 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 178 | 476 | 19 |
| physics/pending | count | 0 | 1107 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 68 | 0 |
| preprocess/bind_groups | count | 14 | 841 | 8 |
| preprocess/bind_groups_created | count | 14 | 841 | 8 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 333 | 452 | 331 |
| process/cpu_usage | % | 23.08 | 37.76 | 23.08 |
| process/mem_usage | GiB | 5.13 | 5.30 | 5.30 |
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
| started_load_count |  | 4764 | 4858 | 4858 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.25 | 63.53 | 25.94 |
| system/mem_usage | % | 70.83 | 75.45 | 70.71 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 165 | 388 | 158 |
| views/all/bins | count | 6465 | 8131 | 5692 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.4 k | 24.9 k | 13.3 k |
| views/all/pipelines | count | 206 | 374 | 113 |
| views/all/pipelines_distinct | count | 98 | 109 | 91 |
| views/all/work_items | count | 17.5 k | 23.7 k | 13.6 k |
| views/cascade/batch_sets | count | 68 | 85 | 64 |
| views/cascade/bins | count | 2593 | 3184 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8141 | 10.0 k | 5663 |
| views/cascade/pipelines | count | 56 | 72 | 47 |
| views/cascade/work_items | count | 8141 | 10.0 k | 5663 |
| views/lamp_face/batch_sets | count | 6 | 226 | 0 |
| views/lamp_face/bins | count | 1088 | 2477 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1215 | 2852 | 0 |
| views/lamp_face/pipelines | count | 86 | 242 | 0 |
| views/lamp_face/work_items | count | 66 | 2445 | 0 |
| views/main/batch_sets | count | 92 | 100 | 94 |
| views/main/bins | count | 2866 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8492 | 13.0 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8651 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 57.15 | 957 | 258.4 k |
| anim/events/count | count | 0.13 | 3 | 581 |
| assets/Image/added | count | 0.15 | 43 | 699 |
| assets/Image/modified | count | 0.00 | 3 | 18 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.96 | 152 | 4359 |
| assets/Mesh/removed | count | 1.08 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.13 | 35 | 580 |
| assets/StandardMaterial/modified | count | 4.79 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 4.11 | 3369 | 18.6 k |
| physics/inserts | count | 0.73 | 150 | 3317 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.52 | 145 | 2335 |
| physics/steps | count | 0.87 | 2 | 3940 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 22 | 378 |
| stream/upload/bytes | bytes | 7488 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1642 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 35.3 ms; slowest 16, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.61 | 2611.6 | main/busy/ms 2577, surface/advance_cpu_ms 67.08, surface/cell_upload_ms 0.76, stream/drive/ms 0.29, weather/map/ms 0.13, stream/tick/ms 0.08, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, script/update/ms 0.03, weather/eval/ms 0.02, anim/evaluate/ms 0.01, script/ui/ms 0.01 |
| warm-up | 3.32 | 706.4 | main/busy/ms 2622, render/thread/ms 658, render/busy/ms 514, render/acquire/ms 142, surface/advance_cpu_ms 67.08, render/graph/ms 5.40, render/present/ms 2.03, stream/drive/ms 1.38, stream/tick/ms 0.79, surface/cell_upload_ms 0.76, terrain/update/ms 0.39, bigspace/total/ms 0.34 |
| warm-up | 3.59 | 273.9 | pipelines/wait_ms 3472, render/thread/ms 668, render/busy/ms 523, pipelines/compile_ms 295, pipelines/front_ms 274, render/acquire/ms 142, main/busy/ms 60.53, terrain/update/ms 22.95, pipelines/driver_ms 20.36, render/graph/ms 9.16, render/present/ms 3.13, stream/drive/ms 1.60 |
| warm-up | 3.84 | 85.2 | pipelines/compile_ms 235, pipelines/front_ms 219, main/busy/ms 64.09, render/thread/ms 52.88, render/busy/ms 51.15, pipelines/driver_ms 15.95, render/graph/ms 4.28, stream/drive/ms 1.58, render/present/ms 1.43, stream/tick/ms 1.22, terrain/update/ms 0.72, terrain/select/ms 0.59 |
| warm-up | 3.98 | 50.1 | main/busy/ms 37.55, render/thread/ms 36.78, render/busy/ms 35.63, stream/drive/ms 26.11, render/graph/ms 3.34, render/present/ms 0.97, stream/tick/ms 0.80, stream/spawn/ms 0.78, terrain/update/ms 0.76, terrain/select/ms 0.63, bigspace/total/ms 0.26, render/acquire/ms 0.18 |
| warm-up | 4.04 | 47.4 | pipelines/wait_ms 678, pipelines/compile_ms 174, pipelines/front_ms 156, render/thread/ms 39.87, main/busy/ms 39.73, render/busy/ms 38.00, stream/drive/ms 27.10, pipelines/driver_ms 18.70, render/graph/ms 4.89, stream/spawn/ms 1.77, render/present/ms 1.64, stream/tick/ms 1.05 |
| warm-up | 4.14 | 76.2 | pipelines/wait_ms 397, render/thread/ms 59.83, render/busy/ms 58.02, main/busy/ms 36.48, pipelines/compile_ms 35.18, pipelines/front_ms 33.61, stream/drive/ms 11.81, render/graph/ms 5.59, stream/tick/ms 2.09, pipelines/driver_ms 1.57, render/present/ms 1.54, stream/spawn/ms 1.00 |
| warm-up | 4.22 | 49.3 | pipelines/wait_ms 2634, pipelines/compile_ms 180, pipelines/front_ms 169, main/busy/ms 47.62, render/thread/ms 47.53, render/busy/ms 46.42, stream/drive/ms 30.18, pipelines/driver_ms 11.22, stream/spawn/ms 3.62, render/graph/ms 3.44, render/present/ms 0.89, terrain/update/ms 0.88 |
| warm-up | 4.44 | 85.8 | pipelines/wait_ms 1407, main/busy/ms 73.97, pipelines/compile_ms 64.87, pipelines/front_ms 61.56, stream/drive/ms 49.43, render/thread/ms 39.70, render/busy/ms 38.58, render/graph/ms 3.96, stream/spawn/ms 3.89, pipelines/driver_ms 3.31, stream/tick/ms 1.66, terrain/update/ms 1.16 |
| warm-up | 4.55 | 68.5 | pipelines/wait_ms 513, pipelines/compile_ms 119, pipelines/front_ms 116, main/busy/ms 70.14, stream/drive/ms 46.98, render/thread/ms 45.26, render/busy/ms 44.08, render/graph/ms 3.54, pipelines/driver_ms 3.38, stream/spawn/ms 1.73, stream/activate/ms 1.09, terrain/update/ms 1.06 |
| warm-up | 4.61 | 49.5 | pipelines/wait_ms 582, pipelines/compile_ms 121, pipelines/front_ms 117, main/busy/ms 49.72, stream/drive/ms 34.81, render/thread/ms 26.08, render/busy/ms 24.90, pipelines/driver_ms 3.91, render/graph/ms 3.33, stream/activate/ms 1.19, stream/tick/ms 1.09, render/present/ms 1.00 |
| warm-up | 5.65 | 42.5 | pipelines/wait_ms 830, pipelines/compile_ms 155, pipelines/front_ms 149, main/busy/ms 49.49, render/thread/ms 42.97, render/busy/ms 41.39, stream/drive/ms 8.51, pipelines/driver_ms 6.38, render/graph/ms 6.01, stream/spawn/ms 1.35, render/present/ms 1.27, physics/insert/ms 0.84 |
| warm-up | 6.02 | 37.3 | pipelines/wait_ms 7111, pipelines/compile_ms 62.48, pipelines/front_ms 58.93, render/thread/ms 56.92, render/busy/ms 55.53, main/busy/ms 34.86, stream/drive/ms 15.24, render/graph/ms 8.11, pipelines/driver_ms 3.55, stream/spawn/ms 2.20, bigspace/total/ms 1.21, render/present/ms 1.17 |
| warm-up | 6.09 | 52.0 | main/busy/ms 45.33, render/thread/ms 45.18, render/busy/ms 43.74, stream/drive/ms 8.40, render/graph/ms 8.33, stream/spawn/ms 2.14, physics/insert/ms 1.35, render/present/ms 1.23, stream/tick/ms 0.92, bigspace/total/ms 0.74, terrain/update/ms 0.73, terrain/select/ms 0.59 |
| measure | 2.74 | 35.7 | pipelines/wait_ms 6079, pipelines/compile_ms 156, pipelines/front_ms 149, render/thread/ms 48.55, render/busy/ms 47.18, render/graph/ms 25.23, main/busy/ms 18.48, pipelines/driver_ms 6.51, render/present/ms 1.17, bigspace/total/ms 0.65, stream/drive/ms 0.60, controller/feed/ms 0.60 |
| measure | 59.06 | 36.8 | render/thread/ms 45.05, render/busy/ms 43.91, main/busy/ms 23.46, render/graph/ms 14.74, stream/drive/ms 1.12, stream/tick/ms 1.00, render/present/ms 0.95, bigspace/total/ms 0.90, terrain/update/ms 0.82, far/select/ms 0.76, terrain/select/ms 0.73, controller/feed/ms 0.70 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_dlssperf\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=performance
- before the run: GPU 0%, CPU 11%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **74%** (min 50%) · 2575 MHz (min 2565) · memory 8001 MHz · 76.8 W · 76 °C (max 81) · VRAM 5477 MiB · throttled 100% of samples
