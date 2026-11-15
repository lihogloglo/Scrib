# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1462 frames over 15.0 s · **median 10.20 ms (98 fps)** · mean 10.26 · p95 11.36 · p99 12.13 (1% low 82 fps) · worst 24.73 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 12.0/min)
- warm-up: 297 frames over 7.6 s, worst 2518.5 ms
- hitches (> 25.5 ms): **0** measured, 29 in warm-up
- timed GPU passes: 8.41 ms of a 10.26 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.20 ms, limit 11 ms
- pass worst frame 24.73 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.555 | 2.567 | 2.981 | 1.00 | 0.053 |
| dlss_super_resolution | 2.158 | 2.168 | 2.464 | 1.00 | 0.190 |
| early prepass | 0.609 | 0.612 | 0.956 | 1.00 | 0.094 |
| atmosphere_luts | 0.352 | 0.353 | 0.480 | 1.00 | 0.003 |
| bloom | 0.339 | 0.340 | 0.667 | 1.00 | 0.064 |
| main_transmissive_pass_3d | 0.278 | 0.279 | 0.299 | 1.00 | 0.008 |
| water_velocity | 0.264 | 0.265 | 0.439 | 1.00 | 0.002 |
| water_sspr | 0.241 | 0.242 | 0.340 | 1.00 | 0.002 |
| ssao | 0.207 | 0.208 | 0.216 | 1.00 | 0.008 |
| uniform_allocation | 0.181 | 0.181 | 0.183 | 1.00 | 0.030 |
| atmos_froxel_inject | 0.140 | 0.140 | 0.147 | 1.00 | 0.001 |
| water_fft | 0.138 | 0.139 | 0.138 | 1.00 | 0.003 |
| bin_unpacking | 0.135 | 0.135 | 0.144 | 1.00 | 0.023 |
| upscaling | 0.113 | 0.113 | 0.146 | 1.00 | 0.004 |
| clouds_composite | 0.109 | 0.110 | 0.108 | 1.00 | 0.002 |
| tonemapping | 0.094 | 0.095 | 0.099 | 1.00 | 0.003 |
| clustering | 0.072 | 0.072 | 0.085 | 1.00 | 0.022 |
| render_sky | 0.056 | 0.056 | 0.056 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.054 | 0.433 | 0.434 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.040 | 0.040 | 0.042 | 1.00 | 0.006 |
| water_snapshot | 0.038 | 0.038 | 0.052 | 1.00 | 0.002 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.032 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.029 | 0.236 | 0.266 | 0.12 | 0.004 |
| clouds_trace | 0.028 | 0.028 | 0.034 | 1.00 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.022 | 0.023 | 0.024 | 1.00 | 0.074 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.029 | 1.00 | 0.007 |
| atmosphere_environment | 0.011 | 0.011 | 0.014 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.009 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.016 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 44.999 | 0.000 | 6806.30 | 65788.7 |
| render/thread/ms | 9.324 | 10.378 | 18.31 | 13631.1 |
| render/busy/ms | 7.614 | 8.925 | 17.68 | 11132.2 |
| main/busy/ms | 7.084 | 8.187 | 14.22 | 10356.9 |
| render/graph/ms | 4.455 | 5.507 | 7.48 | 6513.5 |
| render/acquire/ms | 1.227 | 2.464 | 3.82 | 1794.1 |
| pipelines/compile_ms | 1.140 | 0.000 | 120.05 | 1666.9 |
| pipelines/front_ms | 1.103 | 0.000 | 117.28 | 1612.0 |
| render/present/ms | 0.482 | 0.576 | 3.03 | 704.8 |
| bigspace/total/ms | 0.360 | 0.684 | 1.88 | 526.9 |
| terrain/update/ms | 0.229 | 0.298 | 0.70 | 334.8 |
| terrain/select/ms | 0.199 | 0.259 | 0.66 | 291.2 |
| stream/drive/ms | 0.181 | 0.258 | 0.59 | 264.6 |
| far/select/ms | 0.164 | 0.236 | 0.50 | 240.0 |
| controller/feed/ms | 0.156 | 0.206 | 0.47 | 227.4 |
| stream/tick/ms | 0.155 | 0.226 | 0.55 | 226.0 |
| controller/move/ms | 0.149 | 0.291 | 0.59 | 217.7 |
| bigspace/high_precision/ms | 0.121 | 0.277 | 1.61 | 176.3 |
| bigspace/low_precision/ms | 0.093 | 0.211 | 1.31 | 135.3 |
| anim/evaluate/ms | 0.088 | 0.205 | 1.05 | 128.5 |
| anim/evaluate_cpu/ms | 0.060 | 0.075 | 0.27 | 87.5 |
| pipelines/driver_ms | 0.038 | 0.000 | 4.28 | 54.9 |
| physics/step/ms | 0.033 | 0.063 | 0.83 | 47.6 |
| weather/map/ms | 0.024 | 0.034 | 0.30 | 35.0 |
| weather/eval/ms | 0.011 | 0.015 | 0.09 | 15.6 |
| physics/insert/ms | 0.008 | 0.028 | 0.54 | 11.0 |
| script/update/ms | 0.006 | 0.009 | 0.06 | 9.5 |
| controller/avoid/ms | 0.006 | 0.008 | 0.10 | 8.8 |
| script/input/ms | 0.005 | 0.006 | 0.10 | 7.0 |
| anim/apply/ms | 0.004 | 0.006 | 0.04 | 6.4 |
| script/ui/ms | 0.004 | 0.006 | 0.02 | 6.2 |
| controller/write/ms | 0.002 | 0.002 | 0.03 | 2.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.08 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 22 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1080 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 16.5 k | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.0 k | 47.0 k | 47.0 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.06 | 17.68 | 8.70 |
| gpu/timed/ms | ms | 9.05 | 10.91 | 8.70 |
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
| phases/opaque3d/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 31 | 36 | 31 |
| phases/shadow/bins | count | 2029 | 2336 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5672 | 5345 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 36 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 696 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 177 | 177 | 177 |
| pipelines/waiting | count | 0 | 21 | 0 |
| preprocess/bind_groups | count | 0 | 199 | 0 |
| preprocess/bind_groups_created | count | 0 | 199 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 26.44 | 42.79 | 12.50 |
| process/mem_usage | GiB | 4.33 | 4.38 | 4.37 |
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
| started_load_count |  | 3547 | 3549 | 3549 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
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
| system/cpu_usage | % | 33.02 | 52.72 | 28.02 |
| system/mem_usage | % | 56.19 | 56.38 | 56.35 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 94 | 89 |
| views/all/bins | count | 5199 | 5506 | 5198 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 15.0 k | 14.6 k |
| views/all/pipelines | count | 85 | 90 | 85 |
| views/all/pipelines_distinct | count | 75 | 79 | 75 |
| views/all/work_items | count | 14.8 k | 15.1 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 36 | 31 |
| views/cascade/bins | count | 2029 | 2336 | 2028 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5672 | 5345 |
| views/cascade/pipelines | count | 31 | 36 | 31 |
| views/cascade/work_items | count | 5346 | 5672 | 5345 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 62 | 58 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 58 | 54 |
| views/main/work_items | count | 9462 | 9462 | 9462 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.6 k | 24.6 k | 24.6 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 2 | 142 |
| assets/Image/added | count | 0.03 | 22 | 42 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.12 | 109 | 169 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.03 | 24 | 43 |
| assets/StandardMaterial/modified | count | 3.39 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.10 | 1130 | 1615 |
| physics/inserts | count | 0.48 | 177 | 697 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.62 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 8 | 12 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 2 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.5 ms; slowest 29, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.52 | 2518.5 | main/busy/ms 2493, surface/advance_cpu_ms 70.06, surface/cell_upload_ms 1.09, stream/drive/ms 0.33, weather/map/ms 0.16, bigspace/total/ms 0.10, stream/tick/ms 0.08, bigspace/high_precision/ms 0.06, script/update/ms 0.03, weather/eval/ms 0.03, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.22 | 705.1 | main/busy/ms 2529, render/thread/ms 673, render/busy/ms 554, render/acquire/ms 117, surface/advance_cpu_ms 70.06, render/graph/ms 5.22, render/present/ms 1.97, surface/cell_upload_ms 1.09, stream/drive/ms 0.89, stream/tick/ms 0.46, terrain/update/ms 0.32, weather/map/ms 0.25 |
| warm-up | 3.47 | 250.1 | pipelines/wait_ms 1816, render/thread/ms 685, render/busy/ms 565, pipelines/compile_ms 262, pipelines/front_ms 242, render/acquire/ms 117, main/busy/ms 53.06, pipelines/driver_ms 19.43, terrain/update/ms 18.86, render/graph/ms 9.29, render/present/ms 3.10, stream/drive/ms 1.11 |
| warm-up | 3.72 | 98.5 | pipelines/compile_ms 232, pipelines/front_ms 216, main/busy/ms 82.54, render/thread/ms 57.94, render/busy/ms 55.75, pipelines/driver_ms 16.27, render/graph/ms 4.27, render/present/ms 1.74, stream/drive/ms 1.71, stream/tick/ms 1.42, terrain/update/ms 0.71, terrain/select/ms 0.54 |
| warm-up | 3.84 | 28.0 | main/busy/ms 41.39, render/thread/ms 40.72, render/busy/ms 39.51, stream/drive/ms 31.48, terrain/update/ms 5.15, render/graph/ms 3.10, render/present/ms 0.93, stream/tick/ms 0.61, terrain/select/ms 0.40, render/acquire/ms 0.28, bigspace/total/ms 0.15, stream/spawn/ms 0.10 |
| warm-up | 3.88 | 38.7 | render/thread/ms 47.24, render/busy/ms 46.14, main/busy/ms 41.78, stream/drive/ms 31.45, render/graph/ms 2.51, render/present/ms 0.85, stream/tick/ms 0.57, terrain/update/ms 0.51, terrain/select/ms 0.41, render/acquire/ms 0.25, bigspace/total/ms 0.18, stream/spawn/ms 0.09 |
| warm-up | 3.93 | 50.5 | pipelines/wait_ms 2306, pipelines/compile_ms 193, pipelines/front_ms 180, main/busy/ms 32.87, render/thread/ms 29.33, render/busy/ms 27.93, stream/drive/ms 22.76, pipelines/driver_ms 13.19, render/graph/ms 2.66, render/present/ms 1.24, stream/tick/ms 0.64, terrain/update/ms 0.55 |
| warm-up | 3.99 | 51.3 | pipelines/wait_ms 2466, pipelines/compile_ms 83.50, pipelines/front_ms 78.19, main/busy/ms 45.20, stream/drive/ms 32.69, render/thread/ms 21.34, render/busy/ms 20.05, pipelines/driver_ms 5.31, render/graph/ms 4.02, render/present/ms 1.07, stream/spawn/ms 1.03, stream/tick/ms 0.58 |
| warm-up | 4.06 | 55.6 | pipelines/wait_ms 2322, pipelines/compile_ms 43.79, render/thread/ms 43.43, pipelines/front_ms 41.63, render/busy/ms 41.32, main/busy/ms 35.54, stream/drive/ms 19.93, render/graph/ms 6.33, stream/spawn/ms 2.90, pipelines/driver_ms 2.16, render/present/ms 1.80, stream/tick/ms 1.27 |
| warm-up | 4.18 | 98.9 | render/thread/ms 69.52, main/busy/ms 44.33, render/busy/ms 43.72, stream/drive/ms 31.48, render/present/ms 25.51, render/graph/ms 4.32, stream/tick/ms 1.26, terrain/update/ms 0.69, stream/activate/ms 0.57, terrain/select/ms 0.56, bigspace/total/ms 0.37, render/acquire/ms 0.29 |
| warm-up | 4.29 | 45.0 | render/thread/ms 37.88, render/busy/ms 35.74, main/busy/ms 26.69, render/graph/ms 5.73, stream/drive/ms 5.45, terrain/update/ms 5.03, stream/tick/ms 1.53, render/present/ms 1.51, render/acquire/ms 0.63, terrain/select/ms 0.61, stream/spawn/ms 0.54, bigspace/total/ms 0.39 |
| warm-up | 4.38 | 58.9 | render/thread/ms 47.21, main/busy/ms 46.38, render/busy/ms 45.49, render/graph/ms 5.44, stream/drive/ms 3.84, terrain/update/ms 2.43, stream/tick/ms 1.67, render/present/ms 1.44, stream/spawn/ms 1.02, terrain/select/ms 0.53, anim/evaluate_cpu/ms 0.42, stream/activate/ms 0.39 |
| warm-up | 4.47 | 72.5 | pipelines/wait_ms 2870, pipelines/compile_ms 231, pipelines/front_ms 218, main/busy/ms 75.57, stream/drive/ms 56.05, render/thread/ms 24.93, render/busy/ms 23.87, pipelines/driver_ms 13.23, render/graph/ms 4.18, stream/spawn/ms 2.08, stream/tick/ms 1.02, render/present/ms 0.89 |
| warm-up | 4.54 | 57.2 | main/busy/ms 49.78, stream/drive/ms 34.49, render/thread/ms 26.64, render/busy/ms 25.48, render/graph/ms 4.21, stream/spawn/ms 1.46, render/present/ms 0.98, stream/activate/ms 0.66, stream/tick/ms 0.62, terrain/update/ms 0.50, terrain/select/ms 0.42, bigspace/total/ms 0.40 |
| warm-up | 4.81 | 34.7 | pipelines/wait_ms 4542, pipelines/compile_ms 45.20, render/thread/ms 43.39, pipelines/front_ms 42.73, render/busy/ms 42.07, main/busy/ms 23.96, render/graph/ms 5.97, stream/drive/ms 4.77, pipelines/driver_ms 2.47, stream/spawn/ms 2.34, render/present/ms 1.11, stream/tick/ms 0.88 |
| warm-up | 4.84 | 26.8 | pipelines/wait_ms 3872, render/thread/ms 45.27, render/busy/ms 44.26, main/busy/ms 20.47, pipelines/compile_ms 17.28, pipelines/front_ms 16.32, render/graph/ms 4.13, stream/drive/ms 3.68, stream/spawn/ms 2.17, pipelines/driver_ms 0.97, render/present/ms 0.84, bigspace/total/ms 0.68 |
| warm-up | 4.92 | 29.4 | render/thread/ms 36.84, render/busy/ms 35.81, main/busy/ms 15.88, render/graph/ms 4.36, stream/drive/ms 3.41, stream/spawn/ms 1.31, render/present/ms 0.85, bigspace/total/ms 0.82, stream/tick/ms 0.55, terrain/update/ms 0.46, terrain/select/ms 0.39, bigspace/high_precision/ms 0.24 |
| warm-up | 4.96 | 33.2 | pipelines/wait_ms 714, pipelines/compile_ms 136, pipelines/front_ms 130, render/thread/ms 40.90, render/busy/ms 39.10, main/busy/ms 20.00, render/graph/ms 6.81, pipelines/driver_ms 5.83, stream/drive/ms 3.46, stream/spawn/ms 2.09, render/present/ms 1.39, bigspace/total/ms 0.90 |
| warm-up | 5.02 | 36.1 | pipelines/wait_ms 8355, pipelines/compile_ms 86.43, pipelines/front_ms 82.57, render/thread/ms 41.61, render/busy/ms 40.34, main/busy/ms 20.88, render/graph/ms 4.74, stream/drive/ms 3.87, pipelines/driver_ms 3.86, stream/spawn/ms 2.24, render/present/ms 1.05, stream/tick/ms 0.73 |
| warm-up | 5.04 | 25.7 | pipelines/wait_ms 8355, pipelines/compile_ms 86.43, pipelines/front_ms 82.57, render/thread/ms 47.67, render/busy/ms 46.58, main/busy/ms 23.61, stream/drive/ms 4.39, render/graph/ms 4.10, pipelines/driver_ms 3.86, stream/spawn/ms 2.23, render/present/ms 0.91, stream/tick/ms 0.72 |
| warm-up | 5.07 | 26.4 | render/thread/ms 46.80, render/busy/ms 45.87, main/busy/ms 20.18, stream/drive/ms 3.89, render/graph/ms 3.52, stream/spawn/ms 1.92, terrain/update/ms 0.79, render/present/ms 0.78, stream/tick/ms 0.69, terrain/select/ms 0.63, bigspace/total/ms 0.56, bigspace/high_precision/ms 0.34 |
| warm-up | 5.13 | 25.6 | render/thread/ms 28.37, render/busy/ms 26.77, main/busy/ms 24.58, render/graph/ms 6.00, stream/drive/ms 5.69, stream/spawn/ms 1.88, render/present/ms 1.38, stream/tick/ms 0.90, bigspace/total/ms 0.68, terrain/update/ms 0.68, terrain/select/ms 0.54, bigspace/high_precision/ms 0.33 |
| warm-up | 5.16 | 33.8 | pipelines/wait_ms 7529, pipelines/compile_ms 149, pipelines/front_ms 140, render/thread/ms 38.72, render/busy/ms 37.22, main/busy/ms 27.39, pipelines/driver_ms 9.36, render/graph/ms 5.83, stream/drive/ms 3.74, stream/spawn/ms 2.03, render/present/ms 1.19, stream/tick/ms 1.04 |
| warm-up | 5.20 | 40.1 | pipelines/wait_ms 7529, pipelines/compile_ms 149, pipelines/front_ms 140, render/thread/ms 46.62, render/busy/ms 45.07, main/busy/ms 40.17, stream/drive/ms 16.75, pipelines/driver_ms 9.36, render/graph/ms 7.42, stream/spawn/ms 2.38, render/present/ms 1.28, stream/tick/ms 1.08 |
| warm-up | 5.32 | 26.6 | main/busy/ms 29.89, render/thread/ms 20.25, render/busy/ms 18.97, stream/drive/ms 15.07, render/graph/ms 5.81, stream/spawn/ms 1.86, render/present/ms 1.11, stream/tick/ms 0.57, bigspace/total/ms 0.51, terrain/update/ms 0.46, terrain/select/ms 0.38, bigspace/high_precision/ms 0.27 |
| warm-up | 5.59 | 38.0 | pipelines/wait_ms 1730, pipelines/compile_ms 141, pipelines/front_ms 126, render/thread/ms 36.09, render/busy/ms 34.46, main/busy/ms 30.93, pipelines/driver_ms 14.65, stream/drive/ms 10.92, render/graph/ms 5.78, stream/spawn/ms 1.73, render/present/ms 1.33, stream/tick/ms 1.05 |
| warm-up | 5.91 | 29.4 | pipelines/wait_ms 1767, main/busy/ms 40.32, render/thread/ms 29.39, render/busy/ms 27.64, pipelines/compile_ms 19.16, pipelines/front_ms 18.18, render/graph/ms 9.48, terrain/update/ms 2.79, render/present/ms 1.49, stream/drive/ms 1.26, pipelines/driver_ms 0.98, bigspace/total/ms 0.88 |
| warm-up | 5.95 | 38.6 | render/thread/ms 41.85, main/busy/ms 40.42, render/busy/ms 40.35, render/graph/ms 7.65, render/present/ms 1.23, bigspace/total/ms 1.19, stream/drive/ms 1.06, terrain/update/ms 0.62, bigspace/low_precision/ms 0.56, terrain/select/ms 0.52, stream/tick/ms 0.52, anim/evaluate_cpu/ms 0.31 |
| warm-up | 6.79 | 32.8 | pipelines/wait_ms 3521, pipelines/compile_ms 37.27, pipelines/front_ms 35.11, render/thread/ms 31.26, render/busy/ms 29.53, main/busy/ms 29.29, render/graph/ms 8.05, pipelines/driver_ms 2.16, render/present/ms 1.58, stream/drive/ms 0.93, physics/insert/ms 0.65, bigspace/total/ms 0.62 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_clouds\balmora@2560x1440_off=normalmaps_orm_parallax_clouds-1 --bench-baseline none --off normalmaps,orm,parallax,clouds
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 16%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **98%** (min 87%) · 2580 MHz (min 2580) · memory 8001 MHz · 95.0 W · 79 °C (max 81) · VRAM 5737 MiB · throttled 100% of samples
