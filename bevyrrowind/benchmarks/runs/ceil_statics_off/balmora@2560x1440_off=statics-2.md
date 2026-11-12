# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1976 frames over 15.0 s · **median 7.52 ms (133 fps)** · mean 7.59 · p95 8.49 · p99 8.91 (1% low 112 fps) · worst 16.56 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 306 frames over 3.2 s, worst 579.5 ms
- hitches (> 25.0 ms): **0** measured, 5 in warm-up
- timed GPU passes: 7.26 ms of a 7.59 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 7.52 ms, limit 11 ms
- pass worst frame 16.56 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.186 | 2.537 | 0.174 |
| main_opaque_pass_3d | 1.690 | 2.101 | 0.018 |
| early prepass | 0.467 | 0.805 | 0.033 |
| lightprobe_irradiance_map | 0.435 | 0.657 | 0.001 |
| atmosphere_luts | 0.355 | 0.577 | 0.003 |
| bloom | 0.337 | 0.347 | 0.057 |
| lightprobe_radiance_map | 0.239 | 0.272 | 0.003 |
| ssao | 0.218 | 0.216 | 0.006 |
| main_transparent_pass_3d | 0.166 | 0.176 | 0.012 |
| clouds_trace | 0.135 | 0.156 | 0.001 |
| water_fft | 0.133 | 0.131 | 0.002 |
| upscaling | 0.118 | 0.147 | 0.004 |
| atmos_froxel_inject | 0.092 | 0.093 | 0.001 |
| tonemapping | 0.092 | 0.094 | 0.003 |
| water_velocity | 0.081 | 0.077 | 0.001 |
| clustering | 0.068 | 0.077 | 0.019 |
| clouds_composite | 0.066 | 0.066 | 0.002 |
| render_sky | 0.055 | 0.054 | 0.002 |
| uniform_allocation | 0.037 | 0.036 | 0.007 |
| clouds_resolve | 0.037 | 0.042 | 0.001 |
| clouds_shadow | 0.033 | 0.037 | 0.003 |
| bin_unpacking | 0.029 | 0.030 | 0.005 |
| atmos_froxel_integrate | 0.024 | 0.032 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.028 | 0.006 |
| early_mesh_preprocessing | 0.021 | 0.020 | 0.004 |
| lightprobe_copy | 0.016 | 0.017 | 0.003 |
| lightprobe_downsampling_first_pass | 0.014 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.001 |
| atmosphere_environment | 0.011 | 0.014 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| sparse buffer updates | 0.006 | 0.007 | 0.002 |
| night_sky | 0.004 | 0.004 | 0.002 |
| blas_build | 0.002 | 0.002 | 0.005 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.866 | 7.779 | 15.88 | 13567.5 |
| render/busy/ms | 4.439 | 5.433 | 14.23 | 8772.4 |
| main/busy/ms | 4.184 | 4.984 | 6.35 | 8267.3 |
| render/graph/ms | 2.465 | 3.437 | 11.88 | 4870.8 |
| render/acquire/ms | 2.003 | 2.936 | 3.84 | 3957.2 |
| render/present/ms | 0.424 | 0.531 | 4.27 | 837.9 |
| terrain/update/ms | 0.239 | 0.310 | 0.62 | 472.5 |
| terrain/select/ms | 0.206 | 0.267 | 0.55 | 406.5 |
| bigspace/total/ms | 0.185 | 0.359 | 1.03 | 365.0 |
| stream/drive/ms | 0.178 | 0.216 | 0.51 | 352.3 |
| stream/tick/ms | 0.151 | 0.184 | 0.48 | 297.5 |
| far/select/ms | 0.146 | 0.173 | 0.45 | 289.1 |
| bigspace/high_precision/ms | 0.108 | 0.224 | 0.97 | 214.0 |
| bigspace/low_precision/ms | 0.039 | 0.140 | 0.80 | 76.5 |
| weather/map/ms | 0.024 | 0.032 | 0.14 | 48.3 |
| physics/step/ms | 0.011 | 0.026 | 0.18 | 21.6 |
| weather/eval/ms | 0.010 | 0.014 | 0.05 | 20.4 |
| script/update/ms | 0.006 | 0.007 | 0.15 | 11.3 |
| script/input/ms | 0.005 | 0.006 | 0.08 | 9.0 |
| script/ui/ms | 0.003 | 0.004 | 0.20 | 5.2 |
| controller/move/ms | 0.002 | 0.003 | 0.02 | 3.8 |
| anim/evaluate/ms | 0.002 | 0.002 | 0.04 | 3.4 |
| anim/apply/ms | 0.001 | 0.002 | 0.12 | 2.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.1 |
| weather/apply/ms | 0.000 | 0.000 | 0.00 | 0.5 |
| controller/feed/ms | 0.000 | 0.001 | 0.00 | 0.5 |
| controller/avoid/ms | 0.000 | 0.000 | 0.01 | 0.3 |
| controller/write/ms | 0.000 | 0.000 | 0.00 | 0.1 |
| anim/evaluate_cpu/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| anim/a3/count | count | 0 | 0 | 0 |
| anim/characters/count | count | 0 | 0 | 0 |
| anim/evaluated/count | count | 0 | 0 | 0 |
| controller/movers/count | count | 0 | 0 | 0 |
| ecs/entities | count | 6837 | 6838 | 6838 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 7.24 | 14.23 | 7.28 |
| gpu/timed/ms | ms | 7.24 | 8.94 | 7.28 |
| mesh_allocator_allocations |  meshes | 8081 | 8081 | 8081 |
| mesh_allocator_slabs |  slabs | 11 | 11 | 11 |
| mesh_allocator_slabs_size |  bytes | 298.98 M | 298.98 M | 298.98 M |
| phases/alpha_mask3d/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d/bins | count | 219 | 219 | 219 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 219 | 219 | 219 |
| phases/alpha_mask3d/pipelines | count | 1 | 1 | 1 |
| phases/alpha_mask3d_prepass/batch_sets | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/bins | count | 219 | 219 | 219 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 219 | 219 | 219 |
| phases/alpha_mask3d_prepass/pipelines | count | 1 | 1 | 1 |
| phases/opaque3d/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d/bins | count | 247 | 247 | 247 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 371 | 371 | 371 |
| phases/opaque3d/pipelines | count | 3 | 3 | 3 |
| phases/opaque3d_prepass/batch_sets | count | 4 | 4 | 4 |
| phases/opaque3d_prepass/bins | count | 247 | 247 | 247 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 371 | 371 | 371 |
| phases/opaque3d_prepass/pipelines | count | 3 | 3 | 3 |
| phases/shadow/batch_sets | count | 5 | 5 | 5 |
| phases/shadow/bins | count | 75 | 75 | 75 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 197 | 197 | 197 |
| phases/shadow/pipelines | count | 5 | 5 | 5 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2077 | 2077 | 2077 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 106 | 106 | 106 |
| pipelines/waiting | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 54 | 54 | 54 |
| preprocess/bind_groups_created | count | 54 | 54 | 54 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 20.67 | 35.10 | 20.29 |
| process/mem_usage | GiB | 1.89 | 1.95 | 1.95 |
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
| started_load_count |  | 2372 | 2374 | 2374 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 0 | 0 | 0 |
| statics/material_classes | count | 0 | 0 | 0 |
| statics/material_keys | count | 0 | 0 | 0 |
| statics/material_pipelines | count | 0 | 0 | 0 |
| statics/materials | count | 0 | 0 | 0 |
| statics/materials_bound | count | 0 | 0 | 0 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.24 | 0.24 | 0.24 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 656 | 656 | 656 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 26.13 | 41.04 | 24.92 |
| system/mem_usage | % | 47.53 | 47.78 | 47.77 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 17 | 17 | 17 |
| views/all/bins | count | 1007 | 1007 | 1007 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 1377 | 1377 | 1377 |
| views/all/pipelines | count | 13 | 13 | 13 |
| views/all/pipelines_distinct | count | 11 | 11 | 11 |
| views/all/work_items | count | 1501 | 1501 | 1501 |
| views/cascade/batch_sets | count | 5 | 5 | 5 |
| views/cascade/bins | count | 75 | 75 | 75 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 197 | 197 | 197 |
| views/cascade/pipelines | count | 5 | 5 | 5 |
| views/cascade/work_items | count | 197 | 197 | 197 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 12 | 12 | 12 |
| views/main/bins | count | 932 | 932 | 932 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 1180 | 1180 | 1180 |
| views/main/pipelines | count | 8 | 8 | 8 |
| views/main/work_items | count | 1304 | 1304 | 1304 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 2697 | 2697 | 2697 |
| world/point_lights | count | 0 | 0 | 0 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0 | 0 | 0 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.46 | 1 | 900 |
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

## Hitches (> 25.0 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 579.5 | main/busy/ms 557, surface/advance_cpu_ms 65.34, surface/cell_upload_ms 0.71, stream/drive/ms 0.30, weather/map/ms 0.16, stream/tick/ms 0.09, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, script/ui/ms 0.01 |
| warm-up | 1.03 | 448.9 | main/busy/ms 592, render/thread/ms 411, render/busy/ms 337, render/acquire/ms 72.10, surface/advance_cpu_ms 65.34, render/graph/ms 4.77, render/present/ms 1.99, stream/drive/ms 0.98, surface/cell_upload_ms 0.71, stream/tick/ms 0.54, terrain/update/ms 0.38, weather/map/ms 0.28 |
| warm-up | 1.17 | 27.5 | render/thread/ms 31.68, render/busy/ms 17.29, render/present/ms 14.16, main/busy/ms 13.96, render/graph/ms 4.28, stream/drive/ms 2.07, stream/tick/ms 1.78, terrain/update/ms 0.76, terrain/select/ms 0.61, render/acquire/ms 0.23, bigspace/total/ms 0.18, bigspace/high_precision/ms 0.11 |
| warm-up | 1.29 | 91.7 | pipelines/compile_ms 209, pipelines/front_ms 188, main/busy/ms 63.77, render/thread/ms 58.74, render/busy/ms 56.93, pipelines/driver_ms 20.50, pipelines/wait_ms 13.77, render/graph/ms 4.84, stream/drive/ms 1.71, render/present/ms 1.56, stream/tick/ms 1.36, terrain/update/ms 0.59 |
| warm-up | 1.34 | 28.5 | pipelines/wait_ms 202, pipelines/compile_ms 52.04, pipelines/front_ms 47.96, render/thread/ms 37.82, render/busy/ms 36.72, main/busy/ms 19.55, pipelines/driver_ms 4.08, stream/drive/ms 2.86, render/graph/ms 2.76, stream/tick/ms 1.67, render/present/ms 0.93, stream/spawn/ms 0.74 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ceil_statics_off\balmora@2560x1440_off=statics-2 --bench-baseline none --off statics
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (59 samples): GPU load **98%** (min 93%) · 2580 MHz (min 2580) · memory 8001 MHz · 100.0 W · 73 °C (max 76) · VRAM 3470 MiB · throttled 100% of samples
