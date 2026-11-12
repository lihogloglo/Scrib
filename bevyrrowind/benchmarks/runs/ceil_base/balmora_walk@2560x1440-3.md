# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5878 frames over 65.7 s · **median 10.86 ms (92 fps)** · mean 11.17 · p95 13.47 · p99 14.86 (1% low 67 fps) · worst 28.38 ms · 29 spikes over 1.5× median · 18.3 stutters/min (work 21.9/min)
- warm-up: 489 frames over 6.0 s, worst 579.9 ms
- hitches (> 27.2 ms): **1** measured, 5 in warm-up
- timed GPU passes: 7.45 ms of a 11.17 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.86 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- pass worst frame 28.38 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.226 | 2.611 | 0.180 |
| main_opaque_pass_3d | 1.459 | 2.058 | 0.044 |
| early prepass | 0.512 | 0.890 | 0.078 |
| lightprobe_irradiance_map | 0.441 | 0.757 | 0.001 |
| bloom | 0.337 | 0.496 | 0.066 |
| atmosphere_luts | 0.328 | 0.506 | 0.003 |
| lightprobe_radiance_map | 0.225 | 0.236 | 0.004 |
| ssao | 0.218 | 0.214 | 0.007 |
| uniform_allocation | 0.184 | 0.186 | 0.016 |
| auxiliary_views | 0.152 | 0.611 | 0.135 |
| water_fft | 0.129 | 0.128 | 0.002 |
| upscaling | 0.128 | 0.158 | 0.004 |
| bin_unpacking | 0.128 | 0.141 | 0.012 |
| clouds_trace | 0.127 | 0.214 | 0.001 |
| atmos_froxel_inject | 0.099 | 0.108 | 0.001 |
| tonemapping | 0.092 | 0.091 | 0.003 |
| water_velocity | 0.086 | 0.085 | 0.002 |
| clustering | 0.067 | 0.069 | 0.019 |
| clouds_composite | 0.064 | 0.063 | 0.002 |
| main_transparent_pass_3d | 0.060 | 0.103 | 0.105 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| render_sky | 0.054 | 0.054 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.049 | 0.005 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.031 | 0.034 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.030 | 0.001 |
| water_snapshot | 0.024 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.024 | 0.006 |
| lightprobe_copy | 0.013 | 0.014 | 0.004 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.011 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.605 | 11.817 | 26.64 | 56457.9 |
| render/busy/ms | 9.069 | 11.279 | 23.25 | 53305.0 |
| main/busy/ms | 8.340 | 10.039 | 16.34 | 49024.7 |
| render/graph/ms | 4.509 | 5.788 | 12.84 | 26505.9 |
| pipelines/wait_ms | 2.439 | 0.000 | 4616.92 | 14338.3 |
| bigspace/total/ms | 0.448 | 0.727 | 3.70 | 2631.7 |
| render/present/ms | 0.443 | 0.539 | 14.44 | 2605.9 |
| terrain/update/ms | 0.292 | 0.373 | 1.63 | 1713.8 |
| stream/drive/ms | 0.259 | 0.440 | 2.16 | 1525.1 |
| controller/feed/ms | 0.257 | 0.339 | 0.62 | 1510.1 |
| terrain/select/ms | 0.256 | 0.327 | 0.76 | 1507.5 |
| far/select/ms | 0.219 | 0.375 | 0.75 | 1288.8 |
| stream/tick/ms | 0.209 | 0.365 | 0.75 | 1230.6 |
| bigspace/low_precision/ms | 0.170 | 0.292 | 3.42 | 1002.2 |
| anim/evaluate_cpu/ms | 0.161 | 0.206 | 0.55 | 949.0 |
| anim/evaluate/ms | 0.143 | 0.232 | 1.21 | 843.3 |
| controller/move/ms | 0.143 | 0.297 | 0.65 | 842.3 |
| bigspace/high_precision/ms | 0.104 | 0.194 | 1.56 | 609.4 |
| render/acquire/ms | 0.093 | 0.122 | 0.51 | 547.0 |
| pipelines/compile_ms | 0.091 | 0.000 | 153.55 | 535.5 |
| pipelines/front_ms | 0.088 | 0.000 | 148.21 | 517.4 |
| anim/apply/ms | 0.081 | 0.121 | 1.20 | 475.0 |
| physics/step/ms | 0.048 | 0.084 | 0.93 | 279.3 |
| physics/insert/ms | 0.044 | 0.407 | 1.07 | 260.7 |
| weather/map/ms | 0.027 | 0.035 | 0.29 | 159.3 |
| controller/avoid/ms | 0.012 | 0.017 | 0.42 | 67.9 |
| weather/eval/ms | 0.011 | 0.014 | 0.19 | 62.1 |
| script/update/ms | 0.007 | 0.014 | 0.45 | 42.7 |
| stream/despawn/ms | 0.006 | 0.000 | 1.23 | 33.0 |
| script/input/ms | 0.005 | 0.006 | 0.15 | 28.6 |
| script/ui/ms | 0.005 | 0.006 | 0.06 | 27.9 |
| stream/spawn/ms | 0.004 | 0.000 | 1.10 | 26.1 |
| controller/write/ms | 0.004 | 0.006 | 0.03 | 20.7 |
| pipelines/driver_ms | 0.003 | 0.000 | 5.34 | 18.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.68 | 16.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 6.1 |
| shapes/activate/ms | 0.001 | 0.002 | 0.03 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.43 | 4.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 13 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 28 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.85 | 23.25 | 11.09 |
| gpu/timed/ms | ms | 7.47 | 10.51 | 7.76 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 822 | 1420 | 618 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 822 | 1420 | 618 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1296 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3422 | 5129 | 3181 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1296 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3422 | 5129 | 3181 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 39 | 248 | 28 |
| phases/shadow/bins | count | 3787 | 5459 | 2214 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9614 | 12.9 k | 5669 |
| phases/shadow/pipelines | count | 114 | 258 | 28 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2349 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 549 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 172 | 174 | 174 |
| pipelines/waiting | count | 0 | 4 | 0 |
| preprocess/bind_groups | count | 186 | 646 | 164 |
| preprocess/bind_groups_created | count | 186 | 646 | 164 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| process/cpu_usage | % | 24.04 | 35.10 | 21.15 |
| process/mem_usage | GiB | 2.66 | 2.72 | 2.72 |
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
| started_load_count |  | 3483 | 3541 | 3541 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.24 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.15 | 49.05 | 32.17 |
| system/mem_usage | % | 50.12 | 50.42 | 50.15 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 83 | 288 | 72 |
| views/all/bins | count | 6505 | 8351 | 5694 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.4 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 156 | 294 | 68 |
| views/all/pipelines_distinct | count | 62 | 68 | 55 |
| views/all/work_items | count | 17.4 k | 24.6 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2605 | 3201 | 2214 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8172 | 10.2 k | 5669 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8172 | 10.2 k | 5669 |
| views/lamp_face/batch_sets | count | 0 | 203 | 0 |
| views/lamp_face/bins | count | 1087 | 2483 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1210 | 2839 | 0 |
| views/lamp_face/pipelines | count | 74 | 210 | 0 |
| views/lamp_face/work_items | count | 0 | 2577 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2892 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8476 | 13.0 k | 7598 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8636 | 13.2 k | 7906 |
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
| anim/bones_written/count | count | 51.26 | 442 | 301.3 k |
| anim/events/count | count | 0.26 | 3 | 1557 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.70 | 160 | 4086 |
| assets/Mesh/removed | count | 0.83 | 1242 | 4892 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.68 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.99 | 1571 | 5847 |
| physics/inserts | count | 0.47 | 92 | 2791 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 129 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 24 | 378 |
| stream/upload/bytes | bytes | 5759 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1263 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.2 ms; slowest 6, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 579.9 | main/busy/ms 559, surface/advance_cpu_ms 64.74, surface/cell_upload_ms 0.70, stream/drive/ms 0.28, weather/map/ms 0.08, bigspace/total/ms 0.07, stream/tick/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 0.96 | 376.6 | main/busy/ms 590, render/thread/ms 341, render/busy/ms 259, render/acquire/ms 79.82, surface/advance_cpu_ms 64.74, render/graph/ms 5.59, render/present/ms 1.96, stream/drive/ms 0.99, surface/cell_upload_ms 0.70, stream/tick/ms 0.46, terrain/update/ms 0.44, terrain/select/ms 0.30 |
| warm-up | 1.03 | 73.7 | pipelines/wait_ms 656, render/thread/ms 360, render/busy/ms 276, render/acquire/ms 80.10, pipelines/compile_ms 69.21, pipelines/front_ms 63.33, main/busy/ms 48.52, terrain/update/ms 17.56, render/graph/ms 10.71, pipelines/driver_ms 5.88, render/present/ms 3.28, stream/drive/ms 1.14 |
| warm-up | 1.27 | 94.7 | pipelines/compile_ms 90.73, pipelines/front_ms 84.73, main/busy/ms 66.61, render/thread/ms 59.35, render/busy/ms 57.73, pipelines/wait_ms 43.73, pipelines/driver_ms 6.00, render/graph/ms 4.21, stream/drive/ms 1.88, stream/tick/ms 1.58, render/present/ms 1.39, terrain/update/ms 0.81 |
| warm-up | 1.30 | 28.4 | pipelines/wait_ms 129, pipelines/compile_ms 109, pipelines/front_ms 102, render/thread/ms 30.38, render/busy/ms 28.83, main/busy/ms 16.66, pipelines/driver_ms 7.44, render/graph/ms 2.63, stream/drive/ms 2.35, render/present/ms 1.35, stream/tick/ms 1.35, terrain/update/ms 1.14 |
| measure | 21.42 | 28.4 | render/thread/ms 38.74, render/busy/ms 23.62, main/busy/ms 21.79, render/present/ms 14.85, render/graph/ms 11.98, anim/apply/ms 1.14, bigspace/total/ms 0.99, controller/feed/ms 0.66, terrain/update/ms 0.56, anim/evaluate_cpu/ms 0.54, stream/drive/ms 0.53, terrain/select/ms 0.49 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\ceil_base\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 2%, CPU 16%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (254 samples): GPU load **79%** (min 60%) · 2580 MHz (min 2580) · memory 8001 MHz · 77.4 W · 71 °C (max 76) · VRAM 4054 MiB · throttled 100% of samples
