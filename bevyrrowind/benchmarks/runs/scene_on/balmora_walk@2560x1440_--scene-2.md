# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4436 frames over 65.7 s · **median 14.50 ms (69 fps)** · mean 14.80 · p95 17.92 · p99 20.71 (1% low 48 fps) · worst 30.42 ms · 27 spikes over 1.5× median · 21.9 stutters/min (work 30.2/min)
- warm-up: 362 frames over 6.0 s, worst 589.3 ms
- hitches (> 36.3 ms): **0** measured, 5 in warm-up
- timed GPU passes: 7.66 ms of a 14.80 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 14.50 ms, limit 11 ms
- **FAIL** frame_ms: 24 stutters (> 1.5x moving median of 60) in 1.09 min = 21.93/min, limit 1/min
- pass worst frame 30.42 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.246 | 2.989 | 0.197 |
| main_opaque_pass_3d | 1.453 | 2.241 | 0.054 |
| early prepass | 0.519 | 1.050 | 0.090 |
| lightprobe_irradiance_map | 0.460 | 0.827 | 0.002 |
| bloom | 0.344 | 0.514 | 0.075 |
| atmosphere_luts | 0.333 | 0.500 | 0.003 |
| lightprobe_radiance_map | 0.226 | 0.234 | 0.004 |
| ssao | 0.222 | 0.214 | 0.008 |
| auxiliary_views | 0.179 | 0.689 | 0.180 |
| uniform_allocation | 0.168 | 0.170 | 0.020 |
| water_fft | 0.135 | 0.131 | 0.003 |
| clouds_trace | 0.127 | 0.213 | 0.001 |
| upscaling | 0.127 | 0.136 | 0.005 |
| bin_unpacking | 0.124 | 0.130 | 0.016 |
| atmos_froxel_inject | 0.098 | 0.104 | 0.002 |
| tonemapping | 0.093 | 0.091 | 0.004 |
| water_velocity | 0.089 | 0.085 | 0.002 |
| surface_trail | 0.077 | 0.061 | 0.002 |
| early_mesh_preprocessing | 0.071 | 0.076 | 0.004 |
| clustering | 0.068 | 0.069 | 0.021 |
| clouds_composite | 0.065 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.062 | 0.109 | 0.114 |
| render_sky | 0.054 | 0.054 | 0.002 |
| gpu_instance_materialization | 0.051 | 0.049 | 0.004 |
| per_view_shadow_pass | 0.039 | 0.038 | 0.009 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.031 | 0.034 | 0.003 |
| atmos_froxel_integrate | 0.026 | 0.030 | 0.001 |
| water_snapshot | 0.025 | 0.024 | 0.002 |
| water_mips | 0.024 | 0.024 | 0.003 |
| range_unpacking | 0.019 | 0.019 | 0.004 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.013 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.006 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.915 | 14.654 | 27.27 | 52855.7 |
| render/busy/ms | 11.327 | 13.962 | 26.11 | 50247.4 |
| main/busy/ms | 9.887 | 11.920 | 22.51 | 43858.8 |
| render/graph/ms | 5.226 | 6.776 | 17.13 | 23181.2 |
| bigspace/total/ms | 0.505 | 0.854 | 2.50 | 2238.6 |
| render/present/ms | 0.491 | 0.631 | 3.85 | 2179.5 |
| controller/feed/ms | 0.303 | 0.450 | 0.94 | 1343.4 |
| terrain/update/ms | 0.291 | 0.367 | 1.69 | 1292.4 |
| terrain/select/ms | 0.254 | 0.319 | 0.72 | 1128.0 |
| stream/drive/ms | 0.229 | 0.358 | 1.96 | 1015.1 |
| anim/evaluate/ms | 0.200 | 0.403 | 1.98 | 887.2 |
| far/select/ms | 0.185 | 0.277 | 0.72 | 822.5 |
| bigspace/low_precision/ms | 0.180 | 0.307 | 1.44 | 798.3 |
| stream/tick/ms | 0.179 | 0.233 | 0.66 | 792.6 |
| anim/evaluate_cpu/ms | 0.171 | 0.228 | 0.59 | 757.6 |
| pipelines/wait_ms | 0.153 | 0.000 | 677.65 | 677.7 |
| controller/move/ms | 0.135 | 0.294 | 0.68 | 600.9 |
| bigspace/high_precision/ms | 0.117 | 0.253 | 1.86 | 518.4 |
| anim/apply/ms | 0.115 | 0.279 | 1.58 | 512.1 |
| render/acquire/ms | 0.097 | 0.129 | 4.28 | 428.8 |
| physics/step/ms | 0.067 | 0.101 | 0.54 | 298.7 |
| pipelines/compile_ms | 0.061 | 0.000 | 150.02 | 271.3 |
| pipelines/front_ms | 0.059 | 0.000 | 144.97 | 262.2 |
| physics/insert/ms | 0.040 | 0.383 | 1.01 | 179.4 |
| weather/map/ms | 0.028 | 0.036 | 0.29 | 122.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.22 | 50.3 |
| controller/avoid/ms | 0.011 | 0.015 | 0.13 | 49.0 |
| script/update/ms | 0.008 | 0.011 | 0.18 | 33.8 |
| stream/despawn/ms | 0.006 | 0.000 | 1.18 | 27.6 |
| script/ui/ms | 0.006 | 0.008 | 0.17 | 26.5 |
| script/input/ms | 0.005 | 0.007 | 0.07 | 23.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.40 | 22.1 |
| controller/write/ms | 0.004 | 0.006 | 0.16 | 16.3 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 12.6 |
| pipelines/driver_ms | 0.002 | 0.000 | 5.05 | 9.2 |
| stream/upload/ms | 0.001 | 0.000 | 0.36 | 6.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.38 | 4.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 2 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 19 |
| controller/movers/count | count | 20 | 23 | 19 |
| ecs/entities | count | 70.6 k | 79.7 k | 71.1 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 11.10 | 26.11 | 9.99 |
| gpu/timed/ms | ms | 7.54 | 11.95 | 7.90 |
| mesh_allocator_allocations |  meshes | 13.5 k | 14.7 k | 13.4 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 323.10 M | 323.10 M | 323.10 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 185 | 400 | 393 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 805 | 1201 | 579 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 185 | 400 | 393 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 805 | 1201 | 579 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/opaque3d/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d/bins | count | 1040 | 1186 | 1088 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 1983 | 2058 | 2058 |
| phases/opaque3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d/items | count | 4124 | 5665 | 3428 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 1040 | 1186 | 1088 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 1983 | 2058 | 2058 |
| phases/opaque3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/opaque3d_prepass/items | count | 4124 | 5665 | 3428 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 49 | 218 | 40 |
| phases/shadow/bins | count | 1976 | 2767 | 1258 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 7947 | 8322 | 8322 |
| phases/shadow/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| phases/shadow/items | count | 8312 | 10.9 k | 5131 |
| phases/shadow/pipelines | count | 106 | 220 | 30 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2686 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 165 | 166 | 166 |
| pipelines/waiting | count | 0 | 1 | 0 |
| preprocess/bind_groups | count | 187 | 563 | 167 |
| preprocess/bind_groups_created | count | 180 | 556 | 160 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 21.63 | 38.12 | 17.70 |
| process/mem_usage | GiB | 2.58 | 2.64 | 2.64 |
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1594 | 1624 | 1624 |
| scene/chains/live | count | 1594 | 1624 | 1624 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 12.1 k | 12.7 k | 12.7 k |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 9654 | 10.3 k | 10.3 k |
| scene/slots/batch_capacity | count | 16.4 k | 16.4 k | 16.4 k |
| scene/slots/batch_capacity_bytes | bytes | 1.05 M | 1.05 M | 1.05 M |
| scene/slots/batch_live | count | 9654 | 10.3 k | 10.3 k |
| scene/slots/scene_bump | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/scene_live | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 5363 | 5464 | 5464 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 392 | 486 | 352 |
| statics/materials_bound | count | 392 | 486 | 352 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.05 | 0.10 | 0.07 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 622 | 716 | 545 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 35.85 | 77.51 | 34.75 |
| system/mem_usage | % | 60.69 | 62.21 | 62.21 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 99 | 264 | 90 |
| views/all/bins | count | 4377 | 5228 | 4220 |
| views/all/count | count | 22 | 52 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 13.2 k | 13.9 k | 13.9 k |
| views/all/instance_slots | count | 81.9 k | 81.9 k | 81.9 k |
| views/all/items | count | 18.5 k | 24.0 k | 13.1 k |
| views/all/pipelines | count | 145 | 258 | 68 |
| views/all/pipelines_distinct | count | 62 | 66 | 53 |
| views/all/work_items | count | 100.2 k | 105.5 k | 95.4 k |
| views/cascade/batch_sets | count | 46 | 55 | 40 |
| views/cascade/bins | count | 1753 | 2130 | 1258 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 7947 | 8322 | 8322 |
| views/cascade/instance_slots | count | 49.2 k | 49.2 k | 49.2 k |
| views/cascade/items | count | 7900 | 10.2 k | 5131 |
| views/cascade/pipelines | count | 37 | 46 | 30 |
| views/cascade/work_items | count | 57.0 k | 59.3 k | 54.3 k |
| views/lamp_face/batch_sets | count | 0 | 169 | 0 |
| views/lamp_face/bins | count | 214 | 679 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 364 | 1035 | 0 |
| views/lamp_face/pipelines | count | 66 | 177 | 0 |
| views/lamp_face/work_items | count | 0 | 892 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2476 | 2976 | 2962 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 5298 | 5548 | 5548 |
| views/main/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| views/main/items | count | 9856 | 13.7 k | 8014 |
| views/main/pipelines | count | 40 | 42 | 38 |
| views/main/work_items | count | 42.8 k | 46.6 k | 41.1 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 42.7 k | 49.4 k | 42.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 55.23 | 190 | 245.0 k |
| anim/events/count | count | 0.37 | 4 | 1622 |
| assets/Image/added | count | 0.07 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.81 | 242 | 3573 |
| assets/Mesh/removed | count | 1.10 | 1220 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 488 |
| assets/StandardMaterial/modified | count | 4.88 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.63 | 1562 | 7239 |
| physics/inserts | count | 0.53 | 111 | 2353 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.53 | 145 | 2335 |
| physics/steps | count | 0.89 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 5 |
| scene/overflow/cells | count | 0.01 | 1 | 27 |
| scene/slots/overflow | count | 0.67 | 520 | 2950 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 16 | 351 |
| stream/upload/bytes | bytes | 7631 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 54 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1673 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 36.3 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 589.3 | main/busy/ms 567, surface/advance_cpu_ms 68.53, surface/cell_upload_ms 0.70, stream/drive/ms 0.27, weather/map/ms 0.12, bigspace/total/ms 0.07, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, script/update/ms 0.03, weather/eval/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.91 | 322.6 | main/busy/ms 606, render/thread/ms 282, render/busy/ms 204, render/acquire/ms 75.44, surface/advance_cpu_ms 68.53, render/graph/ms 5.31, render/present/ms 2.37, stream/drive/ms 1.08, surface/cell_upload_ms 0.70, stream/tick/ms 0.48, terrain/update/ms 0.37, bigspace/total/ms 0.30 |
| warm-up | 1.02 | 104.4 | pipelines/wait_ms 645, render/thread/ms 301, render/busy/ms 221, pipelines/compile_ms 97.01, pipelines/front_ms 89.46, render/acquire/ms 75.69, main/busy/ms 58.06, terrain/update/ms 22.83, render/graph/ms 11.42, pipelines/driver_ms 7.54, render/present/ms 3.65, stream/drive/ms 1.32 |
| warm-up | 1.27 | 92.6 | pipelines/wait_ms 396, pipelines/compile_ms 247, pipelines/front_ms 222, main/busy/ms 69.35, render/thread/ms 49.82, render/busy/ms 47.89, pipelines/driver_ms 24.71, render/graph/ms 4.82, stream/drive/ms 1.72, render/present/ms 1.72, stream/tick/ms 1.34, terrain/update/ms 0.98 |
| warm-up | 2.69 | 38.1 | main/busy/ms 51.94, render/thread/ms 38.55, render/busy/ms 37.05, render/graph/ms 8.31, stream/drive/ms 2.45, physics/insert/ms 1.87, render/present/ms 1.24, stream/spawn/ms 1.01, stream/tick/ms 0.76, bigspace/total/ms 0.67, terrain/update/ms 0.60, terrain/select/ms 0.49 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\scene_on\balmora_walk@2560x1440_--scene-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 6%, CPU 17%
- during the run: other processes 14% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **63%** (min 43%) · 2580 MHz (min 2580) · memory 8001 MHz · 63.5 W · 66 °C (max 70) · VRAM 4130 MiB · throttled 100% of samples
