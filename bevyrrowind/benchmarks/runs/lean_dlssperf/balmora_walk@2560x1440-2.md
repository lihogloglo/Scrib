# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=performance · resolution: 2560x1440 · rings: near None far None
- 4612 frames over 65.6 s · **median 13.76 ms (73 fps)** · mean 14.23 · p95 17.70 · p99 20.16 (1% low 50 fps) · worst 34.70 ms · 39 spikes over 1.5× median · 18.3 stutters/min (work 18.3/min)
- warm-up: 349 frames over 8.6 s, worst 2661.2 ms
- hitches (> 34.4 ms): **1** measured, 16 in warm-up
- timed GPU passes: 8.32 ms of a 14.23 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.76 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- **FAIL** worst frame 34.70 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.746 | 2.748 | 2.767 | 1.00 | 0.186 |
| main_opaque_pass_3d | 1.639 | 1.639 | 2.022 | 1.00 | 0.071 |
| early prepass | 0.514 | 0.514 | 0.730 | 1.00 | 0.122 |
| atmosphere_luts | 0.314 | 0.314 | 0.372 | 1.00 | 0.003 |
| uniform_allocation | 0.313 | 0.313 | 0.331 | 1.00 | 0.056 |
| bloom | 0.312 | 0.312 | 0.315 | 1.00 | 0.068 |
| water_velocity | 0.283 | 0.284 | 0.353 | 1.00 | 0.002 |
| bin_unpacking | 0.244 | 0.244 | 0.268 | 1.00 | 0.039 |
| water_sspr | 0.210 | 0.210 | 0.211 | 1.00 | 0.002 |
| auxiliary_views | 0.202 | 0.202 | 0.742 | 1.00 | 0.187 |
| ssao | 0.193 | 0.193 | 0.194 | 1.00 | 0.008 |
| atmos_froxel_inject | 0.149 | 0.149 | 0.150 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.141 | 0.228 | 1.00 | 0.001 |
| water_fft | 0.133 | 0.133 | 0.137 | 1.00 | 0.003 |
| upscaling | 0.123 | 0.123 | 0.130 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.110 | 1.00 | 0.002 |
| tonemapping | 0.088 | 0.088 | 0.091 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.087 | 0.087 | 0.165 | 1.00 | 0.008 |
| clustering | 0.065 | 0.065 | 0.068 | 1.00 | 0.020 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.418 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.050 | 0.057 | 0.057 | 0.87 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.048 | 1.00 | 0.005 |
| water_snapshot | 0.031 | 0.032 | 0.035 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.224 | 0.233 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.029 | 1.00 | 0.001 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 1.00 | 0.108 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 38.497 | 0.000 | 12579.14 | 177548.5 |
| render/thread/ms | 13.011 | 16.367 | 33.19 | 60009.0 |
| render/busy/ms | 12.453 | 15.729 | 32.34 | 57432.2 |
| main/busy/ms | 9.561 | 12.571 | 27.64 | 44095.4 |
| render/graph/ms | 7.763 | 9.839 | 19.73 | 35803.7 |
| pipelines/compile_ms | 0.651 | 0.000 | 146.14 | 3002.3 |
| pipelines/front_ms | 0.629 | 0.000 | 139.85 | 2898.7 |
| render/present/ms | 0.466 | 0.554 | 2.47 | 2150.5 |
| bigspace/total/ms | 0.436 | 0.674 | 4.73 | 2013.0 |
| terrain/update/ms | 0.290 | 0.364 | 1.92 | 1338.4 |
| controller/feed/ms | 0.269 | 0.369 | 0.77 | 1239.1 |
| terrain/select/ms | 0.258 | 0.324 | 0.53 | 1188.1 |
| stream/drive/ms | 0.254 | 0.368 | 19.23 | 1170.2 |
| stream/tick/ms | 0.192 | 0.272 | 0.68 | 883.5 |
| far/select/ms | 0.191 | 0.266 | 0.76 | 882.0 |
| bigspace/low_precision/ms | 0.162 | 0.274 | 4.31 | 748.3 |
| anim/evaluate/ms | 0.159 | 0.314 | 2.64 | 732.0 |
| anim/evaluate_cpu/ms | 0.152 | 0.195 | 0.42 | 700.9 |
| controller/move/ms | 0.144 | 0.304 | 0.63 | 664.4 |
| bigspace/high_precision/ms | 0.106 | 0.193 | 4.21 | 487.1 |
| render/acquire/ms | 0.092 | 0.121 | 0.35 | 426.3 |
| physics/insert/ms | 0.065 | 0.470 | 0.92 | 298.2 |
| physics/step/ms | 0.061 | 0.091 | 0.57 | 279.5 |
| weather/map/ms | 0.024 | 0.034 | 0.06 | 111.7 |
| pipelines/driver_ms | 0.022 | 0.000 | 6.29 | 103.5 |
| anim/apply/ms | 0.011 | 0.021 | 0.19 | 52.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.22 | 47.5 |
| controller/avoid/ms | 0.010 | 0.013 | 0.18 | 45.5 |
| script/update/ms | 0.007 | 0.010 | 0.17 | 32.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.00 | 29.3 |
| stream/spawn/ms | 0.006 | 0.000 | 1.09 | 25.5 |
| script/input/ms | 0.005 | 0.006 | 0.12 | 21.6 |
| script/ui/ms | 0.005 | 0.006 | 0.11 | 21.6 |
| controller/write/ms | 0.004 | 0.004 | 0.02 | 16.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 14.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.47 | 5.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.16 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 29 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1272 | 2262 | 535 |
| crowd/c0/meshes/count | count | 349 | 646 | 144 |
| crowd/c1/count | count | 146 | 157 | 149 |
| crowd/c1/joints/count | count | 5531 | 6046 | 5661 |
| crowd/c1/meshes/count | count | 1181 | 1322 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 222 |
| crowd/joints_extracted/count | count | 4262 | 9221 | 6135 |
| crowd/joints_uploaded/count | count | 4237 | 6643 | 6135 |
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
| frame/work/ms | ms | 12.05 | 32.34 | 11.18 |
| gpu/timed/ms | ms | 9.01 | 12.30 | 9.21 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 802 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 802 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 36 | 41 | 36 |
| phases/opaque3d/bins | count | 1291 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3368 | 5123 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 36 | 41 | 36 |
| phases/opaque3d_prepass/bins | count | 1291 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3368 | 5123 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 76 | 329 | 61 |
| phases/shadow/bins | count | 3793 | 5331 | 2207 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9606 | 12.3 k | 5658 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 145 | 325 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2347 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 855 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 35 | 0 |
| preprocess/bind_groups | count | 13 | 903 | 6 |
| preprocess/bind_groups_created | count | 13 | 903 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 317 | 451 | 311 |
| process/cpu_usage | % | 22.71 | 43.55 | 20.19 |
| process/mem_usage | GiB | 5.18 | 5.35 | 5.35 |
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
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.36 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.81 | 62.70 | 25.59 |
| system/mem_usage | % | 75.12 | 75.69 | 75.56 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 164 | 419 | 147 |
| views/all/bins | count | 6498 | 8203 | 5687 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.0 k | 25.1 k | 13.3 k |
| views/all/pipelines | count | 209 | 385 | 113 |
| views/all/pipelines_distinct | count | 99 | 109 | 91 |
| views/all/work_items | count | 17.1 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 69 | 85 | 61 |
| views/cascade/bins | count | 2602 | 3176 | 2207 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8138 | 10.0 k | 5658 |
| views/cascade/pipelines | count | 57 | 72 | 47 |
| views/cascade/work_items | count | 8138 | 10.0 k | 5658 |
| views/lamp_face/batch_sets | count | 3 | 257 | 0 |
| views/lamp_face/bins | count | 1101 | 2516 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1227 | 2790 | 0 |
| views/lamp_face/pipelines | count | 86 | 264 | 0 |
| views/lamp_face/work_items | count | 38 | 2701 | 0 |
| views/main/batch_sets | count | 88 | 98 | 86 |
| views/main/bins | count | 2876 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8352 | 13.0 k | 7594 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8517 | 13.2 k | 7902 |
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
| anim/bones_written/count | count | 52.87 | 244 | 243.9 k |
| anim/events/count | count | 0.12 | 3 | 541 |
| assets/Image/added | count | 0.13 | 43 | 613 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.90 | 168 | 4143 |
| assets/Mesh/removed | count | 1.06 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 487 |
| assets/StandardMaterial/modified | count | 4.69 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.09 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.50 | 1485 | 6913 |
| physics/inserts | count | 0.66 | 119 | 3041 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.51 | 145 | 2335 |
| physics/steps | count | 0.85 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 19 | 378 |
| stream/upload/bytes | bytes | 7340 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1609 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 34.4 ms; slowest 17, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.66 | 2661.2 | main/busy/ms 2635, surface/advance_cpu_ms 69.93, surface/cell_upload_ms 0.78, stream/drive/ms 0.34, weather/map/ms 0.16, bigspace/total/ms 0.09, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.19 | 533.2 | main/busy/ms 2675, render/thread/ms 490, render/busy/ms 407, render/acquire/ms 81.16, surface/advance_cpu_ms 69.93, render/graph/ms 5.50, render/present/ms 2.03, stream/drive/ms 1.12, surface/cell_upload_ms 0.78, stream/tick/ms 0.52, terrain/update/ms 0.51, bigspace/total/ms 0.42 |
| warm-up | 3.41 | 210.8 | pipelines/wait_ms 1099, render/thread/ms 500, render/busy/ms 416, pipelines/compile_ms 219, pipelines/front_ms 205, render/acquire/ms 81.33, main/busy/ms 56.53, terrain/update/ms 23.21, pipelines/driver_ms 14.01, render/graph/ms 9.02, render/present/ms 3.12, stream/drive/ms 1.35 |
| warm-up | 3.62 | 81.8 | pipelines/compile_ms 218, pipelines/front_ms 208, main/busy/ms 55.94, render/thread/ms 44.53, render/busy/ms 42.69, pipelines/driver_ms 10.11, render/graph/ms 4.23, stream/drive/ms 1.67, render/present/ms 1.48, stream/tick/ms 1.32, terrain/update/ms 0.82, terrain/select/ms 0.69 |
| warm-up | 3.77 | 71.8 | main/busy/ms 46.42, render/thread/ms 41.47, render/busy/ms 40.07, stream/drive/ms 29.83, render/graph/ms 4.17, render/present/ms 1.22, stream/tick/ms 0.97, stream/spawn/ms 0.70, terrain/update/ms 0.68, terrain/select/ms 0.56, bigspace/total/ms 0.27, render/acquire/ms 0.18 |
| warm-up | 3.86 | 84.0 | pipelines/wait_ms 2337, pipelines/compile_ms 237, pipelines/front_ms 214, render/thread/ms 55.98, render/busy/ms 54.71, main/busy/ms 44.06, stream/drive/ms 27.77, pipelines/driver_ms 23.82, render/graph/ms 3.33, stream/tick/ms 1.77, stream/spawn/ms 1.18, render/present/ms 1.10 |
| warm-up | 3.97 | 74.6 | pipelines/wait_ms 4105, render/thread/ms 72.11, render/busy/ms 70.34, pipelines/compile_ms 61.53, pipelines/front_ms 54.13, render/graph/ms 30.37, main/busy/ms 29.63, pipelines/driver_ms 7.40, stream/drive/ms 5.09, stream/spawn/ms 2.12, render/present/ms 1.41, terrain/update/ms 1.20 |
| warm-up | 4.01 | 41.9 | pipelines/wait_ms 1905, render/thread/ms 86.95, render/busy/ms 85.53, pipelines/compile_ms 39.36, pipelines/front_ms 36.71, render/graph/ms 30.82, main/busy/ms 30.51, terrain/update/ms 4.90, stream/drive/ms 4.35, pipelines/driver_ms 2.65, stream/spawn/ms 2.18, render/present/ms 1.19 |
| warm-up | 4.05 | 45.1 | pipelines/wait_ms 920, pipelines/compile_ms 134, pipelines/front_ms 127, render/thread/ms 67.62, render/busy/ms 66.48, main/busy/ms 28.00, pipelines/driver_ms 6.55, stream/drive/ms 4.84, terrain/update/ms 4.73, render/graph/ms 4.35, stream/spawn/ms 2.50, render/present/ms 0.96 |
| warm-up | 4.14 | 65.5 | pipelines/wait_ms 3711, pipelines/compile_ms 61.95, pipelines/front_ms 58.62, main/busy/ms 47.55, render/thread/ms 46.93, render/busy/ms 44.56, stream/drive/ms 29.73, render/graph/ms 8.63, pipelines/driver_ms 3.33, stream/spawn/ms 2.38, render/present/ms 2.11, stream/tick/ms 1.79 |
| warm-up | 4.18 | 42.5 | pipelines/wait_ms 3104, render/thread/ms 61.86, render/busy/ms 59.18, pipelines/compile_ms 38.27, pipelines/front_ms 35.45, main/busy/ms 34.18, stream/drive/ms 15.48, render/graph/ms 13.41, pipelines/driver_ms 2.82, bigspace/total/ms 2.59, bigspace/high_precision/ms 2.37, render/present/ms 2.36 |
| warm-up | 4.23 | 52.1 | pipelines/wait_ms 2766, pipelines/compile_ms 95.88, pipelines/front_ms 90.69, render/thread/ms 59.71, render/busy/ms 58.24, main/busy/ms 28.62, stream/drive/ms 13.85, render/graph/ms 9.51, pipelines/driver_ms 5.19, bigspace/total/ms 2.48, bigspace/high_precision/ms 2.35, render/present/ms 1.25 |
| warm-up | 4.30 | 38.0 | pipelines/wait_ms 1600, pipelines/compile_ms 76.23, pipelines/front_ms 72.23, main/busy/ms 59.71, render/thread/ms 42.77, stream/drive/ms 41.20, render/busy/ms 41.14, render/graph/ms 13.29, pipelines/driver_ms 4.00, stream/spawn/ms 2.57, render/present/ms 1.34, stream/tick/ms 1.07 |
| warm-up | 4.38 | 60.4 | pipelines/wait_ms 959, main/busy/ms 67.18, stream/drive/ms 46.95, render/thread/ms 36.91, render/busy/ms 35.75, render/graph/ms 4.86, pipelines/compile_ms 2.57, pipelines/front_ms 2.37, stream/spawn/ms 1.81, render/present/ms 0.95, stream/tick/ms 0.86, terrain/update/ms 0.81 |
| warm-up | 4.44 | 43.5 | pipelines/wait_ms 9333, pipelines/compile_ms 174, pipelines/front_ms 166, main/busy/ms 45.54, stream/drive/ms 30.37, render/thread/ms 27.39, render/busy/ms 26.11, pipelines/driver_ms 8.58, render/graph/ms 3.96, render/present/ms 1.12, stream/activate/ms 1.06, terrain/update/ms 0.83 |
| warm-up | 4.89 | 41.9 | pipelines/wait_ms 1951, pipelines/compile_ms 178, pipelines/front_ms 166, render/thread/ms 59.60, render/busy/ms 58.57, render/graph/ms 25.59, main/busy/ms 16.49, pipelines/driver_ms 12.49, stream/drive/ms 1.99, terrain/update/ms 0.99, render/present/ms 0.85, terrain/select/ms 0.83 |
| measure | 19.28 | 34.7 | render/thread/ms 48.03, render/busy/ms 46.62, render/graph/ms 28.95, main/busy/ms 18.86, render/present/ms 1.24, bigspace/total/ms 0.76, controller/feed/ms 0.66, anim/evaluate/ms 0.63, far/select/ms 0.60, terrain/update/ms 0.59, terrain/select/ms 0.53, stream/drive/ms 0.46 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_dlssperf\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=performance
- before the run: GPU 0%, CPU 0%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **75%** (min 52%) · 2576 MHz (min 2565) · memory 8001 MHz · 78.3 W · 76 °C (max 82) · VRAM 5486 MiB · throttled 100% of samples
