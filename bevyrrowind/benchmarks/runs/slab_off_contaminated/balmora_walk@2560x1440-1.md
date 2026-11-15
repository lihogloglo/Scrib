# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 2304 frames over 65.7 s · **median 12.86 ms (78 fps)** · mean 28.50 · p95 79.52 · p99 86.41 (1% low 12 fps) · worst 288.41 ms · 779 spikes over 1.5× median · 10.1 stutters/min (work 5.5/min)
- warm-up: 894 frames over 33.0 s, worst 3902.2 ms
- hitches (> 32.1 ms): **666** measured, 372 in warm-up
- timed GPU passes: 18.60 ms of a 28.50 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.86 ms, limit 11 ms
- **FAIL** frame_ms: 11 stutters (> 1.5x moving median of 60) in 1.09 min = 10.05/min, limit 1/min
- **FAIL** worst frame 288.41 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 4.705 | 4.748 | 13.697 | 0.99 | 0.080 |
| dlss_super_resolution | 3.167 | 3.196 | 7.133 | 0.99 | 0.190 |
| early prepass | 3.083 | 3.110 | 10.349 | 0.99 | 0.121 |
| bin_unpacking | 1.442 | 1.454 | 5.904 | 0.99 | 0.041 |
| auxiliary_views | 1.083 | 1.092 | 6.583 | 0.99 | 0.134 |
| uniform_allocation | 0.600 | 0.605 | 1.725 | 0.99 | 0.060 |
| atmosphere_luts | 0.499 | 0.503 | 1.186 | 0.99 | 0.004 |
| bloom | 0.491 | 0.496 | 1.188 | 0.99 | 0.067 |
| water_velocity | 0.379 | 0.382 | 1.030 | 0.99 | 0.002 |
| early_mesh_preprocessing | 0.377 | 0.380 | 1.651 | 0.99 | 0.006 |
| water_sspr | 0.326 | 0.330 | 0.836 | 0.99 | 0.002 |
| ssao | 0.310 | 0.313 | 0.815 | 0.99 | 0.008 |
| main_transmissive_pass_3d | 0.266 | 0.269 | 0.992 | 0.99 | 0.008 |
| clustering | 0.236 | 0.239 | 0.903 | 0.99 | 0.022 |
| atmos_froxel_inject | 0.206 | 0.208 | 0.716 | 0.99 | 0.002 |
| upscaling | 0.175 | 0.177 | 0.540 | 0.99 | 0.004 |
| water_fft | 0.145 | 0.147 | 0.138 | 0.99 | 0.003 |
| clouds_trace | 0.142 | 0.143 | 0.180 | 0.99 | 0.001 |
| clouds_composite | 0.142 | 0.143 | 0.115 | 0.99 | 0.002 |
| tonemapping | 0.137 | 0.138 | 0.655 | 0.99 | 0.003 |
| surface_trail | 0.089 | 0.103 | 0.062 | 0.87 | 0.002 |
| lightprobe_irradiance_map | 0.089 | 0.724 | 1.741 | 0.12 | 0.001 |
| water_mips | 0.073 | 0.073 | 0.582 | 0.99 | 0.002 |
| render_sky | 0.067 | 0.068 | 0.056 | 0.99 | 0.002 |
| water_snapshot | 0.064 | 0.064 | 0.054 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.054 | 0.055 | 0.062 | 0.99 | 0.006 |
| clouds_resolve | 0.050 | 0.051 | 0.053 | 0.99 | 0.001 |
| atmos_froxel_integrate | 0.046 | 0.046 | 0.036 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.044 | 0.358 | 0.820 | 0.12 | 0.004 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 0.99 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.025 | 0.031 | 0.99 | 0.122 |
| atmosphere_environment | 0.015 | 0.016 | 0.013 | 0.99 | 0.003 |
| clouds_fog_light | 0.011 | 0.011 | 0.012 | 0.99 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.99 | 0.004 |
| lightprobe_copy | 0.007 | 0.055 | 0.073 | 0.12 | 0.004 |
| night_sky | 0.006 | 0.006 | 0.008 | 0.99 | 0.002 |
| lightprobe_downsampling_first_pass | 0.005 | 0.039 | 0.034 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.002 | 0.015 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.003 | 0.04 | 0.007 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 27.180 | 77.683 | 286.58 | 62623.0 |
| render/acquire/ms | 13.563 | 58.203 | 260.69 | 31248.6 |
| render/busy/ms | 13.061 | 21.260 | 58.51 | 30092.3 |
| main/busy/ms | 9.220 | 12.454 | 56.46 | 21242.6 |
| render/graph/ms | 8.002 | 13.320 | 44.87 | 18437.4 |
| pipelines/compile_ms | 3.960 | 0.000 | 1889.11 | 9123.6 |
| pipelines/driver_ms | 3.560 | 0.000 | 1767.74 | 8202.8 |
| render/present/ms | 0.556 | 0.685 | 26.63 | 1282.1 |
| bigspace/total/ms | 0.436 | 0.647 | 3.86 | 1005.5 |
| pipelines/front_ms | 0.400 | 0.000 | 144.23 | 920.8 |
| terrain/update/ms | 0.311 | 0.416 | 2.15 | 717.6 |
| controller/feed/ms | 0.295 | 0.421 | 1.06 | 680.6 |
| stream/drive/ms | 0.293 | 0.649 | 25.67 | 675.0 |
| terrain/select/ms | 0.273 | 0.364 | 0.67 | 628.0 |
| far/select/ms | 0.208 | 0.298 | 0.87 | 479.7 |
| stream/tick/ms | 0.185 | 0.268 | 0.97 | 425.4 |
| controller/move/ms | 0.173 | 0.277 | 1.63 | 397.5 |
| pipelines/wait_ms | 0.171 | 0.000 | 130.72 | 394.5 |
| anim/evaluate_cpu/ms | 0.161 | 0.240 | 0.54 | 372.1 |
| bigspace/low_precision/ms | 0.153 | 0.252 | 3.38 | 353.2 |
| anim/evaluate/ms | 0.150 | 0.276 | 0.76 | 344.9 |
| bigspace/high_precision/ms | 0.112 | 0.197 | 3.17 | 258.5 |
| physics/insert/ms | 0.102 | 0.539 | 1.16 | 235.4 |
| physics/step/ms | 0.091 | 0.195 | 0.87 | 209.9 |
| weather/map/ms | 0.026 | 0.037 | 0.61 | 59.3 |
| stream/despawn/ms | 0.015 | 0.000 | 3.18 | 35.0 |
| stream/spawn/ms | 0.012 | 0.000 | 1.10 | 27.7 |
| weather/eval/ms | 0.011 | 0.016 | 0.17 | 25.6 |
| controller/avoid/ms | 0.010 | 0.014 | 0.16 | 24.2 |
| anim/apply/ms | 0.010 | 0.021 | 0.15 | 23.3 |
| script/update/ms | 0.008 | 0.011 | 0.45 | 18.6 |
| script/input/ms | 0.006 | 0.008 | 0.06 | 13.4 |
| stream/deactivate/ms | 0.006 | 0.000 | 1.19 | 12.7 |
| script/ui/ms | 0.005 | 0.007 | 0.02 | 10.9 |
| controller/write/ms | 0.003 | 0.005 | 0.03 | 8.0 |
| stream/activate/ms | 0.002 | 0.000 | 0.50 | 5.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 2.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.8 |
| stream/upload/ms | 0.001 | 0.000 | 0.08 | 1.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 0 | 9 | 0 |
| anim/a3/count | count | 571 | 579 | 578 |
| anim/characters/count | count | 571 | 579 | 578 |
| anim/evaluated/count | count | 19 | 33 | 18 |
| controller/movers/count | count | 19 | 21 | 18 |
| crowd/c0/count | count | 12 | 43 | 14 |
| crowd/c0/joints/count | count | 583 | 2262 | 641 |
| crowd/c0/meshes/count | count | 162 | 646 | 174 |
| crowd/c1/count | count | 149 | 156 | 147 |
| crowd/c1/joints/count | count | 5687 | 5991 | 5555 |
| crowd/c1/meshes/count | count | 1194 | 1295 | 1167 |
| crowd/c2/count | count | 406 | 417 | 417 |
| crowd/c2/joints/count | count | 6564 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1880 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 214 | 236 | 223 |
| crowd/joints_extracted/count | count | 5699 | 11.9 k | 6179 |
| crowd/joints_uploaded/count | count | 5615 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 58.7 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 11.09 | 74.81 | 11.14 |
| gpu/timed/ms | ms | 9.10 | 74.81 | 9.44 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 3085 | 3145 | 3145 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 2 | 2 | 2 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 279.9 k | 282.2 k | 282.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 395 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 694 | 1416 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 395 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 694 | 1416 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 32 | 38 | 34 |
| phases/opaque3d/bins | count | 1344 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3217 | 5117 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 32 | 38 | 34 |
| phases/opaque3d_prepass/bins | count | 1344 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3217 | 5117 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 58 | 346 | 56 |
| phases/shadow/bins | count | 2342 | 5411 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5732 | 12.8 k | 5675 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 47 | 317 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2580 | 2968 | 2347 |
| physics/items | count | 19 | 245 | 19 |
| physics/pending | count | 0 | 517 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 231 | 231 | 231 |
| pipelines/waiting | count | 0 | 3 | 0 |
| preprocess/bind_groups | count | 11 | 925 | 10 |
| preprocess/bind_groups_created | count | 11 | 925 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 288 | 466 | 289 |
| process/cpu_usage | % | 17.23 | 36.61 | 19.81 |
| process/mem_usage | GiB | 6.92 | 7.16 | 7.10 |
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
| started_load_count |  | 4805 | 4864 | 4864 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 396 | 528 | 396 |
| statics/materials_bound | count | 396 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 4 | 0 |
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 3 | 0 |
| stream/latency/s | s | 0.12 | 0.81 | 0.09 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 253 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 1.56 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1124 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 2 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.84 | 62.57 | 27.65 |
| system/mem_usage | % | 87.26 | 89.92 | 68.86 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2838 | 2934 | 2934 |
| textures/images | count | 2600 | 2644 | 2644 |
| views/all/batch_sets | count | 137 | 430 | 138 |
| views/all/bins | count | 5840 | 8167 | 5700 |
| views/all/count | count | 4 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 13.5 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 114 | 379 | 113 |
| views/all/pipelines_distinct | count | 91 | 108 | 91 |
| views/all/work_items | count | 13.8 k | 23.7 k | 13.6 k |
| views/cascade/batch_sets | count | 58 | 86 | 56 |
| views/cascade/bins | count | 2342 | 3121 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5732 | 10.2 k | 5675 |
| views/cascade/pipelines | count | 47 | 70 | 47 |
| views/cascade/work_items | count | 5732 | 10.2 k | 5675 |
| views/lamp_face/batch_sets | count | 0 | 270 | 0 |
| views/lamp_face/bins | count | 0 | 2499 | 0 |
| views/lamp_face/count | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 2862 | 0 |
| views/lamp_face/pipelines | count | 0 | 247 | 0 |
| views/lamp_face/work_items | count | 0 | 2583 | 0 |
| views/main/batch_sets | count | 80 | 92 | 82 |
| views/main/bins | count | 3464 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7746 | 13.0 k | 7600 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8063 | 13.2 k | 7908 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 30.4 k | 37.7 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 32.84 | 210 | 75.7 k |
| anim/events/count | count | 0.24 | 4 | 555 |
| assets/Image/added | count | 0.26 | 43 | 594 |
| assets/Image/modified | count | 0.01 | 1 | 14 |
| assets/Image/removed | count | 0.02 | 6 | 46 |
| assets/Mesh/added | count | 1.78 | 191 | 4096 |
| assets/Mesh/removed | count | 2.14 | 1224 | 4920 |
| assets/StandardMaterial/added | count | 0.20 | 35 | 466 |
| assets/StandardMaterial/modified | count | 6.40 | 11 | 14.8 k |
| assets/StandardMaterial/removed | count | 0.18 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 5 | 28 |
| assets/WorldAsset/removed | count | 0.04 | 6 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.05 | 1492 | 4716 |
| physics/inserts | count | 0.91 | 104 | 2101 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.86 | 118 | 1978 |
| physics/steps | count | 1.41 | 3 | 3246 |
| physics/steps_dropped | count | 0.30 | 12 | 693 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.10 | 21 | 219 |
| stream/deactivate/items | count | 0.10 | 21 | 223 |
| stream/despawn/items | count | 0.10 | 7 | 222 |
| stream/fetch/items | count | 0.04 | 3 | 83 |
| stream/spawn/items | count | 0.16 | 22 | 378 |
| stream/upload/bytes | bytes | 14.7 k | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.02 | 3 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.02 | 1 | 36 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 3221 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.1 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.90 | 3902.2 | main/busy/ms 3798, surface/advance_cpu_ms 75.05, stream/drive/ms 1.77, stream/tick/ms 1.16, surface/cell_upload_ms 1.09, weather/eval/ms 0.43, script/update/ms 0.29, bigspace/total/ms 0.24, weather/map/ms 0.21, bigspace/high_precision/ms 0.21, script/input/ms 0.12, script/ui/ms 0.01 |
| warm-up | 4.58 | 673.6 | main/busy/ms 3870, render/thread/ms 624, render/busy/ms 496, render/acquire/ms 124, surface/advance_cpu_ms 75.05, render/graph/ms 11.67, stream/drive/ms 5.15, render/present/ms 3.93, stream/tick/ms 2.68, terrain/update/ms 1.56, surface/cell_upload_ms 1.09, terrain/select/ms 0.65 |
| warm-up | 4.89 | 109.8 | pipelines/compile_ms 744, pipelines/driver_ms 722, pipelines/wait_ms 211, main/busy/ms 76.23, render/thread/ms 59.56, render/busy/ms 57.65, pipelines/front_ms 22.31, render/graph/ms 4.57, render/present/ms 1.62, stream/drive/ms 1.53, stream/tick/ms 1.25, terrain/update/ms 0.99 |
| warm-up | 5.15 | 95.5 | pipelines/wait_ms 3946, pipelines/compile_ms 430, pipelines/driver_ms 370, render/thread/ms 93.64, render/busy/ms 91.14, main/busy/ms 83.13, pipelines/front_ms 60.63, stream/drive/ms 60.47, render/graph/ms 6.55, stream/tick/ms 2.33, render/present/ms 2.21, stream/spawn/ms 1.79 |
| warm-up | 5.88 | 136.9 | pipelines/wait_ms 1244, pipelines/compile_ms 234, pipelines/driver_ms 205, main/busy/ms 105, stream/drive/ms 74.85, render/thread/ms 54.45, render/busy/ms 52.98, pipelines/front_ms 28.99, stream/spawn/ms 5.40, render/graph/ms 4.91, stream/tick/ms 1.78, render/present/ms 1.16 |
| warm-up | 6.01 | 84.1 | pipelines/wait_ms 3868, pipelines/compile_ms 312, pipelines/driver_ms 248, main/busy/ms 85.28, render/thread/ms 65.51, render/busy/ms 63.63, pipelines/front_ms 63.28, stream/drive/ms 49.43, render/graph/ms 6.70, stream/spawn/ms 2.30, render/present/ms 1.56, stream/tick/ms 1.36 |
| warm-up | 7.73 | 193.3 | render/thread/ms 206, render/acquire/ms 166, render/busy/ms 38.41, main/busy/ms 32.27, render/graph/ms 6.89, stream/drive/ms 6.85, stream/spawn/ms 2.23, render/present/ms 1.83, stream/tick/ms 0.90, terrain/update/ms 0.84, bigspace/total/ms 0.79, terrain/select/ms 0.69 |
| warm-up | 9.80 | 263.7 | pipelines/wait_ms 6449, pipelines/compile_ms 3545, pipelines/driver_ms 3362, render/thread/ms 296, render/acquire/ms 250, pipelines/front_ms 183, main/busy/ms 25.12, render/busy/ms 24.44, render/present/ms 21.52, render/graph/ms 6.89, stream/drive/ms 2.20, stream/tick/ms 1.24 |
| warm-up | 14.69 | 136.9 | pipelines/wait_ms 69.7 k, pipelines/compile_ms 336, pipelines/driver_ms 284, render/thread/ms 194, render/acquire/ms 164, pipelines/front_ms 52.27, render/busy/ms 27.97, main/busy/ms 21.43, render/graph/ms 13.24, render/present/ms 1.58, terrain/update/ms 0.96, terrain/select/ms 0.79 |
| measure | 0.50 | 94.4 | render/thread/ms 145, render/busy/ms 78.42, render/acquire/ms 65.49, render/graph/ms 56.40, main/busy/ms 20.21, render/present/ms 1.47, bigspace/total/ms 0.85, controller/feed/ms 0.65, terrain/update/ms 0.60, terrain/select/ms 0.53, far/select/ms 0.48, stream/drive/ms 0.43 |
| measure | 2.30 | 86.8 | render/thread/ms 132, render/busy/ms 69.57, render/acquire/ms 60.67, render/graph/ms 49.39, main/busy/ms 19.59, pipelines/compile_ms 18.26, pipelines/front_ms 17.99, render/present/ms 1.46, bigspace/total/ms 0.85, terrain/update/ms 0.63, terrain/select/ms 0.57, controller/feed/ms 0.49 |
| measure | 2.87 | 94.1 | render/thread/ms 138, render/busy/ms 73.66, render/acquire/ms 61.41, render/graph/ms 51.66, main/busy/ms 27.18, render/present/ms 3.43, stream/drive/ms 1.51, bigspace/total/ms 1.00, terrain/update/ms 0.75, terrain/select/ms 0.67, controller/feed/ms 0.62, stream/tick/ms 0.58 |
| measure | 7.60 | 288.4 | render/thread/ms 324, render/acquire/ms 261, render/busy/ms 46.36, main/busy/ms 38.73, render/graph/ms 28.26, render/present/ms 16.33, stream/drive/ms 6.18, stream/despawn/ms 4.41, bigspace/total/ms 1.38, controller/feed/ms 1.04, stream/spawn/ms 1.01, physics/insert/ms 0.90 |
| measure | 13.53 | 86.2 | render/thread/ms 163, render/acquire/ms 129, render/busy/ms 33.01, render/graph/ms 19.86, main/busy/ms 18.41, render/present/ms 1.19, bigspace/total/ms 0.82, terrain/update/ms 0.63, controller/feed/ms 0.62, terrain/select/ms 0.57, physics/insert/ms 0.43, stream/drive/ms 0.42 |
| measure | 13.68 | 85.4 | render/thread/ms 144, render/acquire/ms 104, render/busy/ms 38.81, render/graph/ms 25.12, main/busy/ms 19.81, render/present/ms 1.09, physics/insert/ms 0.93, bigspace/total/ms 0.82, terrain/update/ms 0.67, terrain/select/ms 0.60, controller/feed/ms 0.60, stream/drive/ms 0.54 |
| measure | 13.84 | 83.3 | render/thread/ms 161, render/acquire/ms 123, render/busy/ms 37.33, main/busy/ms 25.15, render/graph/ms 23.88, bigspace/total/ms 1.19, render/present/ms 1.09, controller/feed/ms 0.97, terrain/update/ms 0.62, terrain/select/ms 0.55, anim/evaluate_cpu/ms 0.51, stream/drive/ms 0.48 |
| measure | 14.07 | 84.2 | render/thread/ms 151, render/acquire/ms 109, render/busy/ms 40.97, render/graph/ms 27.39, main/busy/ms 21.51, render/present/ms 1.12, bigspace/total/ms 0.95, controller/feed/ms 0.64, physics/insert/ms 0.63, terrain/update/ms 0.62, terrain/select/ms 0.56, stream/drive/ms 0.53 |
| measure | 16.07 | 92.9 | render/thread/ms 169, render/acquire/ms 131, render/busy/ms 36.54, render/graph/ms 23.93, main/busy/ms 20.52, render/present/ms 1.09, terrain/update/ms 0.85, terrain/select/ms 0.77, bigspace/total/ms 0.77, physics/insert/ms 0.77, stream/drive/ms 0.75, controller/feed/ms 0.61 |
| measure | 16.16 | 86.3 | render/thread/ms 176, render/acquire/ms 138, render/busy/ms 37.25, render/graph/ms 24.71, main/busy/ms 18.54, render/present/ms 1.03, bigspace/total/ms 0.89, physics/insert/ms 0.75, terrain/update/ms 0.68, terrain/select/ms 0.62, controller/feed/ms 0.51, stream/drive/ms 0.42 |
| measure | 16.41 | 95.6 | render/thread/ms 171, render/acquire/ms 126, render/busy/ms 44.30, render/graph/ms 24.93, main/busy/ms 19.51, render/present/ms 1.12, bigspace/total/ms 0.78, terrain/update/ms 0.71, physics/insert/ms 0.58, controller/feed/ms 0.58, stream/drive/ms 0.53, terrain/select/ms 0.52 |
| measure | 16.79 | 84.2 | render/thread/ms 159, render/acquire/ms 123, render/busy/ms 34.23, main/busy/ms 24.49, render/graph/ms 21.46, stream/drive/ms 1.32, render/present/ms 1.04, bigspace/total/ms 0.88, physics/insert/ms 0.86, stream/spawn/ms 0.83, terrain/update/ms 0.82, far/select/ms 0.77 |
| measure | 16.96 | 83.2 | render/thread/ms 163, render/acquire/ms 125, render/busy/ms 36.87, main/busy/ms 22.42, render/graph/ms 21.47, stream/drive/ms 1.62, render/present/ms 0.99, terrain/update/ms 0.83, bigspace/total/ms 0.83, terrain/select/ms 0.77, controller/feed/ms 0.62, physics/insert/ms 0.52 |
| measure | 18.78 | 84.5 | render/thread/ms 161, render/acquire/ms 118, render/busy/ms 42.12, main/busy/ms 29.00, render/graph/ms 25.46, stream/drive/ms 1.22, terrain/update/ms 1.13, render/present/ms 1.06, controller/feed/ms 1.05, bigspace/total/ms 1.04, stream/spawn/ms 0.70, anim/evaluate_cpu/ms 0.61 |
| measure | 19.03 | 84.1 | render/thread/ms 157, render/acquire/ms 118, pipelines/compile_ms 40.75, render/busy/ms 38.41, pipelines/driver_ms 30.89, render/graph/ms 22.15, main/busy/ms 21.57, pipelines/front_ms 9.86, render/present/ms 0.99, bigspace/total/ms 0.84, stream/drive/ms 0.77, physics/insert/ms 0.72 |
| measure | 19.43 | 82.8 | render/thread/ms 160, render/acquire/ms 122, render/busy/ms 36.85, render/graph/ms 23.27, main/busy/ms 18.66, render/present/ms 1.27, bigspace/total/ms 0.92, controller/feed/ms 0.66, terrain/update/ms 0.55, stream/drive/ms 0.49, terrain/select/ms 0.49, far/select/ms 0.38 |
| measure | 19.55 | 119.9 | render/thread/ms 199, render/acquire/ms 131, render/busy/ms 66.99, render/graph/ms 45.18, main/busy/ms 20.70, render/present/ms 1.56, bigspace/total/ms 0.96, physics/insert/ms 0.61, terrain/update/ms 0.58, stream/drive/ms 0.57, controller/feed/ms 0.56, terrain/select/ms 0.52 |
| measure | 19.72 | 113.5 | render/thread/ms 171, render/acquire/ms 123, render/busy/ms 46.30, render/graph/ms 28.27, main/busy/ms 24.20, render/present/ms 1.60, bigspace/total/ms 0.93, stream/drive/ms 0.79, physics/insert/ms 0.71, controller/feed/ms 0.69, far/select/ms 0.67, terrain/update/ms 0.67 |
| measure | 19.81 | 87.5 | render/thread/ms 198, render/acquire/ms 148, render/busy/ms 48.56, render/graph/ms 30.20, main/busy/ms 24.12, render/present/ms 1.46, physics/insert/ms 1.09, bigspace/total/ms 1.00, stream/drive/ms 0.77, controller/feed/ms 0.75, terrain/update/ms 0.64, terrain/select/ms 0.57 |
| measure | 19.98 | 84.8 | render/thread/ms 163, render/acquire/ms 121, render/busy/ms 40.46, render/graph/ms 27.04, main/busy/ms 23.69, stream/drive/ms 1.67, render/present/ms 1.18, bigspace/total/ms 1.07, controller/feed/ms 0.93, physics/insert/ms 0.80, stream/despawn/ms 0.76, terrain/update/ms 0.65 |
| measure | 20.24 | 100.8 | render/thread/ms 177, render/acquire/ms 133, render/busy/ms 42.34, render/graph/ms 25.64, main/busy/ms 24.99, stream/drive/ms 2.02, render/present/ms 1.69, bigspace/total/ms 1.14, stream/spawn/ms 1.10, physics/insert/ms 0.78, controller/feed/ms 0.75, terrain/update/ms 0.67 |
| measure | 20.32 | 86.3 | render/thread/ms 183, render/acquire/ms 133, render/busy/ms 48.10, main/busy/ms 28.31, render/graph/ms 27.92, stream/drive/ms 2.06, render/present/ms 1.61, stream/spawn/ms 1.32, bigspace/total/ms 1.23, physics/insert/ms 0.76, controller/feed/ms 0.71, terrain/update/ms 0.64 |
| measure | 20.41 | 90.6 | pipelines/compile_ms 1537, pipelines/driver_ms 1427, render/thread/ms 172, render/acquire/ms 125, pipelines/front_ms 110, render/busy/ms 46.24, main/busy/ms 31.87, render/graph/ms 26.17, pipelines/wait_ms 9.88, stream/drive/ms 1.82, stream/spawn/ms 1.23, render/present/ms 1.21 |
| measure | 20.58 | 84.7 | render/thread/ms 162, render/acquire/ms 115, render/busy/ms 45.21, main/busy/ms 25.82, render/graph/ms 25.55, render/present/ms 1.10, stream/drive/ms 1.04, bigspace/total/ms 0.78, terrain/update/ms 0.60, controller/feed/ms 0.59, terrain/select/ms 0.54, far/select/ms 0.45 |
| measure | 20.67 | 85.0 | pipelines/compile_ms 1446, pipelines/driver_ms 1302, render/thread/ms 167, pipelines/front_ms 144, render/acquire/ms 121, render/busy/ms 45.01, render/graph/ms 28.40, main/busy/ms 24.18, render/present/ms 1.10, bigspace/total/ms 0.79, stream/drive/ms 0.75, terrain/update/ms 0.70 |
| measure | 20.84 | 94.1 | render/thread/ms 169, render/acquire/ms 133, render/busy/ms 34.87, main/busy/ms 24.68, render/graph/ms 21.52, stream/drive/ms 4.00, render/present/ms 1.17, bigspace/total/ms 0.80, controller/feed/ms 0.67, physics/insert/ms 0.63, terrain/update/ms 0.60, terrain/select/ms 0.55 |
| measure | 20.92 | 82.8 | render/thread/ms 173, render/acquire/ms 137, render/busy/ms 34.55, main/busy/ms 24.76, render/graph/ms 20.88, stream/drive/ms 3.94, render/present/ms 1.13, controller/feed/ms 0.77, bigspace/total/ms 0.73, physics/insert/ms 0.61, terrain/update/ms 0.59, terrain/select/ms 0.53 |
| measure | 21.01 | 89.1 | render/thread/ms 168, render/acquire/ms 129, render/busy/ms 36.92, main/busy/ms 23.95, render/graph/ms 22.30, render/present/ms 1.48, stream/drive/ms 1.06, controller/feed/ms 0.83, bigspace/total/ms 0.83, terrain/update/ms 0.71, terrain/select/ms 0.64, stream/spawn/ms 0.55 |
| measure | 21.81 | 87.3 | render/thread/ms 157, render/acquire/ms 118, render/busy/ms 38.62, render/graph/ms 26.77, main/busy/ms 19.93, render/present/ms 1.17, physics/insert/ms 1.00, stream/drive/ms 0.95, bigspace/total/ms 0.85, terrain/update/ms 0.68, terrain/select/ms 0.61, controller/feed/ms 0.56 |
| measure | 23.25 | 87.8 | render/thread/ms 160, render/acquire/ms 123, render/busy/ms 35.97, render/graph/ms 24.05, main/busy/ms 18.59, render/present/ms 1.08, bigspace/total/ms 0.76, terrain/update/ms 0.58, controller/feed/ms 0.57, terrain/select/ms 0.53, stream/drive/ms 0.49, physics/insert/ms 0.45 |
| measure | 24.02 | 84.9 | render/thread/ms 162, render/acquire/ms 121, render/busy/ms 40.43, render/graph/ms 25.21, main/busy/ms 21.24, render/present/ms 1.07, terrain/update/ms 0.95, stream/drive/ms 0.88, terrain/select/ms 0.86, bigspace/total/ms 0.73, physics/insert/ms 0.62, controller/feed/ms 0.55 |
| measure | 24.10 | 86.1 | render/thread/ms 167, render/acquire/ms 121, render/busy/ms 45.13, render/graph/ms 29.40, main/busy/ms 22.71, render/present/ms 1.23, physics/insert/ms 0.98, bigspace/total/ms 0.95, terrain/update/ms 0.84, stream/drive/ms 0.78, terrain/select/ms 0.77, controller/feed/ms 0.58 |
| measure | 24.26 | 83.3 | render/thread/ms 149, render/acquire/ms 105, render/busy/ms 43.27, main/busy/ms 31.83, render/graph/ms 25.26, physics/insert/ms 1.79, stream/drive/ms 1.14, render/present/ms 1.04, controller/feed/ms 1.01, bigspace/total/ms 0.81, far/select/ms 0.78, terrain/update/ms 0.70 |
| measure | 24.89 | 85.0 | render/thread/ms 157, pipelines/wait_ms 121, render/acquire/ms 106, pipelines/compile_ms 74.59, pipelines/driver_ms 60.51, render/busy/ms 48.53, main/busy/ms 38.70, render/graph/ms 27.49, pipelines/front_ms 14.08, stream/drive/ms 1.76, render/present/ms 1.55, bigspace/total/ms 1.26 |
| measure | 25.05 | 88.2 | render/thread/ms 143, render/acquire/ms 79.38, main/busy/ms 67.54, render/busy/ms 62.39, render/graph/ms 32.50, render/present/ms 1.28, stream/drive/ms 1.27, far/select/ms 1.14, controller/feed/ms 1.12, physics/insert/ms 1.08, terrain/update/ms 0.79, bigspace/total/ms 0.76 |
| measure | 25.21 | 86.4 | render/thread/ms 153, render/acquire/ms 108, render/busy/ms 43.92, render/graph/ms 27.41, main/busy/ms 22.33, render/present/ms 1.35, bigspace/total/ms 0.84, stream/drive/ms 0.81, controller/feed/ms 0.75, physics/insert/ms 0.64, terrain/update/ms 0.62, terrain/select/ms 0.56 |
| measure | 25.52 | 83.5 | render/thread/ms 155, render/acquire/ms 112, render/busy/ms 41.14, main/busy/ms 33.69, render/graph/ms 23.56, stream/drive/ms 1.72, render/present/ms 1.20, stream/spawn/ms 1.13, physics/insert/ms 1.01, bigspace/total/ms 0.93, controller/feed/ms 0.89, terrain/update/ms 0.75 |
| measure | 25.69 | 83.8 | render/thread/ms 154, pipelines/wait_ms 131, render/acquire/ms 101, pipelines/compile_ms 60.66, render/busy/ms 51.76, pipelines/driver_ms 37.27, main/busy/ms 30.72, pipelines/front_ms 23.39, render/graph/ms 21.46, physics/insert/ms 1.81, stream/drive/ms 1.75, terrain/update/ms 1.61 |
| measure | 25.77 | 83.8 | render/thread/ms 160, render/acquire/ms 117, render/busy/ms 42.13, main/busy/ms 24.34, render/graph/ms 22.61, terrain/update/ms 1.61, stream/drive/ms 1.33, physics/insert/ms 1.25, render/present/ms 1.17, bigspace/total/ms 0.92, terrain/select/ms 0.78, controller/feed/ms 0.76 |
| measure | 25.86 | 86.4 | render/thread/ms 167, render/acquire/ms 125, render/busy/ms 40.54, render/graph/ms 24.89, main/busy/ms 24.04, stream/drive/ms 1.29, render/present/ms 1.27, physics/insert/ms 0.99, bigspace/total/ms 0.97, controller/feed/ms 0.87, terrain/update/ms 0.84, terrain/select/ms 0.75 |
| measure | 26.09 | 83.5 | render/thread/ms 159, render/acquire/ms 124, render/busy/ms 33.70, main/busy/ms 25.45, render/graph/ms 18.66, stream/drive/ms 1.56, render/present/ms 1.17, controller/feed/ms 0.99, stream/spawn/ms 0.89, bigspace/total/ms 0.88, physics/insert/ms 0.79, terrain/update/ms 0.59 |
| measure | 26.25 | 86.7 | render/thread/ms 155, render/acquire/ms 114, render/busy/ms 39.54, main/busy/ms 23.72, render/graph/ms 21.12, render/present/ms 1.19, stream/drive/ms 0.90, bigspace/total/ms 0.88, controller/feed/ms 0.85, physics/insert/ms 0.74, anim/evaluate_cpu/ms 0.62, terrain/update/ms 0.61 |
| measure | 27.51 | 84.8 | render/thread/ms 150, render/acquire/ms 102, render/busy/ms 46.20, render/graph/ms 31.40, main/busy/ms 23.79, render/present/ms 1.21, bigspace/total/ms 0.86, controller/feed/ms 0.85, stream/drive/ms 0.79, physics/insert/ms 0.71, terrain/update/ms 0.66, stream/tick/ms 0.66 |
| measure | 28.36 | 91.8 | render/thread/ms 147, render/acquire/ms 81.05, main/busy/ms 75.04, render/busy/ms 49.07, render/graph/ms 21.62, render/present/ms 17.08, physics/insert/ms 1.36, bigspace/total/ms 1.24, stream/drive/ms 1.00, controller/feed/ms 0.85, far/select/ms 0.68, anim/evaluate_cpu/ms 0.64 |
| measure | 28.60 | 181.7 | render/thread/ms 242, render/acquire/ms 191, render/busy/ms 46.04, main/busy/ms 29.96, render/graph/ms 25.47, render/present/ms 4.90, physics/insert/ms 1.37, bigspace/total/ms 1.08, terrain/update/ms 0.79, terrain/select/ms 0.69, controller/feed/ms 0.68, stream/drive/ms 0.68 |
| measure | 28.92 | 86.5 | render/thread/ms 162, render/acquire/ms 121, render/busy/ms 39.69, render/graph/ms 25.15, main/busy/ms 24.91, render/present/ms 1.15, bigspace/total/ms 1.04, controller/feed/ms 0.77, physics/insert/ms 0.76, terrain/update/ms 0.69, terrain/select/ms 0.61, anim/evaluate_cpu/ms 0.61 |
| measure | 29.69 | 82.8 | render/thread/ms 149, render/acquire/ms 104, render/busy/ms 43.41, render/graph/ms 27.44, main/busy/ms 25.50, physics/insert/ms 1.68, render/present/ms 1.31, terrain/update/ms 1.23, bigspace/total/ms 1.05, controller/feed/ms 0.70, terrain/select/ms 0.63, stream/drive/ms 0.46 |
| measure | 30.30 | 82.7 | render/thread/ms 161, render/acquire/ms 120, render/busy/ms 39.20, render/graph/ms 26.34, main/busy/ms 22.98, render/present/ms 1.12, bigspace/total/ms 0.92, physics/insert/ms 0.92, controller/feed/ms 0.76, terrain/update/ms 0.70, terrain/select/ms 0.63, stream/drive/ms 0.50 |
| measure | 31.24 | 83.2 | render/thread/ms 160, render/acquire/ms 116, render/busy/ms 43.30, render/graph/ms 27.75, main/busy/ms 20.88, physics/insert/ms 1.26, render/present/ms 1.11, bigspace/total/ms 0.79, controller/feed/ms 0.78, terrain/update/ms 0.61, terrain/select/ms 0.53, stream/drive/ms 0.47 |
| measure | 31.87 | 83.5 | render/thread/ms 157, render/acquire/ms 124, render/busy/ms 32.27, render/graph/ms 21.18, main/busy/ms 19.33, controller/feed/ms 1.37, physics/insert/ms 1.15, render/present/ms 1.13, bigspace/total/ms 0.75, terrain/update/ms 0.64, terrain/select/ms 0.58, stream/drive/ms 0.43 |
| measure | 32.42 | 88.4 | render/thread/ms 163, render/acquire/ms 120, render/busy/ms 41.65, render/graph/ms 23.49, main/busy/ms 21.83, stream/drive/ms 1.63, controller/feed/ms 1.32, render/present/ms 1.12, stream/despawn/ms 1.08, bigspace/total/ms 0.92, terrain/update/ms 0.65, terrain/select/ms 0.58 |
| measure | 32.96 | 87.6 | render/thread/ms 156, render/acquire/ms 120, render/busy/ms 35.29, main/busy/ms 23.43, render/graph/ms 22.70, stream/drive/ms 1.59, bigspace/total/ms 1.22, render/present/ms 1.18, stream/despawn/ms 1.11, physics/insert/ms 1.05, terrain/update/ms 0.85, terrain/select/ms 0.75 |
| measure | 33.36 | 83.5 | render/thread/ms 157, render/acquire/ms 109, render/busy/ms 47.04, render/graph/ms 32.40, main/busy/ms 20.47, render/present/ms 1.27, bigspace/total/ms 0.97, controller/feed/ms 0.75, terrain/update/ms 0.75, terrain/select/ms 0.67, physics/insert/ms 0.50, physics/step/ms 0.46 |
| measure | 36.14 | 84.0 | render/thread/ms 161, render/acquire/ms 123, render/busy/ms 37.37, render/graph/ms 25.07, main/busy/ms 23.10, bigspace/total/ms 1.28, render/present/ms 1.11, physics/insert/ms 0.99, terrain/update/ms 0.72, terrain/select/ms 0.65, controller/feed/ms 0.62, stream/drive/ms 0.53 |
| measure | 46.64 | 88.0 | render/thread/ms 101, render/acquire/ms 75.88, render/busy/ms 23.59, main/busy/ms 16.87, render/graph/ms 14.61, render/present/ms 1.16, bigspace/total/ms 0.83, controller/feed/ms 0.64, terrain/update/ms 0.55, terrain/select/ms 0.49, stream/drive/ms 0.40, controller/move/ms 0.37 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab_off\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 12% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **95%** (min 76%) · 2580 MHz (min 2580) · memory 8001 MHz · 57.2 W · 70 °C (max 77) · VRAM 7853 MiB · throttled 100% of samples
