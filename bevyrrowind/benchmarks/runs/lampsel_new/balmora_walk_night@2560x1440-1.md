# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5203 frames over 65.6 s · **median 11.97 ms (84 fps)** · mean 12.62 · p95 16.53 · p99 18.59 (1% low 54 fps) · worst 36.09 ms · 85 spikes over 1.5× median · 26.5 stutters/min (work 21.0/min)
- warm-up: 329 frames over 8.6 s, worst 2553.2 ms
- hitches (> 29.9 ms): **2** measured, 21 in warm-up
- timed GPU passes: 7.16 ms of a 12.62 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.97 ms, limit 11 ms
- **FAIL** frame_ms: 29 stutters (> 1.5x moving median of 60) in 1.09 min = 26.51/min, limit 1/min
- **FAIL** worst frame 36.09 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.061 | 2.068 | 2.103 | 1.00 | 0.178 |
| main_opaque_pass_3d | 1.348 | 1.352 | 1.711 | 1.00 | 0.063 |
| early prepass | 0.533 | 0.535 | 0.756 | 1.00 | 0.103 |
| auxiliary_views | 0.341 | 0.342 | 1.181 | 1.00 | 0.289 |
| bloom | 0.321 | 0.322 | 0.328 | 1.00 | 0.065 |
| atmosphere_luts | 0.315 | 0.316 | 0.380 | 1.00 | 0.003 |
| water_velocity | 0.267 | 0.268 | 0.357 | 1.00 | 0.001 |
| water_sspr | 0.208 | 0.209 | 0.210 | 1.00 | 0.002 |
| uniform_allocation | 0.207 | 0.208 | 0.221 | 1.00 | 0.035 |
| ssao | 0.191 | 0.192 | 0.195 | 1.00 | 0.008 |
| bin_unpacking | 0.169 | 0.169 | 0.181 | 1.00 | 0.025 |
| water_fft | 0.131 | 0.131 | 0.137 | 1.00 | 0.003 |
| atmos_froxel_inject | 0.129 | 0.130 | 0.131 | 1.00 | 0.002 |
| upscaling | 0.124 | 0.125 | 0.133 | 1.00 | 0.004 |
| clouds_composite | 0.107 | 0.107 | 0.110 | 1.00 | 0.002 |
| tonemapping | 0.087 | 0.087 | 0.091 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.080 | 0.080 | 0.144 | 1.00 | 0.008 |
| clustering | 0.065 | 0.065 | 0.069 | 1.00 | 0.019 |
| clouds_trace | 0.054 | 0.054 | 0.073 | 1.00 | 0.001 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.419 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| surface_trail | 0.045 | 0.057 | 0.057 | 0.79 | 0.002 |
| early_mesh_preprocessing | 0.034 | 0.034 | 0.038 | 1.00 | 0.005 |
| water_snapshot | 0.033 | 0.033 | 0.036 | 1.00 | 0.002 |
| lightprobe_radiance_map | 0.027 | 0.224 | 0.233 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.029 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.019 | 0.019 | 0.033 | 1.00 | 0.111 |
| night_sky | 0.016 | 0.016 | 0.034 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.014 | 0.014 | 0.015 | 1.00 | 0.005 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 42.370 | 0.000 | 25266.68 | 220451.3 |
| render/thread/ms | 11.400 | 15.150 | 34.52 | 59316.1 |
| render/busy/ms | 10.827 | 14.503 | 33.76 | 56335.3 |
| main/busy/ms | 9.276 | 11.760 | 29.13 | 48261.1 |
| render/graph/ms | 6.534 | 9.247 | 18.18 | 33994.0 |
| pipelines/compile_ms | 0.689 | 0.000 | 179.17 | 3587.0 |
| pipelines/front_ms | 0.666 | 0.000 | 172.30 | 3462.6 |
| render/present/ms | 0.484 | 0.567 | 4.51 | 2517.7 |
| bigspace/total/ms | 0.469 | 0.790 | 3.23 | 2442.3 |
| terrain/update/ms | 0.300 | 0.378 | 1.98 | 1558.8 |
| controller/feed/ms | 0.273 | 0.352 | 0.61 | 1420.4 |
| terrain/select/ms | 0.267 | 0.337 | 0.67 | 1389.9 |
| stream/drive/ms | 0.237 | 0.324 | 20.09 | 1234.7 |
| far/select/ms | 0.191 | 0.254 | 0.51 | 996.0 |
| bigspace/low_precision/ms | 0.186 | 0.313 | 2.89 | 970.2 |
| stream/tick/ms | 0.181 | 0.245 | 0.76 | 940.7 |
| anim/evaluate_cpu/ms | 0.166 | 0.209 | 0.52 | 863.8 |
| anim/evaluate/ms | 0.135 | 0.259 | 2.63 | 703.7 |
| controller/move/ms | 0.117 | 0.197 | 0.39 | 610.6 |
| bigspace/high_precision/ms | 0.114 | 0.218 | 2.58 | 595.1 |
| render/acquire/ms | 0.089 | 0.114 | 0.31 | 463.1 |
| physics/step/ms | 0.056 | 0.090 | 0.51 | 290.0 |
| physics/insert/ms | 0.049 | 0.430 | 0.86 | 255.3 |
| pipelines/driver_ms | 0.024 | 0.000 | 7.04 | 124.3 |
| weather/map/ms | 0.015 | 0.020 | 0.13 | 80.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.33 | 55.3 |
| anim/apply/ms | 0.011 | 0.021 | 0.23 | 55.2 |
| weather/eval/ms | 0.010 | 0.012 | 0.34 | 49.8 |
| script/update/ms | 0.007 | 0.009 | 0.32 | 35.9 |
| stream/despawn/ms | 0.005 | 0.000 | 1.08 | 28.5 |
| script/input/ms | 0.005 | 0.007 | 0.07 | 26.3 |
| script/ui/ms | 0.005 | 0.006 | 0.08 | 25.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.05 | 24.3 |
| controller/write/ms | 0.003 | 0.004 | 0.06 | 18.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 14.1 |
| stream/activate/ms | 0.001 | 0.000 | 0.55 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 12 | 0 |
| anim/a2/count | count | 1 | 7 | 0 |
| anim/a3/count | count | 569 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 36 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 42 | 14 |
| crowd/c0/joints/count | count | 1247 | 2218 | 641 |
| crowd/c0/meshes/count | count | 349 | 634 | 174 |
| crowd/c1/count | count | 147 | 155 | 147 |
| crowd/c1/joints/count | count | 5541 | 5937 | 5555 |
| crowd/c1/meshes/count | count | 1171 | 1281 | 1167 |
| crowd/c2/count | count | 399 | 417 | 417 |
| crowd/c2/joints/count | count | 6406 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 160 | 215 | 202 |
| crowd/joints_extracted/count | count | 3677 | 7667 | 5619 |
| crowd/joints_uploaded/count | count | 3592 | 6129 | 5619 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.32 | 33.76 | 8.84 |
| gpu/timed/ms | ms | 7.84 | 11.31 | 8.11 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2523 | 2580 | 2580 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 278.4 k | 278.4 k | 278.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 789 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 199 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 789 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d/bins | count | 1292 | 1399 | 1349 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3334 | 5120 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d_prepass/bins | count | 1292 | 1399 | 1349 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3334 | 5120 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 35 | 250 | 29 |
| phases/shadow/bins | count | 3580 | 4385 | 1136 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4910 | 7389 | 2441 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 243 | 297 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1052 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 218 | 223 | 223 |
| pipelines/waiting | count | 0 | 50 | 0 |
| preprocess/bind_groups | count | 10 | 696 | 6 |
| preprocess/bind_groups_created | count | 10 | 696 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 220 | 404 | 218 |
| process/cpu_usage | % | 25 | 42.18 | 28.99 |
| process/mem_usage | GiB | 5.09 | 5.26 | 5.26 |
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
| started_load_count |  | 4766 | 4856 | 4856 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 429 | 528 | 396 |
| statics/materials_bound | count | 429 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.11 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 327 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.37 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.84 | 47.72 | 28.97 |
| system/mem_usage | % | 63.63 | 64.10 | 64.00 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2821 | 2936 | 2936 |
| textures/images | count | 2589 | 2647 | 2647 |
| views/all/batch_sets | count | 106 | 316 | 101 |
| views/all/bins | count | 6017 | 7246 | 4628 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 13.2 k | 19.5 k | 10.0 k |
| views/all/pipelines | count | 307 | 367 | 97 |
| views/all/pipelines_distinct | count | 101 | 110 | 92 |
| views/all/work_items | count | 11.0 k | 17.2 k | 10.3 k |
| views/cascade/batch_sets | count | 32 | 41 | 29 |
| views/cascade/bins | count | 1182 | 1548 | 1136 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2735 | 4298 | 2441 |
| views/cascade/pipelines | count | 32 | 41 | 29 |
| views/cascade/work_items | count | 2735 | 4298 | 2441 |
| views/lamp_face/batch_sets | count | 0 | 215 | 0 |
| views/lamp_face/bins | count | 2524 | 2990 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2782 | 3303 | 0 |
| views/lamp_face/pipelines | count | 212 | 259 | 0 |
| views/lamp_face/work_items | count | 0 | 2780 | 0 |
| views/main/batch_sets | count | 70 | 76 | 72 |
| views/main/bins | count | 2880 | 3574 | 3492 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8240 | 13.0 k | 7598 |
| views/main/pipelines | count | 66 | 72 | 68 |
| views/main/work_items | count | 8405 | 13.2 k | 7906 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.5 k | 37.8 k | 30.7 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 47.12 | 391 | 245.1 k |
| anim/events/count | count | 0.11 | 2 | 551 |
| assets/Image/added | count | 0.12 | 43 | 601 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.79 | 152 | 4116 |
| assets/Mesh/removed | count | 0.94 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 468 |
| assets/StandardMaterial/modified | count | 4.16 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 48 | 394 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.49 | 1562 | 7778 |
| physics/inserts | count | 0.58 | 92 | 3008 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 161 | 2335 |
| physics/steps | count | 0.76 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 15 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6506 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1426 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 29.9 ms; slowest 23, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2553.2 | main/busy/ms 2528, surface/advance_cpu_ms 0.73, surface/cell_upload_ms 0.46, stream/drive/ms 0.36, stream/tick/ms 0.09, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06, weather/map/ms 0.04, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.02, far/select/ms 0.01 |
| warm-up | 3.43 | 880.1 | main/busy/ms 2561, render/thread/ms 846, render/busy/ms 685, render/acquire/ms 158, render/graph/ms 6.16, render/present/ms 2.10, stream/drive/ms 1.08, surface/advance_cpu_ms 0.73, stream/tick/ms 0.55, terrain/update/ms 0.53, surface/cell_upload_ms 0.46, terrain/select/ms 0.40 |
| warm-up | 3.50 | 69.4 | render/thread/ms 859, render/busy/ms 697, pipelines/wait_ms 439, render/acquire/ms 158, pipelines/compile_ms 76.96, pipelines/front_ms 69.68, main/busy/ms 50.32, terrain/update/ms 17.94, render/graph/ms 9.51, pipelines/driver_ms 7.28, render/present/ms 2.90, stream/drive/ms 1.33 |
| warm-up | 3.78 | 125.4 | pipelines/wait_ms 273, pipelines/compile_ms 222, pipelines/front_ms 194, render/thread/ms 110, render/busy/ms 95.20, main/busy/ms 73.53, pipelines/driver_ms 28.42, render/acquire/ms 12.98, render/graph/ms 4.18, stream/drive/ms 3.09, stream/tick/ms 2.49, render/present/ms 1.75 |
| warm-up | 3.93 | 53.3 | pipelines/wait_ms 234, pipelines/compile_ms 200, pipelines/front_ms 194, render/thread/ms 46.70, render/busy/ms 45.40, main/busy/ms 43.87, stream/drive/ms 30.46, pipelines/driver_ms 6.07, render/graph/ms 3.42, stream/tick/ms 1.21, render/present/ms 0.95, terrain/update/ms 0.85 |
| warm-up | 4.00 | 61.2 | main/busy/ms 43.47, render/thread/ms 35.89, render/busy/ms 34.25, stream/drive/ms 25.47, terrain/update/ms 5.09, render/graph/ms 4.60, render/present/ms 1.49, stream/spawn/ms 1.27, stream/tick/ms 0.80, terrain/select/ms 0.59, bigspace/total/ms 0.30, render/acquire/ms 0.15 |
| warm-up | 4.11 | 91.3 | render/thread/ms 71.03, render/busy/ms 67.89, main/busy/ms 36.14, stream/drive/ms 14.79, render/graph/ms 6.85, stream/spawn/ms 2.99, render/present/ms 2.78, stream/tick/ms 2.00, terrain/update/ms 1.32, terrain/select/ms 1.05, render/acquire/ms 0.35, bigspace/total/ms 0.31 |
| warm-up | 4.15 | 47.8 | pipelines/wait_ms 1011, pipelines/compile_ms 221, pipelines/front_ms 201, render/thread/ms 92.70, render/busy/ms 89.43, main/busy/ms 31.67, pipelines/driver_ms 19.94, render/graph/ms 7.60, stream/drive/ms 7.45, render/present/ms 2.98, stream/spawn/ms 2.50, stream/tick/ms 2.38 |
| warm-up | 4.27 | 117.8 | pipelines/wait_ms 13.8 k, pipelines/compile_ms 412, pipelines/front_ms 330, render/thread/ms 116, render/busy/ms 114, pipelines/driver_ms 82.00, main/busy/ms 29.60, stream/drive/ms 5.09, render/graph/ms 3.80, stream/tick/ms 1.80, render/present/ms 1.51, terrain/update/ms 1.26 |
| warm-up | 4.31 | 37.0 | pipelines/wait_ms 15.6 k, pipelines/compile_ms 231, pipelines/front_ms 166, render/thread/ms 112, render/busy/ms 111, pipelines/driver_ms 64.65, main/busy/ms 56.59, stream/drive/ms 33.37, terrain/update/ms 3.43, render/graph/ms 2.63, stream/tick/ms 1.67, stream/spawn/ms 1.20 |
| warm-up | 4.35 | 41.4 | pipelines/wait_ms 4352, pipelines/compile_ms 78.36, pipelines/front_ms 73.37, main/busy/ms 56.95, render/thread/ms 42.63, render/busy/ms 41.33, stream/drive/ms 33.68, render/graph/ms 6.10, pipelines/driver_ms 4.99, terrain/update/ms 3.25, stream/tick/ms 1.74, stream/spawn/ms 1.62 |
| warm-up | 4.47 | 46.4 | main/busy/ms 46.83, render/thread/ms 37.56, render/busy/ms 35.80, stream/drive/ms 32.69, render/graph/ms 8.06, render/present/ms 1.52, stream/tick/ms 1.40, stream/spawn/ms 1.15, terrain/update/ms 0.76, terrain/select/ms 0.63, bigspace/total/ms 0.35, render/acquire/ms 0.23 |
| warm-up | 4.55 | 46.7 | pipelines/wait_ms 9143, pipelines/compile_ms 193, pipelines/front_ms 176, main/busy/ms 66.44, stream/drive/ms 53.00, render/thread/ms 39.09, render/busy/ms 37.81, pipelines/driver_ms 17.10, render/graph/ms 6.99, stream/spawn/ms 2.08, render/present/ms 1.08, terrain/update/ms 1.02 |
| warm-up | 4.61 | 41.3 | pipelines/wait_ms 3853, main/busy/ms 54.14, pipelines/compile_ms 44.63, pipelines/front_ms 42.41, stream/drive/ms 33.76, render/thread/ms 33.13, render/busy/ms 31.51, render/graph/ms 4.26, pipelines/driver_ms 2.23, stream/spawn/ms 2.05, render/present/ms 1.16, stream/tick/ms 0.94 |
| warm-up | 5.46 | 38.8 | render/thread/ms 47.67, render/busy/ms 46.42, main/busy/ms 27.55, stream/drive/ms 5.02, render/graph/ms 4.99, stream/tick/ms 1.89, stream/spawn/ms 1.69, render/present/ms 1.00, bigspace/total/ms 0.76, terrain/update/ms 0.68, terrain/select/ms 0.59, bigspace/high_precision/ms 0.42 |
| warm-up | 5.79 | 35.9 | main/busy/ms 41.63, render/thread/ms 31.92, render/busy/ms 30.61, stream/drive/ms 20.31, render/graph/ms 4.45, stream/spawn/ms 2.27, stream/tick/ms 1.19, render/present/ms 0.92, bigspace/total/ms 0.72, terrain/update/ms 0.66, terrain/select/ms 0.56, render/acquire/ms 0.38 |
| warm-up | 6.33 | 42.5 | main/busy/ms 53.61, render/thread/ms 31.70, render/busy/ms 30.09, render/graph/ms 9.34, render/present/ms 1.39, physics/insert/ms 1.25, stream/drive/ms 1.13, bigspace/total/ms 0.75, stream/tick/ms 0.60, terrain/update/ms 0.55, anim/evaluate_cpu/ms 0.52, terrain/select/ms 0.48 |
| warm-up | 6.36 | 34.4 | pipelines/wait_ms 4924, pipelines/compile_ms 136, pipelines/front_ms 131, render/thread/ms 40.43, render/busy/ms 38.78, main/busy/ms 37.77, render/graph/ms 5.78, pipelines/driver_ms 4.64, render/present/ms 1.42, physics/insert/ms 1.34, stream/drive/ms 1.18, bigspace/total/ms 0.64 |
| warm-up | 6.69 | 30.3 | main/busy/ms 43.85, render/thread/ms 36.25, render/busy/ms 34.72, render/graph/ms 11.62, physics/insert/ms 1.38, render/present/ms 1.26, stream/drive/ms 1.18, bigspace/total/ms 1.16, stream/tick/ms 0.65, terrain/update/ms 0.64, anim/evaluate_cpu/ms 0.59, terrain/select/ms 0.55 |
| warm-up | 6.73 | 39.9 | main/busy/ms 58.36, render/thread/ms 46.34, render/busy/ms 44.93, render/graph/ms 6.83, physics/insert/ms 1.52, anim/evaluate_cpu/ms 1.12, render/present/ms 1.07, stream/drive/ms 1.07, bigspace/total/ms 0.92, terrain/update/ms 0.63, stream/tick/ms 0.59, terrain/select/ms 0.55 |
| warm-up | 6.77 | 35.8 | main/busy/ms 44.66, render/thread/ms 44.17, render/busy/ms 42.87, render/graph/ms 6.85, physics/insert/ms 1.52, stream/drive/ms 1.13, bigspace/total/ms 1.13, render/present/ms 1.06, anim/evaluate_cpu/ms 0.92, stream/tick/ms 0.66, terrain/update/ms 0.56, terrain/select/ms 0.49 |
| measure | 2.73 | 35.6 | pipelines/wait_ms 25.3 k, render/thread/ms 48.51, render/busy/ms 47.21, pipelines/compile_ms 25.97, render/graph/ms 25.16, pipelines/front_ms 24.47, main/busy/ms 22.97, pipelines/driver_ms 1.51, render/present/ms 1.11, physics/insert/ms 0.94, bigspace/total/ms 0.81, terrain/update/ms 0.61 |
| measure | 18.18 | 36.1 | main/busy/ms 37.62, render/thread/ms 26.51, render/busy/ms 25.34, stream/drive/ms 20.33, render/graph/ms 11.43, render/present/ms 0.98, bigspace/total/ms 0.87, controller/feed/ms 0.61, terrain/update/ms 0.57, terrain/select/ms 0.51, far/select/ms 0.37, stream/tick/ms 0.35 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_new.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_new\balmora_walk_night@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **70%** (min 44%) · 2580 MHz (min 2580) · memory 8001 MHz · 70.7 W · 74 °C (max 80) · VRAM 5405 MiB · throttled 100% of samples
