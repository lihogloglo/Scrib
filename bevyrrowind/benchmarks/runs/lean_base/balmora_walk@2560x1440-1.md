# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4516 frames over 65.7 s · **median 14.15 ms (71 fps)** · mean 14.54 · p95 18.29 · p99 22.37 (1% low 45 fps) · worst 53.16 ms · 62 spikes over 1.5× median · 27.4 stutters/min (work 20.1/min)
- warm-up: 1997 frames over 27.3 s, worst 4060.9 ms
- hitches (> 35.4 ms): **3** measured, 19 in warm-up
- timed GPU passes: 8.05 ms of a 14.54 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 14.15 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- **FAIL** worst frame 53.16 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.211 | 2.218 | 2.625 | 1.00 | 0.193 |
| main_opaque_pass_3d | 1.738 | 1.743 | 2.404 | 1.00 | 0.075 |
| early prepass | 0.551 | 0.553 | 0.856 | 1.00 | 0.130 |
| bloom | 0.342 | 0.343 | 0.503 | 1.00 | 0.072 |
| atmosphere_luts | 0.327 | 0.328 | 0.457 | 1.00 | 0.004 |
| uniform_allocation | 0.326 | 0.327 | 0.625 | 1.00 | 0.060 |
| water_velocity | 0.291 | 0.292 | 0.510 | 1.00 | 0.002 |
| bin_unpacking | 0.251 | 0.252 | 0.272 | 1.00 | 0.042 |
| water_sspr | 0.229 | 0.230 | 0.227 | 1.00 | 0.002 |
| ssao | 0.207 | 0.207 | 0.213 | 1.00 | 0.008 |
| auxiliary_views | 0.165 | 0.165 | 0.685 | 1.00 | 0.156 |
| atmos_froxel_inject | 0.158 | 0.158 | 0.153 | 1.00 | 0.002 |
| clouds_trace | 0.142 | 0.142 | 0.234 | 1.00 | 0.001 |
| water_fft | 0.139 | 0.139 | 0.139 | 1.00 | 0.003 |
| upscaling | 0.125 | 0.126 | 0.139 | 1.00 | 0.005 |
| clouds_composite | 0.111 | 0.111 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.092 | 0.093 | 0.092 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.091 | 0.091 | 0.173 | 1.00 | 0.009 |
| clustering | 0.071 | 0.071 | 0.070 | 1.00 | 0.022 |
| lightprobe_irradiance_map | 0.055 | 0.448 | 0.754 | 0.12 | 0.001 |
| render_sky | 0.055 | 0.055 | 0.054 | 1.00 | 0.002 |
| surface_trail | 0.052 | 0.059 | 0.057 | 0.88 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.046 | 0.046 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.036 | 0.037 | 0.045 | 1.00 | 0.002 |
| clouds_shadow | 0.032 | 0.032 | 0.035 | 1.00 | 0.004 |
| lightprobe_radiance_map | 0.028 | 0.230 | 0.236 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.032 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 1.00 | 0.117 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.002 | 0.013 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.03 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 114.616 | 0.000 | 70233.49 | 517606.1 |
| render/thread/ms | 13.221 | 16.827 | 50.74 | 59705.4 |
| render/busy/ms | 12.625 | 16.165 | 49.75 | 57015.9 |
| main/busy/ms | 9.954 | 13.131 | 35.68 | 44952.5 |
| render/graph/ms | 7.664 | 9.867 | 33.85 | 34611.6 |
| pipelines/compile_ms | 5.869 | 0.000 | 2878.54 | 26504.0 |
| pipelines/driver_ms | 5.334 | 0.000 | 2746.73 | 24087.2 |
| pipelines/front_ms | 0.535 | 0.000 | 168.16 | 2416.8 |
| render/present/ms | 0.499 | 0.610 | 18.60 | 2253.3 |
| bigspace/total/ms | 0.482 | 0.756 | 4.47 | 2177.0 |
| terrain/update/ms | 0.298 | 0.382 | 1.54 | 1345.1 |
| controller/feed/ms | 0.298 | 0.408 | 0.89 | 1344.6 |
| stream/drive/ms | 0.275 | 0.411 | 25.91 | 1240.1 |
| terrain/select/ms | 0.264 | 0.336 | 0.73 | 1191.3 |
| far/select/ms | 0.205 | 0.292 | 0.59 | 926.1 |
| stream/tick/ms | 0.202 | 0.291 | 0.72 | 911.9 |
| bigspace/low_precision/ms | 0.183 | 0.315 | 3.74 | 826.9 |
| anim/evaluate_cpu/ms | 0.165 | 0.211 | 0.51 | 744.5 |
| controller/move/ms | 0.158 | 0.328 | 0.64 | 712.3 |
| anim/evaluate/ms | 0.153 | 0.316 | 1.66 | 693.0 |
| bigspace/high_precision/ms | 0.116 | 0.238 | 4.15 | 524.8 |
| render/acquire/ms | 0.097 | 0.129 | 0.43 | 436.3 |
| physics/step/ms | 0.066 | 0.102 | 0.84 | 298.1 |
| physics/insert/ms | 0.048 | 0.428 | 1.06 | 216.0 |
| weather/map/ms | 0.027 | 0.037 | 0.32 | 120.8 |
| anim/apply/ms | 0.011 | 0.022 | 0.32 | 51.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.30 | 50.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.25 | 49.7 |
| script/update/ms | 0.008 | 0.011 | 0.17 | 34.6 |
| stream/despawn/ms | 0.007 | 0.000 | 1.16 | 31.8 |
| stream/spawn/ms | 0.006 | 0.000 | 1.12 | 26.6 |
| script/ui/ms | 0.005 | 0.008 | 0.10 | 24.5 |
| script/input/ms | 0.005 | 0.007 | 0.06 | 22.4 |
| controller/write/ms | 0.004 | 0.005 | 0.17 | 17.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 14.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 4.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.08 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.05 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 36 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 538 |
| crowd/c0/meshes/count | count | 353 | 646 | 129 |
| crowd/c1/count | count | 147 | 155 | 149 |
| crowd/c1/joints/count | count | 5531 | 5885 | 5658 |
| crowd/c1/meshes/count | count | 1175 | 1243 | 1212 |
| crowd/c2/count | count | 397 | 417 | 417 |
| crowd/c2/joints/count | count | 6402 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4280 | 7481 | 6179 |
| crowd/joints_uploaded/count | count | 4262 | 6643 | 6179 |
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
| frame/work/ms | ms | 12.30 | 49.75 | 11.12 |
| gpu/timed/ms | ms | 8.75 | 13.10 | 9.19 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 799 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 799 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 34 | 40 | 33 |
| phases/opaque3d/bins | count | 1288 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3360 | 5153 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 34 | 40 | 33 |
| phases/opaque3d_prepass/bins | count | 1288 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3360 | 5153 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 69 | 318 | 57 |
| phases/shadow/bins | count | 3710 | 5409 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9352 | 12.5 k | 5667 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 136 | 315 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 201 | 476 | 19 |
| physics/pending | count | 0 | 709 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 23 | 0 |
| preprocess/bind_groups | count | 13 | 873 | 10 |
| preprocess/bind_groups_created | count | 13 | 873 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 303 | 475 | 287 |
| process/cpu_usage | % | 24.07 | 51.44 | 21.88 |
| process/mem_usage | GiB | 5.30 | 5.48 | 5.47 |
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
| started_load_count |  | 4769 | 4862 | 4862 |
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
| stream/latency/s | s | 0.05 | 0.14 | 0.05 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.41 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.03 | 63.77 | 47.40 |
| system/mem_usage | % | 76.81 | 80.23 | 77.25 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 155 | 404 | 137 |
| views/all/bins | count | 6308 | 8206 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.9 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 202 | 381 | 113 |
| views/all/pipelines_distinct | count | 96 | 108 | 91 |
| views/all/work_items | count | 17.2 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 68 | 90 | 57 |
| views/cascade/bins | count | 2583 | 3156 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8126 | 9777 | 5667 |
| views/cascade/pipelines | count | 55 | 73 | 47 |
| views/cascade/work_items | count | 8126 | 9777 | 5667 |
| views/lamp_face/batch_sets | count | 0 | 246 | 0 |
| views/lamp_face/bins | count | 1058 | 2484 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1181 | 2852 | 0 |
| views/lamp_face/pipelines | count | 81 | 254 | 0 |
| views/lamp_face/work_items | count | 0 | 2613 | 0 |
| views/main/batch_sets | count | 84 | 96 | 80 |
| views/main/bins | count | 2873 | 3586 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8320 | 13.1 k | 7600 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8488 | 13.3 k | 7908 |
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
| anim/bones_written/count | count | 48.13 | 258 | 217.4 k |
| anim/events/count | count | 0.11 | 2 | 490 |
| assets/Image/added | count | 0.13 | 23 | 596 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.91 | 152 | 4101 |
| assets/Mesh/removed | count | 1.08 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.10 | 29 | 469 |
| assets/StandardMaterial/modified | count | 4.79 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.09 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.20 | 1492 | 5434 |
| physics/inserts | count | 0.64 | 109 | 2910 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.52 | 145 | 2335 |
| physics/steps | count | 0.87 | 3 | 3938 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 6 | 24 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 18 | 378 |
| stream/upload/bytes | bytes | 7496 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1643 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 35.4 ms; slowest 22, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 4.06 | 4060.9 | main/busy/ms 4034, surface/advance_cpu_ms 70.41, surface/cell_upload_ms 0.74, stream/drive/ms 0.39, weather/map/ms 0.17, stream/tick/ms 0.13, bigspace/total/ms 0.11, bigspace/high_precision/ms 0.06, weather/eval/ms 0.06, script/update/ms 0.03, script/input/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 4.96 | 902.3 | main/busy/ms 4075, render/thread/ms 866, render/busy/ms 671, render/acquire/ms 188, surface/advance_cpu_ms 70.41, render/graph/ms 9.54, render/present/ms 6.88, stream/drive/ms 1.39, surface/cell_upload_ms 0.74, stream/tick/ms 0.71, terrain/update/ms 0.39, weather/map/ms 0.28 |
| warm-up | 5.21 | 248.8 | pipelines/wait_ms 1355, render/thread/ms 879, render/busy/ms 683, pipelines/compile_ms 423, pipelines/driver_ms 372, render/acquire/ms 188, main/busy/ms 60.49, pipelines/front_ms 50.79, terrain/update/ms 22.38, render/graph/ms 13.75, render/present/ms 8.08, stream/drive/ms 1.95 |
| warm-up | 5.53 | 98.4 | main/busy/ms 71.08, render/thread/ms 58.88, render/busy/ms 56.65, render/graph/ms 4.78, stream/drive/ms 2.48, render/present/ms 1.94, stream/tick/ms 1.83, terrain/update/ms 0.96, terrain/select/ms 0.78, render/acquire/ms 0.29, bigspace/total/ms 0.26, weather/map/ms 0.15 |
| warm-up | 5.73 | 36.7 | main/busy/ms 32.63, render/thread/ms 32.32, render/busy/ms 30.61, stream/drive/ms 19.60, render/graph/ms 4.51, render/present/ms 1.43, stream/tick/ms 1.42, terrain/update/ms 0.78, terrain/select/ms 0.63, bigspace/total/ms 0.29, render/acquire/ms 0.28, bigspace/high_precision/ms 0.16 |
| warm-up | 5.78 | 38.1 | main/busy/ms 46.14, stream/drive/ms 32.08, render/thread/ms 23.68, render/busy/ms 22.07, render/graph/ms 3.82, render/present/ms 1.37, stream/tick/ms 1.34, terrain/update/ms 0.81, terrain/select/ms 0.67, render/acquire/ms 0.24, bigspace/total/ms 0.21, bigspace/high_precision/ms 0.09 |
| warm-up | 6.28 | 48.0 | pipelines/compile_ms 768, pipelines/driver_ms 709, pipelines/wait_ms 255, pipelines/front_ms 59.09, main/busy/ms 51.01, stream/drive/ms 34.08, render/thread/ms 23.62, render/busy/ms 22.03, render/graph/ms 4.13, stream/tick/ms 2.00, render/present/ms 1.30, stream/spawn/ms 1.20 |
| warm-up | 6.39 | 37.2 | render/thread/ms 51.66, render/busy/ms 49.22, main/busy/ms 20.26, render/graph/ms 10.50, stream/drive/ms 4.25, render/present/ms 2.22, stream/tick/ms 1.97, terrain/update/ms 0.87, terrain/select/ms 0.69, stream/activate/ms 0.46, bigspace/total/ms 0.25, render/acquire/ms 0.22 |
| warm-up | 6.42 | 35.8 | render/thread/ms 58.53, render/busy/ms 56.18, main/busy/ms 23.78, render/graph/ms 13.42, stream/drive/ms 5.61, stream/tick/ms 2.19, render/present/ms 2.13, stream/spawn/ms 1.10, terrain/update/ms 0.94, terrain/select/ms 0.74, stream/activate/ms 0.71, physics/insert/ms 0.44 |
| warm-up | 6.46 | 38.1 | render/thread/ms 59.46, render/busy/ms 56.99, main/busy/ms 25.23, render/graph/ms 21.07, stream/drive/ms 6.78, stream/spawn/ms 3.18, stream/tick/ms 2.30, render/present/ms 2.18, terrain/update/ms 1.09, terrain/select/ms 0.86, render/acquire/ms 0.29, physics/insert/ms 0.28 |
| warm-up | 6.55 | 57.8 | main/busy/ms 87.67, stream/drive/ms 72.51, render/thread/ms 42.85, render/busy/ms 41.02, render/graph/ms 5.29, stream/spawn/ms 1.77, stream/tick/ms 1.76, render/present/ms 1.45, terrain/update/ms 0.84, terrain/select/ms 0.70, render/acquire/ms 0.38, bigspace/total/ms 0.36 |
| warm-up | 6.60 | 53.9 | main/busy/ms 62.34, render/thread/ms 45.01, render/busy/ms 43.21, stream/drive/ms 39.86, render/graph/ms 4.82, stream/spawn/ms 2.35, stream/tick/ms 2.08, render/present/ms 1.44, terrain/update/ms 0.75, terrain/select/ms 0.60, bigspace/total/ms 0.38, render/acquire/ms 0.36 |
| warm-up | 6.70 | 60.4 | main/busy/ms 88.67, stream/drive/ms 67.81, render/thread/ms 46.65, render/busy/ms 44.86, render/graph/ms 6.52, stream/tick/ms 1.57, stream/spawn/ms 1.42, render/present/ms 1.40, terrain/update/ms 0.85, terrain/select/ms 0.70, stream/activate/ms 0.56, bigspace/total/ms 0.44 |
| warm-up | 6.74 | 37.6 | main/busy/ms 53.49, render/thread/ms 44.64, render/busy/ms 42.93, stream/drive/ms 32.03, render/graph/ms 5.94, stream/spawn/ms 1.72, stream/tick/ms 1.71, render/present/ms 1.44, stream/activate/ms 1.12, terrain/update/ms 0.88, bigspace/total/ms 0.88, terrain/select/ms 0.71 |
| warm-up | 6.80 | 38.9 | main/busy/ms 58.85, stream/drive/ms 37.96, render/thread/ms 36.15, render/busy/ms 34.15, render/graph/ms 6.02, render/present/ms 1.74, stream/tick/ms 1.73, terrain/update/ms 0.77, terrain/select/ms 0.63, stream/spawn/ms 0.54, stream/activate/ms 0.39, physics/insert/ms 0.31 |
| warm-up | 8.09 | 36.6 | main/busy/ms 44.74, stream/drive/ms 27.01, render/thread/ms 24.99, render/busy/ms 23.21, terrain/update/ms 5.95, render/graph/ms 5.16, render/present/ms 1.51, stream/tick/ms 1.01, terrain/select/ms 0.59, stream/spawn/ms 0.41, bigspace/total/ms 0.38, render/acquire/ms 0.27 |
| warm-up | 8.78 | 36.9 | main/busy/ms 40.31, render/thread/ms 29.94, render/busy/ms 28.49, render/graph/ms 5.59, stream/drive/ms 2.35, stream/tick/ms 1.27, render/present/ms 1.21, terrain/update/ms 0.90, terrain/select/ms 0.69, physics/insert/ms 0.63, bigspace/total/ms 0.62, stream/spawn/ms 0.51 |
| warm-up | 9.23 | 46.8 | main/busy/ms 48.97, render/thread/ms 31.17, render/busy/ms 29.02, render/graph/ms 4.82, stream/drive/ms 2.19, render/present/ms 1.84, stream/tick/ms 1.30, physics/insert/ms 1.03, terrain/update/ms 0.87, terrain/select/ms 0.72, bigspace/total/ms 0.57, render/acquire/ms 0.32 |
| warm-up | 11.80 | 36.9 | pipelines/wait_ms 1721, pipelines/compile_ms 1631, pipelines/driver_ms 1474, pipelines/front_ms 157, render/thread/ms 31.18, render/busy/ms 29.66, main/busy/ms 27.82, render/graph/ms 5.27, render/present/ms 1.35, bigspace/total/ms 0.93, terrain/update/ms 0.82, physics/insert/ms 0.72 |
| measure | 2.74 | 42.1 | render/thread/ms 57.51, render/busy/ms 55.79, render/graph/ms 30.22, main/busy/ms 22.57, render/present/ms 1.49, bigspace/total/ms 0.86, physics/insert/ms 0.69, controller/feed/ms 0.68, terrain/update/ms 0.60, stream/drive/ms 0.57, terrain/select/ms 0.54, stream/tick/ms 0.48 |
| measure | 18.21 | 39.8 | main/busy/ms 47.40, render/thread/ms 29.49, render/busy/ms 28.30, stream/drive/ms 26.16, render/graph/ms 15.09, bigspace/total/ms 1.10, render/present/ms 0.98, controller/feed/ms 0.86, terrain/update/ms 0.63, terrain/select/ms 0.57, far/select/ms 0.40, anim/evaluate/ms 0.38 |
| measure | 19.30 | 53.2 | render/thread/ms 71.03, render/busy/ms 69.35, render/graph/ms 44.32, main/busy/ms 22.81, render/present/ms 1.48, controller/feed/ms 1.24, bigspace/total/ms 1.02, terrain/update/ms 0.72, physics/insert/ms 0.67, terrain/select/ms 0.63, stream/drive/ms 0.50, far/select/ms 0.43 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_base\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (250 samples): GPU load **72%** (min 48%) · 2580 MHz (min 2580) · memory 8001 MHz · 70.0 W · 71 °C (max 76) · VRAM 6285 MiB · throttled 100% of samples
