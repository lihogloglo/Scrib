# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4641 frames over 65.7 s · **median 13.76 ms (73 fps)** · mean 14.15 · p95 17.26 · p99 19.25 (1% low 52 fps) · worst 36.23 ms · 31 spikes over 1.5× median · 17.4 stutters/min (work 14.6/min)
- warm-up: 332 frames over 8.5 s, worst 2516.8 ms
- hitches (> 34.4 ms): **3** measured, 15 in warm-up
- timed GPU passes: 8.12 ms of a 14.15 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.76 ms, limit 11 ms
- **FAIL** frame_ms: 19 stutters (> 1.5x moving median of 60) in 1.09 min = 17.36/min, limit 1/min
- **FAIL** worst frame 36.23 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.209 | 2.211 | 2.776 | 1.00 | 0.184 |
| main_opaque_pass_3d | 1.761 | 1.762 | 2.422 | 1.00 | 0.088 |
| early prepass | 0.545 | 0.545 | 0.824 | 1.00 | 0.122 |
| bloom | 0.343 | 0.344 | 0.501 | 1.00 | 0.067 |
| atmosphere_luts | 0.331 | 0.331 | 0.462 | 1.00 | 0.003 |
| uniform_allocation | 0.328 | 0.328 | 0.624 | 1.00 | 0.060 |
| water_velocity | 0.306 | 0.307 | 0.562 | 1.00 | 0.002 |
| bin_unpacking | 0.247 | 0.247 | 0.271 | 1.00 | 0.041 |
| water_sspr | 0.224 | 0.224 | 0.219 | 1.00 | 0.002 |
| ssao | 0.207 | 0.207 | 0.210 | 1.00 | 0.008 |
| auxiliary_views | 0.198 | 0.198 | 0.747 | 1.00 | 0.177 |
| atmos_froxel_inject | 0.159 | 0.159 | 0.153 | 1.00 | 0.002 |
| water_fft | 0.142 | 0.142 | 0.141 | 1.00 | 0.003 |
| clouds_trace | 0.140 | 0.140 | 0.228 | 1.00 | 0.001 |
| upscaling | 0.127 | 0.127 | 0.133 | 1.00 | 0.004 |
| clouds_composite | 0.112 | 0.112 | 0.111 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.094 | 0.094 | 0.171 | 1.00 | 0.009 |
| tonemapping | 0.093 | 0.093 | 0.093 | 1.00 | 0.004 |
| clustering | 0.070 | 0.070 | 0.069 | 1.00 | 0.021 |
| lightprobe_irradiance_map | 0.056 | 0.446 | 0.756 | 0.12 | 0.001 |
| render_sky | 0.055 | 0.055 | 0.054 | 1.00 | 0.002 |
| surface_trail | 0.051 | 0.059 | 0.057 | 0.86 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.047 | 0.047 | 0.048 | 1.00 | 0.006 |
| water_snapshot | 0.039 | 0.039 | 0.046 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.029 | 0.234 | 0.237 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.031 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.030 | 1.00 | 0.110 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.003 | 0.020 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.016 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 33.516 | 0.000 | 19990.31 | 155548.9 |
| render/thread/ms | 12.949 | 15.922 | 34.99 | 60097.1 |
| render/busy/ms | 12.353 | 15.298 | 34.27 | 57330.5 |
| main/busy/ms | 9.539 | 12.469 | 27.58 | 44271.9 |
| render/graph/ms | 7.587 | 9.487 | 18.26 | 35212.5 |
| pipelines/compile_ms | 0.683 | 0.000 | 153.17 | 3169.7 |
| pipelines/front_ms | 0.660 | 0.000 | 148.98 | 3063.5 |
| render/present/ms | 0.506 | 0.587 | 16.07 | 2346.0 |
| bigspace/total/ms | 0.452 | 0.702 | 4.23 | 2097.6 |
| terrain/update/ms | 0.291 | 0.365 | 1.79 | 1350.0 |
| controller/feed/ms | 0.284 | 0.384 | 0.75 | 1316.9 |
| stream/drive/ms | 0.264 | 0.380 | 18.95 | 1227.0 |
| terrain/select/ms | 0.258 | 0.323 | 0.64 | 1195.5 |
| stream/tick/ms | 0.201 | 0.278 | 0.56 | 932.6 |
| far/select/ms | 0.193 | 0.271 | 0.55 | 893.8 |
| bigspace/low_precision/ms | 0.173 | 0.287 | 3.63 | 805.2 |
| anim/evaluate_cpu/ms | 0.167 | 0.210 | 0.57 | 776.2 |
| controller/move/ms | 0.163 | 0.388 | 0.66 | 756.0 |
| anim/evaluate/ms | 0.127 | 0.283 | 1.52 | 587.2 |
| bigspace/high_precision/ms | 0.107 | 0.210 | 3.90 | 496.0 |
| render/acquire/ms | 0.091 | 0.117 | 0.30 | 420.6 |
| physics/insert/ms | 0.067 | 0.469 | 1.36 | 309.2 |
| physics/step/ms | 0.061 | 0.090 | 0.71 | 281.3 |
| weather/map/ms | 0.024 | 0.032 | 0.18 | 109.8 |
| pipelines/driver_ms | 0.023 | 0.000 | 6.04 | 106.2 |
| anim/apply/ms | 0.011 | 0.020 | 0.29 | 52.9 |
| weather/eval/ms | 0.010 | 0.014 | 0.10 | 47.3 |
| controller/avoid/ms | 0.010 | 0.013 | 0.08 | 46.4 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 34.2 |
| stream/despawn/ms | 0.007 | 0.000 | 1.04 | 30.2 |
| script/input/ms | 0.006 | 0.008 | 0.03 | 26.0 |
| stream/spawn/ms | 0.006 | 0.000 | 1.05 | 25.9 |
| script/ui/ms | 0.004 | 0.006 | 0.12 | 20.7 |
| controller/write/ms | 0.004 | 0.004 | 0.22 | 16.9 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.61 | 14.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.07 | 4.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 28 | 19 |
| controller/movers/count | count | 19 | 22 | 19 |
| crowd/c0/count | count | 25 | 43 | 14 |
| crowd/c0/joints/count | count | 1249 | 2262 | 641 |
| crowd/c0/meshes/count | count | 343 | 646 | 174 |
| crowd/c1/count | count | 146 | 157 | 147 |
| crowd/c1/joints/count | count | 5480 | 5990 | 5555 |
| crowd/c1/meshes/count | count | 1152 | 1286 | 1167 |
| crowd/c2/count | count | 397 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 222 |
| crowd/joints_extracted/count | count | 4270 | 12.4 k | 6124 |
| crowd/joints_uploaded/count | count | 4249 | 6643 | 6124 |
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
| frame/work/ms | ms | 12.01 | 34.27 | 10.24 |
| gpu/timed/ms | ms | 8.76 | 13.43 | 8.89 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 3073 | 3143 | 3143 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 2 | 2 | 2 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 277.8 k | 282.2 k | 282.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 197 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 800 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 35 | 40 | 35 |
| phases/opaque3d/bins | count | 1284 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3370 | 5116 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 35 | 40 | 35 |
| phases/opaque3d_prepass/bins | count | 1284 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3370 | 5116 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 73 | 342 | 57 |
| phases/shadow/bins | count | 3751 | 5524 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9362 | 13.0 k | 5637 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 140 | 323 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 176 | 476 | 19 |
| physics/pending | count | 0 | 1279 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 35 | 0 |
| preprocess/bind_groups | count | 12 | 921 | 10 |
| preprocess/bind_groups_created | count | 12 | 921 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 310 | 455 | 295 |
| process/cpu_usage | % | 22.38 | 44.23 | 21.05 |
| process/mem_usage | GiB | 5.15 | 5.33 | 5.33 |
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
| started_load_count |  | 4768 | 4859 | 4859 |
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
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.33 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.63 | 43.52 | 28.29 |
| system/mem_usage | % | 63.72 | 64.27 | 64.24 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 159 | 428 | 141 |
| views/all/bins | count | 6265 | 8406 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.9 k | 25.5 k | 13.2 k |
| views/all/pipelines | count | 204 | 389 | 113 |
| views/all/pipelines_distinct | count | 96 | 109 | 91 |
| views/all/work_items | count | 17.2 k | 23.9 k | 13.5 k |
| views/cascade/batch_sets | count | 71 | 91 | 57 |
| views/cascade/bins | count | 2602 | 3125 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8133 | 10.2 k | 5637 |
| views/cascade/pipelines | count | 55 | 71 | 47 |
| views/cascade/work_items | count | 8133 | 10.2 k | 5637 |
| views/lamp_face/batch_sets | count | 0 | 265 | 0 |
| views/lamp_face/bins | count | 1058 | 2475 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1201 | 2828 | 0 |
| views/lamp_face/pipelines | count | 83 | 254 | 0 |
| views/lamp_face/work_items | count | 0 | 2650 | 0 |
| views/main/batch_sets | count | 84 | 96 | 84 |
| views/main/bins | count | 2866 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8338 | 13.0 k | 7600 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8508 | 13.2 k | 7908 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.7 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 43.30 | 229 | 201.0 k |
| anim/events/count | count | 0.12 | 3 | 571 |
| assets/Image/added | count | 0.13 | 43 | 621 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.91 | 152 | 4227 |
| assets/Mesh/removed | count | 1.05 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 497 |
| assets/StandardMaterial/modified | count | 4.66 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.43 | 1386 | 6614 |
| physics/inserts | count | 0.70 | 125 | 3233 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.50 | 145 | 2335 |
| physics/steps | count | 0.85 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.01 | 6 | 26 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 7294 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1599 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 34.4 ms; slowest 18, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2516.8 | main/busy/ms 2490, surface/advance_cpu_ms 68.13, surface/cell_upload_ms 0.69, stream/drive/ms 0.27, weather/map/ms 0.16, bigspace/total/ms 0.09, stream/tick/ms 0.06, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 3.22 | 699.1 | main/busy/ms 2524, render/thread/ms 659, render/busy/ms 545, render/acquire/ms 113, surface/advance_cpu_ms 68.13, render/graph/ms 4.84, render/present/ms 1.99, stream/drive/ms 0.96, surface/cell_upload_ms 0.69, terrain/update/ms 0.52, stream/tick/ms 0.42, terrain/select/ms 0.36 |
| warm-up | 3.43 | 215.3 | pipelines/wait_ms 1570, render/thread/ms 675, render/busy/ms 559, pipelines/compile_ms 216, pipelines/front_ms 204, render/acquire/ms 113, main/busy/ms 48.61, terrain/update/ms 17.79, render/graph/ms 12.55, pipelines/driver_ms 12.52, render/present/ms 3.09, stream/drive/ms 1.57 |
| warm-up | 3.63 | 86.4 | main/busy/ms 56.33, render/thread/ms 51.51, render/busy/ms 49.93, render/graph/ms 3.63, stream/drive/ms 2.46, stream/tick/ms 1.56, render/present/ms 1.36, terrain/update/ms 0.67, terrain/select/ms 0.55, bigspace/total/ms 0.35, stream/spawn/ms 0.33, render/acquire/ms 0.22 |
| warm-up | 3.80 | 71.4 | render/thread/ms 53.92, main/busy/ms 52.65, render/busy/ms 52.57, stream/drive/ms 35.20, terrain/update/ms 7.39, render/graph/ms 3.81, stream/tick/ms 1.27, stream/spawn/ms 1.03, render/present/ms 1.01, terrain/select/ms 0.73, render/acquire/ms 0.35, bigspace/total/ms 0.35 |
| warm-up | 3.88 | 72.5 | pipelines/wait_ms 1974, pipelines/compile_ms 200, pipelines/front_ms 184, render/thread/ms 50.15, render/busy/ms 48.29, main/busy/ms 38.69, stream/drive/ms 22.28, pipelines/driver_ms 15.56, render/graph/ms 5.33, render/present/ms 1.61, stream/tick/ms 1.19, stream/spawn/ms 1.11 |
| warm-up | 4.03 | 126.9 | pipelines/wait_ms 4970, pipelines/compile_ms 112, pipelines/front_ms 104, render/thread/ms 86.36, render/busy/ms 84.82, main/busy/ms 51.52, stream/drive/ms 21.17, pipelines/driver_ms 7.85, terrain/update/ms 6.04, stream/spawn/ms 5.55, render/graph/ms 3.13, stream/tick/ms 1.53 |
| warm-up | 4.08 | 47.1 | pipelines/wait_ms 4099, render/thread/ms 107, render/busy/ms 106, pipelines/compile_ms 82.40, main/busy/ms 77.21, pipelines/front_ms 76.10, stream/drive/ms 52.46, pipelines/driver_ms 6.30, terrain/update/ms 5.95, stream/spawn/ms 5.56, render/graph/ms 3.68, stream/tick/ms 1.19 |
| warm-up | 4.14 | 59.2 | pipelines/wait_ms 2437, pipelines/compile_ms 160, pipelines/front_ms 153, main/busy/ms 67.30, render/thread/ms 63.91, render/busy/ms 62.20, stream/drive/ms 49.83, pipelines/driver_ms 7.56, render/graph/ms 5.39, stream/spawn/ms 2.50, render/present/ms 1.48, stream/tick/ms 1.44 |
| warm-up | 4.28 | 119.2 | pipelines/wait_ms 7840, pipelines/compile_ms 125, pipelines/front_ms 118, main/busy/ms 96.84, render/thread/ms 49.83, render/busy/ms 48.58, stream/drive/ms 47.59, pipelines/driver_ms 7.11, render/graph/ms 4.14, stream/spawn/ms 1.94, render/present/ms 1.08, stream/tick/ms 0.91 |
| warm-up | 4.33 | 35.3 | pipelines/wait_ms 2510, pipelines/compile_ms 58.16, pipelines/front_ms 55.82, main/busy/ms 39.35, render/thread/ms 24.72, stream/drive/ms 24.63, render/busy/ms 23.36, render/graph/ms 4.89, terrain/update/ms 4.17, pipelines/driver_ms 2.34, stream/spawn/ms 1.13, render/present/ms 1.06 |
| warm-up | 4.48 | 40.3 | pipelines/wait_ms 6267, pipelines/compile_ms 174, pipelines/front_ms 166, main/busy/ms 41.60, render/thread/ms 31.24, render/busy/ms 29.92, stream/drive/ms 28.07, pipelines/driver_ms 7.75, render/graph/ms 6.26, stream/spawn/ms 1.13, render/present/ms 1.10, stream/tick/ms 0.94 |
| warm-up | 4.56 | 69.9 | pipelines/wait_ms 2630, pipelines/compile_ms 78.46, pipelines/front_ms 73.53, main/busy/ms 52.79, stream/drive/ms 32.90, render/thread/ms 30.08, render/busy/ms 28.90, pipelines/driver_ms 4.93, render/graph/ms 4.84, stream/spawn/ms 2.32, stream/tick/ms 1.10, render/present/ms 1.01 |
| warm-up | 5.36 | 38.1 | pipelines/wait_ms 10.2 k, pipelines/compile_ms 187, pipelines/front_ms 177, render/thread/ms 51.35, render/busy/ms 50.15, main/busy/ms 29.58, pipelines/driver_ms 10.11, stream/drive/ms 5.81, render/graph/ms 5.27, stream/spawn/ms 2.23, stream/tick/ms 1.58, render/present/ms 0.99 |
| warm-up | 7.16 | 39.1 | pipelines/wait_ms 6416, pipelines/compile_ms 98.48, pipelines/front_ms 95.43, main/busy/ms 49.67, render/thread/ms 31.67, render/busy/ms 30.40, render/graph/ms 11.44, pipelines/driver_ms 3.05, bigspace/total/ms 1.26, stream/drive/ms 1.11, render/present/ms 1.07, anim/evaluate_cpu/ms 0.77 |
| measure | 2.19 | 35.0 | render/thread/ms 47.70, render/busy/ms 46.19, render/graph/ms 26.85, main/busy/ms 20.06, pipelines/compile_ms 13.22, pipelines/front_ms 13.00, render/present/ms 1.33, bigspace/total/ms 0.72, terrain/update/ms 0.66, terrain/select/ms 0.60, stream/drive/ms 0.54, controller/feed/ms 0.52 |
| measure | 2.73 | 36.2 | render/thread/ms 49.37, render/busy/ms 48.17, render/graph/ms 24.91, main/busy/ms 18.77, render/present/ms 1.03, bigspace/total/ms 0.77, terrain/update/ms 0.61, terrain/select/ms 0.55, controller/feed/ms 0.53, stream/drive/ms 0.52, stream/tick/ms 0.47, physics/insert/ms 0.46 |
| measure | 59.05 | 36.0 | render/thread/ms 44.38, render/busy/ms 43.02, main/busy/ms 25.51, render/graph/ms 13.49, bigspace/total/ms 1.77, render/present/ms 1.08, controller/feed/ms 1.06, terrain/update/ms 0.84, stream/drive/ms 0.84, terrain/select/ms 0.74, stream/tick/ms 0.73, far/select/ms 0.62 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab_off\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **74%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 72.8 W · 74 °C (max 79) · VRAM 5299 MiB · throttled 100% of samples
