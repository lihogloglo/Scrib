# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4987 frames over 65.6 s · **median 12.90 ms (78 fps)** · mean 13.16 · p95 15.89 · p99 18.29 (1% low 55 fps) · worst 34.29 ms · 28 spikes over 1.5× median · 25.6 stutters/min (work 21.9/min)
- warm-up: 338 frames over 8.5 s, worst 2598.5 ms
- hitches (> 32.3 ms): **2** measured, 18 in warm-up
- timed GPU passes: 7.91 ms of a 13.16 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.90 ms, limit 11 ms
- **FAIL** frame_ms: 28 stutters (> 1.5x moving median of 60) in 1.09 min = 25.59/min, limit 1/min
- **FAIL** worst frame 34.29 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.197 | 2.217 | 2.773 | 0.99 | 0.181 |
| main_opaque_pass_3d | 1.751 | 1.767 | 2.426 | 0.99 | 0.069 |
| early prepass | 0.548 | 0.553 | 1.002 | 0.99 | 0.107 |
| bloom | 0.338 | 0.341 | 0.488 | 0.99 | 0.066 |
| atmosphere_luts | 0.329 | 0.332 | 0.520 | 0.99 | 0.003 |
| water_velocity | 0.311 | 0.314 | 0.590 | 0.99 | 0.002 |
| uniform_allocation | 0.260 | 0.263 | 0.263 | 0.99 | 0.049 |
| water_sspr | 0.222 | 0.224 | 0.222 | 0.99 | 0.002 |
| ssao | 0.205 | 0.206 | 0.209 | 0.99 | 0.008 |
| bin_unpacking | 0.197 | 0.199 | 0.210 | 0.99 | 0.030 |
| atmos_froxel_inject | 0.156 | 0.158 | 0.152 | 0.99 | 0.002 |
| auxiliary_views | 0.146 | 0.148 | 0.586 | 0.99 | 0.134 |
| clouds_trace | 0.139 | 0.140 | 0.228 | 0.99 | 0.001 |
| water_fft | 0.139 | 0.140 | 0.138 | 0.99 | 0.003 |
| upscaling | 0.131 | 0.132 | 0.159 | 0.99 | 0.004 |
| clouds_composite | 0.110 | 0.111 | 0.111 | 0.99 | 0.002 |
| main_transmissive_pass_3d | 0.095 | 0.096 | 0.173 | 0.99 | 0.008 |
| tonemapping | 0.093 | 0.094 | 0.093 | 0.99 | 0.004 |
| clustering | 0.067 | 0.067 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.055 | 0.055 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.447 | 0.756 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 0.99 | 0.001 |
| surface_trail | 0.048 | 0.059 | 0.057 | 0.81 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.044 | 0.048 | 0.99 | 0.006 |
| water_snapshot | 0.039 | 0.039 | 0.044 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.030 | 0.253 | 0.562 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 0.99 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.023 | 0.99 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.032 | 0.99 | 0.109 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 43.634 | 0.000 | 13054.94 | 217600.9 |
| render/thread/ms | 11.915 | 14.477 | 33.21 | 59420.0 |
| render/busy/ms | 11.320 | 13.791 | 32.38 | 56451.5 |
| main/busy/ms | 9.579 | 12.150 | 29.83 | 47772.4 |
| render/graph/ms | 6.792 | 8.447 | 17.07 | 33873.5 |
| pipelines/compile_ms | 0.713 | 0.000 | 175.52 | 3553.3 |
| pipelines/front_ms | 0.688 | 0.000 | 169.02 | 3428.8 |
| render/present/ms | 0.503 | 0.611 | 10.91 | 2506.3 |
| bigspace/total/ms | 0.458 | 0.692 | 4.22 | 2282.1 |
| terrain/update/ms | 0.295 | 0.371 | 1.90 | 1471.7 |
| controller/feed/ms | 0.276 | 0.368 | 0.80 | 1374.3 |
| terrain/select/ms | 0.262 | 0.328 | 0.63 | 1306.4 |
| stream/drive/ms | 0.252 | 0.352 | 20.93 | 1256.2 |
| far/select/ms | 0.196 | 0.273 | 0.65 | 978.8 |
| stream/tick/ms | 0.193 | 0.264 | 0.57 | 961.1 |
| bigspace/low_precision/ms | 0.168 | 0.276 | 3.99 | 835.5 |
| anim/evaluate_cpu/ms | 0.153 | 0.196 | 0.52 | 761.7 |
| anim/evaluate/ms | 0.133 | 0.288 | 3.65 | 664.2 |
| controller/move/ms | 0.125 | 0.242 | 0.57 | 622.8 |
| bigspace/high_precision/ms | 0.116 | 0.225 | 3.48 | 576.9 |
| render/acquire/ms | 0.093 | 0.121 | 0.49 | 462.3 |
| physics/insert/ms | 0.060 | 0.455 | 1.07 | 300.9 |
| physics/step/ms | 0.058 | 0.089 | 0.59 | 287.2 |
| pipelines/driver_ms | 0.025 | 0.000 | 8.82 | 124.5 |
| weather/map/ms | 0.024 | 0.032 | 0.22 | 121.4 |
| anim/apply/ms | 0.011 | 0.022 | 0.38 | 56.2 |
| controller/avoid/ms | 0.010 | 0.014 | 0.08 | 51.8 |
| weather/eval/ms | 0.010 | 0.014 | 0.17 | 50.4 |
| script/update/ms | 0.007 | 0.009 | 0.12 | 36.2 |
| stream/despawn/ms | 0.006 | 0.000 | 1.03 | 28.5 |
| script/input/ms | 0.005 | 0.007 | 0.08 | 26.8 |
| stream/spawn/ms | 0.005 | 0.000 | 1.06 | 25.7 |
| script/ui/ms | 0.005 | 0.007 | 0.16 | 25.2 |
| controller/write/ms | 0.003 | 0.004 | 0.06 | 16.5 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.71 | 14.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 3.6 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.06 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 20 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 60 | 17 |
| controller/movers/count | count | 19 | 21 | 17 |
| crowd/c0/count | count | 26 | 52 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 528 |
| crowd/c0/meshes/count | count | 353 | 646 | 135 |
| crowd/c1/count | count | 146 | 156 | 149 |
| crowd/c1/joints/count | count | 5507 | 5991 | 5668 |
| crowd/c1/meshes/count | count | 1167 | 1295 | 1206 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4268 | 12.8 k | 6179 |
| crowd/joints_uploaded/count | count | 4249 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.17 | 32.38 | 10.10 |
| gpu/timed/ms | ms | 8.68 | 12.61 | 8.98 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1287 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3410 | 5150 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1287 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3409 | 5150 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 57 | 270 | 47 |
| phases/shadow/bins | count | 4804 | 5817 | 2228 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.5 k | 13.2 k | 5679 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 246 | 305 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2346 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1172 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 47 | 0 |
| preprocess/bind_groups | count | 11 | 737 | 6 |
| preprocess/bind_groups_created | count | 11 | 737 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 258 | 367 | 251 |
| process/cpu_usage | % | 24.55 | 40.38 | 19.55 |
| process/mem_usage | GiB | 5.08 | 5.25 | 5.25 |
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
| started_load_count |  | 4765 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
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
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.34 | 72.10 | 27.95 |
| system/mem_usage | % | 58.70 | 59.34 | 59.34 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 125 | 336 | 117 |
| views/all/bins | count | 7367 | 8687 | 5708 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.4 k | 25.7 k | 13.3 k |
| views/all/pipelines | count | 311 | 371 | 113 |
| views/all/pipelines_distinct | count | 101 | 110 | 91 |
| views/all/work_items | count | 17.3 k | 23.1 k | 13.6 k |
| views/cascade/batch_sets | count | 55 | 70 | 47 |
| views/cascade/bins | count | 2601 | 3128 | 2228 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8139 | 10.2 k | 5679 |
| views/cascade/pipelines | count | 55 | 70 | 47 |
| views/cascade/work_items | count | 8139 | 10.2 k | 5679 |
| views/lamp_face/batch_sets | count | 0 | 214 | 0 |
| views/lamp_face/bins | count | 2128 | 2752 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2389 | 3042 | 0 |
| views/lamp_face/pipelines | count | 188 | 236 | 0 |
| views/lamp_face/work_items | count | 0 | 2856 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2870 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8430 | 13.0 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8591 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 54.77 | 735 | 273.1 k |
| anim/events/count | count | 0.11 | 4 | 562 |
| assets/Image/added | count | 0.13 | 43 | 646 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.85 | 152 | 4227 |
| assets/Mesh/removed | count | 0.98 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 524 |
| assets/StandardMaterial/modified | count | 4.35 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.07 | 2520 | 10.3 k |
| physics/inserts | count | 0.68 | 157 | 3376 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.47 | 145 | 2335 |
| physics/steps | count | 0.79 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 6 | 24 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 18 | 378 |
| stream/upload/bytes | bytes | 6788 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 60 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1488 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.3 ms; slowest 20, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.60 | 2598.5 | main/busy/ms 2574, surface/advance_cpu_ms 70.25, surface/cell_upload_ms 0.73, stream/drive/ms 0.44, stream/tick/ms 0.14, weather/map/ms 0.12, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.03, script/input/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 3.58 | 977.1 | main/busy/ms 2613, render/thread/ms 934, render/busy/ms 733, render/acquire/ms 198, surface/advance_cpu_ms 70.25, render/graph/ms 4.55, render/present/ms 2.73, stream/drive/ms 1.71, stream/tick/ms 0.83, surface/cell_upload_ms 0.73, terrain/update/ms 0.41, terrain/select/ms 0.28 |
| warm-up | 3.65 | 76.0 | render/thread/ms 946, render/busy/ms 744, pipelines/wait_ms 708, render/acquire/ms 198, pipelines/compile_ms 74.70, pipelines/front_ms 66.96, main/busy/ms 55.52, terrain/update/ms 21.24, render/graph/ms 8.71, pipelines/driver_ms 7.75, render/present/ms 3.68, stream/drive/ms 1.81 |
| warm-up | 3.90 | 93.4 | pipelines/compile_ms 241, pipelines/front_ms 229, pipelines/wait_ms 227, main/busy/ms 56.25, render/thread/ms 55.79, render/busy/ms 54.04, pipelines/driver_ms 12.49, render/graph/ms 4.13, render/present/ms 1.52, stream/drive/ms 1.00, stream/tick/ms 0.79, terrain/update/ms 0.67 |
| warm-up | 3.95 | 37.5 | render/thread/ms 40.79, render/busy/ms 38.29, main/busy/ms 36.54, render/graph/ms 6.07, terrain/update/ms 5.27, stream/drive/ms 4.91, stream/tick/ms 2.43, render/present/ms 2.04, stream/spawn/ms 1.63, terrain/select/ms 0.74, render/acquire/ms 0.46, stream/upload/ms 0.32 |
| warm-up | 4.05 | 48.9 | main/busy/ms 40.74, render/thread/ms 38.47, render/busy/ms 37.32, stream/drive/ms 29.10, render/graph/ms 3.00, render/present/ms 0.94, stream/tick/ms 0.89, stream/spawn/ms 0.78, terrain/update/ms 0.62, terrain/select/ms 0.51, render/acquire/ms 0.22, bigspace/total/ms 0.20 |
| warm-up | 4.11 | 51.3 | pipelines/wait_ms 632, pipelines/compile_ms 224, pipelines/front_ms 206, render/thread/ms 45.23, render/busy/ms 43.43, main/busy/ms 38.67, stream/drive/ms 27.07, pipelines/driver_ms 18.16, render/graph/ms 4.72, render/present/ms 1.54, stream/spawn/ms 1.11, stream/tick/ms 0.72 |
| warm-up | 4.22 | 90.3 | render/thread/ms 65.57, render/busy/ms 63.08, main/busy/ms 37.28, stream/drive/ms 12.20, render/graph/ms 8.52, stream/tick/ms 2.33, render/present/ms 2.11, stream/spawn/ms 0.88, terrain/update/ms 0.86, terrain/select/ms 0.69, render/acquire/ms 0.39, bigspace/total/ms 0.27 |
| warm-up | 4.35 | 100.0 | pipelines/wait_ms 1439, pipelines/compile_ms 246, pipelines/front_ms 238, render/thread/ms 76.85, render/busy/ms 55.37, main/busy/ms 49.35, stream/drive/ms 29.59, render/present/ms 21.20, pipelines/driver_ms 8.26, terrain/update/ms 6.80, render/graph/ms 2.40, stream/spawn/ms 1.14 |
| warm-up | 4.48 | 36.1 | render/thread/ms 53.59, render/busy/ms 51.24, main/busy/ms 29.96, render/graph/ms 14.24, stream/drive/ms 7.48, stream/spawn/ms 4.52, render/present/ms 2.03, stream/tick/ms 1.84, terrain/update/ms 0.88, terrain/select/ms 0.72, stream/activate/ms 0.41, bigspace/total/ms 0.37 |
| warm-up | 4.53 | 43.3 | main/busy/ms 74.80, render/thread/ms 57.84, render/busy/ms 55.96, stream/drive/ms 53.28, render/graph/ms 13.11, stream/spawn/ms 2.80, stream/tick/ms 2.01, render/present/ms 1.58, terrain/update/ms 1.01, terrain/select/ms 0.83, render/acquire/ms 0.30, bigspace/total/ms 0.30 |
| warm-up | 4.59 | 59.3 | pipelines/wait_ms 4059, pipelines/compile_ms 172, pipelines/front_ms 167, main/busy/ms 69.97, stream/drive/ms 52.06, render/thread/ms 51.25, render/busy/ms 49.75, render/graph/ms 9.42, pipelines/driver_ms 4.96, stream/spawn/ms 2.67, render/present/ms 1.28, stream/tick/ms 1.02 |
| warm-up | 4.67 | 69.3 | pipelines/wait_ms 6878, pipelines/compile_ms 78.62, pipelines/front_ms 74.32, main/busy/ms 69.51, stream/drive/ms 47.73, render/thread/ms 40.29, render/busy/ms 39.01, render/graph/ms 6.76, pipelines/driver_ms 4.30, stream/spawn/ms 1.58, render/present/ms 1.06, terrain/update/ms 0.94 |
| warm-up | 4.74 | 38.9 | pipelines/wait_ms 3008, main/busy/ms 46.72, render/thread/ms 33.94, stream/drive/ms 33.24, pipelines/compile_ms 33.20, render/busy/ms 32.20, pipelines/front_ms 31.51, render/graph/ms 5.54, pipelines/driver_ms 1.69, render/present/ms 1.56, stream/activate/ms 1.05, stream/tick/ms 1.01 |
| warm-up | 4.78 | 35.8 | main/busy/ms 37.51, stream/drive/ms 24.94, render/thread/ms 17.38, render/busy/ms 16.24, render/graph/ms 3.97, stream/activate/ms 1.45, stream/spawn/ms 0.98, render/present/ms 0.95, stream/tick/ms 0.80, physics/insert/ms 0.73, terrain/update/ms 0.71, terrain/select/ms 0.59 |
| warm-up | 5.56 | 39.9 | render/thread/ms 46.36, render/busy/ms 45.19, main/busy/ms 25.21, render/graph/ms 4.78, stream/drive/ms 4.21, stream/spawn/ms 2.06, stream/tick/ms 1.08, render/present/ms 0.98, terrain/update/ms 0.67, terrain/select/ms 0.58, bigspace/total/ms 0.54, bigspace/high_precision/ms 0.24 |
| warm-up | 5.89 | 37.1 | pipelines/wait_ms 13.7 k, pipelines/compile_ms 170, pipelines/front_ms 159, main/busy/ms 32.90, render/thread/ms 31.07, render/busy/ms 29.65, stream/drive/ms 13.82, pipelines/driver_ms 10.45, render/graph/ms 5.23, stream/spawn/ms 2.07, render/present/ms 1.24, terrain/update/ms 0.96 |
| warm-up | 6.07 | 41.1 | main/busy/ms 47.78, render/thread/ms 35.51, render/busy/ms 34.00, stream/drive/ms 8.62, render/graph/ms 6.15, physics/insert/ms 2.06, stream/spawn/ms 1.72, render/present/ms 1.31, terrain/update/ms 0.77, stream/tick/ms 0.74, terrain/select/ms 0.68, physics/step/ms 0.65 |
| measure | 2.72 | 34.3 | render/thread/ms 45.78, render/busy/ms 44.31, render/graph/ms 22.74, main/busy/ms 19.82, render/present/ms 1.26, bigspace/total/ms 0.84, physics/insert/ms 0.70, stream/drive/ms 0.69, stream/tick/ms 0.62, terrain/update/ms 0.59, controller/feed/ms 0.56, terrain/select/ms 0.53 |
| measure | 59.05 | 32.6 | render/thread/ms 41.92, render/busy/ms 40.77, main/busy/ms 25.90, render/graph/ms 11.76, anim/evaluate/ms 1.28, bigspace/total/ms 1.23, controller/feed/ms 1.06, stream/drive/ms 0.98, render/present/ms 0.90, stream/tick/ms 0.84, terrain/update/ms 0.80, far/select/ms 0.73 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_new.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_new\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 11%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **78%** (min 59%) · 2580 MHz (min 2580) · memory 8001 MHz · 75.8 W · 76 °C (max 81) · VRAM 5279 MiB · throttled 100% of samples
