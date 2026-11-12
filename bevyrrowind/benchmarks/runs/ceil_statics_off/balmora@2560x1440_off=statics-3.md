# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: near · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1950 frames over 15.0 s · **median 7.61 ms (131 fps)** · mean 7.69 · p95 8.74 · p99 9.32 (1% low 107 fps) · worst 26.00 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 0.0/min)
- warm-up: 312 frames over 3.3 s, worst 580.7 ms
- hitches (> 25.0 ms): **1** measured, 4 in warm-up
- timed GPU passes: 7.34 ms of a 7.69 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 7.61 ms, limit 11 ms
- pass worst frame 26.00 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.229 | 2.732 | 0.181 |
| main_opaque_pass_3d | 1.694 | 2.289 | 0.019 |
| early prepass | 0.460 | 0.787 | 0.033 |
| lightprobe_irradiance_map | 0.450 | 0.766 | 0.001 |
| atmosphere_luts | 0.362 | 0.672 | 0.003 |
| bloom | 0.346 | 0.499 | 0.057 |
| lightprobe_radiance_map | 0.237 | 0.239 | 0.003 |
| ssao | 0.227 | 0.221 | 0.006 |
| main_transparent_pass_3d | 0.173 | 0.193 | 0.012 |
| clouds_trace | 0.135 | 0.156 | 0.001 |
| water_fft | 0.131 | 0.128 | 0.002 |
| upscaling | 0.102 | 0.129 | 0.004 |
| atmos_froxel_inject | 0.096 | 0.094 | 0.001 |
| tonemapping | 0.094 | 0.094 | 0.003 |
| water_velocity | 0.085 | 0.078 | 0.001 |
| clustering | 0.068 | 0.078 | 0.020 |
| clouds_composite | 0.067 | 0.066 | 0.002 |
| render_sky | 0.055 | 0.054 | 0.002 |
| uniform_allocation | 0.038 | 0.036 | 0.006 |
| clouds_resolve | 0.036 | 0.041 | 0.001 |
| clouds_shadow | 0.033 | 0.037 | 0.003 |
| bin_unpacking | 0.031 | 0.031 | 0.005 |
| water_mips | 0.025 | 0.024 | 0.002 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| atmos_froxel_integrate | 0.024 | 0.030 | 0.001 |
| per_view_shadow_pass | 0.022 | 0.029 | 0.006 |
| early_mesh_preprocessing | 0.020 | 0.020 | 0.004 |
| lightprobe_copy | 0.015 | 0.017 | 0.003 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.013 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.011 | 0.015 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| night_sky | 0.004 | 0.005 | 0.002 |
| blas_build | 0.002 | 0.002 | 0.005 |
| auxiliary_views | 0.002 | 0.003 | 0.003 |
| surface_trail | — | — | — |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 6.949 | 7.992 | 25.11 | 13551.4 |
| render/busy/ms | 4.756 | 5.747 | 7.62 | 9274.8 |
| main/busy/ms | 4.345 | 5.079 | 6.45 | 8472.8 |
| render/graph/ms | 2.765 | 3.688 | 5.26 | 5391.7 |
| render/acquire/ms | 1.739 | 2.991 | 4.34 | 3391.9 |
| render/present/ms | 0.454 | 0.566 | 18.81 | 884.6 |
| terrain/update/ms | 0.236 | 0.303 | 0.60 | 459.3 |
| bigspace/total/ms | 0.202 | 0.409 | 0.93 | 394.6 |
| terrain/select/ms | 0.202 | 0.259 | 0.56 | 394.2 |
| stream/drive/ms | 0.178 | 0.229 | 0.52 | 346.9 |
| stream/tick/ms | 0.150 | 0.196 | 0.49 | 292.8 |
| far/select/ms | 0.149 | 0.200 | 0.44 | 290.6 |
| bigspace/high_precision/ms | 0.115 | 0.251 | 0.75 | 223.4 |
| bigspace/low_precision/ms | 0.040 | 0.144 | 0.77 | 78.6 |
| weather/map/ms | 0.024 | 0.032 | 0.20 | 47.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.14 | 21.6 |
| physics/step/ms | 0.007 | 0.016 | 0.03 | 13.0 |
| script/update/ms | 0.006 | 0.007 | 0.16 | 11.0 |
| script/input/ms | 0.005 | 0.006 | 0.06 | 9.4 |
| script/ui/ms | 0.003 | 0.005 | 0.04 | 5.6 |
| controller/move/ms | 0.002 | 0.003 | 0.08 | 3.8 |
| anim/evaluate/ms | 0.002 | 0.002 | 0.15 | 3.5 |
| anim/apply/ms | 0.001 | 0.002 | 0.12 | 2.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| controller/feed/ms | 0.000 | 0.001 | 0.01 | 0.5 |
| controller/avoid/ms | 0.000 | 0.000 | 0.00 | 0.3 |
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
| frame/work/ms | ms | 7.28 | 9.34 | 7.18 |
| gpu/timed/ms | ms | 7.28 | 9.34 | 7.18 |
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
| process/cpu_usage | % | 21.15 | 35.44 | 19.23 |
| process/mem_usage | GiB | 1.84 | 1.91 | 1.91 |
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
| stream/latency/s | s | 0.28 | 0.28 | 0.28 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 656 | 656 | 656 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.63 | 44.42 | 26.79 |
| system/mem_usage | % | 47.77 | 48.05 | 48.01 |
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
| physics/steps | count | 0.46 | 2 | 900 |
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
| warm-up | 0.58 | 580.7 | main/busy/ms 559, surface/advance_cpu_ms 66.94, surface/cell_upload_ms 0.69, stream/drive/ms 0.46, weather/map/ms 0.12, stream/tick/ms 0.10, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.82 | 239.2 | main/busy/ms 594, render/thread/ms 202, render/busy/ms 131, render/acquire/ms 69.24, surface/advance_cpu_ms 66.94, render/graph/ms 5.04, render/present/ms 1.95, stream/drive/ms 1.08, surface/cell_upload_ms 0.69, stream/tick/ms 0.51, terrain/update/ms 0.41, terrain/select/ms 0.27 |
| warm-up | 1.04 | 219.5 | pipelines/wait_ms 1352, pipelines/compile_ms 224, render/thread/ms 219, pipelines/front_ms 206, render/busy/ms 146, render/acquire/ms 69.46, main/busy/ms 52.04, terrain/update/ms 19.39, pipelines/driver_ms 17.75, render/graph/ms 10.08, render/present/ms 3.25, stream/drive/ms 1.02 |
| warm-up | 1.28 | 84.7 | main/busy/ms 63.01, render/thread/ms 52.34, render/busy/ms 50.72, render/graph/ms 4.05, stream/drive/ms 1.63, render/present/ms 1.36, stream/tick/ms 1.31, terrain/update/ms 0.57, terrain/select/ms 0.44, render/acquire/ms 0.26, bigspace/total/ms 0.21, bigspace/high_precision/ms 0.10 |
| measure | 6.21 | 26.0 | render/thread/ms 33.15, render/present/ms 19.31, render/busy/ms 10.68, main/busy/ms 9.79, render/graph/ms 6.05, render/acquire/ms 3.16, stream/drive/ms 0.49, terrain/update/ms 0.47, stream/tick/ms 0.42, terrain/select/ms 0.41, far/select/ms 0.37, bigspace/total/ms 0.35 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\ceil_statics_off\balmora@2560x1440_off=statics-3 --bench-baseline none --off statics
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 14%, CPU 9%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 95%) · 2580 MHz (min 2580) · memory 8001 MHz · 100.9 W · 76 °C (max 78) · VRAM 3455 MiB · throttled 100% of samples
