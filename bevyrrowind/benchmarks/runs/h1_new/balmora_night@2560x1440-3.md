# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1393 frames over 15.0 s · **median 10.85 ms (92 fps)** · mean 10.77 · p95 12.51 · p99 13.95 (1% low 72 fps) · worst 33.31 ms · 7 spikes over 1.5× median · 52.0 stutters/min (work 80.0/min)
- warm-up: 294 frames over 7.4 s, worst 2510.9 ms
- hitches (> 27.1 ms): **2** measured, 27 in warm-up
- timed GPU passes: 8.42 ms of a 10.77 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.85 ms, limit 11 ms
- **FAIL** worst frame 33.31 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.360 | 2.430 | 3.107 | 0.97 | 0.072 |
| dlss_super_resolution | 2.104 | 2.166 | 2.447 | 0.97 | 0.188 |
| early prepass | 0.563 | 0.580 | 0.985 | 0.97 | 0.108 |
| atmosphere_luts | 0.346 | 0.356 | 0.494 | 0.97 | 0.004 |
| bloom | 0.330 | 0.340 | 0.649 | 0.97 | 0.064 |
| water_velocity | 0.266 | 0.274 | 0.272 | 0.97 | 0.002 |
| main_transmissive_pass_3d | 0.234 | 0.241 | 0.278 | 0.97 | 0.008 |
| tonemapping | 0.233 | 0.240 | 0.336 | 0.97 | 0.004 |
| uniform_allocation | 0.230 | 0.244 | 0.255 | 0.94 | 0.039 |
| water_sspr | 0.223 | 0.230 | 0.353 | 0.97 | 0.002 |
| upscaling | 0.223 | 0.229 | 0.290 | 0.97 | 0.004 |
| ssao | 0.199 | 0.205 | 0.211 | 0.97 | 0.008 |
| bin_unpacking | 0.183 | 0.188 | 0.196 | 0.97 | 0.026 |
| water_fft | 0.136 | 0.140 | 0.138 | 0.97 | 0.003 |
| atmos_froxel_inject | 0.122 | 0.132 | 0.133 | 0.92 | 0.002 |
| clouds_composite | 0.106 | 0.109 | 0.110 | 0.97 | 0.002 |
| clouds_trace | 0.071 | 0.077 | 0.099 | 0.92 | 0.001 |
| clustering | 0.066 | 0.068 | 0.082 | 0.97 | 0.022 |
| render_sky | 0.059 | 0.061 | 0.067 | 0.97 | 0.002 |
| water_snapshot | 0.052 | 0.054 | 0.069 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.049 | 0.426 | 0.426 | 0.11 | 0.001 |
| clouds_resolve | 0.048 | 0.052 | 0.054 | 0.92 | 0.001 |
| main_transparent_pass_3d | 0.042 | 0.044 | 0.048 | 0.97 | 0.110 |
| early_mesh_preprocessing | 0.037 | 0.038 | 0.042 | 0.97 | 0.005 |
| atmos_froxel_integrate | 0.030 | 0.032 | 0.033 | 0.92 | 0.001 |
| lightprobe_radiance_map | 0.027 | 0.234 | 0.267 | 0.11 | 0.004 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.018 | 0.019 | 0.020 | 0.97 | 0.005 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 0.97 | 0.003 |
| clouds_fog_light | 0.009 | 0.010 | 0.011 | 0.91 | 0.002 |
| night_sky | 0.009 | 0.009 | 0.027 | 0.97 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.005 | 0.97 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.97 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 178.271 | 0.000 | 12512.53 | 248331.5 |
| render/thread/ms | 9.703 | 11.385 | 24.54 | 13515.8 |
| render/busy/ms | 8.621 | 10.091 | 20.33 | 12008.9 |
| main/busy/ms | 7.088 | 8.558 | 21.23 | 9873.9 |
| render/graph/ms | 5.235 | 6.484 | 10.79 | 7292.4 |
| pipelines/compile_ms | 2.680 | 0.000 | 140.57 | 3733.3 |
| pipelines/front_ms | 2.577 | 0.000 | 135.67 | 3589.5 |
| render/acquire/ms | 0.554 | 1.713 | 3.16 | 771.5 |
| render/present/ms | 0.528 | 0.644 | 15.96 | 735.4 |
| bigspace/total/ms | 0.315 | 0.511 | 3.06 | 438.7 |
| terrain/update/ms | 0.224 | 0.283 | 0.52 | 311.6 |
| stream/drive/ms | 0.199 | 0.280 | 0.55 | 277.2 |
| terrain/select/ms | 0.194 | 0.246 | 0.49 | 270.6 |
| stream/tick/ms | 0.170 | 0.246 | 0.50 | 237.1 |
| far/select/ms | 0.169 | 0.245 | 0.53 | 235.3 |
| controller/feed/ms | 0.150 | 0.212 | 0.45 | 209.0 |
| bigspace/high_precision/ms | 0.109 | 0.226 | 2.84 | 152.4 |
| pipelines/driver_ms | 0.103 | 0.000 | 11.08 | 143.8 |
| controller/move/ms | 0.091 | 0.174 | 0.36 | 126.4 |
| anim/evaluate/ms | 0.082 | 0.202 | 1.33 | 114.2 |
| bigspace/low_precision/ms | 0.070 | 0.151 | 2.74 | 97.2 |
| anim/evaluate_cpu/ms | 0.047 | 0.067 | 0.45 | 64.9 |
| physics/insert/ms | 0.037 | 0.207 | 0.60 | 51.4 |
| physics/step/ms | 0.033 | 0.061 | 1.05 | 46.6 |
| weather/map/ms | 0.015 | 0.021 | 0.16 | 21.4 |
| weather/eval/ms | 0.010 | 0.013 | 0.07 | 13.7 |
| script/update/ms | 0.007 | 0.010 | 0.14 | 9.9 |
| controller/avoid/ms | 0.005 | 0.007 | 0.18 | 7.0 |
| script/input/ms | 0.005 | 0.006 | 0.21 | 7.0 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.2 |
| anim/apply/ms | 0.004 | 0.005 | 0.03 | 5.7 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.0 |
| weather/apply/ms | 0.001 | 0.001 | 0.05 | 0.8 |
| stream/activate/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/spawn/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 0 | 0 |
| anim/a1/count | count | 0 | 0 | 0 |
| anim/a2/count | count | 0 | 39 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 95 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 46 | 8 |
| crowd/c0/joints/count | count | 272 | 1564 | 272 |
| crowd/c0/meshes/count | count | 48 | 276 | 48 |
| crowd/c1/count | count | 122 | 124 | 123 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 11.4 k | 2958 |
| crowd/joints_uploaded/count | count | 2958 | 2958 | 2958 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.67 | 21.23 | 9.60 |
| gpu/timed/ms | ms | 9.63 | 11.27 | 9.60 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 465.71 M | 465.71 M | 465.71 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 3189 | 3189 | 3189 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 330 | 330 | 330 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 3189 | 3189 | 3189 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 34 | 36 | 34 |
| phases/opaque3d/bins | count | 3892 | 3892 | 3892 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 9292 | 9292 | 9292 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 34 | 36 | 34 |
| phases/opaque3d_prepass/bins | count | 3892 | 3892 | 3892 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 9292 | 9292 | 9292 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 36 | 36 | 36 |
| phases/shadow/bins | count | 6686 | 6686 | 6686 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 20.0 k | 20.0 k | 20.0 k |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 31 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1371 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 200 | 200 | 200 |
| pipelines/waiting | count | 0 | 50 | 0 |
| preprocess/bind_groups | count | 0 | 256 | 0 |
| preprocess/bind_groups_created | count | 0 | 256 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 262 | 262 | 262 |
| process/cpu_usage | % | 25.37 | 34.39 | 13.88 |
| process/mem_usage | GiB | 4.35 | 4.42 | 4.42 |
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
| started_load_count |  | 3545 | 3547 | 3547 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 4 | 4 |
| statics/material_pipelines | count | 4 | 4 | 4 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.62 | 46.58 | 28.00 |
| system/mem_usage | % | 62.77 | 62.97 | 62.96 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2478 | 2478 | 2478 |
| textures/images | count | 1917 | 1917 | 1917 |
| views/all/batch_sets | count | 120 | 124 | 120 |
| views/all/bins | count | 15.1 k | 15.1 k | 15.1 k |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 45.0 k | 45.0 k | 45.0 k |
| views/all/pipelines | count | 99 | 103 | 99 |
| views/all/pipelines_distinct | count | 91 | 95 | 91 |
| views/all/work_items | count | 45.4 k | 45.4 k | 45.4 k |
| views/cascade/batch_sets | count | 36 | 36 | 36 |
| views/cascade/bins | count | 6686 | 6686 | 6686 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 20.0 k | 20.0 k | 20.0 k |
| views/cascade/pipelines | count | 31 | 31 | 31 |
| views/cascade/work_items | count | 20.0 k | 20.0 k | 20.0 k |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 84 | 88 | 84 |
| views/main/bins | count | 8444 | 8444 | 8444 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 25.0 k | 25.0 k | 25.0 k |
| views/main/pipelines | count | 68 | 72 | 68 |
| views/main/work_items | count | 25.4 k | 25.4 k | 25.4 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.40 | 330 | 561 |
| anim/events/count | count | 0.07 | 3 | 99 |
| assets/Image/added | count | 0.10 | 34 | 141 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.29 | 111 | 399 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 152 |
| assets/StandardMaterial/modified | count | 3.55 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 7.30 | 5461 | 10.2 k |
| physics/inserts | count | 0.93 | 159 | 1296 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.65 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 8 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0 | 0 | 0 |
| stream/deactivate/items | count | 0 | 0 | 0 |
| stream/despawn/items | count | 0 | 0 | 0 |
| stream/fetch/items | count | 0 | 0 | 0 |
| stream/spawn/items | count | 0 | 0 | 0 |
| stream/upload/bytes | bytes | 0 | 0 | 0 |
| stream/upload/items | count | 0 | 0 | 0 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.1 ms; slowest 29, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2510.9 | main/busy/ms 2485, surface/advance_cpu_ms 0.74, surface/cell_upload_ms 0.31, stream/drive/ms 0.30, bigspace/total/ms 0.14, stream/tick/ms 0.08, weather/map/ms 0.06, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.19 | 683.1 | main/busy/ms 2516, render/thread/ms 646, render/busy/ms 513, render/acquire/ms 131, render/graph/ms 5.30, render/present/ms 2.12, stream/drive/ms 1.13, surface/advance_cpu_ms 0.74, stream/tick/ms 0.65, terrain/update/ms 0.35, surface/cell_upload_ms 0.31, bigspace/total/ms 0.25 |
| warm-up | 3.40 | 210.9 | pipelines/wait_ms 1268, render/thread/ms 657, render/busy/ms 523, pipelines/compile_ms 227, pipelines/front_ms 212, render/acquire/ms 131, main/busy/ms 48.36, terrain/update/ms 16.47, pipelines/driver_ms 14.72, render/graph/ms 8.45, render/present/ms 3.05, stream/drive/ms 1.44 |
| warm-up | 3.66 | 92.4 | main/busy/ms 68.46, render/thread/ms 55.58, render/busy/ms 53.80, render/graph/ms 3.55, stream/drive/ms 1.60, render/present/ms 1.53, stream/tick/ms 1.31, terrain/update/ms 0.82, terrain/select/ms 0.62, bigspace/total/ms 0.27, render/acquire/ms 0.25, bigspace/high_precision/ms 0.08 |
| warm-up | 3.81 | 86.9 | render/thread/ms 56.67, render/busy/ms 55.58, main/busy/ms 48.76, stream/drive/ms 36.26, render/graph/ms 2.29, render/present/ms 0.93, stream/tick/ms 0.77, terrain/update/ms 0.65, terrain/select/ms 0.51, bigspace/total/ms 0.28, render/acquire/ms 0.16, stream/spawn/ms 0.16 |
| warm-up | 3.89 | 69.6 | pipelines/wait_ms 698, pipelines/compile_ms 176, pipelines/front_ms 160, render/thread/ms 50.48, render/busy/ms 48.83, main/busy/ms 30.96, pipelines/driver_ms 16.90, stream/drive/ms 14.74, render/graph/ms 4.96, stream/spawn/ms 1.46, render/present/ms 1.41, stream/tick/ms 0.68 |
| warm-up | 3.98 | 60.2 | render/thread/ms 46.52, render/busy/ms 44.23, main/busy/ms 42.69, stream/drive/ms 20.38, render/graph/ms 3.19, render/present/ms 1.78, stream/spawn/ms 1.68, stream/tick/ms 1.03, terrain/update/ms 0.89, terrain/select/ms 0.67, render/acquire/ms 0.51, bigspace/total/ms 0.36 |
| warm-up | 4.10 | 107.0 | pipelines/wait_ms 6587, pipelines/compile_ms 247, pipelines/front_ms 232, main/busy/ms 108, stream/drive/ms 91.85, render/thread/ms 36.77, render/busy/ms 35.72, pipelines/driver_ms 15.87, render/graph/ms 2.52, stream/spawn/ms 1.17, render/present/ms 0.84, stream/activate/ms 0.74 |
| warm-up | 4.15 | 27.2 | main/busy/ms 37.60, stream/drive/ms 27.89, render/thread/ms 24.69, render/busy/ms 23.14, render/graph/ms 4.05, render/present/ms 1.37, stream/tick/ms 0.63, terrain/update/ms 0.51, stream/activate/ms 0.47, terrain/select/ms 0.40, render/acquire/ms 0.18, bigspace/total/ms 0.18 |
| warm-up | 4.22 | 39.0 | pipelines/wait_ms 409, pipelines/compile_ms 148, pipelines/front_ms 129, render/thread/ms 41.37, render/busy/ms 39.24, main/busy/ms 27.26, pipelines/driver_ms 18.71, stream/drive/ms 7.60, render/graph/ms 4.53, stream/spawn/ms 4.49, render/present/ms 1.77, stream/tick/ms 1.45 |
| warm-up | 4.31 | 42.7 | pipelines/wait_ms 8603, pipelines/compile_ms 73.31, pipelines/front_ms 66.93, main/busy/ms 50.26, stream/drive/ms 29.81, render/thread/ms 27.27, render/busy/ms 26.22, pipelines/driver_ms 6.37, render/graph/ms 2.74, stream/spawn/ms 1.87, stream/tick/ms 1.17, render/present/ms 0.87 |
| warm-up | 4.41 | 28.9 | main/busy/ms 30.38, render/thread/ms 22.36, render/busy/ms 20.95, stream/drive/ms 9.91, render/graph/ms 3.55, stream/spawn/ms 2.15, render/present/ms 1.20, stream/tick/ms 0.67, terrain/update/ms 0.58, terrain/select/ms 0.47, bigspace/total/ms 0.38, render/acquire/ms 0.21 |
| warm-up | 4.47 | 43.1 | pipelines/wait_ms 3048, pipelines/compile_ms 168, pipelines/front_ms 162, main/busy/ms 38.61, render/thread/ms 35.04, render/busy/ms 33.43, pipelines/driver_ms 6.63, render/graph/ms 5.10, stream/drive/ms 4.04, stream/spawn/ms 2.57, render/present/ms 1.37, stream/tick/ms 0.71 |
| warm-up | 4.68 | 37.6 | pipelines/wait_ms 2741, pipelines/compile_ms 58.66, pipelines/front_ms 55.81, render/thread/ms 44.58, render/busy/ms 41.22, main/busy/ms 33.59, stream/drive/ms 15.25, render/graph/ms 4.34, stream/spawn/ms 3.27, render/present/ms 3.10, pipelines/driver_ms 2.85, stream/tick/ms 1.69 |
| warm-up | 4.72 | 39.2 | pipelines/wait_ms 3998, pipelines/compile_ms 67.37, pipelines/front_ms 63.96, render/thread/ms 58.06, render/busy/ms 55.08, main/busy/ms 30.87, stream/drive/ms 14.31, render/graph/ms 4.11, pipelines/driver_ms 3.41, render/present/ms 2.59, stream/spawn/ms 2.57, stream/tick/ms 1.56 |
| warm-up | 4.77 | 34.9 | render/thread/ms 38.07, render/busy/ms 36.19, main/busy/ms 17.38, render/graph/ms 5.64, stream/drive/ms 2.75, render/present/ms 1.40, stream/spawn/ms 1.24, terrain/update/ms 0.77, stream/tick/ms 0.72, bigspace/total/ms 0.66, terrain/select/ms 0.62, render/acquire/ms 0.48 |
| warm-up | 4.82 | 30.5 | render/thread/ms 41.60, render/busy/ms 40.29, main/busy/ms 17.82, render/graph/ms 4.71, stream/drive/ms 2.73, render/present/ms 1.11, stream/spawn/ms 1.09, stream/tick/ms 0.80, terrain/update/ms 0.52, bigspace/total/ms 0.49, terrain/select/ms 0.41, bigspace/high_precision/ms 0.26 |
| warm-up | 4.85 | 29.0 | pipelines/wait_ms 567, pipelines/compile_ms 152, pipelines/front_ms 146, render/thread/ms 31.57, render/busy/ms 30.27, main/busy/ms 18.97, pipelines/driver_ms 6.06, stream/drive/ms 4.85, render/graph/ms 4.82, stream/spawn/ms 2.06, render/present/ms 1.02, stream/tick/ms 0.62 |
| warm-up | 4.91 | 44.4 | render/thread/ms 55.86, render/busy/ms 53.88, main/busy/ms 20.67, render/graph/ms 5.94, stream/drive/ms 4.51, stream/spawn/ms 3.08, render/present/ms 1.71, bigspace/total/ms 0.66, terrain/update/ms 0.65, stream/tick/ms 0.60, terrain/select/ms 0.49, bigspace/high_precision/ms 0.30 |
| warm-up | 4.97 | 36.6 | render/thread/ms 41.90, render/busy/ms 40.62, main/busy/ms 18.85, render/graph/ms 4.02, stream/drive/ms 3.74, stream/spawn/ms 2.04, render/present/ms 1.05, stream/tick/ms 0.67, terrain/update/ms 0.66, terrain/select/ms 0.53, bigspace/total/ms 0.49, bigspace/high_precision/ms 0.25 |
| warm-up | 5.09 | 30.8 | pipelines/wait_ms 5563, render/thread/ms 44.08, render/busy/ms 41.84, pipelines/compile_ms 39.67, pipelines/front_ms 36.97, main/busy/ms 28.86, render/graph/ms 7.65, stream/drive/ms 4.33, stream/spawn/ms 3.03, pipelines/driver_ms 2.70, render/present/ms 1.78, terrain/update/ms 0.92 |
| warm-up | 5.12 | 28.4 | pipelines/wait_ms 10.4 k, pipelines/compile_ms 69.75, pipelines/front_ms 63.51, render/thread/ms 43.98, render/busy/ms 42.23, main/busy/ms 35.66, stream/drive/ms 14.14, render/graph/ms 7.63, pipelines/driver_ms 6.24, stream/spawn/ms 2.70, render/present/ms 1.42, terrain/update/ms 0.85 |
| warm-up | 5.15 | 27.6 | pipelines/wait_ms 5722, pipelines/compile_ms 51.78, pipelines/front_ms 46.44, render/thread/ms 39.49, render/busy/ms 38.44, main/busy/ms 33.59, stream/drive/ms 13.65, pipelines/driver_ms 5.34, render/graph/ms 5.09, stream/spawn/ms 2.21, bigspace/total/ms 1.19, render/present/ms 0.86 |
| warm-up | 5.62 | 45.8 | pipelines/wait_ms 3235, pipelines/compile_ms 156, pipelines/front_ms 146, render/thread/ms 48.82, render/busy/ms 47.26, main/busy/ms 36.31, stream/drive/ms 12.98, pipelines/driver_ms 10.59, render/graph/ms 5.97, stream/spawn/ms 2.14, bigspace/total/ms 1.28, stream/tick/ms 1.26 |
| warm-up | 5.67 | 27.3 | pipelines/wait_ms 5868, pipelines/compile_ms 39.89, pipelines/front_ms 37.77, render/thread/ms 31.68, main/busy/ms 29.63, render/busy/ms 27.90, render/graph/ms 4.61, render/present/ms 3.54, stream/drive/ms 3.24, pipelines/driver_ms 2.12, stream/spawn/ms 1.70, stream/tick/ms 0.85 |
| warm-up | 6.89 | 37.7 | main/busy/ms 44.30, render/thread/ms 35.63, render/busy/ms 34.03, render/graph/ms 7.59, render/present/ms 1.37, stream/drive/ms 1.21, bigspace/total/ms 1.06, anim/evaluate_cpu/ms 0.69, stream/tick/ms 0.51, terrain/update/ms 0.49, terrain/select/ms 0.39, bigspace/low_precision/ms 0.35 |
| warm-up | 6.92 | 30.4 | main/busy/ms 44.85, render/thread/ms 43.40, render/busy/ms 41.94, render/graph/ms 5.66, render/present/ms 1.18, bigspace/total/ms 1.08, stream/drive/ms 1.03, anim/evaluate_cpu/ms 0.55, terrain/update/ms 0.54, terrain/select/ms 0.46, stream/tick/ms 0.43, far/select/ms 0.37 |
| measure | 1.35 | 28.3 | render/thread/ms 33.75, render/busy/ms 31.44, main/busy/ms 26.45, render/graph/ms 10.21, render/acquire/ms 1.43, render/present/ms 0.88, bigspace/total/ms 0.66, terrain/update/ms 0.45, terrain/select/ms 0.40, stream/drive/ms 0.38, physics/insert/ms 0.35, far/select/ms 0.35 |
| measure | 1.43 | 33.3 | pipelines/wait_ms 13.3 k, pipelines/compile_ms 58.49, pipelines/front_ms 55.12, main/busy/ms 39.36, render/thread/ms 30.60, render/busy/ms 29.48, render/graph/ms 10.22, pipelines/driver_ms 3.37, bigspace/total/ms 1.07, render/present/ms 0.87, anim/evaluate_cpu/ms 0.63, bigspace/low_precision/ms 0.60 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\h1_new\balmora_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_GPU_CULL_STATICS=1
- before the run: GPU 1%, CPU 14%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **94%** (min 38%) · 2580 MHz (min 2580) · memory 8001 MHz · 86.4 W · 75 °C (max 78) · VRAM 5094 MiB · throttled 100% of samples
