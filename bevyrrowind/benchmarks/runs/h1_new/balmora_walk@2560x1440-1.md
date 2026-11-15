# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3903 frames over 65.7 s · **median 16.32 ms (61 fps)** · mean 16.82 · p95 20.15 · p99 23.16 (1% low 43 fps) · worst 40.35 ms · 26 spikes over 1.5× median · 21.9 stutters/min (work 17.4/min)
- warm-up: 335 frames over 8.5 s, worst 2715.2 ms
- hitches (> 40.8 ms): **0** measured, 14 in warm-up
- timed GPU passes: 8.13 ms of a 16.82 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 16.32 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.93/min, limit 1/min
- **FAIL** worst frame 40.35 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.181 | 2.183 | 2.546 | 1.00 | 0.186 |
| main_opaque_pass_3d | 1.705 | 1.705 | 2.343 | 1.00 | 0.089 |
| early prepass | 0.536 | 0.536 | 0.786 | 1.00 | 0.131 |
| uniform_allocation | 0.399 | 0.402 | 0.690 | 0.99 | 0.063 |
| bloom | 0.335 | 0.335 | 0.481 | 1.00 | 0.069 |
| atmosphere_luts | 0.327 | 0.332 | 0.625 | 0.99 | 0.004 |
| water_velocity | 0.301 | 0.302 | 0.489 | 1.00 | 0.002 |
| bin_unpacking | 0.300 | 0.300 | 0.324 | 1.00 | 0.044 |
| water_sspr | 0.228 | 0.228 | 0.228 | 1.00 | 0.003 |
| ssao | 0.203 | 0.203 | 0.208 | 1.00 | 0.008 |
| auxiliary_views | 0.194 | 0.194 | 0.738 | 1.00 | 0.188 |
| atmos_froxel_inject | 0.156 | 0.157 | 0.153 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.142 | 0.239 | 1.00 | 0.001 |
| water_fft | 0.135 | 0.136 | 0.138 | 0.99 | 0.003 |
| upscaling | 0.127 | 0.127 | 0.154 | 1.00 | 0.004 |
| clouds_composite | 0.110 | 0.110 | 0.111 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.094 | 0.094 | 0.175 | 1.00 | 0.009 |
| tonemapping | 0.092 | 0.092 | 0.094 | 1.00 | 0.004 |
| clustering | 0.068 | 0.068 | 0.069 | 1.00 | 0.021 |
| surface_trail | 0.058 | 0.060 | 0.057 | 0.97 | 0.002 |
| early_mesh_preprocessing | 0.056 | 0.056 | 0.061 | 1.00 | 0.006 |
| render_sky | 0.055 | 0.055 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.055 | 0.443 | 0.729 | 0.13 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| water_snapshot | 0.037 | 0.037 | 0.047 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.226 | 0.234 | 0.13 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.031 | 1.00 | 0.001 |
| water_mips | 0.023 | 0.023 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.018 | 0.018 | 0.031 | 1.00 | 0.152 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.019 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.016 | 0.015 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.03 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 99.032 | 0.000 | 15893.98 | 386520.2 |
| render/thread/ms | 15.482 | 18.581 | 34.71 | 60425.6 |
| render/busy/ms | 14.887 | 17.950 | 33.92 | 58103.7 |
| main/busy/ms | 9.556 | 13.256 | 30.21 | 37295.6 |
| render/graph/ms | 9.429 | 11.232 | 20.82 | 36802.8 |
| pipelines/compile_ms | 0.969 | 0.000 | 182.13 | 3780.1 |
| pipelines/front_ms | 0.908 | 0.000 | 158.71 | 3544.5 |
| render/present/ms | 0.502 | 0.608 | 15.99 | 1960.2 |
| bigspace/total/ms | 0.446 | 0.720 | 4.30 | 1740.4 |
| terrain/update/ms | 0.292 | 0.365 | 2.07 | 1138.5 |
| controller/feed/ms | 0.288 | 0.380 | 0.88 | 1122.4 |
| stream/drive/ms | 0.260 | 0.379 | 22.38 | 1014.8 |
| terrain/select/ms | 0.258 | 0.324 | 1.05 | 1007.3 |
| far/select/ms | 0.197 | 0.274 | 0.59 | 770.0 |
| stream/tick/ms | 0.190 | 0.255 | 0.64 | 740.3 |
| anim/evaluate/ms | 0.181 | 0.285 | 3.70 | 706.8 |
| bigspace/low_precision/ms | 0.173 | 0.325 | 3.92 | 677.0 |
| anim/evaluate_cpu/ms | 0.141 | 0.184 | 0.48 | 550.9 |
| controller/move/ms | 0.137 | 0.258 | 0.44 | 534.3 |
| bigspace/high_precision/ms | 0.112 | 0.233 | 1.15 | 439.1 |
| render/acquire/ms | 0.093 | 0.121 | 0.38 | 361.7 |
| physics/step/ms | 0.073 | 0.111 | 0.54 | 284.6 |
| physics/insert/ms | 0.071 | 0.479 | 0.78 | 276.3 |
| pipelines/driver_ms | 0.060 | 0.000 | 54.97 | 235.5 |
| weather/map/ms | 0.024 | 0.033 | 0.22 | 93.7 |
| anim/apply/ms | 0.011 | 0.021 | 0.20 | 43.5 |
| controller/avoid/ms | 0.010 | 0.014 | 0.17 | 40.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.19 | 40.5 |
| stream/despawn/ms | 0.008 | 0.000 | 1.12 | 30.1 |
| script/update/ms | 0.007 | 0.009 | 0.18 | 28.9 |
| stream/spawn/ms | 0.007 | 0.000 | 1.12 | 25.5 |
| script/input/ms | 0.005 | 0.007 | 0.05 | 20.1 |
| script/ui/ms | 0.005 | 0.006 | 0.15 | 19.1 |
| stream/deactivate/ms | 0.004 | 0.000 | 0.62 | 14.4 |
| controller/write/ms | 0.003 | 0.004 | 0.16 | 13.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.42 | 4.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 3.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 2.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 27 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 91 | 18 |
| controller/movers/count | count | 20 | 21 | 18 |
| crowd/c0/count | count | 27 | 58 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 528 |
| crowd/c0/meshes/count | count | 355 | 646 | 135 |
| crowd/c1/count | count | 146 | 155 | 149 |
| crowd/c1/joints/count | count | 5531 | 5937 | 5668 |
| crowd/c1/meshes/count | count | 1171 | 1338 | 1206 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 10.0 k | 6179 |
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
| frame/work/ms | ms | 14.48 | 33.92 | 14.11 |
| gpu/timed/ms | ms | 8.85 | 12.13 | 9.08 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 602.13 M | 603.31 M | 603.31 M |
| phases/alpha_mask3d/batch_sets | count | 10 | 12 | 9 |
| phases/alpha_mask3d/bins | count | 245 | 478 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3887 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 10 | 12 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 245 | 478 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3887 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 40 | 43 | 41 |
| phases/opaque3d/bins | count | 4093 | 4327 | 4326 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 40 | 43 | 41 |
| phases/opaque3d_prepass/bins | count | 4093 | 4327 | 4326 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 88 | 360 | 76 |
| phases/shadow/bins | count | 12.0 k | 16.0 k | 11.9 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 38.9 k | 43.0 k | 39.3 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 154 | 349 | 51 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2347 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 1380 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 80 | 0 |
| preprocess/bind_groups | count | 27 | 989 | 10 |
| preprocess/bind_groups_created | count | 27 | 989 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 367 | 494 | 365 |
| process/cpu_usage | % | 21.03 | 43.27 | 20 |
| process/mem_usage | GiB | 5.18 | 5.36 | 5.36 |
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
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.05 | 0.13 | 0.05 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.40 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 25.89 | 49.21 | 24.62 |
| system/mem_usage | % | 59.88 | 60.67 | 60.56 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 189 | 462 | 176 |
| views/all/bins | count | 21.3 k | 24.5 k | 21.5 k |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 67.5 k | 71.3 k | 68.9 k |
| views/all/pipelines | count | 218 | 411 | 117 |
| views/all/pipelines_distinct | count | 100 | 110 | 91 |
| views/all/work_items | count | 66.8 k | 69.9 k | 69.6 k |
| views/cascade/batch_sets | count | 85 | 103 | 76 |
| views/cascade/bins | count | 11.8 k | 12.5 k | 11.9 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 38.3 k | 39.9 k | 39.3 k |
| views/cascade/pipelines | count | 62 | 79 | 51 |
| views/cascade/work_items | count | 38.3 k | 39.9 k | 39.3 k |
| views/lamp_face/batch_sets | count | 0 | 268 | 0 |
| views/lamp_face/bins | count | 1668 | 3966 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1819 | 4340 | 0 |
| views/lamp_face/pipelines | count | 90 | 270 | 0 |
| views/lamp_face/work_items | count | 0 | 3984 | 0 |
| views/main/batch_sets | count | 100 | 108 | 100 |
| views/main/bins | count | 8662 | 9594 | 9594 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 66 | 70 | 66 |
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
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 52.66 | 926 | 205.5 k |
| anim/events/count | count | 0.13 | 3 | 512 |
| assets/Image/added | count | 0.16 | 33 | 641 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 1.08 | 152 | 4225 |
| assets/Mesh/removed | count | 1.25 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.13 | 35 | 517 |
| assets/StandardMaterial/modified | count | 5.55 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.10 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.27 | 4145 | 12.8 k |
| physics/inserts | count | 0.92 | 158 | 3589 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.60 | 145 | 2335 |
| physics/steps | count | 1.01 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.06 | 21 | 219 |
| stream/deactivate/items | count | 0.06 | 21 | 223 |
| stream/despawn/items | count | 0.06 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.10 | 22 | 378 |
| stream/upload/bytes | bytes | 8673 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1902 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 40.8 ms; slowest 14, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.72 | 2715.2 | main/busy/ms 2690, surface/advance_cpu_ms 71.39, surface/cell_upload_ms 0.79, stream/drive/ms 0.27, weather/map/ms 0.11, bigspace/total/ms 0.08, stream/tick/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 3.34 | 628.0 | main/busy/ms 2724, render/thread/ms 591, render/busy/ms 406, render/acquire/ms 180, surface/advance_cpu_ms 71.39, render/graph/ms 4.52, render/present/ms 4.33, stream/drive/ms 1.01, surface/cell_upload_ms 0.79, stream/tick/ms 0.54, terrain/update/ms 0.40, bigspace/total/ms 0.27 |
| warm-up | 3.54 | 199.5 | render/thread/ms 616, pipelines/wait_ms 567, render/busy/ms 427, pipelines/compile_ms 214, pipelines/front_ms 199, render/acquire/ms 181, main/busy/ms 52.52, terrain/update/ms 18.19, pipelines/driver_ms 14.78, render/graph/ms 9.45, render/present/ms 7.91, stream/drive/ms 1.55 |
| warm-up | 3.79 | 76.7 | pipelines/compile_ms 238, pipelines/front_ms 228, pipelines/wait_ms 226, render/thread/ms 80.27, main/busy/ms 63.26, render/busy/ms 60.17, render/acquire/ms 18.73, pipelines/driver_ms 9.89, render/graph/ms 4.05, stream/drive/ms 2.15, stream/tick/ms 1.91, render/present/ms 1.37 |
| warm-up | 3.94 | 61.6 | main/busy/ms 47.83, render/thread/ms 47.04, render/busy/ms 45.85, stream/drive/ms 32.99, render/graph/ms 3.22, stream/tick/ms 1.36, render/present/ms 0.91, terrain/update/ms 0.88, terrain/select/ms 0.70, stream/spawn/ms 0.64, bigspace/total/ms 0.31, render/acquire/ms 0.28 |
| warm-up | 4.08 | 69.3 | pipelines/wait_ms 3295, pipelines/compile_ms 130, pipelines/front_ms 122, render/thread/ms 53.85, render/busy/ms 52.36, main/busy/ms 45.03, stream/drive/ms 23.21, pipelines/driver_ms 8.30, render/graph/ms 3.11, stream/tick/ms 2.24, render/present/ms 1.27, stream/spawn/ms 1.20 |
| warm-up | 4.12 | 41.7 | pipelines/wait_ms 3391, pipelines/compile_ms 98.96, pipelines/front_ms 92.49, render/thread/ms 66.73, render/busy/ms 65.50, main/busy/ms 59.37, stream/drive/ms 33.54, pipelines/driver_ms 6.46, render/graph/ms 2.79, stream/spawn/ms 2.22, stream/tick/ms 2.14, render/present/ms 1.01 |
| warm-up | 4.21 | 95.5 | pipelines/wait_ms 1789, render/thread/ms 98.75, render/busy/ms 58.95, main/busy/ms 55.78, render/present/ms 39.58, pipelines/compile_ms 35.86, pipelines/front_ms 33.99, stream/drive/ms 32.56, render/graph/ms 3.79, stream/spawn/ms 2.28, pipelines/driver_ms 1.88, stream/tick/ms 1.44 |
| warm-up | 4.28 | 42.3 | pipelines/wait_ms 4116, pipelines/compile_ms 180, pipelines/front_ms 172, render/thread/ms 53.28, render/busy/ms 51.34, main/busy/ms 25.24, render/graph/ms 10.75, pipelines/driver_ms 8.10, stream/drive/ms 3.78, render/present/ms 1.68, stream/tick/ms 1.56, terrain/update/ms 1.14 |
| warm-up | 4.50 | 89.0 | pipelines/wait_ms 2289, pipelines/compile_ms 147, pipelines/front_ms 139, render/thread/ms 72.21, main/busy/ms 65.28, render/busy/ms 46.40, stream/drive/ms 45.26, render/present/ms 25.62, pipelines/driver_ms 8.00, render/graph/ms 3.52, stream/spawn/ms 1.36, stream/tick/ms 0.85 |
| warm-up | 4.59 | 68.9 | pipelines/wait_ms 6112, pipelines/compile_ms 147, pipelines/front_ms 137, main/busy/ms 64.51, stream/drive/ms 48.49, render/thread/ms 40.17, render/busy/ms 38.40, pipelines/driver_ms 10.04, render/graph/ms 8.68, stream/spawn/ms 2.08, render/present/ms 1.59, stream/tick/ms 0.77 |
| warm-up | 4.65 | 42.0 | pipelines/wait_ms 4682, main/busy/ms 50.75, render/thread/ms 39.55, render/busy/ms 38.07, stream/drive/ms 34.06, pipelines/compile_ms 31.14, pipelines/front_ms 28.03, render/graph/ms 4.73, pipelines/driver_ms 3.12, render/present/ms 1.18, stream/spawn/ms 1.11, stream/activate/ms 1.05 |
| warm-up | 5.02 | 49.1 | pipelines/wait_ms 6102, pipelines/compile_ms 272, pipelines/front_ms 252, render/thread/ms 53.96, render/busy/ms 52.50, main/busy/ms 48.07, pipelines/driver_ms 19.95, stream/drive/ms 6.16, render/graph/ms 5.95, render/present/ms 1.13, stream/tick/ms 0.89, terrain/update/ms 0.72 |
| warm-up | 5.99 | 43.7 | pipelines/wait_ms 2336, pipelines/compile_ms 76.38, pipelines/front_ms 73.25, render/thread/ms 53.34, render/busy/ms 51.71, main/busy/ms 41.89, stream/drive/ms 15.74, render/graph/ms 6.56, pipelines/driver_ms 3.13, stream/spawn/ms 2.13, render/present/ms 1.31, stream/tick/ms 1.22 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_new\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 13%, CPU 12%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **64%** (min 45%) · 2580 MHz (min 2580) · memory 8001 MHz · 63.8 W · 71 °C (max 75) · VRAM 5655 MiB · throttled 100% of samples
