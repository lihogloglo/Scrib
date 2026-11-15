# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5715 frames over 65.6 s · **median 11.18 ms (89 fps)** · mean 11.49 · p95 14.21 · p99 17.05 (1% low 59 fps) · worst 53.93 ms · 60 spikes over 1.5× median · 41.1 stutters/min (work 41.1/min)
- warm-up: 440 frames over 8.9 s, worst 2483.2 ms
- hitches (> 27.9 ms): **6** measured, 29 in warm-up
- timed GPU passes: 7.17 ms of a 11.49 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.18 ms, limit 11 ms
- **FAIL** frame_ms: 45 stutters (> 1.5x moving median of 60) in 1.09 min = 41.13/min, limit 1/min
- **FAIL** worst frame 53.93 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.054 | 2.186 | 2.525 | 0.94 | 0.179 |
| main_opaque_pass_3d | 1.467 | 1.561 | 2.204 | 0.94 | 0.062 |
| early prepass | 0.463 | 0.492 | 0.736 | 0.94 | 0.092 |
| bloom | 0.317 | 0.337 | 0.487 | 0.94 | 0.066 |
| atmosphere_luts | 0.308 | 0.328 | 0.472 | 0.94 | 0.003 |
| water_velocity | 0.286 | 0.305 | 0.552 | 0.94 | 0.002 |
| uniform_allocation | 0.242 | 0.258 | 0.262 | 0.94 | 0.048 |
| water_sspr | 0.207 | 0.220 | 0.218 | 0.94 | 0.002 |
| ssao | 0.190 | 0.202 | 0.210 | 0.94 | 0.008 |
| bin_unpacking | 0.176 | 0.188 | 0.197 | 0.94 | 0.027 |
| atmos_froxel_inject | 0.147 | 0.156 | 0.153 | 0.94 | 0.002 |
| auxiliary_views | 0.137 | 0.146 | 0.622 | 0.94 | 0.137 |
| clouds_trace | 0.133 | 0.141 | 0.234 | 0.94 | 0.001 |
| water_fft | 0.131 | 0.139 | 0.138 | 0.94 | 0.003 |
| upscaling | 0.121 | 0.129 | 0.150 | 0.94 | 0.004 |
| clouds_composite | 0.104 | 0.111 | 0.111 | 0.94 | 0.002 |
| tonemapping | 0.086 | 0.092 | 0.093 | 0.94 | 0.004 |
| main_transmissive_pass_3d | 0.084 | 0.089 | 0.169 | 0.94 | 0.008 |
| clustering | 0.064 | 0.068 | 0.070 | 0.94 | 0.020 |
| early_mesh_preprocessing | 0.054 | 0.057 | 0.062 | 0.94 | 0.005 |
| render_sky | 0.052 | 0.055 | 0.055 | 0.94 | 0.002 |
| lightprobe_irradiance_map | 0.048 | 0.445 | 0.758 | 0.11 | 0.001 |
| clouds_resolve | 0.047 | 0.050 | 0.051 | 0.94 | 0.001 |
| surface_trail | 0.042 | 0.060 | 0.057 | 0.70 | 0.002 |
| water_snapshot | 0.034 | 0.036 | 0.045 | 0.94 | 0.002 |
| clouds_shadow | 0.029 | 0.031 | 0.034 | 0.94 | 0.003 |
| atmos_froxel_integrate | 0.026 | 0.028 | 0.030 | 0.94 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.240 | 0.285 | 0.11 | 0.004 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.023 | 0.024 | 0.94 | 0.007 |
| main_transparent_pass_3d | 0.017 | 0.018 | 0.032 | 0.94 | 0.149 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.94 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.94 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.94 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.94 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 20.817 | 0.000 | 37751.13 | 118970.9 |
| render/thread/ms | 10.214 | 12.739 | 51.91 | 58374.0 |
| render/busy/ms | 9.571 | 12.116 | 51.34 | 54699.9 |
| main/busy/ms | 8.263 | 10.098 | 50.92 | 47222.6 |
| render/graph/ms | 5.165 | 6.702 | 45.58 | 29516.2 |
| render/present/ms | 0.552 | 1.387 | 13.57 | 3152.3 |
| bigspace/total/ms | 0.443 | 0.710 | 2.70 | 2532.9 |
| pipelines/compile_ms | 0.350 | 0.000 | 160.36 | 2001.4 |
| pipelines/front_ms | 0.328 | 0.000 | 154.38 | 1876.3 |
| controller/feed/ms | 0.298 | 0.392 | 0.85 | 1705.0 |
| terrain/update/ms | 0.296 | 0.376 | 1.80 | 1692.4 |
| terrain/select/ms | 0.264 | 0.334 | 0.75 | 1508.8 |
| stream/drive/ms | 0.243 | 0.338 | 16.77 | 1388.1 |
| far/select/ms | 0.193 | 0.267 | 0.66 | 1105.3 |
| stream/tick/ms | 0.188 | 0.258 | 0.77 | 1076.1 |
| bigspace/low_precision/ms | 0.168 | 0.291 | 2.26 | 958.1 |
| anim/evaluate_cpu/ms | 0.164 | 0.207 | 0.56 | 936.2 |
| controller/move/ms | 0.144 | 0.262 | 0.64 | 820.5 |
| anim/evaluate/ms | 0.138 | 0.267 | 2.10 | 788.8 |
| bigspace/high_precision/ms | 0.113 | 0.230 | 2.20 | 644.3 |
| render/acquire/ms | 0.091 | 0.117 | 0.64 | 521.8 |
| physics/step/ms | 0.049 | 0.084 | 0.93 | 281.0 |
| physics/insert/ms | 0.047 | 0.429 | 1.03 | 266.1 |
| weather/map/ms | 0.025 | 0.034 | 0.31 | 141.9 |
| pipelines/driver_ms | 0.022 | 0.000 | 56.66 | 125.1 |
| anim/apply/ms | 0.011 | 0.019 | 0.13 | 62.8 |
| controller/avoid/ms | 0.010 | 0.014 | 0.27 | 59.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.15 | 58.7 |
| script/update/ms | 0.007 | 0.009 | 0.27 | 40.1 |
| stream/despawn/ms | 0.005 | 0.000 | 0.99 | 29.9 |
| script/input/ms | 0.005 | 0.006 | 0.11 | 28.4 |
| script/ui/ms | 0.005 | 0.007 | 0.14 | 28.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.04 | 25.8 |
| controller/write/ms | 0.004 | 0.005 | 0.13 | 20.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.60 | 14.8 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 5.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 2.6 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 12 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 21 |
| controller/movers/count | count | 20 | 23 | 21 |
| crowd/c0/count | count | 27 | 43 | 11 |
| crowd/c0/joints/count | count | 1301 | 2262 | 492 |
| crowd/c0/meshes/count | count | 355 | 646 | 113 |
| crowd/c1/count | count | 147 | 156 | 149 |
| crowd/c1/joints/count | count | 5531 | 5991 | 5672 |
| crowd/c1/meshes/count | count | 1165 | 1295 | 1220 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 14.9 k | 6179 |
| crowd/joints_uploaded/count | count | 4250 | 6643 | 6179 |
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
| frame/work/ms | ms | 9.29 | 51.34 | 9.37 |
| gpu/timed/ms | ms | 8.35 | 11.78 | 9.02 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 249 | 478 | 471 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3912 | 4343 | 4307 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 249 | 478 | 471 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3912 | 4343 | 4307 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 4112 | 4327 | 4326 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 4112 | 4327 | 4326 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 10.1 k | 10.7 k | 10.5 k |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 54 | 293 | 42 |
| phases/shadow/bins | count | 12.1 k | 15.5 k | 11.9 k |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 38.9 k | 42.1 k | 39.3 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 139 | 300 | 42 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2350 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1769 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 204 | 208 | 208 |
| pipelines/waiting | count | 0 | 24 | 0 |
| preprocess/bind_groups | count | 11 | 759 | 6 |
| preprocess/bind_groups_created | count | 11 | 759 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 229 | 367 | 221 |
| process/cpu_usage | % | 23.32 | 37.80 | 19.20 |
| process/mem_usage | GiB | 5.18 | 5.35 | 5.35 |
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
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.61 | 49.10 | 28.02 |
| system/mem_usage | % | 63.86 | 64.66 | 64.40 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2591 | 2644 | 2644 |
| views/all/batch_sets | count | 112 | 347 | 102 |
| views/all/bins | count | 21.4 k | 24.0 k | 21.5 k |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 67.6 k | 70.5 k | 68.9 k |
| views/all/pipelines | count | 195 | 356 | 98 |
| views/all/pipelines_distinct | count | 84 | 94 | 77 |
| views/all/work_items | count | 66.8 k | 70.2 k | 69.6 k |
| views/cascade/batch_sets | count | 52 | 67 | 42 |
| views/cascade/bins | count | 11.8 k | 12.4 k | 11.9 k |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 38.3 k | 39.8 k | 39.3 k |
| views/cascade/pipelines | count | 52 | 67 | 42 |
| views/cascade/work_items | count | 38.3 k | 39.8 k | 39.3 k |
| views/lamp_face/batch_sets | count | 0 | 235 | 0 |
| views/lamp_face/bins | count | 1644 | 3755 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1769 | 4111 | 0 |
| views/lamp_face/pipelines | count | 86 | 240 | 0 |
| views/lamp_face/work_items | count | 0 | 3812 | 0 |
| views/main/batch_sets | count | 60 | 64 | 60 |
| views/main/bins | count | 8684 | 9594 | 9594 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 28.0 k | 29.7 k | 29.6 k |
| views/main/pipelines | count | 56 | 60 | 56 |
| views/main/work_items | count | 28.5 k | 30.4 k | 30.3 k |
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
| anim/bones_written/count | count | 48.69 | 305 | 278.3 k |
| anim/events/count | count | 0.10 | 3 | 564 |
| assets/Image/added | count | 0.11 | 43 | 627 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.74 | 152 | 4256 |
| assets/Mesh/removed | count | 0.85 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 503 |
| assets/StandardMaterial/modified | count | 3.81 | 18 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.27 | 1340 | 7243 |
| physics/inserts | count | 0.69 | 162 | 3963 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.41 | 145 | 2335 |
| physics/steps | count | 0.69 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 6 | 19 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 20 | 378 |
| stream/upload/bytes | bytes | 5923 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1299 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.9 ms; slowest 35, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2483.2 | main/busy/ms 2459, surface/advance_cpu_ms 68.67, surface/cell_upload_ms 0.76, stream/drive/ms 0.32, bigspace/total/ms 0.11, weather/map/ms 0.10, stream/tick/ms 0.09, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, script/update/ms 0.02, far/select/ms 0.02, script/input/ms 0.01 |
| warm-up | 3.15 | 666.1 | main/busy/ms 2491, render/thread/ms 633, render/busy/ms 511, render/acquire/ms 120, surface/advance_cpu_ms 68.67, render/graph/ms 5.12, render/present/ms 1.97, stream/drive/ms 1.31, surface/cell_upload_ms 0.76, stream/tick/ms 0.72, terrain/update/ms 0.48, terrain/select/ms 0.34 |
| warm-up | 3.24 | 90.8 | pipelines/wait_ms 1235, render/thread/ms 644, render/busy/ms 521, render/acquire/ms 120, pipelines/compile_ms 98.43, pipelines/front_ms 90.41, main/busy/ms 48.04, terrain/update/ms 16.20, render/graph/ms 8.77, pipelines/driver_ms 8.03, render/present/ms 2.90, stream/drive/ms 1.67 |
| warm-up | 3.48 | 83.8 | pipelines/compile_ms 239, pipelines/front_ms 224, pipelines/wait_ms 222, render/thread/ms 65.48, render/busy/ms 63.90, main/busy/ms 61.48, pipelines/driver_ms 14.96, render/graph/ms 3.55, stream/drive/ms 2.12, stream/tick/ms 1.77, render/present/ms 1.33, terrain/update/ms 0.76 |
| warm-up | 3.64 | 69.6 | pipelines/wait_ms 224, pipelines/compile_ms 177, pipelines/front_ms 168, main/busy/ms 44.28, render/thread/ms 43.07, render/busy/ms 41.59, stream/drive/ms 32.53, pipelines/driver_ms 8.55, render/graph/ms 4.14, render/present/ms 1.30, stream/tick/ms 0.85, terrain/update/ms 0.59 |
| warm-up | 3.79 | 57.9 | render/thread/ms 49.91, render/busy/ms 47.83, main/busy/ms 41.74, stream/drive/ms 19.52, render/graph/ms 6.94, stream/spawn/ms 2.40, render/present/ms 1.82, terrain/update/ms 1.19, stream/tick/ms 1.03, terrain/select/ms 0.95, bigspace/total/ms 0.40, render/acquire/ms 0.27 |
| warm-up | 3.83 | 42.6 | render/thread/ms 61.32, render/busy/ms 58.67, main/busy/ms 28.94, render/graph/ms 6.64, stream/drive/ms 4.98, stream/spawn/ms 2.79, render/present/ms 1.79, terrain/update/ms 1.22, stream/tick/ms 1.16, terrain/select/ms 1.00, render/acquire/ms 0.86, bigspace/total/ms 0.44 |
| warm-up | 3.88 | 42.6 | main/busy/ms 54.22, render/thread/ms 54.18, render/busy/ms 52.20, stream/drive/ms 30.07, render/graph/ms 3.95, stream/spawn/ms 2.18, stream/tick/ms 1.21, terrain/update/ms 1.20, render/present/ms 1.16, terrain/select/ms 0.99, render/acquire/ms 0.82, bigspace/total/ms 0.46 |
| warm-up | 3.95 | 68.3 | pipelines/wait_ms 2887, pipelines/compile_ms 297, pipelines/front_ms 283, render/thread/ms 65.31, render/busy/ms 63.89, main/busy/ms 53.30, stream/drive/ms 29.08, pipelines/driver_ms 14.60, render/graph/ms 5.00, render/present/ms 1.20, stream/spawn/ms 1.09, terrain/update/ms 0.99 |
| warm-up | 3.98 | 31.0 | pipelines/wait_ms 3337, pipelines/compile_ms 316, pipelines/front_ms 299, render/thread/ms 63.86, render/busy/ms 62.18, main/busy/ms 24.75, pipelines/driver_ms 16.51, render/graph/ms 9.71, stream/drive/ms 3.53, render/present/ms 1.45, stream/tick/ms 1.08, terrain/update/ms 0.94 |
| warm-up | 4.01 | 28.4 | pipelines/wait_ms 1876, render/thread/ms 47.92, render/busy/ms 46.24, pipelines/compile_ms 41.59, pipelines/front_ms 38.12, main/busy/ms 21.25, render/graph/ms 11.11, stream/drive/ms 3.64, pipelines/driver_ms 3.48, render/present/ms 1.41, stream/tick/ms 1.36, terrain/update/ms 0.97 |
| warm-up | 4.09 | 62.8 | pipelines/wait_ms 732, main/busy/ms 65.64, stream/drive/ms 49.09, pipelines/compile_ms 43.32, pipelines/front_ms 42.10, render/thread/ms 30.93, render/busy/ms 29.37, render/graph/ms 3.73, stream/spawn/ms 2.30, stream/tick/ms 1.58, render/present/ms 1.31, pipelines/driver_ms 1.22 |
| warm-up | 4.12 | 35.3 | pipelines/wait_ms 732, pipelines/compile_ms 43.32, pipelines/front_ms 42.10, render/thread/ms 38.04, render/busy/ms 36.59, main/busy/ms 30.57, stream/drive/ms 9.96, render/graph/ms 3.55, stream/spawn/ms 2.18, stream/tick/ms 1.54, pipelines/driver_ms 1.22, render/present/ms 1.21 |
| warm-up | 4.15 | 31.4 | main/busy/ms 61.19, render/thread/ms 42.50, stream/drive/ms 41.46, render/busy/ms 41.29, render/graph/ms 3.41, stream/spawn/ms 2.06, terrain/update/ms 1.03, render/present/ms 1.01, terrain/select/ms 0.84, stream/tick/ms 0.82, bigspace/total/ms 0.34, render/acquire/ms 0.21 |
| warm-up | 4.21 | 55.8 | pipelines/wait_ms 3253, pipelines/compile_ms 147, pipelines/front_ms 142, main/busy/ms 87.20, stream/drive/ms 70.19, render/thread/ms 47.32, render/busy/ms 46.01, pipelines/driver_ms 5.64, render/graph/ms 4.66, stream/spawn/ms 2.08, render/present/ms 1.09, stream/tick/ms 0.75 |
| warm-up | 4.25 | 42.6 | pipelines/wait_ms 3253, pipelines/compile_ms 147, pipelines/front_ms 142, main/busy/ms 50.80, render/thread/ms 42.78, render/busy/ms 41.62, stream/drive/ms 32.82, pipelines/driver_ms 5.64, render/graph/ms 4.59, stream/spawn/ms 2.09, stream/activate/ms 1.12, render/present/ms 0.97 |
| warm-up | 4.30 | 38.5 | pipelines/wait_ms 3776, pipelines/compile_ms 141, pipelines/front_ms 131, main/busy/ms 39.09, stream/drive/ms 27.69, render/thread/ms 22.22, render/busy/ms 21.07, pipelines/driver_ms 10.03, render/graph/ms 4.05, stream/activate/ms 1.43, render/present/ms 0.96, stream/tick/ms 0.88 |
| warm-up | 4.62 | 39.0 | render/thread/ms 44.23, render/busy/ms 42.90, main/busy/ms 24.31, stream/drive/ms 9.96, render/graph/ms 4.14, render/present/ms 1.01, stream/tick/ms 0.72, terrain/update/ms 0.68, terrain/select/ms 0.57, bigspace/total/ms 0.49, stream/spawn/ms 0.41, bigspace/high_precision/ms 0.32 |
| warm-up | 4.72 | 29.3 | pipelines/wait_ms 5077, pipelines/compile_ms 51.86, pipelines/front_ms 48.04, render/thread/ms 40.60, render/busy/ms 39.00, main/busy/ms 22.43, render/graph/ms 5.61, pipelines/driver_ms 3.83, stream/drive/ms 3.04, stream/tick/ms 1.82, render/present/ms 1.33, terrain/update/ms 1.00 |
| warm-up | 4.94 | 29.8 | main/busy/ms 29.45, render/thread/ms 27.56, render/busy/ms 26.06, render/graph/ms 5.94, stream/drive/ms 1.90, render/present/ms 1.20, stream/tick/ms 0.80, terrain/update/ms 0.79, terrain/select/ms 0.66, bigspace/total/ms 0.47, render/acquire/ms 0.30, bigspace/high_precision/ms 0.27 |
| warm-up | 4.98 | 33.2 | render/thread/ms 34.99, render/busy/ms 33.63, main/busy/ms 24.43, render/graph/ms 5.31, stream/drive/ms 2.70, render/present/ms 1.15, stream/spawn/ms 1.10, stream/tick/ms 0.86, terrain/update/ms 0.77, terrain/select/ms 0.65, bigspace/total/ms 0.45, render/acquire/ms 0.22 |
| warm-up | 5.15 | 28.7 | pipelines/wait_ms 3041, pipelines/compile_ms 131, pipelines/front_ms 126, render/thread/ms 39.18, render/busy/ms 37.50, main/busy/ms 18.62, render/graph/ms 6.72, pipelines/driver_ms 4.90, stream/drive/ms 3.46, stream/spawn/ms 2.12, render/present/ms 1.39, stream/tick/ms 0.69 |
| warm-up | 5.21 | 38.6 | pipelines/wait_ms 5503, render/thread/ms 45.43, render/busy/ms 44.01, pipelines/compile_ms 27.76, pipelines/front_ms 24.63, main/busy/ms 19.61, render/graph/ms 4.71, pipelines/driver_ms 3.14, stream/drive/ms 2.81, stream/spawn/ms 1.54, render/present/ms 1.14, terrain/update/ms 0.85 |
| warm-up | 5.43 | 32.2 | pipelines/wait_ms 3153, render/thread/ms 37.39, render/busy/ms 36.27, pipelines/compile_ms 33.69, pipelines/front_ms 32.02, main/busy/ms 24.89, stream/drive/ms 3.37, render/graph/ms 3.34, stream/spawn/ms 1.87, bigspace/total/ms 1.72, pipelines/driver_ms 1.67, bigspace/low_precision/ms 1.12 |
| warm-up | 5.51 | 32.4 | pipelines/wait_ms 6021, pipelines/compile_ms 97.85, pipelines/front_ms 92.61, render/thread/ms 37.21, render/busy/ms 35.94, main/busy/ms 29.48, stream/drive/ms 13.06, render/graph/ms 5.33, pipelines/driver_ms 5.24, stream/spawn/ms 2.06, render/present/ms 1.06, stream/tick/ms 0.72 |
| warm-up | 5.68 | 35.3 | render/thread/ms 43.05, render/busy/ms 41.73, main/busy/ms 21.00, render/graph/ms 6.17, stream/drive/ms 3.03, stream/spawn/ms 1.28, render/present/ms 1.09, stream/tick/ms 0.85, bigspace/total/ms 0.79, terrain/update/ms 0.72, terrain/select/ms 0.61, bigspace/high_precision/ms 0.33 |
| warm-up | 5.73 | 37.1 | render/thread/ms 50.82, render/busy/ms 49.11, main/busy/ms 26.04, render/graph/ms 7.53, stream/drive/ms 4.75, stream/spawn/ms 2.10, stream/tick/ms 1.26, render/present/ms 1.22, terrain/update/ms 0.87, terrain/select/ms 0.72, bigspace/total/ms 0.69, render/acquire/ms 0.49 |
| warm-up | 7.01 | 37.1 | pipelines/wait_ms 21.1 k, pipelines/compile_ms 97.54, pipelines/front_ms 93.24, main/busy/ms 53.82, render/thread/ms 26.60, render/busy/ms 24.86, render/graph/ms 9.54, pipelines/driver_ms 4.30, stream/drive/ms 1.58, render/present/ms 1.51, physics/insert/ms 1.19, bigspace/total/ms 0.80 |
| warm-up | 7.05 | 40.9 | pipelines/wait_ms 24.4 k, pipelines/compile_ms 84.54, pipelines/front_ms 80.11, main/busy/ms 48.37, render/thread/ms 38.66, render/busy/ms 37.07, render/graph/ms 7.66, pipelines/driver_ms 4.43, stream/drive/ms 1.50, render/present/ms 1.34, physics/insert/ms 1.07, bigspace/total/ms 1.03 |
| measure | 0.56 | 40.4 | pipelines/wait_ms 42.1 k, pipelines/compile_ms 225, pipelines/front_ms 164, pipelines/driver_ms 60.76, render/thread/ms 48.01, render/busy/ms 46.83, render/graph/ms 38.65, main/busy/ms 15.86, render/present/ms 0.97, physics/insert/ms 0.96, bigspace/total/ms 0.68, terrain/update/ms 0.65 |
| measure | 0.60 | 32.3 | pipelines/wait_ms 5490, render/thread/ms 34.23, render/busy/ms 32.96, main/busy/ms 29.42, render/graph/ms 11.43, pipelines/compile_ms 9.40, pipelines/front_ms 8.80, render/present/ms 1.02, physics/insert/ms 0.85, bigspace/total/ms 0.85, terrain/update/ms 0.80, physics/step/ms 0.76 |
| measure | 0.79 | 53.9 | pipelines/wait_ms 5454, pipelines/compile_ms 91.11, pipelines/front_ms 81.27, render/thread/ms 65.10, render/busy/ms 63.96, main/busy/ms 59.96, render/graph/ms 50.28, pipelines/driver_ms 9.84, stream/drive/ms 1.44, bigspace/total/ms 1.12, stream/spawn/ms 1.02, render/present/ms 0.92 |
| measure | 2.17 | 31.2 | render/thread/ms 41.73, render/busy/ms 40.32, render/graph/ms 18.47, main/busy/ms 16.22, pipelines/compile_ms 12.67, pipelines/front_ms 11.99, render/present/ms 1.24, bigspace/total/ms 0.75, pipelines/driver_ms 0.68, terrain/update/ms 0.55, controller/feed/ms 0.53, terrain/select/ms 0.50 |
| measure | 19.26 | 30.0 | render/thread/ms 36.34, render/busy/ms 35.03, render/graph/ms 19.92, main/busy/ms 16.61, render/present/ms 1.13, bigspace/total/ms 0.99, physics/insert/ms 0.87, terrain/update/ms 0.64, terrain/select/ms 0.57, controller/feed/ms 0.56, stream/drive/ms 0.44, stream/tick/ms 0.39 |
| measure | 59.01 | 28.7 | render/thread/ms 36.89, render/busy/ms 34.85, main/busy/ms 28.69, render/graph/ms 12.41, bigspace/total/ms 3.32, bigspace/high_precision/ms 2.34, render/present/ms 1.75, stream/drive/ms 1.71, controller/feed/ms 1.09, stream/spawn/ms 0.97, terrain/update/ms 0.69, far/select/ms 0.64 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_new_nomaps\balmora_walk@2560x1440_off=normalmaps_orm_parallax-2 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 0%, CPU 8%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **86%** (min 56%) · 2574 MHz (min 2565) · memory 8001 MHz · 81.6 W · 78 °C (max 83) · VRAM 5298 MiB · throttled 100% of samples
