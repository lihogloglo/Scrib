# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1509 frames over 15.0 s · **median 9.89 ms (101 fps)** · mean 9.95 · p95 11.03 · p99 11.75 (1% low 85 fps) · worst 20.39 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 604 frames over 7.2 s, worst 580.6 ms
- hitches (> 25.0 ms): **0** measured, 7 in warm-up
- timed GPU passes: 8.74 ms of a 9.95 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.89 ms, limit 11 ms
- pass worst frame 20.39 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.382 | 2.922 | 0.046 |
| dlss_super_resolution | 2.261 | 2.748 | 0.181 |
| early prepass | 0.673 | 1.167 | 0.082 |
| lightprobe_irradiance_map | 0.427 | 0.432 | 0.001 |
| atmosphere_luts | 0.396 | 0.840 | 0.003 |
| bloom | 0.347 | 0.504 | 0.063 |
| lightprobe_radiance_map | 0.233 | 0.236 | 0.004 |
| ssao | 0.225 | 0.218 | 0.007 |
| main_transparent_pass_3d | 0.195 | 0.219 | 0.042 |
| water_fft | 0.141 | 0.129 | 0.002 |
| uniform_allocation | 0.140 | 0.135 | 0.013 |
| clouds_trace | 0.140 | 0.166 | 0.001 |
| upscaling | 0.122 | 0.137 | 0.004 |
| bin_unpacking | 0.107 | 0.105 | 0.010 |
| atmos_froxel_inject | 0.097 | 0.095 | 0.001 |
| tonemapping | 0.094 | 0.096 | 0.003 |
| early_mesh_preprocessing | 0.093 | 0.094 | 0.003 |
| gpu_instance_materialization | 0.089 | 0.093 | 0.003 |
| water_velocity | 0.081 | 0.081 | 0.001 |
| clustering | 0.070 | 0.078 | 0.021 |
| clouds_composite | 0.069 | 0.066 | 0.002 |
| render_sky | 0.055 | 0.054 | 0.002 |
| per_view_shadow_pass | 0.041 | 0.044 | 0.008 |
| clouds_resolve | 0.036 | 0.041 | 0.001 |
| range_unpacking | 0.036 | 0.038 | 0.003 |
| clouds_shadow | 0.032 | 0.036 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.032 | 0.001 |
| water_snapshot | 0.026 | 0.024 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.002 |
| lightprobe_downsampling_first_pass | 0.015 | 0.015 | 0.001 |
| lightprobe_copy | 0.014 | 0.015 | 0.003 |
| lightprobe_downsampling_second_pass | 0.012 | 0.012 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| atmosphere_environment | 0.012 | 0.014 | 0.003 |
| sparse buffer updates | 0.007 | 0.008 | 0.003 |
| night_sky | 0.004 | 0.008 | 0.002 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.001 | 0.002 | 0.005 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.721 | 9.807 | 28.64 | 13160.5 |
| render/busy/ms | 7.448 | 8.431 | 16.58 | 11239.5 |
| main/busy/ms | 6.888 | 7.820 | 8.78 | 10393.9 |
| render/graph/ms | 3.908 | 4.746 | 9.22 | 5897.2 |
| render/acquire/ms | 0.806 | 1.860 | 2.72 | 1215.9 |
| render/present/ms | 0.467 | 0.571 | 11.77 | 705.0 |
| bigspace/total/ms | 0.336 | 0.690 | 1.78 | 506.5 |
| terrain/update/ms | 0.229 | 0.290 | 0.66 | 346.1 |
| terrain/select/ms | 0.197 | 0.248 | 0.61 | 297.0 |
| stream/drive/ms | 0.195 | 0.338 | 0.58 | 294.6 |
| far/select/ms | 0.178 | 0.305 | 0.51 | 267.9 |
| stream/tick/ms | 0.168 | 0.295 | 0.55 | 253.9 |
| controller/move/ms | 0.152 | 0.255 | 0.49 | 229.6 |
| controller/feed/ms | 0.140 | 0.207 | 0.36 | 211.6 |
| bigspace/high_precision/ms | 0.118 | 0.279 | 1.55 | 178.2 |
| anim/evaluate/ms | 0.111 | 0.300 | 1.08 | 167.4 |
| anim/apply/ms | 0.084 | 0.241 | 1.09 | 127.5 |
| bigspace/low_precision/ms | 0.079 | 0.185 | 0.94 | 118.6 |
| anim/evaluate_cpu/ms | 0.062 | 0.081 | 0.26 | 93.5 |
| physics/step/ms | 0.029 | 0.056 | 0.22 | 43.3 |
| weather/map/ms | 0.026 | 0.034 | 0.29 | 38.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 16.7 |
| script/update/ms | 0.007 | 0.014 | 0.16 | 10.6 |
| controller/avoid/ms | 0.007 | 0.011 | 0.10 | 10.0 |
| script/ui/ms | 0.005 | 0.008 | 0.27 | 7.2 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.2 |
| controller/write/ms | 0.002 | 0.004 | 0.02 | 3.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.01 | 1.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| frame/work/ms | ms | 8.72 | 16.58 | 8.53 |
| gpu/timed/ms | ms | 8.70 | 10.03 | 8.53 |
| mesh_allocator_allocations |  meshes | 9463 | 9463 | 9463 |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 309.15 M | 309.15 M | 309.15 M |
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
| phases/shadow/bins | count | 859 | 872 | 859 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 2848 | 2876 | 2848 |
| phases/shadow/pipelines | count | 27 | 30 | 27 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 153 | 153 | 153 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 155 | 163 | 155 |
| preprocess/bind_groups_created | count | 148 | 156 | 148 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 24.52 | 32.69 | 26.21 |
| process/mem_usage | GiB | 2.14 | 2.19 | 2.19 |
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
| started_load_count |  | 4193 | 4195 | 4195 |
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
| system/cpu_usage | % | 32.92 | 38.96 | 32.01 |
| system/mem_usage | % | 48.70 | 48.89 | 48.68 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 84 | 87 | 84 |
| views/all/bins | count | 3111 | 3124 | 3111 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 29.3 k | 29.3 k | 29.3 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 9108 | 9136 | 9108 |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 53 | 53 | 53 |
| views/all/work_items | count | 255.0 k | 255.0 k | 255.0 k |
| views/cascade/batch_sets | count | 34 | 37 | 34 |
| views/cascade/bins | count | 859 | 872 | 859 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 17.6 k | 17.6 k | 17.6 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 2848 | 2876 | 2848 |
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
| anim/events/count | count | 0.11 | 2 | 163 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.28 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.60 | 1 | 901 |
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

## Hitches (> 25.0 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 580.6 | main/busy/ms 559, surface/advance_cpu_ms 64.58, surface/cell_upload_ms 0.70, stream/drive/ms 0.28, weather/map/ms 0.11, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.06, stream/tick/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.91 | 331.0 | main/busy/ms 592, render/thread/ms 293, render/busy/ms 214, render/acquire/ms 74.55, surface/advance_cpu_ms 64.58, render/graph/ms 4.45, render/present/ms 3.99, stream/drive/ms 0.98, surface/cell_upload_ms 0.70, stream/tick/ms 0.52, terrain/update/ms 0.51, terrain/select/ms 0.32 |
| warm-up | 1.00 | 85.1 | pipelines/wait_ms 997, render/thread/ms 315, render/busy/ms 235, pipelines/compile_ms 102, pipelines/front_ms 94.65, render/acquire/ms 74.84, main/busy/ms 52.56, terrain/update/ms 18.15, render/graph/ms 9.48, pipelines/driver_ms 7.33, render/present/ms 5.20, stream/drive/ms 1.05 |
| warm-up | 1.06 | 25.8 | render/thread/ms 40.07, render/acquire/ms 21.76, main/busy/ms 20.68, render/busy/ms 16.78, terrain/update/ms 12.70, render/graph/ms 5.37, render/present/ms 1.52, stream/drive/ms 1.13, stream/tick/ms 0.94, terrain/select/ms 0.63, bigspace/total/ms 0.17, bigspace/high_precision/ms 0.09 |
| warm-up | 1.22 | 103.0 | pipelines/wait_ms 1011, pipelines/compile_ms 216, pipelines/front_ms 200, main/busy/ms 74.32, render/thread/ms 56.45, render/busy/ms 54.83, pipelines/driver_ms 16.03, render/graph/ms 4.58, terrain/update/ms 3.23, stream/drive/ms 1.94, render/present/ms 1.37, stream/tick/ms 1.29 |
| warm-up | 4.17 | 32.9 | main/busy/ms 37.81, render/thread/ms 26.63, render/busy/ms 24.92, render/graph/ms 8.44, stream/drive/ms 1.41, render/present/ms 1.23, bigspace/total/ms 0.92, stream/tick/ms 0.90, terrain/update/ms 0.73, terrain/select/ms 0.62, render/acquire/ms 0.49, physics/insert/ms 0.41 |
| warm-up | 6.73 | 29.0 | main/busy/ms 42.59, render/thread/ms 24.33, render/busy/ms 22.95, render/graph/ms 9.91, physics/insert/ms 1.28, render/present/ms 1.20, stream/drive/ms 1.13, bigspace/total/ms 0.75, physics/step/ms 0.71, anim/evaluate_cpu/ms 0.63, anim/evaluate/ms 0.52, terrain/update/ms 0.47 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --scene --resolution 2560x1440 --bench-out target/perf\fix4_on\balmora@2560x1440_--scene-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 12%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (59 samples): GPU load **97%** (min 74%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.3 W · 76 °C (max 77) · VRAM 3712 MiB · throttled 100% of samples
