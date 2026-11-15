# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3555 frames over 65.7 s · **median 12.19 ms (82 fps)** · mean 18.47 · p95 53.89 · p99 88.54 (1% low 11 fps) · worst 233.63 ms · 524 spikes over 1.5× median · 13.7 stutters/min (work 9.1/min)
- warm-up: 336 frames over 15.4 s, worst 3152.8 ms
- hitches (> 30.5 ms): **486** measured, 174 in warm-up
- timed GPU passes: 12.14 ms of a 18.47 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.19 ms, limit 11 ms
- **FAIL** frame_ms: 15 stutters (> 1.5x moving median of 60) in 1.09 min = 13.71/min, limit 1/min
- **FAIL** worst frame 233.63 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.394 | 2.433 | 5.934 | 0.98 | 0.064 |
| dlss_super_resolution | 2.344 | 2.383 | 5.182 | 0.98 | 0.186 |
| auxiliary_views | 1.221 | 1.241 | 8.948 | 0.98 | 0.172 |
| early prepass | 1.161 | 1.180 | 4.901 | 0.98 | 0.108 |
| bin_unpacking | 0.761 | 0.774 | 5.352 | 0.98 | 0.031 |
| water_fft | 0.648 | 0.658 | 4.444 | 0.98 | 0.003 |
| bloom | 0.360 | 0.366 | 0.695 | 0.98 | 0.066 |
| atmosphere_luts | 0.355 | 0.361 | 0.735 | 0.98 | 0.004 |
| uniform_allocation | 0.349 | 0.355 | 0.996 | 0.98 | 0.051 |
| early_mesh_preprocessing | 0.336 | 0.341 | 2.185 | 0.98 | 0.005 |
| water_velocity | 0.283 | 0.288 | 0.671 | 0.98 | 0.001 |
| water_sspr | 0.256 | 0.260 | 0.774 | 0.98 | 0.002 |
| ssao | 0.212 | 0.215 | 0.255 | 0.98 | 0.008 |
| atmos_froxel_inject | 0.171 | 0.174 | 0.299 | 0.98 | 0.002 |
| water_snapshot | 0.148 | 0.151 | 1.126 | 0.98 | 0.002 |
| upscaling | 0.133 | 0.135 | 0.162 | 0.98 | 0.004 |
| clouds_trace | 0.132 | 0.135 | 0.170 | 0.98 | 0.001 |
| main_transmissive_pass_3d | 0.120 | 0.122 | 0.329 | 0.98 | 0.008 |
| clustering | 0.120 | 0.122 | 0.300 | 0.98 | 0.020 |
| clouds_composite | 0.115 | 0.117 | 0.112 | 0.98 | 0.002 |
| tonemapping | 0.100 | 0.102 | 0.103 | 0.98 | 0.003 |
| render_sky | 0.056 | 0.057 | 0.055 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.476 | 0.992 | 0.11 | 0.001 |
| surface_trail | 0.051 | 0.063 | 0.058 | 0.82 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.98 | 0.001 |
| water_mips | 0.032 | 0.032 | 0.081 | 0.98 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.98 | 0.003 |
| per_view_shadow_pass | 0.030 | 0.031 | 0.049 | 0.98 | 0.007 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.029 | 0.261 | 0.576 | 0.11 | 0.004 |
| main_transparent_pass_3d | 0.021 | 0.022 | 0.033 | 0.98 | 0.111 |
| atmosphere_environment | 0.011 | 0.011 | 0.012 | 0.98 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.007 | 0.98 | 0.001 |
| lightprobe_copy | 0.003 | 0.023 | 0.072 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.021 | 0.033 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.03 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 17.207 | 52.015 | 231.72 | 61170.8 |
| render/busy/ms | 11.965 | 19.957 | 97.93 | 42537.3 |
| pipelines/wait_ms | 11.796 | 0.000 | 20994.08 | 41936.4 |
| main/busy/ms | 9.146 | 13.335 | 32.20 | 32512.9 |
| render/graph/ms | 7.321 | 12.228 | 56.79 | 26027.8 |
| render/acquire/ms | 4.694 | 29.754 | 212.59 | 16685.7 |
| render/present/ms | 0.548 | 0.769 | 29.98 | 1947.7 |
| bigspace/total/ms | 0.464 | 0.754 | 4.18 | 1649.9 |
| pipelines/compile_ms | 0.358 | 0.000 | 210.36 | 1273.1 |
| pipelines/front_ms | 0.347 | 0.000 | 204.84 | 1232.8 |
| terrain/update/ms | 0.306 | 0.419 | 2.85 | 1086.7 |
| controller/feed/ms | 0.278 | 0.391 | 0.79 | 987.1 |
| terrain/select/ms | 0.270 | 0.363 | 0.83 | 959.9 |
| stream/drive/ms | 0.266 | 0.481 | 17.26 | 945.9 |
| far/select/ms | 0.198 | 0.288 | 0.80 | 703.2 |
| stream/tick/ms | 0.190 | 0.291 | 0.94 | 676.7 |
| bigspace/low_precision/ms | 0.174 | 0.280 | 3.54 | 617.2 |
| anim/evaluate/ms | 0.168 | 0.301 | 2.10 | 598.5 |
| controller/move/ms | 0.161 | 0.279 | 0.60 | 572.5 |
| anim/evaluate_cpu/ms | 0.153 | 0.229 | 2.03 | 542.3 |
| bigspace/high_precision/ms | 0.119 | 0.233 | 3.59 | 423.1 |
| physics/insert/ms | 0.077 | 0.509 | 1.66 | 272.1 |
| physics/step/ms | 0.073 | 0.180 | 0.93 | 260.5 |
| weather/map/ms | 0.025 | 0.036 | 0.11 | 87.7 |
| pipelines/driver_ms | 0.011 | 0.000 | 5.52 | 40.4 |
| controller/avoid/ms | 0.011 | 0.015 | 0.06 | 38.9 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 38.2 |
| anim/apply/ms | 0.009 | 0.022 | 0.18 | 33.2 |
| stream/despawn/ms | 0.009 | 0.000 | 1.00 | 32.9 |
| stream/spawn/ms | 0.008 | 0.000 | 1.09 | 27.9 |
| script/update/ms | 0.007 | 0.013 | 0.13 | 26.1 |
| script/input/ms | 0.005 | 0.008 | 0.05 | 19.4 |
| script/ui/ms | 0.005 | 0.008 | 0.08 | 17.4 |
| stream/deactivate/ms | 0.004 | 0.000 | 0.71 | 14.6 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 12.3 |
| stream/activate/ms | 0.002 | 0.000 | 0.82 | 6.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 3.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 2.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.08 | 1.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.07 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 28 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 571 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 20 | 100 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 12 | 61 | 10 |
| crowd/c0/joints/count | count | 631 | 2369 | 434 |
| crowd/c0/meshes/count | count | 187 | 646 | 101 |
| crowd/c1/count | count | 151 | 155 | 151 |
| crowd/c1/joints/count | count | 5742 | 5937 | 5762 |
| crowd/c1/meshes/count | count | 1217 | 1323 | 1240 |
| crowd/c2/count | count | 402 | 417 | 417 |
| crowd/c2/joints/count | count | 6455 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1839 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 236 | 223 |
| crowd/joints_extracted/count | count | 4454 | 9933 | 6179 |
| crowd/joints_uploaded/count | count | 4369 | 6643 | 6179 |
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
| frame/work/ms | ms | 10.42 | 107 | 8.93 |
| gpu/timed/ms | ms | 8.71 | 107 | 8.93 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2533 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.1 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 538.36 M | 539.54 M | 539.54 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 349 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 741 | 1416 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 349 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 741 | 1416 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 29 | 28 |
| phases/opaque3d/bins | count | 1319 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3224 | 5132 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 29 | 28 |
| phases/opaque3d_prepass/bins | count | 1319 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3224 | 5132 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 50 | 315 | 47 |
| phases/shadow/bins | count | 2515 | 5513 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 6052 | 12.6 k | 5667 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 50 | 319 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2605 | 3268 | 2348 |
| physics/items | count | 19 | 476 | 19 |
| physics/pending | count | 0 | 425 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 231 | 231 | 231 |
| pipelines/waiting | count | 0 | 4 | 0 |
| preprocess/bind_groups | count | 11 | 823 | 10 |
| preprocess/bind_groups_created | count | 11 | 823 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 254 | 392 | 247 |
| process/cpu_usage | % | 21.63 | 47.87 | 25.24 |
| process/mem_usage | GiB | 6.51 | 6.67 | 6.67 |
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
| started_load_count |  | 4801 | 4862 | 4862 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 396 | 528 | 396 |
| statics/materials_bound | count | 396 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 3 | 0 |
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 3 | 0 |
| stream/latency/s | s | 0.04 | 0.34 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 253 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.76 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1124 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.13 | 92.36 | 28.82 |
| system/mem_usage | % | 68.28 | 81.29 | 68.50 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2838 | 2938 | 2938 |
| textures/images | count | 2600 | 2649 | 2649 |
| views/all/batch_sets | count | 120 | 379 | 117 |
| views/all/bins | count | 5856 | 8289 | 5692 |
| views/all/count | count | 4 | 52 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 13.7 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 116 | 387 | 113 |
| views/all/pipelines_distinct | count | 92 | 108 | 91 |
| views/all/work_items | count | 14.0 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 50 | 69 | 47 |
| views/cascade/bins | count | 2497 | 3096 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 6052 | 9609 | 5667 |
| views/cascade/pipelines | count | 50 | 69 | 47 |
| views/cascade/work_items | count | 6052 | 9609 | 5667 |
| views/lamp_face/batch_sets | count | 0 | 255 | 0 |
| views/lamp_face/bins | count | 0 | 2614 | 0 |
| views/lamp_face/count | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 2958 | 0 |
| views/lamp_face/pipelines | count | 0 | 258 | 0 |
| views/lamp_face/work_items | count | 0 | 2703 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 3340 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7796 | 13.0 k | 7600 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8109 | 13.3 k | 7908 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 30.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 289 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 28.03 | 994 | 99.6 k |
| anim/events/count | count | 0.15 | 6 | 531 |
| assets/Image/added | count | 0.18 | 43 | 625 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 1.17 | 152 | 4164 |
| assets/Mesh/removed | count | 1.38 | 1224 | 4920 |
| assets/StandardMaterial/added | count | 0.14 | 35 | 498 |
| assets/StandardMaterial/modified | count | 5.11 | 11 | 18.2 k |
| assets/StandardMaterial/removed | count | 0.11 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 6 | 86 |
| draws/skipped_late | count | 0.29 | 246 | 1032 |
| ecs/entities_delta | count | 2.64 | 4604 | 9368 |
| physics/inserts | count | 0.69 | 104 | 2440 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.59 | 145 | 2112 |
| physics/steps | count | 1.02 | 3 | 3618 |
| physics/steps_dropped | count | 0.09 | 11 | 324 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.01 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.06 | 19 | 219 |
| stream/deactivate/items | count | 0.06 | 21 | 223 |
| stream/despawn/items | count | 0.06 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 3 | 83 |
| stream/spawn/items | count | 0.11 | 20 | 378 |
| stream/upload/bytes | bytes | 9522 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.02 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 41 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 2088 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.5 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.15 | 3152.8 | main/busy/ms 3125, surface/advance_cpu_ms 69.99, surface/cell_upload_ms 0.90, stream/drive/ms 0.35, weather/map/ms 0.14, stream/tick/ms 0.10, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.03, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 4.07 | 921.0 | main/busy/ms 3163, render/thread/ms 880, render/busy/ms 638, render/acquire/ms 189, surface/advance_cpu_ms 69.99, render/present/ms 53.78, render/graph/ms 12.45, stream/drive/ms 1.09, surface/cell_upload_ms 0.90, stream/tick/ms 0.53, terrain/update/ms 0.40, bigspace/total/ms 0.27 |
| warm-up | 4.26 | 189.6 | pipelines/wait_ms 1477, render/thread/ms 895, render/busy/ms 651, pipelines/compile_ms 237, pipelines/front_ms 222, render/acquire/ms 189, main/busy/ms 58.15, render/present/ms 55.16, terrain/update/ms 19.22, render/graph/ms 17.74, pipelines/driver_ms 14.75, bigspace/total/ms 2.36 |
| warm-up | 4.53 | 94.3 | main/busy/ms 71.73, render/thread/ms 64.22, render/busy/ms 61.77, render/graph/ms 4.46, stream/drive/ms 3.65, stream/tick/ms 2.18, render/present/ms 1.96, terrain/update/ms 0.88, terrain/select/ms 0.74, stream/spawn/ms 0.74, render/acquire/ms 0.48, bigspace/total/ms 0.23 |
| warm-up | 4.73 | 89.0 | main/busy/ms 55.80, render/thread/ms 52.57, render/busy/ms 50.66, stream/drive/ms 41.14, render/graph/ms 4.56, render/present/ms 1.65, stream/tick/ms 1.27, terrain/update/ms 0.84, terrain/select/ms 0.70, stream/spawn/ms 0.51, script/ui/ms 0.29, bigspace/total/ms 0.29 |
| warm-up | 4.88 | 105.8 | pipelines/wait_ms 3962, pipelines/compile_ms 322, pipelines/front_ms 281, render/thread/ms 92.47, render/busy/ms 76.31, main/busy/ms 73.77, pipelines/driver_ms 40.44, stream/drive/ms 31.52, render/present/ms 15.81, render/graph/ms 8.73, stream/tick/ms 1.65, stream/spawn/ms 0.89 |
| warm-up | 5.11 | 93.3 | pipelines/wait_ms 1934, render/thread/ms 96.40, render/busy/ms 93.69, pipelines/compile_ms 87.11, pipelines/front_ms 83.43, main/busy/ms 82.75, stream/drive/ms 49.28, render/graph/ms 5.61, pipelines/driver_ms 3.68, stream/tick/ms 2.61, render/present/ms 2.27, stream/spawn/ms 2.07 |
| warm-up | 5.60 | 142.7 | main/busy/ms 144, render/thread/ms 83.54, render/busy/ms 80.82, stream/drive/ms 76.50, render/graph/ms 8.05, stream/spawn/ms 4.64, stream/tick/ms 2.27, render/present/ms 1.85, terrain/update/ms 1.29, terrain/select/ms 1.04, render/acquire/ms 0.87, stream/upload/ms 0.77 |
| warm-up | 8.98 | 101.2 | pipelines/wait_ms 14.5 k, pipelines/compile_ms 267, pipelines/front_ms 255, render/thread/ms 103, render/acquire/ms 78.63, main/busy/ms 22.22, render/busy/ms 18.54, pipelines/driver_ms 11.96, render/graph/ms 6.85, render/present/ms 5.92, stream/drive/ms 1.90, stream/tick/ms 1.03 |
| measure | 2.28 | 100.8 | render/thread/ms 149, render/busy/ms 118, render/graph/ms 38.88, render/acquire/ms 29.82, main/busy/ms 28.82, pipelines/compile_ms 16.62, pipelines/front_ms 16.27, render/present/ms 1.55, stream/drive/ms 1.26, bigspace/total/ms 1.23, stream/tick/ms 1.16, terrain/update/ms 0.79 |
| measure | 2.42 | 111.9 | render/thread/ms 132, render/acquire/ms 87.80, render/busy/ms 42.18, main/busy/ms 29.49, render/graph/ms 26.23, render/present/ms 1.58, bigspace/total/ms 1.08, stream/drive/ms 0.86, terrain/update/ms 0.75, stream/tick/ms 0.67, terrain/select/ms 0.67, physics/insert/ms 0.62 |
| measure | 2.86 | 95.7 | render/thread/ms 151, render/acquire/ms 77.99, render/busy/ms 71.87, render/graph/ms 43.36, main/busy/ms 36.21, stream/drive/ms 5.23, render/present/ms 1.59, bigspace/total/ms 1.15, controller/feed/ms 1.01, far/select/ms 0.90, terrain/update/ms 0.80, terrain/select/ms 0.71 |
| measure | 3.04 | 128.4 | render/thread/ms 168, render/acquire/ms 113, render/busy/ms 52.67, render/graph/ms 34.42, main/busy/ms 31.81, render/present/ms 1.91, stream/drive/ms 1.54, bigspace/total/ms 1.13, terrain/update/ms 0.82, terrain/select/ms 0.73, controller/feed/ms 0.66, stream/spawn/ms 0.58 |
| measure | 7.36 | 135.9 | render/thread/ms 205, render/busy/ms 95.98, render/acquire/ms 77.87, render/graph/ms 73.43, main/busy/ms 31.14, render/present/ms 30.83, stream/drive/ms 2.11, physics/insert/ms 1.49, bigspace/total/ms 1.33, stream/activate/ms 0.97, terrain/update/ms 0.85, controller/feed/ms 0.80 |
| measure | 7.63 | 233.6 | render/thread/ms 270, render/acquire/ms 213, main/busy/ms 46.87, render/busy/ms 44.14, render/graph/ms 25.02, render/present/ms 13.49, stream/drive/ms 3.54, physics/insert/ms 1.54, bigspace/total/ms 1.50, stream/despawn/ms 1.45, stream/spawn/ms 1.24, terrain/update/ms 1.03 |
| measure | 7.87 | 91.8 | render/thread/ms 157, render/acquire/ms 112, render/busy/ms 43.93, main/busy/ms 36.42, render/graph/ms 28.07, stream/drive/ms 1.88, render/present/ms 1.45, bigspace/total/ms 1.38, controller/feed/ms 1.09, physics/insert/ms 1.00, physics/step/ms 0.84, stream/despawn/ms 0.84 |
| measure | 8.90 | 88.5 | render/thread/ms 151, render/acquire/ms 115, render/busy/ms 34.98, main/busy/ms 27.09, render/graph/ms 22.44, pipelines/compile_ms 9.45, pipelines/front_ms 9.22, render/present/ms 1.42, bigspace/total/ms 1.25, stream/drive/ms 1.03, controller/feed/ms 0.84, terrain/update/ms 0.78 |
| measure | 10.00 | 88.2 | render/thread/ms 162, render/acquire/ms 123, render/busy/ms 38.05, main/busy/ms 27.59, render/graph/ms 24.53, render/present/ms 1.26, bigspace/total/ms 1.02, physics/insert/ms 0.98, stream/drive/ms 0.84, terrain/update/ms 0.71, anim/evaluate_cpu/ms 0.69, terrain/select/ms 0.63 |
| measure | 10.67 | 103.1 | render/thread/ms 184, render/acquire/ms 141, render/busy/ms 41.02, main/busy/ms 36.66, render/graph/ms 24.31, stream/drive/ms 2.55, render/present/ms 1.49, terrain/update/ms 1.47, physics/insert/ms 1.20, bigspace/total/ms 1.20, stream/despawn/ms 1.13, controller/feed/ms 0.87 |
| measure | 10.76 | 88.5 | render/thread/ms 189, render/acquire/ms 141, render/busy/ms 45.89, main/busy/ms 34.63, render/graph/ms 27.72, stream/drive/ms 2.66, render/present/ms 1.47, bigspace/total/ms 1.19, stream/despawn/ms 1.16, controller/feed/ms 0.89, terrain/update/ms 0.86, terrain/select/ms 0.76 |
| measure | 10.84 | 87.7 | render/thread/ms 173, render/acquire/ms 125, render/busy/ms 46.62, main/busy/ms 30.13, render/graph/ms 27.83, stream/drive/ms 1.62, render/present/ms 1.53, bigspace/total/ms 1.18, terrain/update/ms 0.85, terrain/select/ms 0.76, stream/despawn/ms 0.74, controller/feed/ms 0.73 |
| measure | 11.02 | 91.4 | render/thread/ms 171, render/acquire/ms 126, render/busy/ms 43.64, render/graph/ms 28.54, main/busy/ms 25.26, bigspace/total/ms 1.63, render/present/ms 1.44, terrain/update/ms 0.97, stream/drive/ms 0.87, terrain/select/ms 0.87, anim/evaluate_cpu/ms 0.74, stream/tick/ms 0.70 |
| measure | 11.11 | 91.8 | render/thread/ms 180, render/acquire/ms 133, render/busy/ms 45.81, render/graph/ms 28.16, main/busy/ms 27.10, bigspace/total/ms 1.88, render/present/ms 1.71, terrain/update/ms 0.85, terrain/select/ms 0.77, physics/insert/ms 0.72, stream/drive/ms 0.62, controller/feed/ms 0.57 |
| measure | 11.36 | 87.3 | render/thread/ms 166, render/acquire/ms 121, render/busy/ms 42.85, main/busy/ms 31.02, render/graph/ms 28.32, stream/drive/ms 1.71, physics/insert/ms 1.55, render/present/ms 1.43, stream/tick/ms 1.23, bigspace/total/ms 1.18, controller/feed/ms 0.98, terrain/update/ms 0.69 |
| measure | 11.53 | 88.4 | render/thread/ms 170, render/acquire/ms 132, render/busy/ms 36.01, main/busy/ms 27.41, render/graph/ms 20.56, render/present/ms 1.78, physics/insert/ms 1.06, bigspace/total/ms 1.05, controller/feed/ms 0.80, terrain/update/ms 0.76, stream/drive/ms 0.71, terrain/select/ms 0.67 |
| measure | 11.78 | 88.3 | render/thread/ms 160, render/acquire/ms 121, render/busy/ms 37.62, main/busy/ms 26.64, render/graph/ms 23.33, render/present/ms 1.57, bigspace/total/ms 1.16, terrain/update/ms 0.84, controller/feed/ms 0.83, terrain/select/ms 0.75, stream/drive/ms 0.74, stream/tick/ms 0.66 |
| measure | 11.95 | 86.4 | render/thread/ms 163, render/acquire/ms 127, render/busy/ms 33.95, main/busy/ms 28.85, render/graph/ms 20.17, render/present/ms 1.66, bigspace/total/ms 1.11, far/select/ms 1.10, terrain/update/ms 0.80, controller/feed/ms 0.72, terrain/select/ms 0.71, anim/evaluate_cpu/ms 0.64 |
| measure | 12.04 | 89.1 | render/thread/ms 172, render/acquire/ms 137, render/busy/ms 34.25, main/busy/ms 25.12, render/graph/ms 21.61, render/present/ms 1.38, bigspace/total/ms 0.99, terrain/update/ms 0.76, terrain/select/ms 0.68, controller/feed/ms 0.68, anim/evaluate_cpu/ms 0.51, far/select/ms 0.47 |
| measure | 12.12 | 87.9 | render/thread/ms 174, render/acquire/ms 134, render/busy/ms 38.70, main/busy/ms 28.91, render/graph/ms 23.32, render/present/ms 1.45, bigspace/total/ms 1.14, controller/feed/ms 0.90, terrain/update/ms 0.85, terrain/select/ms 0.76, anim/evaluate_cpu/ms 0.73, anim/evaluate/ms 0.48 |
| measure | 12.23 | 101.4 | render/thread/ms 186, render/acquire/ms 136, render/busy/ms 48.53, main/busy/ms 38.40, render/graph/ms 26.54, physics/insert/ms 1.64, bigspace/total/ms 1.34, render/present/ms 1.25, controller/feed/ms 1.05, terrain/update/ms 1.02, terrain/select/ms 0.91, anim/evaluate_cpu/ms 0.76 |
| measure | 12.32 | 90.0 | render/thread/ms 187, render/acquire/ms 131, render/busy/ms 54.61, main/busy/ms 35.53, render/graph/ms 28.35, terrain/update/ms 3.37, physics/insert/ms 1.79, bigspace/total/ms 1.30, render/present/ms 1.11, controller/feed/ms 0.97, terrain/select/ms 0.94, stream/drive/ms 0.76 |
| measure | 12.42 | 101.2 | render/thread/ms 187, render/acquire/ms 133, render/busy/ms 52.55, render/graph/ms 29.30, main/busy/ms 27.94, terrain/update/ms 3.33, bigspace/total/ms 1.58, render/present/ms 1.12, controller/feed/ms 0.90, terrain/select/ms 0.90, far/select/ms 0.73, physics/insert/ms 0.72 |
| measure | 12.50 | 86.9 | render/thread/ms 184, render/acquire/ms 133, render/busy/ms 49.69, render/graph/ms 31.43, main/busy/ms 27.45, bigspace/total/ms 1.58, render/present/ms 1.16, terrain/update/ms 0.93, terrain/select/ms 0.83, physics/insert/ms 0.82, controller/feed/ms 0.76, anim/evaluate_cpu/ms 0.65 |
| measure | 12.60 | 93.7 | render/thread/ms 177, render/acquire/ms 133, render/busy/ms 43.54, render/graph/ms 28.15, main/busy/ms 26.39, render/present/ms 1.22, bigspace/total/ms 1.19, terrain/update/ms 0.91, terrain/select/ms 0.80, physics/insert/ms 0.77, controller/feed/ms 0.73, stream/drive/ms 0.67 |
| measure | 12.79 | 104.7 | render/thread/ms 187, render/acquire/ms 145, render/busy/ms 40.47, main/busy/ms 27.08, render/graph/ms 26.85, render/present/ms 1.30, bigspace/total/ms 1.23, terrain/update/ms 0.95, physics/insert/ms 0.91, terrain/select/ms 0.87, controller/feed/ms 0.80, bigspace/low_precision/ms 0.50 |
| measure | 12.88 | 87.8 | render/thread/ms 189, render/acquire/ms 145, render/busy/ms 42.75, render/graph/ms 28.58, main/busy/ms 26.74, render/present/ms 1.28, bigspace/total/ms 1.20, physics/insert/ms 1.12, terrain/update/ms 1.03, terrain/select/ms 0.94, controller/feed/ms 0.69, anim/evaluate_cpu/ms 0.50 |
| measure | 12.97 | 95.8 | render/thread/ms 180, render/acquire/ms 137, render/busy/ms 41.48, main/busy/ms 28.75, render/graph/ms 25.74, render/present/ms 1.33, bigspace/total/ms 1.20, stream/drive/ms 1.17, controller/feed/ms 0.92, physics/insert/ms 0.82, terrain/update/ms 0.79, terrain/select/ms 0.71 |
| measure | 13.22 | 86.3 | render/thread/ms 162, render/acquire/ms 124, render/busy/ms 37.22, main/busy/ms 28.95, render/graph/ms 22.38, stream/drive/ms 1.89, render/present/ms 1.27, bigspace/total/ms 1.12, stream/despawn/ms 1.06, physics/insert/ms 0.95, controller/feed/ms 0.77, terrain/update/ms 0.73 |
| measure | 13.31 | 87.2 | render/thread/ms 170, render/acquire/ms 126, render/busy/ms 43.22, main/busy/ms 27.55, render/graph/ms 25.65, stream/drive/ms 1.63, render/present/ms 1.30, bigspace/total/ms 1.15, terrain/update/ms 1.06, stream/despawn/ms 1.04, controller/feed/ms 0.77, terrain/select/ms 0.75 |
| measure | 13.39 | 87.3 | render/thread/ms 171, render/acquire/ms 121, render/busy/ms 47.94, render/graph/ms 29.31, main/busy/ms 26.75, stream/drive/ms 1.54, render/present/ms 1.33, bigspace/total/ms 1.07, terrain/update/ms 1.00, stream/despawn/ms 0.97, controller/feed/ms 0.76, terrain/select/ms 0.70 |
| measure | 13.49 | 96.5 | render/thread/ms 180, render/acquire/ms 129, render/busy/ms 47.78, render/graph/ms 29.56, main/busy/ms 27.84, render/present/ms 3.98, bigspace/total/ms 1.25, stream/drive/ms 1.05, controller/feed/ms 0.78, terrain/update/ms 0.70, terrain/select/ms 0.63, anim/evaluate_cpu/ms 0.61 |
| measure | 13.58 | 86.3 | render/thread/ms 179, render/acquire/ms 134, render/busy/ms 41.49, render/graph/ms 26.18, main/busy/ms 25.33, render/present/ms 3.89, bigspace/total/ms 1.21, controller/feed/ms 0.81, terrain/update/ms 0.81, stream/drive/ms 0.80, terrain/select/ms 0.72, stream/tick/ms 0.70 |
| measure | 13.76 | 96.9 | render/thread/ms 178, render/acquire/ms 137, render/busy/ms 39.30, main/busy/ms 24.87, render/graph/ms 24.52, render/present/ms 1.23, bigspace/total/ms 1.07, controller/feed/ms 0.81, stream/drive/ms 0.71, terrain/update/ms 0.66, stream/tick/ms 0.63, physics/insert/ms 0.61 |
| measure | 13.86 | 99.7 | render/thread/ms 193, render/acquire/ms 148, render/busy/ms 43.12, main/busy/ms 30.82, render/graph/ms 27.77, render/present/ms 1.40, bigspace/total/ms 1.21, controller/feed/ms 1.04, physics/insert/ms 0.80, terrain/update/ms 0.70, stream/drive/ms 0.70, terrain/select/ms 0.63 |
| measure | 13.95 | 92.3 | render/thread/ms 186, render/acquire/ms 140, render/busy/ms 44.30, main/busy/ms 32.24, render/graph/ms 27.10, render/present/ms 1.63, bigspace/total/ms 1.20, controller/feed/ms 1.04, terrain/update/ms 0.76, physics/insert/ms 0.76, terrain/select/ms 0.69, far/select/ms 0.66 |
| measure | 14.06 | 107.7 | render/thread/ms 194, render/acquire/ms 154, render/busy/ms 39.32, main/busy/ms 25.45, render/graph/ms 24.00, render/present/ms 1.60, bigspace/total/ms 1.18, controller/feed/ms 0.82, terrain/update/ms 0.73, terrain/select/ms 0.65, stream/drive/ms 0.58, far/select/ms 0.56 |
| measure | 14.15 | 92.3 | render/thread/ms 196, render/acquire/ms 157, render/busy/ms 37.36, render/graph/ms 24.86, main/busy/ms 24.30, render/present/ms 1.41, bigspace/total/ms 1.11, terrain/update/ms 0.80, controller/feed/ms 0.74, terrain/select/ms 0.72, stream/drive/ms 0.62, stream/tick/ms 0.54 |
| measure | 14.24 | 94.4 | render/thread/ms 183, render/acquire/ms 140, render/busy/ms 41.63, render/graph/ms 27.52, main/busy/ms 26.25, render/present/ms 1.41, bigspace/total/ms 1.16, terrain/update/ms 0.99, terrain/select/ms 0.89, controller/feed/ms 0.75, stream/drive/ms 0.66, stream/tick/ms 0.57 |
| measure | 14.34 | 98.8 | render/thread/ms 190, render/acquire/ms 144, render/busy/ms 43.93, render/graph/ms 29.11, main/busy/ms 26.12, render/present/ms 1.39, terrain/update/ms 1.28, bigspace/total/ms 1.21, terrain/select/ms 1.17, controller/feed/ms 0.73, anim/evaluate_cpu/ms 0.54, stream/drive/ms 0.51 |
| measure | 14.44 | 99.4 | render/thread/ms 195, render/acquire/ms 153, render/busy/ms 40.41, render/graph/ms 25.55, main/busy/ms 24.48, render/present/ms 1.33, terrain/update/ms 1.17, terrain/select/ms 1.07, bigspace/total/ms 1.06, controller/feed/ms 0.71, physics/insert/ms 0.47, stream/drive/ms 0.46 |
| measure | 14.54 | 100.0 | render/thread/ms 196, render/acquire/ms 155, render/busy/ms 39.76, render/graph/ms 24.67, main/busy/ms 23.83, render/present/ms 1.31, bigspace/total/ms 1.07, terrain/update/ms 0.84, controller/feed/ms 0.81, terrain/select/ms 0.74, stream/drive/ms 0.68, stream/tick/ms 0.60 |
| measure | 14.63 | 87.7 | render/thread/ms 184, render/acquire/ms 141, render/busy/ms 41.71, render/graph/ms 28.01, main/busy/ms 26.93, render/present/ms 1.37, bigspace/total/ms 1.15, terrain/update/ms 0.86, stream/drive/ms 0.84, controller/feed/ms 0.83, terrain/select/ms 0.78, stream/tick/ms 0.74 |
| measure | 14.80 | 89.1 | render/thread/ms 169, render/acquire/ms 128, render/busy/ms 39.62, render/graph/ms 25.57, main/busy/ms 24.48, render/present/ms 1.44, physics/insert/ms 1.04, bigspace/total/ms 0.97, controller/feed/ms 0.95, terrain/update/ms 0.59, far/select/ms 0.58, stream/drive/ms 0.58 |
| measure | 14.97 | 86.6 | render/thread/ms 163, render/acquire/ms 120, render/busy/ms 40.98, render/graph/ms 28.20, main/busy/ms 23.29, render/present/ms 1.62, bigspace/total/ms 1.16, controller/feed/ms 0.80, terrain/update/ms 0.78, terrain/select/ms 0.71, anim/evaluate_cpu/ms 0.55, far/select/ms 0.49 |
| measure | 15.06 | 91.5 | render/thread/ms 175, render/acquire/ms 134, render/busy/ms 39.73, render/graph/ms 27.03, main/busy/ms 24.03, bigspace/total/ms 1.59, render/present/ms 1.43, terrain/update/ms 0.94, terrain/select/ms 0.83, bigspace/high_precision/ms 0.75, controller/feed/ms 0.72, anim/evaluate_cpu/ms 0.50 |
| measure | 15.15 | 90.2 | render/thread/ms 179, render/acquire/ms 137, render/busy/ms 40.81, render/graph/ms 26.52, main/busy/ms 23.78, bigspace/total/ms 1.88, render/present/ms 1.23, terrain/update/ms 0.82, bigspace/high_precision/ms 0.81, terrain/select/ms 0.72, controller/feed/ms 0.69, bigspace/low_precision/ms 0.57 |
| measure | 15.24 | 87.7 | render/thread/ms 175, render/acquire/ms 133, render/busy/ms 40.60, render/graph/ms 25.83, main/busy/ms 20.74, bigspace/total/ms 1.34, render/present/ms 1.30, controller/feed/ms 0.74, terrain/update/ms 0.71, terrain/select/ms 0.64, bigspace/low_precision/ms 0.55, anim/evaluate_cpu/ms 0.47 |
| measure | 15.33 | 89.1 | render/thread/ms 174, render/acquire/ms 134, render/busy/ms 38.03, render/graph/ms 23.85, main/busy/ms 22.00, render/present/ms 1.22, bigspace/total/ms 1.14, terrain/update/ms 0.67, controller/feed/ms 0.66, terrain/select/ms 0.60, far/select/ms 0.45, bigspace/low_precision/ms 0.44 |
| measure | 15.43 | 103.4 | render/thread/ms 190, render/acquire/ms 150, render/busy/ms 38.17, render/graph/ms 25.53, main/busy/ms 24.47, render/present/ms 1.24, bigspace/total/ms 1.22, controller/feed/ms 0.68, terrain/update/ms 0.68, terrain/select/ms 0.61, far/select/ms 0.57, physics/insert/ms 0.52 |
| measure | 15.52 | 94.3 | render/thread/ms 195, render/acquire/ms 152, render/busy/ms 41.02, render/graph/ms 28.23, main/busy/ms 23.84, render/present/ms 1.40, bigspace/total/ms 1.16, physics/insert/ms 0.91, terrain/update/ms 0.75, controller/feed/ms 0.69, terrain/select/ms 0.68, far/select/ms 0.62 |
| measure | 15.62 | 95.1 | render/thread/ms 186, render/acquire/ms 145, render/busy/ms 39.47, render/graph/ms 24.96, main/busy/ms 21.39, render/present/ms 1.35, bigspace/total/ms 0.91, physics/insert/ms 0.82, terrain/update/ms 0.70, terrain/select/ms 0.63, controller/feed/ms 0.57, far/select/ms 0.50 |
| measure | 15.71 | 88.6 | render/thread/ms 180, render/acquire/ms 141, render/busy/ms 38.00, render/graph/ms 23.28, main/busy/ms 21.85, render/present/ms 1.38, bigspace/total/ms 1.03, terrain/update/ms 0.73, physics/insert/ms 0.70, terrain/select/ms 0.66, controller/feed/ms 0.63, stream/drive/ms 0.42 |
| measure | 15.80 | 87.6 | render/thread/ms 173, render/acquire/ms 134, render/busy/ms 37.44, render/graph/ms 24.31, main/busy/ms 23.78, render/present/ms 1.40, bigspace/total/ms 1.19, terrain/update/ms 1.12, physics/insert/ms 1.00, controller/feed/ms 0.74, terrain/select/ms 0.63, stream/drive/ms 0.47 |
| measure | 15.90 | 100.6 | render/thread/ms 185, render/acquire/ms 145, render/busy/ms 39.03, render/graph/ms 26.03, main/busy/ms 24.13, render/present/ms 1.33, physics/insert/ms 1.21, bigspace/total/ms 1.10, terrain/update/ms 1.04, controller/feed/ms 0.69, far/select/ms 0.59, terrain/select/ms 0.56 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-on\tex-slab-on.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab2_on\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 15%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **87%** (min 48%) · 2580 MHz (min 2580) · memory 8001 MHz · 67.7 W · 73 °C (max 79) · VRAM 7840 MiB · throttled 100% of samples
