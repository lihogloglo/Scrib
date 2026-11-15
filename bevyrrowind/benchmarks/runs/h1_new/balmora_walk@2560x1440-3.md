# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4219 frames over 65.7 s · **median 15.17 ms (66 fps)** · mean 15.56 · p95 18.45 · p99 21.48 (1% low 47 fps) · worst 46.25 ms · 31 spikes over 1.5× median · 20.1 stutters/min (work 15.5/min)
- warm-up: 402 frames over 9.6 s, worst 2510.9 ms
- hitches (> 37.9 ms): **3** measured, 14 in warm-up
- timed GPU passes: 8.20 ms of a 15.56 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 15.17 ms, limit 11 ms
- **FAIL** frame_ms: 22 stutters (> 1.5x moving median of 60) in 1.09 min = 20.11/min, limit 1/min
- **FAIL** worst frame 46.25 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.206 | 2.208 | 2.666 | 1.00 | 0.190 |
| main_opaque_pass_3d | 1.757 | 1.758 | 2.447 | 1.00 | 0.073 |
| early prepass | 0.548 | 0.548 | 1.020 | 1.00 | 0.122 |
| uniform_allocation | 0.382 | 0.382 | 0.676 | 1.00 | 0.057 |
| bloom | 0.341 | 0.341 | 0.493 | 1.00 | 0.069 |
| atmosphere_luts | 0.333 | 0.334 | 0.606 | 1.00 | 0.003 |
| water_velocity | 0.301 | 0.302 | 0.492 | 1.00 | 0.002 |
| bin_unpacking | 0.289 | 0.289 | 0.338 | 1.00 | 0.039 |
| water_sspr | 0.226 | 0.226 | 0.234 | 1.00 | 0.002 |
| ssao | 0.201 | 0.202 | 0.208 | 1.00 | 0.008 |
| auxiliary_views | 0.183 | 0.183 | 0.677 | 1.00 | 0.166 |
| atmos_froxel_inject | 0.161 | 0.161 | 0.153 | 1.00 | 0.002 |
| water_fft | 0.143 | 0.143 | 0.139 | 1.00 | 0.003 |
| clouds_trace | 0.141 | 0.141 | 0.227 | 1.00 | 0.001 |
| upscaling | 0.127 | 0.127 | 0.153 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.112 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.091 | 0.091 | 0.091 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.089 | 0.089 | 0.168 | 1.00 | 0.009 |
| clustering | 0.066 | 0.066 | 0.069 | 1.00 | 0.020 |
| early_mesh_preprocessing | 0.057 | 0.057 | 0.061 | 1.00 | 0.005 |
| surface_trail | 0.056 | 0.061 | 0.057 | 0.92 | 0.002 |
| render_sky | 0.055 | 0.055 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.055 | 0.436 | 0.723 | 0.13 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| water_snapshot | 0.038 | 0.038 | 0.044 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.029 | 0.235 | 0.237 | 0.13 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.031 | 1.00 | 0.001 |
| water_mips | 0.025 | 0.025 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.018 | 0.018 | 0.031 | 1.00 | 0.150 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.018 | 0.016 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 30.126 | 0.000 | 15678.03 | 127103.1 |
| render/thread/ms | 14.234 | 16.929 | 33.25 | 60054.9 |
| render/busy/ms | 13.659 | 16.332 | 32.47 | 57626.5 |
| main/busy/ms | 9.269 | 12.453 | 45.45 | 39107.5 |
| render/graph/ms | 8.555 | 10.168 | 19.64 | 36092.4 |
| pipelines/compile_ms | 0.547 | 0.000 | 149.69 | 2309.6 |
| pipelines/front_ms | 0.528 | 0.000 | 143.96 | 2228.2 |
| render/present/ms | 0.485 | 0.587 | 9.90 | 2044.9 |
| bigspace/total/ms | 0.433 | 0.687 | 5.69 | 1825.8 |
| terrain/update/ms | 0.290 | 0.363 | 1.51 | 1224.3 |
| controller/feed/ms | 0.279 | 0.371 | 0.71 | 1178.4 |
| terrain/select/ms | 0.258 | 0.324 | 0.62 | 1086.6 |
| stream/drive/ms | 0.247 | 0.356 | 19.83 | 1041.0 |
| far/select/ms | 0.196 | 0.264 | 0.59 | 825.9 |
| stream/tick/ms | 0.182 | 0.247 | 0.62 | 769.3 |
| bigspace/low_precision/ms | 0.159 | 0.290 | 4.05 | 669.1 |
| anim/evaluate_cpu/ms | 0.154 | 0.199 | 0.49 | 651.2 |
| anim/evaluate/ms | 0.129 | 0.260 | 2.25 | 543.9 |
| controller/move/ms | 0.128 | 0.208 | 0.50 | 540.7 |
| bigspace/high_precision/ms | 0.115 | 0.248 | 5.14 | 484.1 |
| render/acquire/ms | 0.091 | 0.117 | 0.33 | 383.6 |
| physics/insert/ms | 0.069 | 0.472 | 1.06 | 290.8 |
| physics/step/ms | 0.064 | 0.091 | 0.59 | 271.4 |
| weather/map/ms | 0.024 | 0.032 | 0.09 | 100.4 |
| pipelines/driver_ms | 0.019 | 0.000 | 5.73 | 81.4 |
| anim/apply/ms | 0.011 | 0.020 | 0.32 | 46.7 |
| controller/avoid/ms | 0.010 | 0.014 | 0.42 | 43.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.22 | 43.1 |
| stream/despawn/ms | 0.007 | 0.000 | 0.95 | 28.9 |
| script/update/ms | 0.007 | 0.009 | 0.16 | 28.7 |
| stream/spawn/ms | 0.006 | 0.000 | 1.13 | 26.0 |
| script/ui/ms | 0.005 | 0.006 | 0.29 | 21.3 |
| script/input/ms | 0.005 | 0.006 | 0.10 | 19.3 |
| controller/write/ms | 0.003 | 0.004 | 0.10 | 14.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.67 | 14.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 5.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 13 | 0 |
| anim/a2/count | count | 1 | 12 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 86 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 26 | 49 | 12 |
| crowd/c0/joints/count | count | 1291 | 2262 | 548 |
| crowd/c0/meshes/count | count | 353 | 646 | 134 |
| crowd/c1/count | count | 146 | 157 | 149 |
| crowd/c1/joints/count | count | 5531 | 6046 | 5648 |
| crowd/c1/meshes/count | count | 1167 | 1322 | 1207 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4266 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4243 | 6643 | 6179 |
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
| frame/work/ms | ms | 13.33 | 45.45 | 12.17 |
| gpu/timed/ms | ms | 8.87 | 12.03 | 8.61 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 11 | 8 |
| phases/alpha_mask3d/bins | count | 248 | 479 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3911 | 4344 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 11 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 248 | 479 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3911 | 4344 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 36 | 41 | 35 |
| phases/opaque3d/bins | count | 4106 | 4327 | 4326 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 36 | 41 | 35 |
| phases/opaque3d_prepass/bins | count | 4106 | 4327 | 4326 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 87 | 352 | 70 |
| phases/shadow/bins | count | 12.0 k | 15.5 k | 11.9 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 38.9 k | 42.1 k | 39.3 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 149 | 335 | 51 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2347 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 443 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 29 | 0 |
| preprocess/bind_groups | count | 19 | 953 | 6 |
| preprocess/bind_groups_created | count | 19 | 953 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 344 | 501 | 329 |
| process/cpu_usage | % | 20.90 | 48.33 | 13.40 |
| process/mem_usage | GiB | 5.19 | 5.37 | 5.37 |
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
| started_load_count |  | 4766 | 4860 | 4860 |
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
| stream/latency/s | s | 0.04 | 0.11 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.36 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 26.37 | 41.02 | 25.91 |
| system/mem_usage | % | 65.88 | 66.66 | 66.55 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2591 | 2644 | 2644 |
| views/all/batch_sets | count | 181 | 444 | 156 |
| views/all/bins | count | 21.3 k | 24.1 k | 21.5 k |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 67.5 k | 70.5 k | 68.9 k |
| views/all/pipelines | count | 215 | 403 | 117 |
| views/all/pipelines_distinct | count | 99 | 112 | 91 |
| views/all/work_items | count | 66.8 k | 70.2 k | 69.6 k |
| views/cascade/batch_sets | count | 83 | 102 | 70 |
| views/cascade/bins | count | 11.8 k | 12.4 k | 11.9 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 38.3 k | 39.8 k | 39.3 k |
| views/cascade/pipelines | count | 62 | 78 | 51 |
| views/cascade/work_items | count | 38.3 k | 39.8 k | 39.3 k |
| views/lamp_face/batch_sets | count | 0 | 262 | 0 |
| views/lamp_face/bins | count | 1649 | 3711 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1781 | 4069 | 0 |
| views/lamp_face/pipelines | count | 85 | 260 | 0 |
| views/lamp_face/work_items | count | 0 | 3937 | 0 |
| views/main/batch_sets | count | 90 | 102 | 86 |
| views/main/bins | count | 8674 | 9594 | 9594 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 66 | 72 | 66 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.7 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 53.94 | 638 | 227.6 k |
| anim/events/count | count | 0.13 | 3 | 536 |
| assets/Image/added | count | 0.14 | 43 | 608 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.98 | 168 | 4127 |
| assets/Mesh/removed | count | 1.16 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 482 |
| assets/StandardMaterial/modified | count | 5.14 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.10 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.22 | 2886 | 9380 |
| physics/inserts | count | 0.64 | 111 | 2688 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.55 | 129 | 2335 |
| physics/steps | count | 0.93 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.09 | 18 | 378 |
| stream/upload/bytes | bytes | 8024 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 55 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1759 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 37.9 ms; slowest 17, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2510.9 | main/busy/ms 2485, surface/advance_cpu_ms 67.73, surface/cell_upload_ms 0.69, stream/drive/ms 0.30, bigspace/total/ms 0.12, weather/map/ms 0.12, stream/tick/ms 0.07, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 3.25 | 740.1 | main/busy/ms 2522, render/thread/ms 701, render/busy/ms 548, render/acquire/ms 150, surface/advance_cpu_ms 67.73, render/graph/ms 6.31, render/present/ms 2.82, stream/drive/ms 1.24, terrain/update/ms 0.79, surface/cell_upload_ms 0.69, stream/tick/ms 0.62, terrain/select/ms 0.56 |
| warm-up | 3.45 | 198.0 | pipelines/wait_ms 2502, render/thread/ms 713, render/busy/ms 559, pipelines/compile_ms 212, pipelines/front_ms 198, render/acquire/ms 150, main/busy/ms 52.46, terrain/update/ms 21.52, pipelines/driver_ms 13.94, render/graph/ms 10.13, render/present/ms 4.07, stream/drive/ms 1.60 |
| warm-up | 3.75 | 91.3 | pipelines/compile_ms 230, pipelines/front_ms 219, pipelines/wait_ms 218, main/busy/ms 68.14, render/thread/ms 45.76, render/busy/ms 44.40, pipelines/driver_ms 10.78, terrain/update/ms 5.42, render/graph/ms 3.47, stream/drive/ms 1.77, stream/tick/ms 1.33, render/present/ms 1.11 |
| warm-up | 3.85 | 50.6 | pipelines/wait_ms 219, pipelines/compile_ms 170, pipelines/front_ms 158, render/thread/ms 38.38, render/busy/ms 37.14, main/busy/ms 36.03, stream/drive/ms 23.90, pipelines/driver_ms 11.92, render/graph/ms 3.47, render/present/ms 1.03, stream/tick/ms 0.75, terrain/update/ms 0.61 |
| warm-up | 3.99 | 60.1 | pipelines/wait_ms 418, render/thread/ms 58.97, pipelines/compile_ms 58.11, pipelines/front_ms 56.54, render/busy/ms 55.89, main/busy/ms 42.43, stream/drive/ms 21.90, render/graph/ms 5.62, render/present/ms 2.76, stream/tick/ms 1.72, pipelines/driver_ms 1.57, stream/spawn/ms 1.31 |
| warm-up | 4.03 | 42.1 | pipelines/wait_ms 418, render/thread/ms 66.95, render/busy/ms 64.69, main/busy/ms 59.75, pipelines/compile_ms 58.11, pipelines/front_ms 56.54, stream/drive/ms 34.03, render/graph/ms 6.48, stream/spawn/ms 2.26, render/present/ms 1.94, stream/tick/ms 1.86, pipelines/driver_ms 1.57 |
| warm-up | 4.14 | 105.8 | pipelines/wait_ms 4047, pipelines/compile_ms 231, pipelines/front_ms 183, render/thread/ms 87.91, render/busy/ms 64.66, main/busy/ms 62.88, pipelines/driver_ms 48.26, stream/drive/ms 36.67, render/present/ms 23.03, stream/spawn/ms 5.63, render/graph/ms 4.49, stream/tick/ms 1.32 |
| warm-up | 4.32 | 40.0 | main/busy/ms 66.35, stream/drive/ms 49.29, render/thread/ms 37.06, render/busy/ms 35.49, render/graph/ms 4.56, stream/spawn/ms 2.14, stream/tick/ms 1.38, render/present/ms 1.33, terrain/update/ms 1.00, terrain/select/ms 0.86, bigspace/total/ms 0.78, render/acquire/ms 0.24 |
| warm-up | 4.39 | 74.3 | pipelines/wait_ms 3155, pipelines/compile_ms 172, pipelines/front_ms 167, main/busy/ms 70.63, stream/drive/ms 49.24, render/thread/ms 47.53, render/busy/ms 46.37, pipelines/driver_ms 5.77, render/graph/ms 3.17, stream/spawn/ms 2.44, terrain/update/ms 1.02, render/present/ms 0.98 |
| warm-up | 4.48 | 64.0 | pipelines/wait_ms 10.7 k, pipelines/compile_ms 95.48, pipelines/front_ms 86.48, main/busy/ms 62.13, stream/drive/ms 43.70, render/thread/ms 41.58, render/busy/ms 40.23, pipelines/driver_ms 9.00, render/graph/ms 4.65, stream/spawn/ms 1.66, render/present/ms 1.13, stream/tick/ms 0.81 |
| warm-up | 4.54 | 40.5 | main/busy/ms 44.45, render/thread/ms 33.26, stream/drive/ms 32.89, render/busy/ms 31.72, render/graph/ms 6.93, render/present/ms 1.23, stream/activate/ms 1.07, stream/tick/ms 0.80, stream/spawn/ms 0.79, terrain/update/ms 0.70, terrain/select/ms 0.58, physics/insert/ms 0.51 |
| warm-up | 5.64 | 39.8 | pipelines/wait_ms 16.7 k, pipelines/compile_ms 146, pipelines/front_ms 138, render/thread/ms 56.48, render/busy/ms 54.52, main/busy/ms 50.98, render/graph/ms 9.37, pipelines/driver_ms 7.99, stream/drive/ms 6.11, stream/spawn/ms 2.51, render/present/ms 1.65, stream/tick/ms 0.69 |
| warm-up | 5.68 | 42.8 | pipelines/wait_ms 11.2 k, render/thread/ms 59.15, pipelines/compile_ms 58.91, render/busy/ms 57.24, pipelines/front_ms 55.67, main/busy/ms 35.47, render/graph/ms 7.55, stream/drive/ms 3.38, pipelines/driver_ms 3.24, stream/spawn/ms 2.03, render/present/ms 1.65, bigspace/total/ms 0.92 |
| measure | 3.04 | 38.1 | main/busy/ms 46.26, render/thread/ms 29.47, render/busy/ms 28.22, render/graph/ms 16.88, render/present/ms 1.01, physics/insert/ms 0.77, bigspace/total/ms 0.71, controller/feed/ms 0.60, terrain/update/ms 0.55, terrain/select/ms 0.50, stream/drive/ms 0.49, stream/tick/ms 0.41 |
| measure | 17.99 | 46.3 | main/busy/ms 57.94, render/thread/ms 27.75, render/busy/ms 26.68, render/graph/ms 15.38, render/present/ms 0.90, controller/feed/ms 0.70, bigspace/total/ms 0.69, physics/step/ms 0.59, terrain/update/ms 0.56, physics/insert/ms 0.51, terrain/select/ms 0.50, stream/drive/ms 0.44 |
| measure | 59.07 | 38.4 | render/thread/ms 47.25, render/busy/ms 46.06, main/busy/ms 19.66, render/graph/ms 15.68, render/present/ms 1.00, controller/feed/ms 0.80, stream/drive/ms 0.77, terrain/update/ms 0.73, stream/tick/ms 0.68, bigspace/total/ms 0.68, terrain/select/ms 0.66, far/select/ms 0.58 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_new\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 12%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **70%** (min 50%) · 2580 MHz (min 2580) · memory 8001 MHz · 67.7 W · 72 °C (max 77) · VRAM 5653 MiB · throttled 100% of samples
