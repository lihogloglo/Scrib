# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1553 frames over 15.0 s · **median 9.58 ms (104 fps)** · mean 9.66 · p95 10.74 · p99 11.36 (1% low 88 fps) · worst 21.17 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 352 frames over 4.5 s, worst 583.2 ms
- hitches (> 25.0 ms): **0** measured, 8 in warm-up
- timed GPU passes: 8.53 ms of a 9.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.58 ms, limit 11 ms
- pass worst frame 21.17 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.341 | 2.744 | 0.046 |
| dlss_super_resolution | 2.199 | 2.570 | 0.186 |
| early prepass | 0.635 | 0.964 | 0.081 |
| lightprobe_irradiance_map | 0.451 | 0.763 | 0.001 |
| atmosphere_luts | 0.373 | 0.671 | 0.003 |
| bloom | 0.342 | 0.573 | 0.062 |
| lightprobe_radiance_map | 0.234 | 0.261 | 0.004 |
| ssao | 0.222 | 0.367 | 0.007 |
| main_transparent_pass_3d | 0.188 | 0.214 | 0.042 |
| uniform_allocation | 0.137 | 0.134 | 0.014 |
| clouds_trace | 0.136 | 0.159 | 0.001 |
| water_fft | 0.130 | 0.128 | 0.003 |
| upscaling | 0.124 | 0.134 | 0.004 |
| bin_unpacking | 0.104 | 0.103 | 0.010 |
| atmos_froxel_inject | 0.095 | 0.095 | 0.001 |
| tonemapping | 0.091 | 0.092 | 0.003 |
| early_mesh_preprocessing | 0.087 | 0.090 | 0.003 |
| water_velocity | 0.082 | 0.077 | 0.001 |
| gpu_instance_materialization | 0.076 | 0.091 | 0.003 |
| clustering | 0.067 | 0.075 | 0.021 |
| clouds_composite | 0.066 | 0.066 | 0.002 |
| render_sky | 0.057 | 0.055 | 0.002 |
| per_view_shadow_pass | 0.040 | 0.044 | 0.008 |
| range_unpacking | 0.038 | 0.038 | 0.003 |
| clouds_resolve | 0.036 | 0.041 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.032 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_copy | 0.014 | 0.015 | 0.003 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.013 | 0.001 |
| atmosphere_environment | 0.011 | 0.013 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| blas_build | — | — | — |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 25.423 | 0.000 | 7766.52 | 39481.2 |
| render/thread/ms | 8.429 | 9.455 | 19.34 | 13089.6 |
| render/busy/ms | 7.395 | 8.285 | 11.15 | 11483.9 |
| main/busy/ms | 6.769 | 7.584 | 9.26 | 10512.2 |
| render/graph/ms | 3.864 | 4.531 | 6.93 | 6000.8 |
| render/acquire/ms | 0.572 | 1.470 | 2.63 | 888.0 |
| pipelines/compile_ms | 0.523 | 0.000 | 141.00 | 812.5 |
| pipelines/front_ms | 0.508 | 0.000 | 138.00 | 788.7 |
| render/present/ms | 0.462 | 0.569 | 11.37 | 717.7 |
| bigspace/total/ms | 0.296 | 0.531 | 1.78 | 460.1 |
| terrain/update/ms | 0.227 | 0.283 | 0.52 | 353.0 |
| stream/drive/ms | 0.206 | 0.355 | 0.62 | 320.6 |
| terrain/select/ms | 0.195 | 0.242 | 0.49 | 303.0 |
| far/select/ms | 0.184 | 0.327 | 0.53 | 285.4 |
| stream/tick/ms | 0.177 | 0.307 | 0.59 | 275.4 |
| controller/move/ms | 0.159 | 0.379 | 0.67 | 247.1 |
| controller/feed/ms | 0.125 | 0.182 | 0.35 | 194.9 |
| bigspace/high_precision/ms | 0.099 | 0.200 | 1.37 | 154.2 |
| anim/evaluate/ms | 0.089 | 0.259 | 1.50 | 137.6 |
| anim/apply/ms | 0.076 | 0.182 | 1.57 | 118.6 |
| bigspace/low_precision/ms | 0.067 | 0.122 | 1.19 | 103.5 |
| anim/evaluate_cpu/ms | 0.062 | 0.079 | 0.26 | 95.9 |
| physics/step/ms | 0.032 | 0.060 | 1.25 | 49.9 |
| weather/map/ms | 0.026 | 0.037 | 0.10 | 40.5 |
| pipelines/driver_ms | 0.015 | 0.000 | 4.17 | 23.8 |
| weather/eval/ms | 0.012 | 0.016 | 0.24 | 18.6 |
| physics/insert/ms | 0.007 | 0.000 | 0.92 | 11.6 |
| script/update/ms | 0.007 | 0.015 | 0.21 | 11.5 |
| controller/avoid/ms | 0.006 | 0.010 | 0.09 | 9.9 |
| script/input/ms | 0.005 | 0.006 | 0.18 | 7.4 |
| script/ui/ms | 0.005 | 0.008 | 0.18 | 7.3 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.7 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 37.3 k | 37.3 k | 37.3 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.52 | 11.15 | 8.31 |
| gpu/timed/ms | ms | 8.51 | 9.69 | 8.31 |
| mesh_allocator_allocations |  meshes | 9463 | 9463 | 9463 |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 275.13 M | 275.13 M | 275.13 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 566 | 566 | 566 |
| phases/alpha_mask3d/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 9 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 249 | 249 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 566 | 566 | 566 |
| phases/alpha_mask3d_prepass/instance_slots | count | 8192 | 8192 | 8192 |
| phases/alpha_mask3d_prepass/items | count | 320 | 320 | 320 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/opaque3d/batch_sets | count | 16 | 16 | 16 |
| phases/opaque3d/bins | count | 877 | 877 | 877 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d/pipelines | count | 13 | 13 | 13 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/bins | count | 877 | 877 | 877 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5298 | 5298 | 5298 |
| phases/opaque3d_prepass/instance_slots | count | 41.0 k | 41.0 k | 41.0 k |
| phases/opaque3d_prepass/items | count | 2810 | 2810 | 2810 |
| phases/opaque3d_prepass/pipelines | count | 13 | 13 | 13 |
| phases/shadow/batch_sets | count | 34 | 37 | 34 |
| phases/shadow/bins | count | 859 | 872 | 858 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 2848 | 2876 | 2847 |
| phases/shadow/pipelines | count | 27 | 30 | 27 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1098 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 155 | 155 | 155 |
| pipelines/waiting | count | 0 | 11 | 0 |
| preprocess/bind_groups | count | 155 | 163 | 155 |
| preprocess/bind_groups_created | count | 148 | 156 | 148 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 25.77 | 38.46 | 27.40 |
| process/mem_usage | GiB | 2.11 | 2.15 | 2.15 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 8192 | 8192 | 8192 |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2389 | 2389 | 2389 |
| scene/batch/masked/live | count | 2389 | 2389 | 2389 |
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
| scene/batch/opaque/capacity | count | 41.0 k | 41.0 k | 41.0 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 16.8 k | 16.8 k | 16.8 k |
| scene/batch/opaque/live | count | 16.8 k | 16.8 k | 16.8 k |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 101 | 101 | 101 |
| scene/chains/high_water | count | 1642 | 1642 | 1642 |
| scene/chains/live | count | 1642 | 1642 | 1642 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 0 | 0 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_bump | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/scene_live | count | 19.2 k | 19.2 k | 19.2 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 4192 | 4194 | 4194 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 174 | 174 | 174 |
| statics/materials_bound | count | 174 | 174 | 174 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 187 | 187 | 187 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.90 | 48.41 | 30.46 |
| system/mem_usage | % | 48.70 | 48.82 | 48.82 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 84 | 87 | 84 |
| views/all/bins | count | 3111 | 3124 | 3110 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 29.3 k | 29.3 k | 29.3 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 9108 | 9136 | 9107 |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 53 | 53 | 53 |
| views/all/work_items | count | 255.0 k | 255.0 k | 255.0 k |
| views/cascade/batch_sets | count | 34 | 37 | 34 |
| views/cascade/bins | count | 859 | 872 | 858 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 2848 | 2876 | 2847 |
| views/cascade/pipelines | count | 27 | 30 | 27 |
| views/cascade/work_items | count | 150.3 k | 150.3 k | 150.3 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 50 | 50 | 50 |
| views/main/bins | count | 2252 | 2252 | 2252 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 11.7 k | 11.7 k | 11.7 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 6260 | 6260 | 6260 |
| views/main/pipelines | count | 38 | 38 | 38 |
| views/main/work_items | count | 104.7 k | 104.7 k | 104.7 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 15.0 k | 15.0 k | 15.0 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 3 | 133 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.19 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| physics/inserts | count | 0.67 | 173 | 1036 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 583.2 | main/busy/ms 561, surface/advance_cpu_ms 65.26, surface/cell_upload_ms 0.68, stream/drive/ms 0.62, weather/map/ms 0.14, bigspace/total/ms 0.11, stream/tick/ms 0.10, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.97 | 387.4 | main/busy/ms 594, render/thread/ms 353, render/busy/ms 281, render/acquire/ms 69.39, surface/advance_cpu_ms 65.26, render/graph/ms 4.79, render/present/ms 2.27, stream/drive/ms 1.18, surface/cell_upload_ms 0.68, stream/tick/ms 0.48, terrain/update/ms 0.34, weather/map/ms 0.26 |
| warm-up | 1.00 | 27.7 | render/thread/ms 371, render/busy/ms 298, pipelines/wait_ms 186, render/acquire/ms 69.85, main/busy/ms 52.34, terrain/update/ms 17.65, render/graph/ms 9.93, pipelines/compile_ms 9.92, pipelines/front_ms 9.60, render/present/ms 3.63, stream/drive/ms 0.94, bigspace/total/ms 0.88 |
| warm-up | 1.24 | 88.8 | pipelines/wait_ms 201, pipelines/compile_ms 111, pipelines/front_ms 104, main/busy/ms 59.78, render/thread/ms 53.02, render/busy/ms 51.11, pipelines/driver_ms 7.59, render/graph/ms 4.96, render/present/ms 1.66, stream/drive/ms 1.30, stream/tick/ms 1.07, terrain/update/ms 0.70 |
| warm-up | 2.68 | 26.7 | main/busy/ms 36.03, render/thread/ms 21.41, render/busy/ms 20.08, render/graph/ms 6.15, stream/drive/ms 1.35, render/present/ms 1.16, bigspace/total/ms 0.54, stream/tick/ms 0.52, physics/insert/ms 0.47, terrain/update/ms 0.46, stream/spawn/ms 0.39, terrain/select/ms 0.38 |
| warm-up | 2.75 | 26.4 | pipelines/wait_ms 6432, pipelines/compile_ms 143, pipelines/front_ms 137, main/busy/ms 41.84, render/thread/ms 26.13, render/busy/ms 24.88, pipelines/driver_ms 6.37, render/graph/ms 4.47, stream/drive/ms 1.13, render/present/ms 1.01, bigspace/total/ms 0.75, anim/evaluate_cpu/ms 0.68 |
| warm-up | 2.77 | 27.4 | pipelines/wait_ms 11.3 k, pipelines/compile_ms 175, pipelines/front_ms 167, main/busy/ms 37.24, render/thread/ms 33.67, render/busy/ms 32.48, pipelines/driver_ms 7.98, render/graph/ms 7.65, stream/drive/ms 1.37, render/present/ms 0.97, bigspace/total/ms 0.84, stream/tick/ms 0.76 |
| warm-up | 2.81 | 26.4 | pipelines/wait_ms 1405, main/busy/ms 35.04, render/thread/ms 28.29, render/busy/ms 26.88, pipelines/compile_ms 14.65, pipelines/front_ms 14.12, render/graph/ms 7.58, bigspace/total/ms 1.55, stream/drive/ms 1.24, render/present/ms 1.23, bigspace/low_precision/ms 0.89, anim/evaluate_cpu/ms 0.61 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\fix4_on\balmora@2560x1440_--scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 12%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (59 samples): GPU load **97%** (min 93%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.2 W · 74 °C (max 77) · VRAM 3461 MiB · throttled 100% of samples
