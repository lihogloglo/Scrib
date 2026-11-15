# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3959 frames over 65.7 s · **median 15.47 ms (65 fps)** · mean 16.59 · p95 25.58 · p99 28.24 (1% low 35 fps) · worst 48.49 ms · 450 spikes over 1.5× median · 23.8 stutters/min (work 21.9/min)
- warm-up: 339 frames over 8.5 s, worst 2493.0 ms
- hitches (> 38.7 ms): **1** measured, 13 in warm-up
- timed GPU passes: 8.06 ms of a 16.59 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 15.47 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.75/min, limit 1/min
- **FAIL** worst frame 48.49 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.193 | 2.195 | 2.572 | 1.00 | 0.200 |
| main_opaque_pass_3d | 1.694 | 1.695 | 2.399 | 1.00 | 0.085 |
| early prepass | 0.520 | 0.520 | 0.830 | 1.00 | 0.139 |
| bloom | 0.340 | 0.340 | 0.498 | 1.00 | 0.078 |
| uniform_allocation | 0.335 | 0.336 | 0.628 | 1.00 | 0.065 |
| atmosphere_luts | 0.334 | 0.334 | 0.616 | 1.00 | 0.004 |
| water_velocity | 0.308 | 0.309 | 0.380 | 1.00 | 0.002 |
| bin_unpacking | 0.266 | 0.266 | 0.380 | 1.00 | 0.046 |
| auxiliary_views | 0.242 | 0.242 | 0.786 | 1.00 | 0.210 |
| water_sspr | 0.222 | 0.222 | 0.223 | 1.00 | 0.003 |
| ssao | 0.202 | 0.202 | 0.210 | 1.00 | 0.008 |
| atmos_froxel_inject | 0.158 | 0.159 | 0.154 | 1.00 | 0.002 |
| water_fft | 0.141 | 0.142 | 0.140 | 1.00 | 0.003 |
| clouds_trace | 0.140 | 0.140 | 0.239 | 1.00 | 0.001 |
| upscaling | 0.126 | 0.126 | 0.137 | 1.00 | 0.005 |
| clouds_composite | 0.113 | 0.113 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.092 | 0.092 | 0.093 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.090 | 0.090 | 0.168 | 1.00 | 0.009 |
| clustering | 0.067 | 0.067 | 0.069 | 1.00 | 0.022 |
| lightprobe_irradiance_map | 0.055 | 0.441 | 0.742 | 0.13 | 0.001 |
| render_sky | 0.054 | 0.054 | 0.054 | 1.00 | 0.002 |
| surface_trail | 0.053 | 0.058 | 0.057 | 0.92 | 0.003 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.045 | 0.045 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.036 | 0.037 | 0.045 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.004 |
| lightprobe_radiance_map | 0.029 | 0.229 | 0.236 | 0.13 | 0.006 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.031 | 1.00 | 0.001 |
| water_mips | 0.025 | 0.025 | 0.025 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.014 | 0.014 | 0.030 | 1.00 | 0.122 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.03 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 56.980 | 0.000 | 14181.12 | 225585.7 |
| render/thread/ms | 15.247 | 23.648 | 35.42 | 60363.5 |
| render/busy/ms | 14.595 | 22.673 | 34.60 | 57781.6 |
| main/busy/ms | 10.838 | 16.212 | 30.00 | 42907.0 |
| render/graph/ms | 9.043 | 13.420 | 23.52 | 35802.7 |
| pipelines/compile_ms | 0.749 | 0.000 | 151.71 | 2964.2 |
| pipelines/front_ms | 0.723 | 0.000 | 145.16 | 2860.8 |
| render/present/ms | 0.550 | 0.756 | 18.33 | 2176.3 |
| bigspace/total/ms | 0.503 | 0.816 | 5.72 | 1992.1 |
| terrain/update/ms | 0.316 | 0.434 | 2.30 | 1252.4 |
| controller/feed/ms | 0.299 | 0.470 | 1.36 | 1181.9 |
| stream/drive/ms | 0.290 | 0.462 | 21.25 | 1146.9 |
| terrain/select/ms | 0.280 | 0.382 | 0.87 | 1108.3 |
| stream/tick/ms | 0.219 | 0.318 | 0.64 | 865.5 |
| far/select/ms | 0.215 | 0.350 | 0.71 | 849.5 |
| bigspace/low_precision/ms | 0.187 | 0.304 | 5.35 | 741.4 |
| anim/evaluate_cpu/ms | 0.181 | 0.244 | 0.65 | 716.4 |
| controller/move/ms | 0.164 | 0.352 | 0.78 | 648.1 |
| anim/evaluate/ms | 0.148 | 0.301 | 3.32 | 587.4 |
| bigspace/high_precision/ms | 0.125 | 0.262 | 3.49 | 494.5 |
| render/acquire/ms | 0.102 | 0.159 | 0.64 | 405.6 |
| physics/insert/ms | 0.083 | 0.498 | 1.08 | 329.0 |
| physics/step/ms | 0.077 | 0.163 | 0.70 | 306.6 |
| pipelines/driver_ms | 0.026 | 0.000 | 6.54 | 103.4 |
| weather/map/ms | 0.026 | 0.036 | 0.31 | 101.7 |
| anim/apply/ms | 0.013 | 0.021 | 0.14 | 50.3 |
| controller/avoid/ms | 0.011 | 0.015 | 0.18 | 44.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.11 | 43.4 |
| script/update/ms | 0.008 | 0.014 | 0.21 | 31.9 |
| stream/despawn/ms | 0.008 | 0.000 | 1.01 | 30.7 |
| stream/spawn/ms | 0.007 | 0.000 | 1.12 | 26.7 |
| script/ui/ms | 0.006 | 0.009 | 0.10 | 22.2 |
| script/input/ms | 0.006 | 0.008 | 0.27 | 21.9 |
| stream/deactivate/ms | 0.004 | 0.000 | 0.69 | 15.5 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 14.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.16 | 4.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 18 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 558 | 579 | 578 |
| anim/characters/count | count | 560 | 579 | 578 |
| anim/evaluated/count | count | 19 | 74 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1306 | 2262 | 535 |
| crowd/c0/meshes/count | count | 355 | 646 | 144 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5445 | 5937 | 5629 |
| crowd/c1/meshes/count | count | 1157 | 1281 | 1189 |
| crowd/c2/count | count | 376 | 418 | 418 |
| crowd/c2/joints/count | count | 6283 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1801 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 236 | 223 |
| crowd/joints_extracted/count | count | 4218 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4207 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.6 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 13.65 | 34.60 | 21.01 |
| gpu/timed/ms | ms | 8.75 | 12.43 | 9.14 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 3076 | 3148 | 3148 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 2 | 2 | 2 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 277.8 k | 282.2 k | 282.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 8 |
| phases/alpha_mask3d/bins | count | 177 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 902 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 177 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 902 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 38 | 42 | 39 |
| phases/opaque3d/bins | count | 1267 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3535 | 5132 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 26 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 38 | 42 | 39 |
| phases/opaque3d_prepass/bins | count | 1267 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3535 | 5132 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 26 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 79 | 344 | 63 |
| phases/shadow/bins | count | 4002 | 5411 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9806 | 12.4 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 162 | 317 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2694 | 3268 | 2347 |
| physics/items | count | 217 | 476 | 19 |
| physics/pending | count | 0 | 1821 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 50 | 0 |
| preprocess/bind_groups | count | 18 | 937 | 9 |
| preprocess/bind_groups_created | count | 18 | 937 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 334 | 492 | 328 |
| process/cpu_usage | % | 21.15 | 52.27 | 17.79 |
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
| started_load_count |  | 4762 | 4858 | 4858 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 436 | 528 | 396 |
| statics/materials_bound | count | 436 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.11 | 0.05 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 337 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.36 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1129 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.57 | 100 | 99.98 |
| system/mem_usage | % | 63.95 | 73.78 | 72.41 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2799 | 2938 | 2938 |
| textures/images | count | 2548 | 2649 | 2649 |
| views/all/batch_sets | count | 168 | 436 | 157 |
| views/all/bins | count | 6685 | 8353 | 5692 |
| views/all/count | count | 28 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.0 k | 25.2 k | 13.3 k |
| views/all/pipelines | count | 227 | 383 | 113 |
| views/all/pipelines_distinct | count | 99 | 109 | 91 |
| views/all/work_items | count | 17.7 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 71 | 90 | 63 |
| views/cascade/bins | count | 2637 | 3141 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8290 | 10.0 k | 5663 |
| views/cascade/pipelines | count | 57 | 73 | 47 |
| views/cascade/work_items | count | 8290 | 10.0 k | 5663 |
| views/lamp_face/batch_sets | count | 8 | 268 | 0 |
| views/lamp_face/bins | count | 1341 | 2497 | 0 |
| views/lamp_face/count | count | 24 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1513 | 2833 | 0 |
| views/lamp_face/pipelines | count | 106 | 254 | 0 |
| views/lamp_face/work_items | count | 82 | 2651 | 0 |
| views/main/batch_sets | count | 92 | 100 | 94 |
| views/main/bins | count | 2832 | 3560 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8868 | 13.0 k | 7592 |
| views/main/pipelines | count | 64 | 70 | 66 |
| views/main/work_items | count | 9015 | 13.2 k | 7900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.9 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 60.86 | 702 | 240.9 k |
| anim/events/count | count | 0.13 | 3 | 522 |
| assets/Image/added | count | 0.17 | 43 | 654 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 1.09 | 152 | 4309 |
| assets/Mesh/removed | count | 1.23 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.13 | 35 | 533 |
| assets/StandardMaterial/modified | count | 5.47 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.10 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.78 | 2784 | 11.0 k |
| physics/inserts | count | 0.92 | 138 | 3628 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.59 | 145 | 2335 |
| physics/steps | count | 1.00 | 3 | 3940 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.06 | 21 | 219 |
| stream/deactivate/items | count | 0.06 | 21 | 223 |
| stream/despawn/items | count | 0.06 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.10 | 22 | 378 |
| stream/upload/bytes | bytes | 8551 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1875 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 38.7 ms; slowest 14, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2493.0 | main/busy/ms 2468, surface/advance_cpu_ms 67.49, surface/cell_upload_ms 0.81, stream/drive/ms 0.33, stream/tick/ms 0.09, weather/map/ms 0.08, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, script/update/ms 0.03, weather/eval/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.19 | 696.3 | main/busy/ms 2504, render/thread/ms 663, render/busy/ms 522, render/acquire/ms 138, surface/advance_cpu_ms 67.49, render/graph/ms 5.05, render/present/ms 2.30, stream/drive/ms 1.23, surface/cell_upload_ms 0.81, terrain/update/ms 0.73, stream/tick/ms 0.63, terrain/select/ms 0.53 |
| warm-up | 3.40 | 214.4 | pipelines/wait_ms 837, render/thread/ms 675, render/busy/ms 533, pipelines/compile_ms 217, pipelines/front_ms 201, render/acquire/ms 138, main/busy/ms 54.91, terrain/update/ms 19.45, pipelines/driver_ms 15.53, render/graph/ms 9.04, render/present/ms 3.38, stream/drive/ms 1.54 |
| warm-up | 3.65 | 92.7 | pipelines/compile_ms 215, pipelines/front_ms 202, main/busy/ms 63.64, render/thread/ms 49.18, render/busy/ms 47.68, pipelines/driver_ms 12.91, render/graph/ms 2.99, stream/drive/ms 1.58, stream/tick/ms 1.37, render/present/ms 1.28, terrain/update/ms 0.67, terrain/select/ms 0.52 |
| warm-up | 3.83 | 76.8 | pipelines/wait_ms 2214, pipelines/compile_ms 182, pipelines/front_ms 172, main/busy/ms 50.72, render/thread/ms 41.06, render/busy/ms 39.63, stream/drive/ms 33.37, pipelines/driver_ms 9.54, render/graph/ms 4.59, stream/tick/ms 1.26, render/present/ms 1.20, terrain/update/ms 1.05 |
| warm-up | 3.96 | 52.1 | render/thread/ms 64.06, render/busy/ms 62.63, main/busy/ms 40.17, stream/drive/ms 23.27, render/graph/ms 3.96, stream/tick/ms 1.32, render/present/ms 1.12, stream/spawn/ms 1.09, terrain/update/ms 0.90, terrain/select/ms 0.72, render/acquire/ms 0.30, bigspace/total/ms 0.25 |
| warm-up | 4.09 | 116.6 | pipelines/wait_ms 7741, pipelines/compile_ms 249, pipelines/front_ms 231, render/thread/ms 66.83, main/busy/ms 51.80, render/busy/ms 40.75, stream/drive/ms 30.42, render/present/ms 25.83, pipelines/driver_ms 17.23, render/graph/ms 2.65, stream/spawn/ms 2.04, stream/tick/ms 1.01 |
| warm-up | 4.30 | 47.8 | main/busy/ms 76.15, stream/drive/ms 51.87, render/thread/ms 48.75, render/busy/ms 46.63, render/graph/ms 6.01, stream/spawn/ms 2.45, stream/tick/ms 2.23, render/present/ms 1.79, terrain/update/ms 0.99, terrain/select/ms 0.81, stream/upload/ms 0.80, bigspace/total/ms 0.47 |
| warm-up | 4.40 | 99.6 | pipelines/wait_ms 777, pipelines/compile_ms 197, pipelines/front_ms 181, render/thread/ms 80.75, render/busy/ms 79.36, main/busy/ms 72.99, stream/drive/ms 49.20, pipelines/driver_ms 16.61, render/graph/ms 4.09, stream/spawn/ms 1.77, render/present/ms 1.19, terrain/update/ms 1.02 |
| warm-up | 4.49 | 65.7 | pipelines/wait_ms 2689, main/busy/ms 67.88, pipelines/compile_ms 61.24, pipelines/front_ms 51.93, stream/drive/ms 48.67, render/thread/ms 44.07, render/busy/ms 42.80, pipelines/driver_ms 9.31, render/graph/ms 4.30, stream/spawn/ms 2.03, stream/tick/ms 1.19, render/present/ms 1.08 |
| warm-up | 4.55 | 42.7 | main/busy/ms 45.69, render/thread/ms 33.89, stream/drive/ms 33.75, render/busy/ms 32.43, render/graph/ms 5.81, render/present/ms 1.22, stream/spawn/ms 1.10, stream/activate/ms 1.06, stream/tick/ms 0.84, terrain/update/ms 0.68, terrain/select/ms 0.56, physics/insert/ms 0.52 |
| warm-up | 5.41 | 54.5 | render/thread/ms 60.38, render/busy/ms 57.66, main/busy/ms 46.28, render/graph/ms 9.76, stream/drive/ms 3.88, render/present/ms 2.27, stream/spawn/ms 2.25, stream/tick/ms 0.85, bigspace/total/ms 0.66, physics/step/ms 0.65, terrain/update/ms 0.58, physics/insert/ms 0.55 |
| warm-up | 5.48 | 46.2 | render/thread/ms 50.06, render/busy/ms 48.86, main/busy/ms 23.51, render/graph/ms 7.14, stream/drive/ms 4.61, stream/spawn/ms 2.41, stream/tick/ms 1.15, render/present/ms 1.00, terrain/update/ms 0.68, bigspace/total/ms 0.67, terrain/select/ms 0.59, bigspace/high_precision/ms 0.32 |
| measure | 59.11 | 48.5 | render/thread/ms 56.37, render/busy/ms 54.81, main/busy/ms 27.33, render/graph/ms 22.40, render/present/ms 1.31, terrain/update/ms 1.20, bigspace/total/ms 1.14, terrain/select/ms 1.09, controller/feed/ms 0.66, stream/drive/ms 0.60, stream/tick/ms 0.51, far/select/ms 0.47 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab_off\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 10% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **64%** (min 37%) · 2580 MHz (min 2580) · memory 8001 MHz · 64.9 W · 73 °C (max 76) · VRAM 5303 MiB · throttled 100% of samples
