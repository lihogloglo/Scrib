# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4853 frames over 65.6 s · **median 12.97 ms (77 fps)** · mean 13.53 · p95 17.25 · p99 19.76 (1% low 51 fps) · worst 50.34 ms · 60 spikes over 1.5× median · 26.5 stutters/min (work 26.5/min)
- warm-up: 768 frames over 12.0 s, worst 2616.5 ms
- hitches (> 32.4 ms): **2** measured, 23 in warm-up
- timed GPU passes: 7.23 ms of a 13.53 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.97 ms, limit 11 ms
- **FAIL** frame_ms: 29 stutters (> 1.5x moving median of 60) in 1.09 min = 26.50/min, limit 1/min
- **FAIL** worst frame 50.34 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.057 | 2.058 | 2.098 | 1.00 | 0.184 |
| main_opaque_pass_3d | 1.525 | 1.525 | 2.074 | 1.00 | 0.065 |
| early prepass | 0.466 | 0.466 | 0.746 | 1.00 | 0.109 |
| uniform_allocation | 0.292 | 0.300 | 0.330 | 0.98 | 0.055 |
| atmosphere_luts | 0.291 | 0.314 | 0.373 | 0.93 | 0.003 |
| bloom | 0.283 | 0.320 | 0.328 | 0.88 | 0.069 |
| water_velocity | 0.275 | 0.275 | 0.353 | 1.00 | 0.002 |
| bin_unpacking | 0.234 | 0.234 | 0.266 | 1.00 | 0.039 |
| water_sspr | 0.204 | 0.209 | 0.213 | 0.97 | 0.002 |
| ssao | 0.184 | 0.192 | 0.195 | 0.96 | 0.008 |
| auxiliary_views | 0.147 | 0.147 | 0.631 | 1.00 | 0.145 |
| atmos_froxel_inject | 0.131 | 0.148 | 0.150 | 0.89 | 0.002 |
| clouds_trace | 0.129 | 0.145 | 0.262 | 0.89 | 0.001 |
| upscaling | 0.122 | 0.122 | 0.146 | 1.00 | 0.004 |
| water_fft | 0.121 | 0.131 | 0.137 | 0.92 | 0.003 |
| clouds_composite | 0.108 | 0.108 | 0.112 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.083 | 0.083 | 0.171 | 1.00 | 0.008 |
| tonemapping | 0.077 | 0.087 | 0.090 | 0.89 | 0.004 |
| clustering | 0.064 | 0.066 | 0.069 | 0.98 | 0.021 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.418 | 0.420 | 0.12 | 0.001 |
| surface_trail | 0.048 | 0.057 | 0.057 | 0.84 | 0.002 |
| clouds_resolve | 0.044 | 0.050 | 0.050 | 0.89 | 0.001 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.048 | 1.00 | 0.006 |
| water_snapshot | 0.031 | 0.031 | 0.035 | 1.00 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.036 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.027 | 0.223 | 0.234 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.023 | 0.026 | 0.029 | 0.89 | 0.001 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.020 | 0.023 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.031 | 1.00 | 0.100 |
| atmosphere_environment | 0.010 | 0.010 | 0.012 | 1.00 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.89 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.014 | 0.92 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 363.723 | 0.000 | 58714.73 | 1765148.7 |
| pipelines/compile_ms | 15.443 | 0.000 | 3056.48 | 74944.5 |
| pipelines/driver_ms | 13.909 | 0.000 | 2922.69 | 67501.4 |
| render/thread/ms | 12.312 | 15.871 | 33.68 | 59748.3 |
| render/busy/ms | 11.726 | 15.265 | 32.72 | 56905.9 |
| main/busy/ms | 9.354 | 12.293 | 25.49 | 45396.0 |
| render/graph/ms | 7.038 | 9.551 | 19.86 | 34153.7 |
| pipelines/front_ms | 1.534 | 0.000 | 205.59 | 7443.2 |
| render/present/ms | 0.490 | 0.600 | 2.90 | 2379.2 |
| bigspace/total/ms | 0.408 | 0.632 | 4.53 | 1978.4 |
| terrain/update/ms | 0.298 | 0.377 | 1.61 | 1444.8 |
| stream/drive/ms | 0.266 | 0.391 | 17.11 | 1291.0 |
| terrain/select/ms | 0.264 | 0.334 | 0.59 | 1282.4 |
| controller/feed/ms | 0.235 | 0.336 | 0.65 | 1142.6 |
| stream/tick/ms | 0.204 | 0.305 | 0.65 | 990.7 |
| far/select/ms | 0.194 | 0.269 | 0.54 | 939.6 |
| anim/evaluate_cpu/ms | 0.138 | 0.208 | 1.78 | 671.7 |
| bigspace/low_precision/ms | 0.135 | 0.245 | 3.22 | 654.5 |
| anim/evaluate/ms | 0.124 | 0.267 | 2.73 | 602.0 |
| bigspace/high_precision/ms | 0.110 | 0.210 | 3.85 | 532.3 |
| controller/move/ms | 0.102 | 0.229 | 0.52 | 496.3 |
| physics/insert/ms | 0.099 | 0.552 | 1.77 | 479.0 |
| render/acquire/ms | 0.095 | 0.131 | 0.49 | 463.1 |
| physics/step/ms | 0.053 | 0.082 | 1.85 | 256.8 |
| weather/map/ms | 0.026 | 0.037 | 0.21 | 124.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.17 | 52.4 |
| anim/apply/ms | 0.010 | 0.020 | 0.16 | 46.7 |
| controller/avoid/ms | 0.009 | 0.013 | 0.15 | 42.1 |
| script/update/ms | 0.007 | 0.011 | 0.17 | 35.3 |
| stream/despawn/ms | 0.007 | 0.000 | 1.75 | 31.7 |
| stream/spawn/ms | 0.006 | 0.000 | 1.05 | 27.2 |
| script/ui/ms | 0.005 | 0.008 | 0.31 | 26.0 |
| script/input/ms | 0.005 | 0.007 | 0.24 | 24.8 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.69 | 15.2 |
| controller/write/ms | 0.003 | 0.004 | 0.09 | 14.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.04 | 4.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 4.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 15 | 0 |
| anim/a2/count | count | 0 | 11 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 51 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 21 | 50 | 13 |
| crowd/c0/joints/count | count | 959 | 2262 | 593 |
| crowd/c0/meshes/count | count | 303 | 646 | 156 |
| crowd/c1/count | count | 146 | 156 | 148 |
| crowd/c1/joints/count | count | 5500 | 5948 | 5603 |
| crowd/c1/meshes/count | count | 1128 | 1272 | 1185 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 181 | 236 | 223 |
| crowd/joints_extracted/count | count | 4229 | 11.8 k | 6179 |
| crowd/joints_uploaded/count | count | 4214 | 6643 | 6179 |
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
| frame/work/ms | ms | 11.26 | 32.72 | 10.87 |
| gpu/timed/ms | ms | 8.23 | 9.82 | 8.49 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 10 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 812 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 33 | 38 | 34 |
| phases/opaque3d/bins | count | 1204 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3377 | 5114 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 33 | 38 | 34 |
| phases/opaque3d_prepass/bins | count | 1204 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3377 | 5114 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 68 | 363 | 60 |
| phases/shadow/bins | count | 3566 | 5545 | 2242 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8853 | 12.9 k | 5693 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 141 | 332 | 50 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2586 | 3268 | 2347 |
| physics/items | count | 91 | 476 | 19 |
| physics/pending | count | 0 | 2613 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 120 | 0 |
| preprocess/bind_groups | count | 12 | 955 | 6 |
| preprocess/bind_groups_created | count | 12 | 955 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 299 | 483 | 301 |
| process/cpu_usage | % | 22.97 | 50.86 | 21.33 |
| process/mem_usage | GiB | 5.31 | 5.45 | 5.45 |
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
| started_load_count |  | 4763 | 4856 | 4856 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 2 | 2 | 2 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.36 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.54 | 60.80 | 27.43 |
| system/mem_usage | % | 65.74 | 66.25 | 66.07 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2820 | 2935 | 2935 |
| textures/images | count | 2587 | 2645 | 2645 |
| views/all/batch_sets | count | 148 | 445 | 142 |
| views/all/bins | count | 5924 | 8284 | 5722 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 17.5 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 205 | 398 | 116 |
| views/all/pipelines_distinct | count | 95 | 109 | 91 |
| views/all/work_items | count | 16.5 k | 24.0 k | 13.6 k |
| views/cascade/batch_sets | count | 68 | 88 | 60 |
| views/cascade/bins | count | 2565 | 3162 | 2242 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7853 | 10.2 k | 5693 |
| views/cascade/pipelines | count | 54 | 72 | 50 |
| views/cascade/work_items | count | 7853 | 10.2 k | 5693 |
| views/lamp_face/batch_sets | count | 0 | 288 | 0 |
| views/lamp_face/bins | count | 1057 | 2586 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1192 | 2909 | 0 |
| views/lamp_face/pipelines | count | 83 | 269 | 0 |
| views/lamp_face/work_items | count | 0 | 2741 | 0 |
| views/main/batch_sets | count | 82 | 92 | 82 |
| views/main/bins | count | 2762 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8430 | 13.0 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8597 | 13.2 k | 7900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 30.9 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 36.62 | 495 | 177.7 k |
| anim/events/count | count | 0.08 | 4 | 406 |
| assets/Image/added | count | 0.17 | 56 | 819 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.95 | 152 | 4615 |
| assets/Mesh/removed | count | 1.01 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.15 | 59 | 715 |
| assets/StandardMaterial/modified | count | 4.46 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 710 | 4867 | 3.45 M |
| ecs/entities_delta | count | 3.39 | 2109 | 16.5 k |
| physics/inserts | count | 0.91 | 135 | 4431 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.47 | 161 | 2268 |
| physics/steps | count | 0.81 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 6 | 15 |
| pipelines/created | count | 0.01 | 15 | 51 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 24 | 378 |
| stream/upload/bytes | bytes | 6976 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1529 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 32.4 ms; slowest 25, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.62 | 2616.5 | main/busy/ms 2575, surface/advance_cpu_ms 68.64, stream/drive/ms 1.58, stream/tick/ms 0.91, surface/cell_upload_ms 0.70, bigspace/total/ms 0.58, weather/map/ms 0.31, bigspace/high_precision/ms 0.23, far/select/ms 0.12, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 3.44 | 821.7 | main/busy/ms 2624, render/thread/ms 787, render/busy/ms 616, render/acquire/ms 167, surface/advance_cpu_ms 68.64, render/graph/ms 6.78, render/present/ms 3.85, stream/drive/ms 2.55, stream/tick/ms 1.49, terrain/update/ms 1.15, bigspace/total/ms 0.77, surface/cell_upload_ms 0.70 |
| warm-up | 3.61 | 167.2 | pipelines/wait_ms 1129, render/thread/ms 802, render/busy/ms 630, pipelines/compile_ms 447, pipelines/driver_ms 398, render/acquire/ms 167, main/busy/ms 71.40, pipelines/front_ms 49.30, terrain/update/ms 18.12, render/graph/ms 10.07, render/present/ms 4.90, stream/drive/ms 1.62 |
| warm-up | 3.88 | 96.1 | pipelines/wait_ms 1054, pipelines/compile_ms 414, pipelines/driver_ms 356, main/busy/ms 75.87, render/thread/ms 75.79, render/busy/ms 67.21, pipelines/front_ms 57.62, render/acquire/ms 6.85, render/graph/ms 4.70, stream/drive/ms 4.37, stream/tick/ms 4.02, render/present/ms 1.73 |
| warm-up | 3.94 | 40.9 | render/thread/ms 51.96, render/busy/ms 50.56, main/busy/ms 19.63, render/graph/ms 3.02, stream/drive/ms 2.87, terrain/update/ms 1.17, render/present/ms 1.10, stream/tick/ms 1.05, stream/spawn/ms 1.02, terrain/select/ms 0.97, bigspace/total/ms 0.58, stream/upload/ms 0.41 |
| warm-up | 4.03 | 62.4 | main/busy/ms 46.29, render/thread/ms 36.63, render/busy/ms 35.44, stream/drive/ms 32.36, render/graph/ms 3.54, render/present/ms 1.02, terrain/update/ms 0.85, stream/tick/ms 0.83, terrain/select/ms 0.69, stream/spawn/ms 0.36, bigspace/total/ms 0.25, render/acquire/ms 0.17 |
| warm-up | 4.08 | 36.5 | render/thread/ms 35.98, render/busy/ms 34.58, main/busy/ms 26.22, stream/drive/ms 8.84, render/graph/ms 3.63, stream/tick/ms 1.21, render/present/ms 1.21, terrain/update/ms 0.89, terrain/select/ms 0.72, stream/spawn/ms 0.48, stream/activate/ms 0.40, bigspace/total/ms 0.25 |
| warm-up | 4.14 | 38.5 | render/thread/ms 39.92, main/busy/ms 39.56, render/busy/ms 38.17, stream/drive/ms 23.15, render/graph/ms 4.40, render/present/ms 1.53, stream/tick/ms 1.43, stream/spawn/ms 1.01, terrain/update/ms 0.84, terrain/select/ms 0.68, bigspace/total/ms 0.28, render/acquire/ms 0.21 |
| warm-up | 4.18 | 34.9 | render/thread/ms 53.29, render/busy/ms 51.16, main/busy/ms 34.69, render/graph/ms 6.12, stream/drive/ms 5.10, stream/spawn/ms 2.26, render/present/ms 1.85, stream/tick/ms 1.54, terrain/update/ms 0.84, terrain/select/ms 0.68, bigspace/total/ms 0.39, render/acquire/ms 0.28 |
| warm-up | 4.26 | 76.8 | pipelines/wait_ms 812, pipelines/compile_ms 86.53, pipelines/driver_ms 86.52, render/thread/ms 83.29, render/busy/ms 80.54, main/busy/ms 74.96, stream/drive/ms 39.47, render/graph/ms 9.30, stream/spawn/ms 2.45, render/present/ms 2.39, stream/tick/ms 2.05, terrain/update/ms 1.06 |
| warm-up | 4.39 | 131.4 | pipelines/wait_ms 3756, pipelines/compile_ms 228, pipelines/driver_ms 206, render/thread/ms 148, render/busy/ms 145, main/busy/ms 68.63, stream/drive/ms 38.90, pipelines/front_ms 22.60, render/graph/ms 8.81, render/present/ms 2.06, stream/spawn/ms 1.59, stream/tick/ms 1.52 |
| warm-up | 4.43 | 41.5 | pipelines/wait_ms 2944, pipelines/compile_ms 142, render/thread/ms 130, render/busy/ms 128, pipelines/driver_ms 119, main/busy/ms 28.58, pipelines/front_ms 22.59, render/graph/ms 11.20, stream/drive/ms 4.30, render/present/ms 1.57, terrain/update/ms 1.07, stream/activate/ms 1.06 |
| warm-up | 4.55 | 41.0 | pipelines/wait_ms 2341, main/busy/ms 70.15, stream/drive/ms 49.71, render/thread/ms 41.91, render/busy/ms 39.89, pipelines/compile_ms 33.02, pipelines/driver_ms 26.62, pipelines/front_ms 6.40, render/graph/ms 4.98, render/present/ms 1.77, stream/tick/ms 1.32, terrain/update/ms 0.78 |
| warm-up | 4.63 | 76.5 | pipelines/wait_ms 3613, pipelines/compile_ms 79.62, main/busy/ms 69.40, pipelines/driver_ms 57.01, stream/drive/ms 51.11, render/thread/ms 49.30, render/busy/ms 47.82, pipelines/front_ms 22.61, render/graph/ms 3.88, stream/tick/ms 1.47, stream/spawn/ms 1.33, render/present/ms 1.28 |
| warm-up | 4.71 | 62.8 | pipelines/wait_ms 2573, main/busy/ms 69.22, pipelines/compile_ms 56.81, stream/drive/ms 48.93, pipelines/driver_ms 44.41, render/thread/ms 33.77, render/busy/ms 32.17, pipelines/front_ms 12.40, render/graph/ms 3.62, stream/spawn/ms 3.07, render/present/ms 1.20, stream/tick/ms 0.91 |
| warm-up | 4.79 | 47.1 | pipelines/wait_ms 1315, pipelines/compile_ms 149, pipelines/driver_ms 122, main/busy/ms 62.59, render/thread/ms 41.00, stream/drive/ms 40.17, render/busy/ms 39.00, pipelines/front_ms 27.01, render/graph/ms 5.96, stream/spawn/ms 2.28, render/present/ms 1.52, stream/tick/ms 0.89 |
| warm-up | 5.62 | 35.0 | render/thread/ms 52.18, render/busy/ms 50.74, main/busy/ms 27.47, stream/drive/ms 5.13, render/graph/ms 3.52, stream/spawn/ms 2.51, stream/tick/ms 1.35, terrain/update/ms 1.01, render/present/ms 1.00, terrain/select/ms 0.85, bigspace/total/ms 0.61, render/acquire/ms 0.44 |
| warm-up | 5.67 | 33.0 | pipelines/compile_ms 2253, pipelines/driver_ms 2118, pipelines/front_ms 135, pipelines/wait_ms 100, render/thread/ms 47.52, render/busy/ms 46.15, main/busy/ms 22.49, stream/drive/ms 4.93, render/graph/ms 3.66, stream/spawn/ms 2.19, stream/tick/ms 1.17, render/present/ms 1.09 |
| warm-up | 5.73 | 39.2 | pipelines/compile_ms 1962, pipelines/driver_ms 1817, pipelines/wait_ms 445, pipelines/front_ms 145, render/thread/ms 45.40, render/busy/ms 43.56, main/busy/ms 34.14, render/graph/ms 4.97, stream/drive/ms 4.68, physics/insert/ms 3.58, stream/spawn/ms 2.35, render/present/ms 1.47 |
| warm-up | 5.79 | 48.1 | main/busy/ms 48.31, render/thread/ms 44.38, render/busy/ms 42.83, stream/drive/ms 12.04, render/graph/ms 3.90, stream/spawn/ms 2.10, physics/step/ms 1.78, render/present/ms 1.30, stream/tick/ms 0.72, terrain/update/ms 0.71, terrain/select/ms 0.60, bigspace/total/ms 0.54 |
| warm-up | 6.01 | 38.4 | pipelines/wait_ms 5227, pipelines/compile_ms 120, pipelines/driver_ms 100, main/busy/ms 41.42, render/thread/ms 31.93, render/busy/ms 30.53, stream/drive/ms 22.39, pipelines/front_ms 19.50, render/graph/ms 3.46, stream/spawn/ms 2.25, stream/tick/ms 1.84, render/present/ms 1.13 |
| warm-up | 11.67 | 33.1 | main/busy/ms 51.47, render/thread/ms 25.30, render/busy/ms 23.57, render/graph/ms 7.87, render/present/ms 1.48, bigspace/total/ms 0.71, stream/drive/ms 0.63, terrain/update/ms 0.61, stream/tick/ms 0.57, anim/evaluate_cpu/ms 0.53, terrain/select/ms 0.52, far/select/ms 0.47 |
| warm-up | 11.72 | 44.9 | main/busy/ms 40.15, render/thread/ms 37.83, render/busy/ms 35.99, render/graph/ms 6.59, render/present/ms 1.53, bigspace/total/ms 0.80, stream/drive/ms 0.63, terrain/update/ms 0.62, stream/tick/ms 0.55, terrain/select/ms 0.51, anim/evaluate_cpu/ms 0.51, far/select/ms 0.46 |
| measure | 0.41 | 50.3 | render/thread/ms 40.59, render/busy/ms 39.30, main/busy/ms 35.14, render/graph/ms 9.80, render/present/ms 1.07, bigspace/total/ms 0.68, terrain/update/ms 0.56, stream/drive/ms 0.50, terrain/select/ms 0.50, far/select/ms 0.44, stream/tick/ms 0.43, controller/feed/ms 0.23 |
| measure | 19.28 | 34.8 | render/thread/ms 47.17, render/busy/ms 45.67, render/graph/ms 28.00, main/busy/ms 20.57, render/present/ms 1.34, bigspace/total/ms 0.77, controller/feed/ms 0.59, terrain/update/ms 0.54, terrain/select/ms 0.49, stream/drive/ms 0.43, far/select/ms 0.38, stream/tick/ms 0.34 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_old\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 7%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **69%** (min 33%) · 2580 MHz (min 2580) · memory 8001 MHz · 69.6 W · 73 °C (max 79) · VRAM 5413 MiB · throttled 100% of samples
