# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4858 frames over 65.7 s · **median 13.12 ms (76 fps)** · mean 13.51 · p95 16.25 · p99 18.45 (1% low 54 fps) · worst 36.12 ms · 32 spikes over 1.5× median · 23.8 stutters/min (work 20.1/min)
- warm-up: 498 frames over 10.0 s, worst 2514.5 ms
- hitches (> 32.8 ms): **3** measured, 17 in warm-up
- timed GPU passes: 7.91 ms of a 13.51 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.12 ms, limit 11 ms
- **FAIL** frame_ms: 26 stutters (> 1.5x moving median of 60) in 1.09 min = 23.76/min, limit 1/min
- **FAIL** worst frame 36.12 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.179 | 2.181 | 2.518 | 1.00 | 0.179 |
| main_opaque_pass_3d | 1.741 | 1.743 | 2.376 | 1.00 | 0.071 |
| early prepass | 0.540 | 0.540 | 0.812 | 1.00 | 0.108 |
| bloom | 0.337 | 0.337 | 0.490 | 1.00 | 0.066 |
| atmosphere_luts | 0.331 | 0.331 | 0.597 | 1.00 | 0.003 |
| water_velocity | 0.298 | 0.298 | 0.506 | 1.00 | 0.002 |
| uniform_allocation | 0.262 | 0.263 | 0.268 | 1.00 | 0.050 |
| water_sspr | 0.228 | 0.229 | 0.228 | 1.00 | 0.002 |
| ssao | 0.204 | 0.204 | 0.209 | 1.00 | 0.008 |
| bin_unpacking | 0.201 | 0.201 | 0.213 | 1.00 | 0.032 |
| auxiliary_views | 0.195 | 0.196 | 0.744 | 1.00 | 0.173 |
| atmos_froxel_inject | 0.157 | 0.157 | 0.153 | 1.00 | 0.002 |
| water_fft | 0.140 | 0.140 | 0.138 | 1.00 | 0.003 |
| clouds_trace | 0.140 | 0.140 | 0.223 | 1.00 | 0.001 |
| upscaling | 0.125 | 0.125 | 0.130 | 1.00 | 0.004 |
| clouds_composite | 0.111 | 0.111 | 0.111 | 1.00 | 0.002 |
| tonemapping | 0.090 | 0.090 | 0.091 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.089 | 0.089 | 0.169 | 1.00 | 0.008 |
| clustering | 0.067 | 0.067 | 0.069 | 1.00 | 0.020 |
| render_sky | 0.055 | 0.055 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.055 | 0.437 | 0.753 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| surface_trail | 0.049 | 0.058 | 0.057 | 0.84 | 0.002 |
| early_mesh_preprocessing | 0.043 | 0.043 | 0.048 | 1.00 | 0.006 |
| water_snapshot | 0.036 | 0.036 | 0.044 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.030 | 0.243 | 0.558 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.031 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.032 | 1.00 | 0.111 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 12.338 | 0.000 | 10454.50 | 59939.4 |
| render/thread/ms | 12.293 | 14.930 | 32.37 | 59720.3 |
| render/busy/ms | 11.707 | 14.322 | 31.51 | 56870.9 |
| main/busy/ms | 9.353 | 12.051 | 29.19 | 45437.9 |
| render/graph/ms | 7.213 | 8.973 | 17.58 | 35041.2 |
| render/present/ms | 0.495 | 0.579 | 8.71 | 2403.3 |
| bigspace/total/ms | 0.466 | 0.747 | 4.42 | 2263.2 |
| pipelines/compile_ms | 0.400 | 0.000 | 172.79 | 1941.9 |
| pipelines/front_ms | 0.386 | 0.000 | 167.12 | 1876.4 |
| terrain/update/ms | 0.295 | 0.373 | 2.07 | 1432.8 |
| controller/feed/ms | 0.280 | 0.371 | 0.80 | 1360.9 |
| terrain/select/ms | 0.262 | 0.330 | 0.69 | 1271.6 |
| stream/drive/ms | 0.256 | 0.360 | 21.09 | 1243.7 |
| far/select/ms | 0.199 | 0.279 | 0.68 | 967.1 |
| stream/tick/ms | 0.195 | 0.270 | 0.64 | 945.2 |
| bigspace/low_precision/ms | 0.181 | 0.311 | 4.19 | 880.1 |
| anim/evaluate_cpu/ms | 0.164 | 0.209 | 0.55 | 797.8 |
| anim/evaluate/ms | 0.152 | 0.282 | 3.71 | 738.9 |
| controller/move/ms | 0.149 | 0.288 | 0.66 | 725.3 |
| bigspace/high_precision/ms | 0.116 | 0.223 | 3.77 | 563.8 |
| render/acquire/ms | 0.092 | 0.119 | 0.47 | 446.1 |
| physics/step/ms | 0.059 | 0.090 | 0.44 | 288.0 |
| physics/insert/ms | 0.042 | 0.409 | 1.17 | 203.0 |
| weather/map/ms | 0.024 | 0.032 | 0.23 | 118.9 |
| pipelines/driver_ms | 0.013 | 0.000 | 5.68 | 65.5 |
| anim/apply/ms | 0.012 | 0.023 | 0.14 | 57.2 |
| controller/avoid/ms | 0.011 | 0.015 | 0.29 | 54.2 |
| weather/eval/ms | 0.010 | 0.014 | 0.30 | 50.6 |
| script/update/ms | 0.007 | 0.009 | 0.11 | 33.9 |
| stream/despawn/ms | 0.006 | 0.000 | 0.95 | 29.5 |
| script/input/ms | 0.005 | 0.007 | 0.15 | 26.2 |
| stream/spawn/ms | 0.005 | 0.000 | 1.10 | 25.9 |
| script/ui/ms | 0.005 | 0.006 | 0.11 | 22.2 |
| controller/write/ms | 0.004 | 0.005 | 0.21 | 17.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.66 | 14.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.45 | 5.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.02 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 21 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 90 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 27 | 49 | 13 |
| crowd/c0/joints/count | count | 1301 | 2262 | 595 |
| crowd/c0/meshes/count | count | 353 | 646 | 158 |
| crowd/c1/count | count | 145 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5949 | 5569 |
| crowd/c1/meshes/count | count | 1164 | 1286 | 1175 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 10.0 k | 6179 |
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
| frame/work/ms | ms | 11.39 | 31.51 | 10.01 |
| gpu/timed/ms | ms | 8.58 | 12.63 | 8.86 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 806 | 1416 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 806 | 1416 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1288 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3396 | 5119 | 3182 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1288 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3396 | 5119 | 3182 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 59 | 315 | 47 |
| phases/shadow/bins | count | 3774 | 5411 | 2220 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9557 | 12.5 k | 5675 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 143 | 326 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 189 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 15 | 0 |
| preprocess/bind_groups | count | 12 | 823 | 10 |
| preprocess/bind_groups_created | count | 12 | 823 | 10 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 258 | 387 | 247 |
| process/cpu_usage | % | 23.49 | 41.83 | 21.84 |
| process/mem_usage | GiB | 5.14 | 5.32 | 5.32 |
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
| started_load_count |  | 4766 | 4860 | 4860 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.00 | 44.63 | 28.65 |
| system/mem_usage | % | 58.90 | 60.43 | 60.36 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 128 | 379 | 117 |
| views/all/bins | count | 6455 | 8376 | 5700 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.4 k | 13.3 k |
| views/all/pipelines | count | 206 | 392 | 113 |
| views/all/pipelines_distinct | count | 97 | 108 | 91 |
| views/all/work_items | count | 17.3 k | 24.1 k | 13.6 k |
| views/cascade/batch_sets | count | 57 | 71 | 47 |
| views/cascade/bins | count | 2606 | 3146 | 2220 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8147 | 9952 | 5675 |
| views/cascade/pipelines | count | 57 | 71 | 47 |
| views/cascade/work_items | count | 8147 | 9952 | 5675 |
| views/lamp_face/batch_sets | count | 0 | 255 | 0 |
| views/lamp_face/bins | count | 1066 | 2561 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1207 | 2947 | 0 |
| views/lamp_face/pipelines | count | 85 | 262 | 0 |
| views/lamp_face/work_items | count | 0 | 2732 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2870 | 3586 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8399 | 13.0 k | 7600 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8569 | 13.2 k | 7908 |
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
| anim/bones_written/count | count | 59.11 | 752 | 287.2 k |
| anim/events/count | count | 0.12 | 3 | 561 |
| assets/Image/added | count | 0.13 | 43 | 655 |
| assets/Image/modified | count | 0.00 | 9 | 24 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.87 | 168 | 4240 |
| assets/Mesh/removed | count | 1.00 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 534 |
| assets/StandardMaterial/modified | count | 4.46 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.16 | 2920 | 10.5 k |
| physics/inserts | count | 0.50 | 111 | 2435 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.48 | 145 | 2335 |
| physics/steps | count | 0.81 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 22 | 378 |
| stream/upload/bytes | bytes | 6968 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 58 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1528 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.8 ms; slowest 20, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2514.5 | main/busy/ms 2489, surface/advance_cpu_ms 65.76, surface/cell_upload_ms 0.71, stream/drive/ms 0.41, weather/map/ms 0.14, bigspace/total/ms 0.11, stream/tick/ms 0.09, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.03, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.22 | 705.0 | main/busy/ms 2532, render/thread/ms 671, render/busy/ms 548, render/acquire/ms 121, surface/advance_cpu_ms 65.76, render/graph/ms 5.33, render/present/ms 2.45, stream/drive/ms 1.40, surface/cell_upload_ms 0.71, terrain/update/ms 0.70, stream/tick/ms 0.69, terrain/select/ms 0.50 |
| warm-up | 3.43 | 212.1 | pipelines/wait_ms 2257, render/thread/ms 683, render/busy/ms 559, pipelines/compile_ms 227, pipelines/front_ms 212, render/acquire/ms 121, main/busy/ms 59.85, terrain/update/ms 22.33, pipelines/driver_ms 15.56, render/graph/ms 9.16, render/present/ms 3.42, stream/drive/ms 1.54 |
| warm-up | 3.73 | 87.0 | pipelines/compile_ms 265, pipelines/front_ms 251, pipelines/wait_ms 222, main/busy/ms 61.80, render/thread/ms 50.53, render/busy/ms 49.01, pipelines/driver_ms 13.67, terrain/update/ms 5.56, render/graph/ms 3.31, render/present/ms 1.33, stream/drive/ms 1.30, stream/tick/ms 1.00 |
| warm-up | 3.83 | 52.1 | pipelines/wait_ms 783, main/busy/ms 38.21, render/thread/ms 36.12, render/busy/ms 34.72, stream/drive/ms 26.02, pipelines/compile_ms 5.07, render/graph/ms 4.25, pipelines/front_ms 4.24, render/present/ms 1.16, stream/tick/ms 0.87, pipelines/driver_ms 0.84, terrain/update/ms 0.80 |
| warm-up | 3.90 | 49.7 | pipelines/wait_ms 1795, pipelines/compile_ms 172, pipelines/front_ms 160, render/thread/ms 51.16, render/busy/ms 48.97, main/busy/ms 44.49, stream/drive/ms 29.66, pipelines/driver_ms 11.65, render/graph/ms 8.41, render/present/ms 1.91, stream/tick/ms 1.54, stream/spawn/ms 0.82 |
| warm-up | 3.97 | 50.9 | pipelines/wait_ms 1588, pipelines/compile_ms 138, pipelines/front_ms 129, render/thread/ms 49.27, render/busy/ms 46.47, main/busy/ms 30.65, pipelines/driver_ms 8.24, render/graph/ms 5.23, stream/drive/ms 4.83, render/present/ms 1.94, stream/tick/ms 1.65, stream/spawn/ms 1.64 |
| warm-up | 4.04 | 40.0 | pipelines/wait_ms 3554, pipelines/compile_ms 58.04, pipelines/front_ms 53.24, render/thread/ms 46.69, render/busy/ms 45.37, main/busy/ms 17.98, pipelines/driver_ms 4.80, render/graph/ms 3.28, stream/drive/ms 2.96, render/present/ms 1.14, stream/spawn/ms 1.11, terrain/update/ms 1.10 |
| warm-up | 4.10 | 43.6 | main/busy/ms 40.33, stream/drive/ms 26.47, render/thread/ms 25.50, render/busy/ms 23.90, render/graph/ms 3.67, render/present/ms 1.40, stream/tick/ms 0.79, terrain/update/ms 0.77, terrain/select/ms 0.62, stream/spawn/ms 0.35, bigspace/total/ms 0.29, render/acquire/ms 0.21 |
| warm-up | 4.22 | 51.4 | pipelines/wait_ms 3100, pipelines/compile_ms 144, pipelines/front_ms 137, render/thread/ms 66.12, render/busy/ms 63.19, main/busy/ms 22.45, render/graph/ms 19.83, pipelines/driver_ms 7.62, stream/drive/ms 4.19, render/present/ms 2.66, stream/tick/ms 2.46, stream/spawn/ms 0.80 |
| warm-up | 4.28 | 57.9 | render/thread/ms 67.39, main/busy/ms 65.03, render/busy/ms 64.96, stream/drive/ms 49.08, render/graph/ms 14.14, stream/tick/ms 2.24, render/present/ms 2.18, terrain/update/ms 0.87, terrain/select/ms 0.71, stream/spawn/ms 0.48, bigspace/total/ms 0.29, render/acquire/ms 0.25 |
| warm-up | 4.35 | 73.3 | pipelines/wait_ms 5409, pipelines/compile_ms 202, pipelines/front_ms 190, main/busy/ms 65.78, render/thread/ms 56.81, render/busy/ms 55.49, stream/drive/ms 49.32, pipelines/driver_ms 11.37, render/graph/ms 3.01, stream/tick/ms 1.67, render/present/ms 1.07, stream/spawn/ms 1.05 |
| warm-up | 4.43 | 66.0 | pipelines/wait_ms 11.8 k, main/busy/ms 70.29, pipelines/compile_ms 63.54, pipelines/front_ms 56.85, stream/drive/ms 49.55, render/thread/ms 37.07, render/busy/ms 35.56, pipelines/driver_ms 6.69, render/graph/ms 5.56, terrain/update/ms 4.36, stream/spawn/ms 2.30, render/present/ms 1.28 |
| warm-up | 4.50 | 42.6 | main/busy/ms 57.25, render/thread/ms 41.19, render/busy/ms 39.87, stream/drive/ms 36.67, render/graph/ms 4.97, stream/spawn/ms 2.13, stream/activate/ms 1.07, render/present/ms 1.05, stream/tick/ms 1.00, terrain/update/ms 0.74, terrain/select/ms 0.61, bigspace/total/ms 0.38 |
| warm-up | 5.26 | 34.7 | pipelines/wait_ms 6646, pipelines/compile_ms 44.07, pipelines/front_ms 40.54, main/busy/ms 39.48, render/thread/ms 18.46, stream/drive/ms 17.95, render/busy/ms 17.22, render/graph/ms 5.16, pipelines/driver_ms 3.53, terrain/update/ms 2.62, render/present/ms 1.05, stream/spawn/ms 0.92 |
| warm-up | 5.38 | 50.8 | pipelines/wait_ms 4499, render/thread/ms 54.13, render/busy/ms 52.43, main/busy/ms 48.80, pipelines/compile_ms 40.23, pipelines/front_ms 38.12, render/graph/ms 12.88, stream/drive/ms 3.41, pipelines/driver_ms 2.12, stream/spawn/ms 2.11, render/present/ms 1.26, stream/tick/ms 0.74 |
| warm-up | 9.68 | 34.5 | pipelines/wait_ms 10.9 k, main/busy/ms 44.61, pipelines/compile_ms 38.17, render/thread/ms 37.57, render/busy/ms 36.44, pipelines/front_ms 35.73, render/graph/ms 15.24, bigspace/total/ms 3.54, bigspace/high_precision/ms 2.92, pipelines/driver_ms 2.44, render/present/ms 0.91, physics/insert/ms 0.77 |
| measure | 0.80 | 36.1 | main/busy/ms 42.97, render/thread/ms 36.15, render/busy/ms 34.83, stream/drive/ms 15.32, render/graph/ms 14.65, bigspace/total/ms 1.05, render/present/ms 1.03, controller/feed/ms 0.72, terrain/update/ms 0.58, anim/evaluate_cpu/ms 0.52, terrain/select/ms 0.52, far/select/ms 0.47 |
| measure | 2.73 | 33.5 | render/thread/ms 44.49, render/busy/ms 43.04, render/graph/ms 22.39, main/busy/ms 17.83, render/present/ms 1.20, bigspace/total/ms 0.83, physics/insert/ms 0.68, terrain/update/ms 0.54, terrain/select/ms 0.49, controller/feed/ms 0.48, stream/drive/ms 0.40, stream/tick/ms 0.35 |
| measure | 59.06 | 34.1 | render/thread/ms 43.72, render/busy/ms 42.57, main/busy/ms 25.58, render/graph/ms 13.04, bigspace/total/ms 1.45, stream/drive/ms 0.92, render/present/ms 0.92, terrain/update/ms 0.91, controller/feed/ms 0.85, stream/tick/ms 0.83, terrain/select/ms 0.80, far/select/ms 0.61 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-on\tex-slab-on.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab2_on\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 20%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **75%** (min 51%) · 2580 MHz (min 2580) · memory 8001 MHz · 73.8 W · 76 °C (max 80) · VRAM 5280 MiB · throttled 100% of samples
