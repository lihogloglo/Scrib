# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cpuculltag · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1556 frames over 15.0 s · **median 9.35 ms (107 fps)** · mean 9.64 · p95 11.09 · p99 18.99 (1% low 53 fps) · worst 24.88 ms · 34 spikes over 1.5× median · 116.0 stutters/min (work 88.0/min)
- warm-up: 403 frames over 4.9 s, worst 531.0 ms
- hitches (> 25.0 ms): **0** measured, 11 in warm-up
- timed GPU passes: 8.85 ms of a 9.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.35 ms, limit 11 ms
- pass worst frame 24.88 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| main_opaque_pass_3d | 2.274 | 2.863 | 0.046 |
| dlss_super_resolution | 2.215 | 2.818 | 0.201 |
| atmosphere_luts | 1.058 | 1.381 | 0.004 |
| early prepass | 0.615 | 0.881 | 0.084 |
| lightprobe_irradiance_map | 0.443 | 0.675 | 0.001 |
| bloom | 0.340 | 0.492 | 0.072 |
| lightprobe_radiance_map | 0.237 | 0.368 | 0.005 |
| ssao | 0.227 | 0.351 | 0.008 |
| main_transparent_pass_3d | 0.189 | 0.209 | 0.076 |
| uniform_allocation | 0.137 | 0.133 | 0.027 |
| water_fft | 0.130 | 0.129 | 0.003 |
| upscaling | 0.126 | 0.151 | 0.005 |
| tonemapping | 0.101 | 0.097 | 0.004 |
| bin_unpacking | 0.096 | 0.099 | 0.019 |
| atmos_froxel_inject | 0.079 | 0.080 | 0.002 |
| water_velocity | 0.078 | 0.076 | 0.002 |
| clouds_trace | 0.068 | 0.087 | 0.001 |
| clustering | 0.067 | 0.077 | 0.024 |
| clouds_composite | 0.066 | 0.066 | 0.002 |
| render_sky | 0.056 | 0.055 | 0.002 |
| clouds_resolve | 0.035 | 0.039 | 0.001 |
| early_mesh_preprocessing | 0.034 | 0.035 | 0.006 |
| water_mips | 0.029 | 0.026 | 0.002 |
| atmos_froxel_integrate | 0.024 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.002 |
| lightprobe_downsampling_first_pass | 0.017 | 0.014 | 0.001 |
| per_view_shadow_pass | 0.016 | 0.022 | 0.006 |
| lightprobe_copy | 0.014 | 0.015 | 0.004 |
| lightprobe_downsampling_second_pass | 0.013 | 0.012 | 0.001 |
| clouds_fog_light | 0.013 | 0.013 | 0.002 |
| atmosphere_environment | 0.011 | 0.014 | 0.003 |
| night_sky | 0.007 | 0.012 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 |
| auxiliary_views | 0.002 | 0.003 | 0.004 |
| blas_build | 0.002 | 0.003 | 0.006 |
| surface_trail | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 37.521 | 0.000 | 4494.46 | 58383.2 |
| render/thread/ms | 8.678 | 10.107 | 23.73 | 13503.1 |
| main/busy/ms | 7.192 | 8.561 | 19.05 | 11190.2 |
| render/busy/ms | 6.927 | 8.343 | 19.39 | 10778.0 |
| render/graph/ms | 3.874 | 4.928 | 13.24 | 6027.8 |
| render/acquire/ms | 1.256 | 2.523 | 4.35 | 1953.9 |
| pipelines/compile_ms | 0.713 | 0.000 | 150.24 | 1109.0 |
| pipelines/front_ms | 0.691 | 0.000 | 145.31 | 1075.4 |
| render/present/ms | 0.496 | 0.680 | 4.27 | 771.1 |
| bigspace/total/ms | 0.411 | 0.941 | 2.59 | 639.7 |
| terrain/update/ms | 0.241 | 0.322 | 0.78 | 375.4 |
| terrain/select/ms | 0.209 | 0.280 | 0.71 | 324.9 |
| stream/drive/ms | 0.195 | 0.292 | 1.08 | 303.1 |
| far/select/ms | 0.179 | 0.275 | 1.02 | 278.8 |
| stream/tick/ms | 0.166 | 0.253 | 0.98 | 259.1 |
| controller/move/ms | 0.156 | 0.293 | 0.76 | 242.2 |
| bigspace/high_precision/ms | 0.147 | 0.411 | 1.77 | 228.0 |
| controller/feed/ms | 0.142 | 0.208 | 0.79 | 220.6 |
| anim/evaluate/ms | 0.108 | 0.248 | 1.43 | 167.9 |
| bigspace/low_precision/ms | 0.101 | 0.223 | 1.52 | 157.6 |
| anim/evaluate_cpu/ms | 0.061 | 0.088 | 0.35 | 94.9 |
| physics/step/ms | 0.033 | 0.071 | 0.43 | 50.7 |
| pipelines/driver_ms | 0.022 | 0.000 | 4.93 | 33.6 |
| weather/map/ms | 0.018 | 0.024 | 0.13 | 27.8 |
| weather/eval/ms | 0.011 | 0.015 | 0.40 | 17.7 |
| script/update/ms | 0.007 | 0.012 | 0.10 | 11.6 |
| controller/avoid/ms | 0.006 | 0.008 | 0.06 | 9.6 |
| script/ui/ms | 0.005 | 0.008 | 0.15 | 8.2 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 8.1 |
| anim/apply/ms | 0.005 | 0.007 | 0.12 | 7.6 |
| physics/insert/ms | 0.003 | 0.000 | 0.61 | 4.8 |
| controller/write/ms | 0.002 | 0.002 | 0.11 | 3.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
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
| anim/a2/count | count | 0 | 2 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 14 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 187.32 M | 187.32 M | 187.32 M |
| far/triangles | count | 4.62 M | 4.62 M | 4.62 M |
| frame/work/ms | ms | 8.73 | 19.39 | 8.70 |
| gpu/timed/ms | ms | 8.72 | 11.88 | 8.70 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 433.72 M | 433.72 M | 433.72 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 282 | 282 | 282 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 876 | 876 | 876 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 282 | 282 | 282 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 876 | 876 | 876 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d/bins | count | 1307 | 1308 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/bins | count | 1307 | 1308 | 1307 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3780 | 3780 | 3780 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 13 | 14 | 13 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 16 | 16 | 16 |
| phases/shadow/bins | count | 988 | 988 | 988 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 2009 | 2009 | 2009 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 16 | 16 | 16 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 124 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 154 | 154 | 154 |
| pipelines/waiting | count | 0 | 15 | 0 |
| preprocess/bind_groups | count | 0 | 98 | 0 |
| preprocess/bind_groups_created | count | 0 | 98 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 139 | 139 | 139 |
| process/cpu_usage | % | 26.08 | 41.35 | 36.16 |
| process/mem_usage | GiB | 1.97 | 2.21 | 2.21 |
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
| started_load_count |  | 2595 | 2597 | 2597 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 388 | 388 | 388 |
| statics/materials_bound | count | 388 | 388 | 388 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 42.35 | 80.70 | 44.23 |
| system/mem_usage | % | 49.73 | 50.33 | 50.33 |
| terrain/patches | count | 370 | 370 | 370 |
| views/all/batch_sets | count | 60 | 62 | 60 |
| views/all/bins | count | 4166 | 4168 | 4166 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 56 | 58 | 56 |
| views/all/pipelines_distinct | count | 55 | 57 | 55 |
| views/all/work_items | count | 11.5 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 16 | 16 | 16 |
| views/cascade/bins | count | 988 | 988 | 988 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2009 | 2009 | 2009 |
| views/cascade/pipelines | count | 16 | 16 | 16 |
| views/cascade/work_items | count | 2009 | 2009 | 2009 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 44 | 46 | 44 |
| views/main/bins | count | 3178 | 3180 | 3178 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9312 | 9312 | 9312 |
| views/main/pipelines | count | 40 | 42 | 40 |
| views/main/work_items | count | 9490 | 9490 | 9490 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 2 | 161 |
| assets/Image/added | count | 0.00 | 2 | 2 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.02 | 25 | 25 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.00 | 2 | 2 |
| assets/StandardMaterial/modified | count | 3.18 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.47 | 721 | 736 |
| physics/inserts | count | 0.09 | 26 | 137 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.58 | 2 | 900 |
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
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 11, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.53 | 531.0 | main/busy/ms 506, surface/advance_cpu_ms 0.73, stream/drive/ms 0.33, surface/cell_upload_ms 0.30, bigspace/total/ms 0.09, stream/tick/ms 0.08, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, weather/map/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.89 | 363.8 | main/busy/ms 548, render/thread/ms 317, render/busy/ms 237, render/acquire/ms 78.14, render/graph/ms 4.84, render/present/ms 2.10, stream/drive/ms 0.83, surface/advance_cpu_ms 0.73, terrain/update/ms 0.50, stream/tick/ms 0.41, bigspace/total/ms 0.38, terrain/select/ms 0.36 |
| warm-up | 0.97 | 72.9 | pipelines/wait_ms 425, render/thread/ms 328, render/busy/ms 246, render/acquire/ms 78.32, pipelines/compile_ms 69.59, pipelines/front_ms 63.53, main/busy/ms 58.62, terrain/update/ms 25.38, render/graph/ms 8.49, pipelines/driver_ms 6.06, render/present/ms 2.95, bigspace/total/ms 1.47 |
| warm-up | 1.22 | 87.6 | pipelines/wait_ms 716, pipelines/compile_ms 109, pipelines/front_ms 99.67, main/busy/ms 70.61, render/thread/ms 63.15, render/busy/ms 61.31, pipelines/driver_ms 9.60, render/graph/ms 4.82, render/present/ms 1.56, stream/drive/ms 1.48, stream/tick/ms 1.13, terrain/update/ms 0.54 |
| warm-up | 1.34 | 28.3 | render/thread/ms 37.68, render/busy/ms 36.28, main/busy/ms 16.33, stream/drive/ms 4.27, render/graph/ms 3.26, render/present/ms 1.22, stream/spawn/ms 1.01, stream/tick/ms 0.59, terrain/update/ms 0.50, terrain/select/ms 0.38, bigspace/total/ms 0.22, render/acquire/ms 0.18 |
| warm-up | 1.37 | 27.8 | pipelines/wait_ms 211, pipelines/compile_ms 194, pipelines/front_ms 182, render/thread/ms 28.33, render/busy/ms 26.59, main/busy/ms 14.50, pipelines/driver_ms 11.91, render/graph/ms 4.09, stream/drive/ms 1.85, render/present/ms 1.48, stream/tick/ms 0.63, terrain/update/ms 0.59 |
| warm-up | 1.41 | 28.1 | render/thread/ms 34.51, render/busy/ms 33.23, main/busy/ms 17.18, render/graph/ms 3.69, stream/drive/ms 3.25, render/present/ms 1.08, stream/tick/ms 0.74, terrain/update/ms 0.62, stream/spawn/ms 0.62, terrain/select/ms 0.48, render/acquire/ms 0.20, bigspace/total/ms 0.19 |
| warm-up | 1.78 | 26.1 | pipelines/wait_ms 3495, pipelines/compile_ms 156, pipelines/front_ms 151, render/thread/ms 34.17, render/busy/ms 32.84, main/busy/ms 18.89, pipelines/driver_ms 5.33, stream/drive/ms 3.98, render/graph/ms 3.93, stream/spawn/ms 2.07, render/present/ms 1.07, bigspace/total/ms 0.90 |
| warm-up | 2.44 | 25.3 | pipelines/wait_ms 4271, main/busy/ms 41.29, pipelines/compile_ms 39.84, pipelines/front_ms 37.12, render/thread/ms 19.07, render/busy/ms 17.13, render/graph/ms 4.36, pipelines/driver_ms 2.72, render/present/ms 1.73, stream/drive/ms 1.59, stream/tick/ms 0.73, bigspace/total/ms 0.63 |
| warm-up | 2.49 | 28.7 | main/busy/ms 37.68, render/thread/ms 32.13, render/busy/ms 30.87, render/graph/ms 4.75, stream/drive/ms 1.49, bigspace/total/ms 1.32, render/present/ms 1.01, bigspace/high_precision/ms 0.73, stream/tick/ms 0.70, terrain/update/ms 0.58, terrain/select/ms 0.45, anim/evaluate/ms 0.40 |
| warm-up | 2.55 | 36.4 | pipelines/wait_ms 6009, pipelines/compile_ms 124, pipelines/front_ms 118, main/busy/ms 47.59, render/thread/ms 21.73, render/busy/ms 20.13, pipelines/driver_ms 6.58, render/graph/ms 5.03, render/present/ms 1.41, stream/drive/ms 1.23, bigspace/total/ms 0.78, anim/evaluate_cpu/ms 0.54 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\push_nocull\balmora_night@2560x1440_off=cpuculltag-2 --bench-baseline none --off cpuculltag
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 16% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **96%** (min 77%) · 2580 MHz (min 2580) · memory 8001 MHz · 90.2 W · 75 °C (max 77) · VRAM 3951 MiB · throttled 100% of samples
