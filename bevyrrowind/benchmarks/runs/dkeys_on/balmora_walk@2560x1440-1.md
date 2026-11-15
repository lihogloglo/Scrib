# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5035 frames over 65.6 s · **median 12.68 ms (79 fps)** · mean 13.04 · p95 16.00 · p99 19.28 (1% low 52 fps) · worst 48.23 ms · 56 spikes over 1.5× median · 30.2 stutters/min (work 29.2/min)
- warm-up: 424 frames over 9.5 s, worst 2534.2 ms
- hitches (> 31.7 ms): **5** measured, 23 in warm-up
- timed GPU passes: 7.42 ms of a 13.04 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.68 ms, limit 11 ms
- **FAIL** frame_ms: 33 stutters (> 1.5x moving median of 60) in 1.09 min = 30.16/min, limit 1/min
- **FAIL** worst frame 48.23 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.083 | 2.088 | 2.292 | 1.00 | 0.177 |
| main_opaque_pass_3d | 1.632 | 1.636 | 2.073 | 1.00 | 0.073 |
| early prepass | 0.521 | 0.522 | 0.750 | 1.00 | 0.100 |
| bloom | 0.321 | 0.322 | 0.327 | 1.00 | 0.065 |
| atmosphere_luts | 0.315 | 0.316 | 0.377 | 1.00 | 0.003 |
| water_velocity | 0.283 | 0.283 | 0.354 | 1.00 | 0.002 |
| uniform_allocation | 0.230 | 0.231 | 0.242 | 1.00 | 0.044 |
| water_sspr | 0.211 | 0.211 | 0.212 | 1.00 | 0.002 |
| ssao | 0.193 | 0.194 | 0.195 | 1.00 | 0.008 |
| bin_unpacking | 0.184 | 0.185 | 0.196 | 1.00 | 0.029 |
| atmos_froxel_inject | 0.149 | 0.150 | 0.150 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.141 | 0.232 | 1.00 | 0.001 |
| water_fft | 0.133 | 0.134 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.120 | 0.120 | 0.131 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.109 | 0.110 | 1.00 | 0.002 |
| auxiliary_views | 0.093 | 0.093 | 0.436 | 1.00 | 0.093 |
| main_transmissive_pass_3d | 0.089 | 0.089 | 0.168 | 1.00 | 0.009 |
| tonemapping | 0.087 | 0.087 | 0.090 | 1.00 | 0.004 |
| clustering | 0.065 | 0.066 | 0.069 | 1.00 | 0.021 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.424 | 0.422 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.046 | 0.057 | 0.057 | 0.81 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.032 | 0.032 | 0.035 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.225 | 0.232 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.028 | 1.00 | 0.001 |
| water_mips | 0.022 | 0.022 | 0.023 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.021 | 0.023 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.032 | 1.00 | 0.109 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 39.708 | 0.000 | 14532.93 | 199928.1 |
| render/thread/ms | 11.793 | 14.594 | 45.74 | 59379.1 |
| render/busy/ms | 11.217 | 13.909 | 40.50 | 56476.5 |
| main/busy/ms | 9.531 | 12.335 | 50.47 | 47990.8 |
| render/graph/ms | 6.826 | 8.573 | 16.09 | 34369.3 |
| pipelines/compile_ms | 0.623 | 0.000 | 198.24 | 3135.5 |
| pipelines/front_ms | 0.601 | 0.000 | 190.45 | 3025.7 |
| bigspace/total/ms | 0.509 | 0.934 | 5.24 | 2561.1 |
| render/present/ms | 0.484 | 0.592 | 5.61 | 2437.9 |
| terrain/update/ms | 0.298 | 0.383 | 1.76 | 1501.6 |
| controller/feed/ms | 0.282 | 0.378 | 0.70 | 1417.9 |
| terrain/select/ms | 0.264 | 0.339 | 1.48 | 1331.2 |
| stream/drive/ms | 0.261 | 0.359 | 23.06 | 1312.4 |
| stream/tick/ms | 0.200 | 0.274 | 0.86 | 1005.5 |
| far/select/ms | 0.197 | 0.282 | 0.67 | 991.3 |
| bigspace/low_precision/ms | 0.195 | 0.364 | 4.15 | 980.5 |
| anim/evaluate_cpu/ms | 0.156 | 0.199 | 0.48 | 787.7 |
| anim/evaluate/ms | 0.140 | 0.279 | 3.48 | 707.1 |
| bigspace/high_precision/ms | 0.133 | 0.261 | 4.73 | 671.5 |
| controller/move/ms | 0.128 | 0.292 | 0.62 | 646.3 |
| render/acquire/ms | 0.092 | 0.120 | 4.44 | 464.8 |
| physics/step/ms | 0.055 | 0.088 | 0.72 | 279.1 |
| physics/insert/ms | 0.046 | 0.421 | 0.86 | 234.1 |
| weather/map/ms | 0.025 | 0.034 | 0.31 | 124.2 |
| pipelines/driver_ms | 0.022 | 0.000 | 7.79 | 109.7 |
| anim/apply/ms | 0.011 | 0.022 | 0.16 | 57.1 |
| weather/eval/ms | 0.010 | 0.014 | 0.12 | 52.8 |
| controller/avoid/ms | 0.010 | 0.013 | 0.27 | 51.3 |
| script/update/ms | 0.007 | 0.010 | 0.28 | 37.0 |
| stream/despawn/ms | 0.006 | 0.000 | 1.11 | 29.4 |
| stream/spawn/ms | 0.005 | 0.000 | 1.11 | 27.4 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 25.9 |
| script/ui/ms | 0.005 | 0.007 | 0.06 | 23.6 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 16.8 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.63 | 14.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.49 | 5.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 10 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 42 | 14 |
| crowd/c0/joints/count | count | 1277 | 2218 | 641 |
| crowd/c0/meshes/count | count | 352 | 633 | 174 |
| crowd/c1/count | count | 145 | 157 | 147 |
| crowd/c1/joints/count | count | 5531 | 5972 | 5555 |
| crowd/c1/meshes/count | count | 1175 | 1278 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 222 |
| crowd/joints_extracted/count | count | 4271 | 9041 | 6135 |
| crowd/joints_uploaded/count | count | 4249 | 6643 | 6135 |
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
| frame/work/ms | ms | 10.97 | 50.47 | 9.23 |
| gpu/timed/ms | ms | 8.12 | 11.85 | 8.34 |
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
| phases/alpha_mask3d/items | count | 813 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 813 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1288 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3408 | 5110 | 3180 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 24 | 27 | 24 |
| phases/opaque3d_prepass/bins | count | 1288 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3408 | 5110 | 3180 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 23 | 26 | 23 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 49 | 265 | 39 |
| phases/shadow/bins | count | 4786 | 5689 | 2199 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 13.1 k | 5650 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 237 | 288 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2347 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 508 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 216 | 221 | 221 |
| pipelines/waiting | count | 0 | 43 | 0 |
| preprocess/bind_groups | count | 11 | 721 | 4 |
| preprocess/bind_groups_created | count | 11 | 721 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 234 | 337 | 229 |
| process/cpu_usage | % | 24.19 | 38.39 | 22.12 |
| process/mem_usage | GiB | 5.14 | 5.31 | 5.31 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.69 | 85.61 | 31.38 |
| system/mem_usage | % | 61.47 | 61.90 | 61.86 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 113 | 328 | 105 |
| views/all/bins | count | 7373 | 8547 | 5679 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.5 k | 25.6 k | 13.2 k |
| views/all/pipelines | count | 298 | 349 | 101 |
| views/all/pipelines_distinct | count | 94 | 101 | 83 |
| views/all/work_items | count | 17.3 k | 23.1 k | 13.6 k |
| views/cascade/batch_sets | count | 48 | 62 | 39 |
| views/cascade/bins | count | 2603 | 3135 | 2199 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8152 | 10.2 k | 5650 |
| views/cascade/pipelines | count | 48 | 62 | 39 |
| views/cascade/work_items | count | 8152 | 10.2 k | 5650 |
| views/lamp_face/batch_sets | count | 0 | 216 | 0 |
| views/lamp_face/bins | count | 2130 | 2738 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2390 | 3037 | 0 |
| views/lamp_face/pipelines | count | 188 | 236 | 0 |
| views/lamp_face/work_items | count | 0 | 2834 | 0 |
| views/main/batch_sets | count | 66 | 71 | 66 |
| views/main/bins | count | 2868 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8432 | 13.0 k | 7594 |
| views/main/pipelines | count | 62 | 67 | 62 |
| views/main/work_items | count | 8596 | 13.2 k | 7902 |
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
| anim/bones_written/count | count | 44.43 | 208 | 223.7 k |
| anim/events/count | count | 0.10 | 3 | 515 |
| assets/Image/added | count | 0.12 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.81 | 152 | 4075 |
| assets/Mesh/removed | count | 0.97 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.30 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.94 | 1386 | 4719 |
| physics/inserts | count | 0.55 | 109 | 2774 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.46 | 161 | 2335 |
| physics/steps | count | 0.78 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 16 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 24 | 378 |
| stream/upload/bytes | bytes | 6723 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1474 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.7 ms; slowest 28, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.53 | 2534.2 | main/busy/ms 2506, surface/advance_cpu_ms 67.14, surface/cell_upload_ms 0.76, stream/drive/ms 0.56, bigspace/total/ms 0.15, stream/tick/ms 0.14, weather/map/ms 0.14, bigspace/high_precision/ms 0.08, weather/eval/ms 0.04, script/update/ms 0.04, script/input/ms 0.02, anim/evaluate/ms 0.02 |
| warm-up | 3.28 | 748.4 | main/busy/ms 2538, render/thread/ms 716, render/busy/ms 549, render/acquire/ms 163, surface/advance_cpu_ms 67.14, render/graph/ms 6.69, render/present/ms 3.84, stream/drive/ms 1.30, surface/cell_upload_ms 0.76, stream/tick/ms 0.58, terrain/update/ms 0.57, terrain/select/ms 0.35 |
| warm-up | 3.49 | 207.6 | render/thread/ms 738, pipelines/wait_ms 715, render/busy/ms 569, pipelines/compile_ms 200, pipelines/front_ms 186, render/acquire/ms 164, main/busy/ms 51.71, render/graph/ms 18.25, terrain/update/ms 17.43, pipelines/driver_ms 13.36, render/present/ms 4.96, stream/drive/ms 1.56 |
| warm-up | 3.68 | 40.8 | main/busy/ms 83.89, render/thread/ms 57.68, render/busy/ms 44.66, render/acquire/ms 11.63, render/graph/ms 3.14, stream/drive/ms 2.62, stream/tick/ms 2.19, render/present/ms 1.40, terrain/update/ms 0.72, terrain/select/ms 0.59, bigspace/total/ms 0.17, weather/map/ms 0.10 |
| warm-up | 3.78 | 92.4 | pipelines/compile_ms 270, pipelines/front_ms 254, main/busy/ms 92.32, render/thread/ms 91.47, render/busy/ms 89.49, pipelines/driver_ms 16.21, render/graph/ms 4.00, stream/drive/ms 3.32, stream/tick/ms 2.26, render/present/ms 1.76, terrain/update/ms 0.64, terrain/select/ms 0.53 |
| warm-up | 3.93 | 82.4 | pipelines/wait_ms 671, pipelines/compile_ms 196, pipelines/front_ms 186, main/busy/ms 48.00, render/thread/ms 43.11, render/busy/ms 41.78, stream/drive/ms 32.35, pipelines/driver_ms 9.62, render/graph/ms 3.37, render/present/ms 1.16, stream/tick/ms 0.91, terrain/update/ms 0.89 |
| warm-up | 4.07 | 82.3 | pipelines/wait_ms 3507, pipelines/compile_ms 105, pipelines/front_ms 99.65, render/thread/ms 56.76, render/busy/ms 55.42, main/busy/ms 50.08, stream/drive/ms 26.93, pipelines/driver_ms 4.95, render/graph/ms 3.09, stream/tick/ms 2.00, render/present/ms 1.16, stream/spawn/ms 1.08 |
| warm-up | 4.10 | 33.1 | pipelines/wait_ms 4173, pipelines/compile_ms 116, pipelines/front_ms 110, render/thread/ms 63.62, render/busy/ms 62.56, main/busy/ms 58.21, stream/drive/ms 33.78, pipelines/driver_ms 6.76, stream/spawn/ms 2.19, render/graph/ms 2.11, stream/tick/ms 1.71, terrain/update/ms 1.08 |
| warm-up | 4.22 | 110.9 | pipelines/wait_ms 1129, render/thread/ms 90.77, main/busy/ms 66.18, render/busy/ms 52.24, render/present/ms 38.33, stream/drive/ms 36.96, pipelines/compile_ms 21.92, pipelines/front_ms 19.49, stream/spawn/ms 5.76, render/graph/ms 2.73, pipelines/driver_ms 2.43, stream/tick/ms 1.52 |
| warm-up | 4.26 | 45.9 | pipelines/wait_ms 2480, pipelines/compile_ms 181, pipelines/front_ms 176, render/thread/ms 103, render/busy/ms 64.91, render/present/ms 38.39, main/busy/ms 37.33, stream/drive/ms 8.28, stream/spawn/ms 5.20, pipelines/driver_ms 5.17, render/graph/ms 3.75, stream/tick/ms 1.22 |
| warm-up | 4.32 | 32.8 | pipelines/wait_ms 2794, render/thread/ms 47.36, render/busy/ms 45.53, pipelines/compile_ms 28.89, pipelines/front_ms 26.58, main/busy/ms 16.82, render/graph/ms 11.70, stream/drive/ms 3.35, pipelines/driver_ms 2.32, render/present/ms 1.58, stream/activate/ms 1.03, terrain/update/ms 0.96 |
| warm-up | 4.36 | 45.0 | render/thread/ms 69.36, render/busy/ms 66.04, render/graph/ms 28.69, main/busy/ms 22.66, stream/drive/ms 5.69, render/present/ms 3.02, stream/spawn/ms 2.74, terrain/update/ms 0.96, stream/tick/ms 0.96, terrain/select/ms 0.77, stream/activate/ms 0.64, bigspace/total/ms 0.41 |
| warm-up | 4.45 | 52.1 | main/busy/ms 88.90, stream/drive/ms 61.61, render/thread/ms 50.14, render/busy/ms 48.47, terrain/update/ms 6.76, render/graph/ms 6.39, stream/spawn/ms 2.47, stream/tick/ms 1.47, render/present/ms 1.35, terrain/select/ms 0.96, bigspace/total/ms 0.49, render/acquire/ms 0.31 |
| warm-up | 4.54 | 97.4 | pipelines/wait_ms 2410, pipelines/compile_ms 192, pipelines/front_ms 183, main/busy/ms 83.00, render/thread/ms 77.07, render/busy/ms 75.71, stream/drive/ms 60.39, pipelines/driver_ms 9.14, terrain/update/ms 6.70, render/graph/ms 5.66, stream/spawn/ms 1.84, render/present/ms 1.08 |
| warm-up | 4.63 | 66.9 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 171, pipelines/front_ms 158, main/busy/ms 70.79, stream/drive/ms 50.47, render/thread/ms 36.01, render/busy/ms 34.92, pipelines/driver_ms 12.85, render/graph/ms 3.60, stream/spawn/ms 2.17, stream/tick/ms 0.92, render/present/ms 0.87 |
| warm-up | 4.70 | 51.3 | pipelines/wait_ms 3088, main/busy/ms 52.43, stream/drive/ms 38.77, render/thread/ms 33.51, render/busy/ms 32.06, pipelines/compile_ms 20.08, pipelines/front_ms 15.74, render/graph/ms 5.96, pipelines/driver_ms 4.35, stream/spawn/ms 2.04, render/present/ms 1.24, stream/tick/ms 0.88 |
| warm-up | 4.76 | 46.8 | main/busy/ms 53.52, stream/drive/ms 35.12, render/thread/ms 30.28, render/busy/ms 28.49, render/graph/ms 7.76, render/present/ms 1.58, stream/activate/ms 1.44, stream/tick/ms 0.75, terrain/update/ms 0.72, stream/spawn/ms 0.70, terrain/select/ms 0.60, physics/insert/ms 0.53 |
| warm-up | 4.87 | 32.0 | pipelines/wait_ms 1876, render/thread/ms 40.93, render/busy/ms 39.19, main/busy/ms 32.39, pipelines/compile_ms 27.54, pipelines/front_ms 26.28, render/graph/ms 9.62, stream/drive/ms 4.05, stream/spawn/ms 2.47, render/present/ms 1.29, pipelines/driver_ms 1.26, stream/tick/ms 1.06 |
| warm-up | 4.90 | 32.7 | pipelines/wait_ms 1876, render/thread/ms 50.54, render/busy/ms 49.31, main/busy/ms 28.18, pipelines/compile_ms 27.54, pipelines/front_ms 26.28, render/graph/ms 8.92, stream/drive/ms 3.67, stream/spawn/ms 1.89, pipelines/driver_ms 1.26, stream/tick/ms 1.08, render/present/ms 0.95 |
| warm-up | 5.00 | 34.5 | render/thread/ms 36.00, render/busy/ms 34.46, main/busy/ms 23.40, stream/drive/ms 6.26, render/graph/ms 5.77, render/present/ms 1.25, stream/tick/ms 1.25, stream/spawn/ms 0.79, terrain/update/ms 0.74, terrain/select/ms 0.61, bigspace/total/ms 0.52, render/acquire/ms 0.29 |
| warm-up | 5.08 | 31.8 | pipelines/wait_ms 746, render/thread/ms 40.95, render/busy/ms 39.36, main/busy/ms 30.56, stream/drive/ms 11.43, pipelines/compile_ms 8.55, pipelines/front_ms 8.24, render/graph/ms 5.91, render/present/ms 1.34, stream/tick/ms 1.32, terrain/update/ms 0.87, terrain/select/ms 0.72 |
| warm-up | 6.03 | 44.4 | pipelines/wait_ms 5354, render/thread/ms 45.74, render/busy/ms 44.51, main/busy/ms 41.80, pipelines/compile_ms 38.42, pipelines/front_ms 36.61, stream/drive/ms 23.60, render/graph/ms 4.73, stream/spawn/ms 1.86, pipelines/driver_ms 1.81, terrain/update/ms 1.04, render/present/ms 1.00 |
| warm-up | 6.51 | 33.4 | main/busy/ms 40.72, render/thread/ms 31.32, render/busy/ms 29.87, render/graph/ms 8.21, terrain/update/ms 2.46, stream/drive/ms 1.29, render/present/ms 1.24, physics/insert/ms 0.85, bigspace/total/ms 0.75, stream/tick/ms 0.66, terrain/select/ms 0.52, stream/spawn/ms 0.34 |
| measure | 2.72 | 32.3 | render/thread/ms 43.89, render/busy/ms 42.59, render/graph/ms 22.70, main/busy/ms 20.49, render/present/ms 1.15, physics/insert/ms 0.92, bigspace/total/ms 0.79, terrain/update/ms 0.55, terrain/select/ms 0.49, stream/drive/ms 0.44, controller/feed/ms 0.43, stream/tick/ms 0.39 |
| measure | 17.45 | 48.2 | main/busy/ms 69.03, render/thread/ms 67.58, render/busy/ms 61.21, render/graph/ms 23.83, render/acquire/ms 4.57, render/present/ms 1.80, bigspace/total/ms 1.63, controller/feed/ms 1.01, terrain/update/ms 0.82, stream/drive/ms 0.74, terrain/select/ms 0.73, far/select/ms 0.71 |
| measure | 17.49 | 35.1 | render/thread/ms 73.29, main/busy/ms 72.98, render/busy/ms 67.19, render/graph/ms 25.03, render/acquire/ms 4.57, bigspace/total/ms 2.07, render/present/ms 1.54, controller/feed/ms 1.17, terrain/update/ms 1.13, terrain/select/ms 1.04, stream/drive/ms 0.79, stream/tick/ms 0.70 |
| measure | 18.20 | 35.6 | main/busy/ms 41.41, render/thread/ms 26.64, render/busy/ms 25.51, stream/drive/ms 23.31, render/graph/ms 15.55, render/present/ms 0.97, bigspace/total/ms 0.88, terrain/update/ms 0.67, controller/feed/ms 0.64, terrain/select/ms 0.60, far/select/ms 0.55, stream/tick/ms 0.40 |
| measure | 59.06 | 36.1 | render/thread/ms 48.05, render/busy/ms 46.80, main/busy/ms 28.93, render/graph/ms 13.24, bigspace/total/ms 1.55, stream/drive/ms 1.49, stream/tick/ms 1.07, far/select/ms 1.02, controller/feed/ms 0.96, render/present/ms 0.96, terrain/update/ms 0.85, terrain/select/ms 0.75 |

## Run

- `C:\Users\leogloglo\AppData\Local\Temp\depth-keys-on\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\dkeys_on\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **73%** (min 46%) · 2580 MHz (min 2580) · memory 8001 MHz · 74.5 W · 74 °C (max 79) · VRAM 6120 MiB · throttled 100% of samples
