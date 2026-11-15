# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4628 frames over 65.7 s · **median 13.64 ms (73 fps)** · mean 14.19 · p95 17.59 · p99 20.14 (1% low 50 fps) · worst 46.28 ms · 44 spikes over 1.5× median · 21.0 stutters/min (work 21.9/min)
- warm-up: 384 frames over 10.0 s, worst 3113.2 ms
- hitches (> 34.1 ms): **3** measured, 20 in warm-up
- timed GPU passes: 8.15 ms of a 14.19 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.64 ms, limit 11 ms
- **FAIL** frame_ms: 23 stutters (> 1.5x moving median of 60) in 1.09 min = 21.02/min, limit 1/min
- **FAIL** worst frame 46.28 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.211 | 2.214 | 2.796 | 1.00 | 0.182 |
| main_opaque_pass_3d | 1.770 | 1.772 | 2.426 | 1.00 | 0.087 |
| early prepass | 0.553 | 0.554 | 0.941 | 1.00 | 0.124 |
| bloom | 0.341 | 0.342 | 0.496 | 1.00 | 0.067 |
| uniform_allocation | 0.336 | 0.336 | 0.626 | 1.00 | 0.060 |
| atmosphere_luts | 0.332 | 0.332 | 0.464 | 1.00 | 0.004 |
| water_velocity | 0.303 | 0.303 | 0.460 | 1.00 | 0.002 |
| bin_unpacking | 0.263 | 0.264 | 0.284 | 1.00 | 0.042 |
| water_sspr | 0.233 | 0.234 | 0.239 | 1.00 | 0.002 |
| ssao | 0.207 | 0.207 | 0.208 | 1.00 | 0.008 |
| auxiliary_views | 0.178 | 0.178 | 0.715 | 1.00 | 0.162 |
| atmos_froxel_inject | 0.161 | 0.161 | 0.154 | 1.00 | 0.002 |
| clouds_trace | 0.141 | 0.141 | 0.228 | 1.00 | 0.001 |
| water_fft | 0.140 | 0.140 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.129 | 0.129 | 0.138 | 1.00 | 0.004 |
| clouds_composite | 0.113 | 0.113 | 0.111 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.095 | 0.095 | 0.174 | 1.00 | 0.009 |
| tonemapping | 0.092 | 0.092 | 0.092 | 1.00 | 0.004 |
| clustering | 0.068 | 0.069 | 0.069 | 1.00 | 0.021 |
| lightprobe_irradiance_map | 0.055 | 0.443 | 0.746 | 0.12 | 0.001 |
| render_sky | 0.054 | 0.054 | 0.055 | 1.00 | 0.002 |
| surface_trail | 0.051 | 0.058 | 0.057 | 0.87 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.047 | 0.047 | 0.049 | 1.00 | 0.006 |
| water_snapshot | 0.037 | 0.037 | 0.047 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.029 | 0.236 | 0.235 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.032 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.003 |
| per_view_shadow_pass | 0.023 | 0.023 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.030 | 1.00 | 0.110 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 33.073 | 0.000 | 12306.25 | 153060.6 |
| render/thread/ms | 12.983 | 16.203 | 45.06 | 60086.0 |
| render/busy/ms | 12.391 | 15.546 | 44.37 | 57346.1 |
| main/busy/ms | 9.522 | 12.442 | 28.01 | 44067.9 |
| render/graph/ms | 7.681 | 9.690 | 39.61 | 35546.1 |
| pipelines/compile_ms | 0.647 | 0.000 | 158.33 | 2993.8 |
| pipelines/front_ms | 0.625 | 0.000 | 152.88 | 2891.8 |
| render/present/ms | 0.501 | 0.586 | 12.47 | 2320.1 |
| bigspace/total/ms | 0.454 | 0.717 | 5.42 | 2100.5 |
| terrain/update/ms | 0.287 | 0.361 | 2.03 | 1330.5 |
| controller/feed/ms | 0.271 | 0.369 | 0.84 | 1254.2 |
| stream/drive/ms | 0.264 | 0.377 | 20.24 | 1223.1 |
| terrain/select/ms | 0.254 | 0.319 | 0.62 | 1177.1 |
| stream/tick/ms | 0.201 | 0.277 | 0.77 | 932.3 |
| far/select/ms | 0.196 | 0.279 | 0.82 | 908.0 |
| bigspace/low_precision/ms | 0.175 | 0.305 | 4.18 | 810.5 |
| anim/evaluate_cpu/ms | 0.163 | 0.208 | 0.45 | 753.0 |
| controller/move/ms | 0.130 | 0.225 | 0.76 | 603.0 |
| anim/evaluate/ms | 0.129 | 0.279 | 5.00 | 595.2 |
| bigspace/high_precision/ms | 0.106 | 0.206 | 3.61 | 491.3 |
| render/acquire/ms | 0.091 | 0.119 | 0.40 | 419.9 |
| physics/step/ms | 0.063 | 0.094 | 0.71 | 292.1 |
| physics/insert/ms | 0.061 | 0.453 | 1.05 | 282.6 |
| weather/map/ms | 0.024 | 0.033 | 0.22 | 110.5 |
| pipelines/driver_ms | 0.022 | 0.000 | 7.15 | 101.9 |
| anim/apply/ms | 0.011 | 0.020 | 0.19 | 52.7 |
| controller/avoid/ms | 0.010 | 0.013 | 0.05 | 47.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.05 | 46.0 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 34.1 |
| stream/despawn/ms | 0.007 | 0.000 | 1.02 | 30.4 |
| stream/spawn/ms | 0.006 | 0.000 | 1.06 | 25.9 |
| script/input/ms | 0.005 | 0.007 | 0.08 | 24.5 |
| script/ui/ms | 0.005 | 0.006 | 0.25 | 21.2 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 15.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 15.0 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 4.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.4 |
| weather/apply/ms | 0.000 | 0.000 | 0.01 | 1.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 19 | 0 |
| anim/a2/count | count | 1 | 15 | 0 |
| anim/a3/count | count | 568 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 78 | 18 |
| controller/movers/count | count | 20 | 22 | 18 |
| crowd/c0/count | count | 26 | 50 | 15 |
| crowd/c0/joints/count | count | 1277 | 2262 | 707 |
| crowd/c0/meshes/count | count | 352 | 646 | 189 |
| crowd/c1/count | count | 145 | 156 | 146 |
| crowd/c1/joints/count | count | 5483 | 5991 | 5489 |
| crowd/c1/meshes/count | count | 1157 | 1295 | 1152 |
| crowd/c2/count | count | 396 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4268 | 13.1 k | 6179 |
| crowd/joints_uploaded/count | count | 4245 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.92 | 44.37 | 10.77 |
| gpu/timed/ms | ms | 8.81 | 12.14 | 8.90 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 3073 | 3143 | 3143 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 2 | 2 | 2 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 277.8 k | 282.2 k | 282.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 801 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 801 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 35 | 41 | 35 |
| phases/opaque3d/bins | count | 1289 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3378 | 5135 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 35 | 41 | 35 |
| phases/opaque3d_prepass/bins | count | 1289 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3378 | 5135 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 75 | 351 | 62 |
| phases/shadow/bins | count | 3704 | 5443 | 2247 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9354 | 12.9 k | 5698 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 136 | 328 | 49 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2347 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 1148 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 220 | 226 | 226 |
| pipelines/waiting | count | 0 | 27 | 0 |
| preprocess/bind_groups | count | 13 | 947 | 6 |
| preprocess/bind_groups_created | count | 13 | 947 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 317 | 458 | 309 |
| process/cpu_usage | % | 22.60 | 49.49 | 26.34 |
| process/mem_usage | GiB | 5.17 | 5.36 | 5.35 |
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
| started_load_count |  | 4767 | 4860 | 4860 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.11 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.35 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.93 | 48.19 | 30.19 |
| system/mem_usage | % | 63.30 | 64.14 | 64.07 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2591 | 2649 | 2649 |
| views/all/batch_sets | count | 164 | 441 | 146 |
| views/all/bins | count | 6300 | 8289 | 5727 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.0 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 198 | 388 | 115 |
| views/all/pipelines_distinct | count | 95 | 108 | 91 |
| views/all/work_items | count | 17.0 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 73 | 93 | 62 |
| views/cascade/bins | count | 2604 | 3111 | 2247 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8134 | 10.2 k | 5698 |
| views/cascade/pipelines | count | 56 | 73 | 49 |
| views/cascade/work_items | count | 8134 | 10.2 k | 5698 |
| views/lamp_face/batch_sets | count | 0 | 274 | 0 |
| views/lamp_face/bins | count | 1052 | 2523 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1177 | 2864 | 0 |
| views/lamp_face/pipelines | count | 78 | 258 | 0 |
| views/lamp_face/work_items | count | 0 | 2659 | 0 |
| views/main/batch_sets | count | 86 | 98 | 84 |
| views/main/bins | count | 2872 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8364 | 13.0 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8533 | 13.3 k | 7900 |
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
| anim/bones_written/count | count | 47.00 | 734 | 217.5 k |
| anim/events/count | count | 0.11 | 3 | 516 |
| assets/Image/added | count | 0.15 | 45 | 715 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.96 | 152 | 4428 |
| assets/Mesh/removed | count | 1.05 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.13 | 50 | 595 |
| assets/StandardMaterial/modified | count | 4.68 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.19 | 3315 | 14.7 k |
| physics/inserts | count | 0.72 | 127 | 3354 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.50 | 161 | 2335 |
| physics/steps | count | 0.85 | 3 | 3940 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.01 | 6 | 26 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 7315 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1604 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 34.1 ms; slowest 23, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.11 | 3113.2 | main/busy/ms 3085, surface/advance_cpu_ms 69.92, surface/cell_upload_ms 1.01, stream/drive/ms 0.62, stream/tick/ms 0.15, weather/map/ms 0.12, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 4.08 | 969.0 | main/busy/ms 3123, render/thread/ms 930, render/busy/ms 738, render/acquire/ms 189, surface/advance_cpu_ms 69.92, render/graph/ms 6.81, render/present/ms 2.68, stream/drive/ms 1.30, surface/cell_upload_ms 1.01, stream/tick/ms 0.54, terrain/update/ms 0.40, weather/map/ms 0.29 |
| warm-up | 4.18 | 100.7 | pipelines/wait_ms 2553, render/thread/ms 943, render/busy/ms 749, render/acquire/ms 189, pipelines/compile_ms 111, pipelines/front_ms 102, main/busy/ms 57.05, terrain/update/ms 19.17, render/graph/ms 10.94, pipelines/driver_ms 9.55, render/present/ms 3.88, stream/drive/ms 1.51 |
| warm-up | 4.46 | 90.7 | pipelines/compile_ms 239, pipelines/front_ms 229, main/busy/ms 65.28, render/thread/ms 49.14, render/busy/ms 47.30, pipelines/driver_ms 9.73, render/graph/ms 3.81, render/present/ms 1.55, stream/drive/ms 1.51, stream/tick/ms 1.20, terrain/update/ms 0.87, terrain/select/ms 0.73 |
| warm-up | 4.61 | 59.9 | main/busy/ms 45.05, stream/drive/ms 32.98, render/thread/ms 30.67, render/busy/ms 29.25, render/graph/ms 3.52, render/present/ms 1.23, stream/tick/ms 1.20, stream/spawn/ms 0.79, terrain/update/ms 0.74, terrain/select/ms 0.60, bigspace/total/ms 0.25, render/acquire/ms 0.19 |
| warm-up | 4.68 | 54.2 | pipelines/wait_ms 727, pipelines/compile_ms 182, pipelines/front_ms 164, render/thread/ms 55.68, render/busy/ms 53.22, main/busy/ms 38.04, stream/drive/ms 23.08, pipelines/driver_ms 17.94, render/graph/ms 6.82, render/present/ms 2.08, stream/spawn/ms 1.17, terrain/update/ms 0.87 |
| warm-up | 4.76 | 73.1 | render/thread/ms 58.34, render/busy/ms 55.91, main/busy/ms 29.55, stream/drive/ms 10.74, render/graph/ms 8.15, stream/spawn/ms 2.17, render/present/ms 2.11, stream/tick/ms 1.54, terrain/update/ms 0.93, terrain/select/ms 0.78, render/acquire/ms 0.32, bigspace/total/ms 0.27 |
| warm-up | 4.82 | 57.6 | render/thread/ms 96.68, render/busy/ms 93.58, main/busy/ms 33.86, render/graph/ms 10.46, stream/drive/ms 8.92, render/present/ms 2.72, stream/tick/ms 2.31, stream/spawn/ms 1.14, terrain/update/ms 0.90, terrain/select/ms 0.72, bigspace/total/ms 0.44, render/acquire/ms 0.38 |
| warm-up | 4.87 | 50.3 | render/thread/ms 75.80, render/busy/ms 73.65, main/busy/ms 33.75, stream/drive/ms 8.22, render/graph/ms 7.30, stream/tick/ms 2.03, render/present/ms 1.84, terrain/update/ms 1.18, terrain/select/ms 0.93, stream/activate/ms 0.81, bigspace/total/ms 0.45, render/acquire/ms 0.31 |
| warm-up | 4.92 | 43.7 | pipelines/wait_ms 1805, pipelines/compile_ms 288, pipelines/front_ms 276, main/busy/ms 56.33, render/thread/ms 56.16, render/busy/ms 51.46, stream/drive/ms 33.62, pipelines/driver_ms 11.96, render/present/ms 4.48, render/graph/ms 3.10, stream/tick/ms 1.41, terrain/update/ms 1.23 |
| warm-up | 4.96 | 43.0 | pipelines/wait_ms 1805, pipelines/compile_ms 288, pipelines/front_ms 276, main/busy/ms 56.83, render/thread/ms 49.83, render/busy/ms 44.36, stream/drive/ms 34.87, pipelines/driver_ms 11.96, render/present/ms 5.26, render/graph/ms 4.60, stream/spawn/ms 2.61, stream/tick/ms 1.29 |
| warm-up | 5.03 | 45.4 | render/thread/ms 50.55, render/busy/ms 48.12, main/busy/ms 24.32, render/graph/ms 6.23, stream/drive/ms 4.54, render/present/ms 2.12, stream/spawn/ms 1.84, stream/tick/ms 1.26, terrain/update/ms 1.00, terrain/select/ms 0.83, bigspace/total/ms 0.59, stream/activate/ms 0.52 |
| warm-up | 5.07 | 38.3 | pipelines/wait_ms 4711, pipelines/compile_ms 176, pipelines/front_ms 165, main/busy/ms 69.82, render/thread/ms 55.89, render/busy/ms 53.83, stream/drive/ms 53.67, pipelines/driver_ms 10.99, render/graph/ms 8.40, render/present/ms 1.83, stream/tick/ms 1.67, stream/spawn/ms 1.45 |
| warm-up | 5.16 | 89.0 | pipelines/wait_ms 5579, pipelines/compile_ms 216, pipelines/front_ms 204, main/busy/ms 69.38, render/thread/ms 62.28, render/busy/ms 60.64, stream/drive/ms 52.58, pipelines/driver_ms 12.25, render/graph/ms 9.04, render/present/ms 1.47, stream/tick/ms 1.41, stream/spawn/ms 1.22 |
| warm-up | 5.20 | 38.7 | pipelines/wait_ms 5374, pipelines/compile_ms 141, pipelines/front_ms 126, render/thread/ms 72.84, render/busy/ms 70.18, main/busy/ms 63.78, stream/drive/ms 44.08, render/graph/ms 19.21, pipelines/driver_ms 15.60, stream/spawn/ms 2.78, render/present/ms 2.46, terrain/update/ms 0.90 |
| warm-up | 5.25 | 49.9 | pipelines/wait_ms 8249, pipelines/compile_ms 134, pipelines/front_ms 117, main/busy/ms 75.75, render/thread/ms 62.54, render/busy/ms 60.01, stream/drive/ms 56.40, pipelines/driver_ms 17.22, render/graph/ms 16.37, stream/spawn/ms 2.74, render/present/ms 2.20, terrain/update/ms 0.89 |
| warm-up | 5.31 | 43.6 | main/busy/ms 53.95, render/thread/ms 37.42, render/busy/ms 35.81, stream/drive/ms 34.83, render/graph/ms 4.92, stream/spawn/ms 2.17, render/present/ms 1.27, stream/activate/ms 1.04, stream/tick/ms 0.85, bigspace/total/ms 0.74, terrain/update/ms 0.62, terrain/select/ms 0.51 |
| warm-up | 6.14 | 38.8 | main/busy/ms 44.78, render/thread/ms 22.34, render/busy/ms 21.22, stream/drive/ms 15.68, render/graph/ms 5.15, stream/spawn/ms 1.12, render/present/ms 0.94, stream/tick/ms 0.87, bigspace/total/ms 0.64, terrain/update/ms 0.59, terrain/select/ms 0.49, stream/activate/ms 0.46 |
| warm-up | 6.75 | 53.6 | pipelines/wait_ms 4793, pipelines/compile_ms 184, pipelines/front_ms 176, render/thread/ms 58.83, render/busy/ms 57.28, main/busy/ms 42.18, stream/drive/ms 19.63, render/graph/ms 8.99, pipelines/driver_ms 8.53, stream/spawn/ms 2.24, render/present/ms 1.28, stream/tick/ms 1.12 |
| warm-up | 8.33 | 40.8 | render/thread/ms 56.71, render/busy/ms 55.23, main/busy/ms 55.07, render/graph/ms 12.22, stream/drive/ms 1.33, render/present/ms 1.07, bigspace/total/ms 0.92, terrain/update/ms 0.79, terrain/select/ms 0.69, stream/tick/ms 0.65, physics/insert/ms 0.51, anim/evaluate_cpu/ms 0.40 |
| measure | 0.41 | 37.2 | render/thread/ms 43.44, render/busy/ms 42.23, main/busy/ms 38.26, render/graph/ms 17.93, render/present/ms 1.01, physics/insert/ms 0.99, bigspace/total/ms 0.73, anim/evaluate_cpu/ms 0.64, terrain/update/ms 0.54, anim/evaluate/ms 0.53, physics/step/ms 0.53, terrain/select/ms 0.48 |
| measure | 19.47 | 46.3 | render/thread/ms 59.04, render/busy/ms 57.79, render/graph/ms 47.92, main/busy/ms 22.35, render/present/ms 1.05, bigspace/total/ms 0.80, physics/insert/ms 0.61, terrain/update/ms 0.55, controller/feed/ms 0.54, stream/drive/ms 0.53, terrain/select/ms 0.50, stream/tick/ms 0.43 |
| measure | 59.04 | 35.0 | render/thread/ms 45.46, render/busy/ms 44.29, main/busy/ms 25.13, render/graph/ms 13.58, bigspace/total/ms 1.33, controller/feed/ms 1.15, terrain/update/ms 0.98, render/present/ms 0.96, stream/drive/ms 0.95, far/select/ms 0.92, terrain/select/ms 0.87, stream/tick/ms 0.86 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-off\tex-slab-off.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab_off\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 12%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **74%** (min 50%) · 2580 MHz (min 2580) · memory 8001 MHz · 72.6 W · 74 °C (max 79) · VRAM 5383 MiB · throttled 100% of samples
