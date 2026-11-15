# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1367 frames over 15.0 s · **median 10.90 ms (92 fps)** · mean 10.98 · p95 12.28 · p99 13.01 (1% low 77 fps) · worst 19.29 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 8.0/min)
- warm-up: 365 frames over 8.3 s, worst 2695.2 ms
- hitches (> 27.3 ms): **0** measured, 25 in warm-up
- timed GPU passes: 9.20 ms of a 10.98 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.90 ms, limit 11 ms
- pass worst frame 19.29 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.757 | 2.800 | 3.440 | 0.98 | 0.071 |
| dlss_super_resolution | 2.341 | 2.378 | 2.928 | 0.98 | 0.189 |
| early prepass | 0.653 | 0.663 | 1.015 | 0.98 | 0.113 |
| bloom | 0.434 | 0.441 | 0.812 | 0.98 | 0.068 |
| atmosphere_luts | 0.367 | 0.373 | 0.687 | 0.98 | 0.004 |
| water_velocity | 0.290 | 0.294 | 0.626 | 0.98 | 0.002 |
| main_transmissive_pass_3d | 0.289 | 0.294 | 0.295 | 0.98 | 0.009 |
| water_sspr | 0.239 | 0.242 | 0.375 | 0.98 | 0.002 |
| uniform_allocation | 0.222 | 0.225 | 0.225 | 0.98 | 0.036 |
| ssao | 0.208 | 0.211 | 0.214 | 0.98 | 0.008 |
| bin_unpacking | 0.179 | 0.182 | 0.188 | 0.98 | 0.027 |
| tonemapping | 0.151 | 0.153 | 0.164 | 0.98 | 0.003 |
| water_fft | 0.145 | 0.148 | 0.140 | 0.98 | 0.003 |
| upscaling | 0.143 | 0.145 | 0.164 | 0.98 | 0.004 |
| atmos_froxel_inject | 0.135 | 0.137 | 0.135 | 0.98 | 0.002 |
| clouds_composite | 0.107 | 0.109 | 0.109 | 0.98 | 0.002 |
| clouds_trace | 0.069 | 0.070 | 0.081 | 0.98 | 0.001 |
| clustering | 0.066 | 0.067 | 0.078 | 0.98 | 0.022 |
| render_sky | 0.054 | 0.055 | 0.056 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.429 | 0.424 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.053 | 0.98 | 0.001 |
| water_snapshot | 0.042 | 0.043 | 0.052 | 0.98 | 0.002 |
| early_mesh_preprocessing | 0.033 | 0.033 | 0.033 | 0.98 | 0.005 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.029 | 0.239 | 0.242 | 0.12 | 0.004 |
| water_mips | 0.028 | 0.028 | 0.025 | 0.98 | 0.002 |
| main_transparent_pass_3d | 0.025 | 0.025 | 0.027 | 0.98 | 0.071 |
| per_view_shadow_pass | 0.017 | 0.017 | 0.020 | 0.98 | 0.006 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| night_sky | 0.007 | 0.008 | 0.012 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.018 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | — | — | — | - | — |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 121.054 | 0.000 | 11133.24 | 165480.6 |
| render/thread/ms | 10.042 | 11.347 | 18.38 | 13726.9 |
| render/busy/ms | 8.564 | 9.696 | 17.28 | 11706.8 |
| main/busy/ms | 7.284 | 8.747 | 14.45 | 9957.5 |
| render/graph/ms | 5.222 | 6.267 | 13.73 | 7138.5 |
| pipelines/compile_ms | 1.978 | 0.000 | 142.62 | 2704.4 |
| pipelines/front_ms | 1.912 | 0.000 | 139.44 | 2613.8 |
| render/acquire/ms | 0.978 | 2.358 | 3.54 | 1336.8 |
| render/present/ms | 0.500 | 0.588 | 4.58 | 683.2 |
| bigspace/total/ms | 0.365 | 0.663 | 2.83 | 498.3 |
| terrain/update/ms | 0.230 | 0.290 | 0.63 | 314.0 |
| terrain/select/ms | 0.200 | 0.251 | 0.50 | 272.7 |
| stream/drive/ms | 0.199 | 0.285 | 0.52 | 272.7 |
| far/select/ms | 0.174 | 0.256 | 0.60 | 237.3 |
| stream/tick/ms | 0.171 | 0.250 | 0.48 | 233.2 |
| controller/move/ms | 0.157 | 0.253 | 0.58 | 214.5 |
| controller/feed/ms | 0.155 | 0.204 | 0.48 | 211.6 |
| bigspace/high_precision/ms | 0.127 | 0.284 | 2.31 | 173.7 |
| bigspace/low_precision/ms | 0.093 | 0.186 | 2.46 | 127.7 |
| anim/evaluate/ms | 0.076 | 0.189 | 1.16 | 103.5 |
| pipelines/driver_ms | 0.066 | 0.000 | 5.48 | 90.6 |
| anim/evaluate_cpu/ms | 0.059 | 0.077 | 0.26 | 80.9 |
| physics/step/ms | 0.035 | 0.060 | 1.00 | 47.6 |
| weather/map/ms | 0.016 | 0.021 | 0.30 | 22.1 |
| physics/insert/ms | 0.012 | 0.026 | 0.59 | 16.0 |
| weather/eval/ms | 0.010 | 0.013 | 0.15 | 13.7 |
| script/update/ms | 0.007 | 0.010 | 0.10 | 9.4 |
| controller/avoid/ms | 0.007 | 0.009 | 0.15 | 9.0 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.0 |
| script/ui/ms | 0.005 | 0.006 | 0.02 | 6.5 |
| anim/apply/ms | 0.004 | 0.005 | 0.02 | 5.7 |
| controller/write/ms | 0.002 | 0.002 | 0.11 | 2.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.05 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 0.6 |
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
| anim/a2/count | count | 0 | 1 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 9 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 122 | 124 | 122 |
| crowd/c1/joints/count | count | 4848 | 4884 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1090 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 3596 | 2958 |
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
| frame/work/ms | ms | 9.92 | 17.28 | 10.70 |
| gpu/timed/ms | ms | 9.91 | 11.45 | 10.70 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 478.47 M | 478.47 M | 478.47 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 280 | 280 | 280 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 874 | 874 | 874 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 34 | 34 | 34 |
| phases/opaque3d/bins | count | 1303 | 1303 | 1303 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 34 | 34 | 34 |
| phases/opaque3d_prepass/bins | count | 1303 | 1303 | 1303 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 29 | 29 | 29 |
| phases/shadow/bins | count | 985 | 985 | 985 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 2006 | 2006 | 2006 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 24 | 24 | 24 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1454 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 197 | 197 | 197 |
| pipelines/waiting | count | 0 | 46 | 0 |
| preprocess/bind_groups | count | 0 | 170 | 0 |
| preprocess/bind_groups_created | count | 0 | 170 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 244 | 244 | 244 |
| process/cpu_usage | % | 30.80 | 44.71 | 35.51 |
| process/mem_usage | GiB | 4.38 | 4.45 | 4.45 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.40 | 44.56 | 32.54 |
| system/mem_usage | % | 57.42 | 57.61 | 57.61 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2478 | 2478 | 2478 |
| textures/images | count | 1917 | 1917 | 1917 |
| views/all/batch_sets | count | 111 | 111 | 111 |
| views/all/bins | count | 4151 | 4151 | 4151 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 90 | 90 | 90 |
| views/all/pipelines_distinct | count | 89 | 89 | 89 |
| views/all/work_items | count | 11.5 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 29 | 29 | 29 |
| views/cascade/bins | count | 985 | 985 | 985 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2006 | 2006 | 2006 |
| views/cascade/pipelines | count | 24 | 24 | 24 |
| views/cascade/work_items | count | 2006 | 2006 | 2006 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 82 | 82 | 82 |
| views/main/bins | count | 3166 | 3166 | 3166 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9300 | 9300 | 9300 |
| views/main/pipelines | count | 66 | 66 | 66 |
| views/main/work_items | count | 9480 | 9480 | 9480 |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.11 | 2 | 156 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.63 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.32 | 320 | 433 |
| physics/inserts | count | 0.86 | 127 | 1179 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.66 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
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

## Hitches (> 27.3 ms; slowest 25, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.70 | 2695.2 | main/busy/ms 2670, surface/advance_cpu_ms 0.72, stream/drive/ms 0.36, surface/cell_upload_ms 0.28, bigspace/total/ms 0.27, bigspace/high_precision/ms 0.22, stream/tick/ms 0.11, weather/map/ms 0.04, weather/eval/ms 0.02, anim/evaluate/ms 0.02, script/update/ms 0.02, far/select/ms 0.02 |
| warm-up | 3.50 | 807.2 | main/busy/ms 2708, render/thread/ms 773, render/busy/ms 610, render/acquire/ms 161, render/graph/ms 5.43, render/present/ms 2.15, stream/drive/ms 0.95, surface/advance_cpu_ms 0.72, stream/tick/ms 0.50, terrain/update/ms 0.36, bigspace/total/ms 0.34, surface/cell_upload_ms 0.28 |
| warm-up | 3.62 | 113.3 | pipelines/wait_ms 2846, render/thread/ms 784, render/busy/ms 620, render/acquire/ms 161, pipelines/compile_ms 124, pipelines/front_ms 113, main/busy/ms 53.46, terrain/update/ms 21.48, pipelines/driver_ms 11.60, render/graph/ms 9.63, render/present/ms 3.11, stream/drive/ms 1.25 |
| warm-up | 3.87 | 96.2 | pipelines/compile_ms 202, pipelines/front_ms 181, render/thread/ms 64.38, render/busy/ms 62.37, main/busy/ms 59.77, pipelines/wait_ms 39.44, pipelines/driver_ms 20.52, render/graph/ms 4.21, render/present/ms 1.59, stream/drive/ms 1.37, stream/tick/ms 1.10, terrain/update/ms 0.77 |
| warm-up | 4.02 | 58.7 | pipelines/wait_ms 770, render/thread/ms 47.91, render/busy/ms 46.55, main/busy/ms 40.78, stream/drive/ms 30.73, pipelines/compile_ms 4.00, pipelines/front_ms 3.33, render/graph/ms 2.72, render/present/ms 1.16, stream/tick/ms 0.67, pipelines/driver_ms 0.66, terrain/update/ms 0.48 |
| warm-up | 4.07 | 47.6 | pipelines/wait_ms 205, pipelines/compile_ms 188, pipelines/front_ms 177, render/thread/ms 38.20, render/busy/ms 36.20, main/busy/ms 33.08, stream/drive/ms 17.50, pipelines/driver_ms 10.65, render/graph/ms 5.35, render/present/ms 1.72, stream/spawn/ms 1.33, stream/tick/ms 1.00 |
| warm-up | 4.21 | 105.7 | render/thread/ms 65.62, render/busy/ms 64.45, main/busy/ms 58.20, stream/drive/ms 36.37, render/graph/ms 2.54, stream/tick/ms 1.58, stream/spawn/ms 1.28, render/present/ms 1.00, terrain/update/ms 0.74, terrain/select/ms 0.59, bigspace/total/ms 0.39, render/acquire/ms 0.18 |
| warm-up | 4.27 | 66.8 | pipelines/wait_ms 980, pipelines/compile_ms 219, pipelines/front_ms 209, render/thread/ms 101, render/busy/ms 69.60, main/busy/ms 60.38, stream/drive/ms 35.55, render/present/ms 31.54, pipelines/driver_ms 9.54, render/graph/ms 3.47, stream/spawn/ms 2.24, stream/tick/ms 1.23 |
| warm-up | 4.31 | 42.1 | pipelines/wait_ms 4154, pipelines/compile_ms 337, pipelines/front_ms 298, render/thread/ms 80.93, main/busy/ms 59.55, render/busy/ms 48.93, pipelines/driver_ms 38.47, stream/drive/ms 34.80, render/present/ms 31.75, render/graph/ms 7.10, stream/spawn/ms 1.77, stream/tick/ms 1.18 |
| warm-up | 4.43 | 61.7 | pipelines/wait_ms 3431, main/busy/ms 48.80, pipelines/compile_ms 47.49, pipelines/front_ms 43.05, render/thread/ms 41.79, render/busy/ms 40.27, stream/drive/ms 26.93, pipelines/driver_ms 4.44, render/graph/ms 4.02, stream/spawn/ms 1.49, render/present/ms 1.31, stream/tick/ms 1.29 |
| warm-up | 4.54 | 61.5 | pipelines/wait_ms 4905, pipelines/compile_ms 150, pipelines/front_ms 139, main/busy/ms 63.73, stream/drive/ms 53.40, render/thread/ms 23.28, render/busy/ms 22.04, pipelines/driver_ms 11.53, render/graph/ms 2.97, render/present/ms 1.05, stream/tick/ms 0.84, stream/spawn/ms 0.63 |
| warm-up | 4.60 | 53.9 | main/busy/ms 50.57, stream/drive/ms 36.49, render/thread/ms 18.54, render/busy/ms 17.26, render/graph/ms 2.91, stream/spawn/ms 2.08, render/present/ms 1.13, terrain/update/ms 0.68, stream/tick/ms 0.63, terrain/select/ms 0.50, bigspace/total/ms 0.35, stream/activate/ms 0.34 |
| warm-up | 4.71 | 29.4 | pipelines/wait_ms 4008, pipelines/compile_ms 46.66, pipelines/front_ms 43.35, main/busy/ms 25.56, render/thread/ms 24.20, render/busy/ms 22.20, stream/drive/ms 11.41, render/graph/ms 5.42, pipelines/driver_ms 3.31, render/present/ms 1.74, stream/spawn/ms 1.28, stream/tick/ms 0.91 |
| warm-up | 4.80 | 29.5 | main/busy/ms 31.93, render/thread/ms 30.57, render/busy/ms 28.46, stream/drive/ms 10.97, render/graph/ms 7.05, stream/spawn/ms 3.36, render/present/ms 1.85, stream/tick/ms 1.05, terrain/update/ms 0.66, terrain/select/ms 0.52, bigspace/total/ms 0.39, stream/activate/ms 0.33 |
| warm-up | 5.03 | 35.9 | render/thread/ms 52.21, render/busy/ms 50.82, main/busy/ms 23.58, stream/drive/ms 4.98, render/graph/ms 4.92, stream/spawn/ms 2.52, stream/tick/ms 1.27, render/present/ms 1.16, bigspace/total/ms 0.74, terrain/update/ms 0.65, terrain/select/ms 0.51, bigspace/high_precision/ms 0.38 |
| warm-up | 5.09 | 29.2 | pipelines/wait_ms 1230, pipelines/compile_ms 231, pipelines/front_ms 220, render/thread/ms 43.96, render/busy/ms 42.61, main/busy/ms 21.67, pipelines/driver_ms 11.60, render/graph/ms 5.51, stream/drive/ms 4.19, stream/spawn/ms 2.04, render/present/ms 1.11, stream/tick/ms 0.78 |
| warm-up | 5.14 | 36.6 | render/thread/ms 46.44, render/busy/ms 44.96, main/busy/ms 22.90, stream/drive/ms 6.03, render/graph/ms 5.76, stream/spawn/ms 2.08, render/present/ms 1.25, stream/tick/ms 0.79, bigspace/total/ms 0.67, terrain/update/ms 0.58, terrain/select/ms 0.46, bigspace/high_precision/ms 0.40 |
| warm-up | 5.20 | 34.6 | pipelines/wait_ms 10.5 k, pipelines/compile_ms 132, pipelines/front_ms 122, render/thread/ms 49.66, render/busy/ms 47.71, main/busy/ms 23.10, pipelines/driver_ms 10.05, render/graph/ms 6.08, stream/drive/ms 3.98, stream/spawn/ms 2.31, render/present/ms 1.65, stream/tick/ms 0.83 |
| warm-up | 5.23 | 27.4 | pipelines/wait_ms 10.4 k, pipelines/compile_ms 65.55, pipelines/front_ms 58.74, render/thread/ms 42.20, render/busy/ms 40.19, main/busy/ms 31.67, stream/drive/ms 14.18, pipelines/driver_ms 6.81, render/graph/ms 6.00, stream/spawn/ms 2.32, render/present/ms 1.82, stream/tick/ms 0.84 |
| warm-up | 5.27 | 39.4 | pipelines/wait_ms 1333, render/thread/ms 42.79, render/busy/ms 41.47, main/busy/ms 32.87, pipelines/compile_ms 14.48, stream/drive/ms 13.66, pipelines/front_ms 12.72, render/graph/ms 4.68, stream/spawn/ms 2.15, pipelines/driver_ms 1.76, render/present/ms 1.17, bigspace/total/ms 0.73 |
| warm-up | 5.66 | 29.9 | render/thread/ms 32.87, render/busy/ms 30.99, main/busy/ms 27.30, render/graph/ms 5.92, stream/drive/ms 3.62, stream/spawn/ms 1.72, render/present/ms 1.60, stream/tick/ms 1.20, bigspace/total/ms 0.84, terrain/update/ms 0.70, terrain/select/ms 0.54, bigspace/high_precision/ms 0.44 |
| warm-up | 6.00 | 28.0 | main/busy/ms 43.19, render/thread/ms 38.05, render/busy/ms 36.56, render/graph/ms 6.85, render/present/ms 1.20, stream/drive/ms 1.20, bigspace/total/ms 1.01, anim/evaluate_cpu/ms 0.64, stream/tick/ms 0.59, bigspace/low_precision/ms 0.53, terrain/update/ms 0.52, terrain/select/ms 0.41 |
| warm-up | 6.04 | 36.5 | main/busy/ms 49.27, render/thread/ms 44.96, render/busy/ms 43.53, render/graph/ms 6.48, stream/drive/ms 1.26, render/present/ms 1.13, anim/evaluate_cpu/ms 0.66, bigspace/total/ms 0.61, stream/tick/ms 0.56, terrain/update/ms 0.46, terrain/select/ms 0.36, stream/spawn/ms 0.36 |
| warm-up | 6.07 | 34.7 | render/thread/ms 49.48, render/busy/ms 48.23, main/busy/ms 45.06, render/graph/ms 5.48, stream/drive/ms 1.43, bigspace/total/ms 1.07, render/present/ms 0.98, stream/tick/ms 0.64, terrain/update/ms 0.48, stream/spawn/ms 0.39, terrain/select/ms 0.37, anim/evaluate/ms 0.34 |
| warm-up | 8.12 | 39.0 | pipelines/wait_ms 5958, pipelines/compile_ms 115, pipelines/front_ms 111, render/thread/ms 46.65, render/busy/ms 45.35, main/busy/ms 42.07, render/graph/ms 37.33, pipelines/driver_ms 3.63, render/present/ms 1.13, bigspace/total/ms 0.64, stream/drive/ms 0.49, stream/tick/ms 0.43 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\h1_old\balmora_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 4% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 95%) · 2580 MHz (min 2580) · memory 8001 MHz · 90.5 W · 74 °C (max 77) · VRAM 5094 MiB · throttled 100% of samples
