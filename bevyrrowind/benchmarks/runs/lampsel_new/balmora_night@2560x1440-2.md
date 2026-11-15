# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 143 frames over 15.1 s · **median 106.73 ms (9 fps)** · mean 105.58 · p95 113.22 · p99 116.33 (1% low 9 fps) · worst 128.62 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 315 frames over 10.2 s, worst 2567.6 ms
- hitches (> 266.8 ms): **0** measured, 2 in warm-up
- timed GPU passes: 89.45 ms of a 105.58 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 106.73 ms, limit 11 ms
- **FAIL** worst frame 128.62 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 28.324 | 28.524 | 32.497 | 0.99 | 0.064 |
| early prepass | 18.019 | 18.145 | 31.688 | 0.99 | 0.098 |
| tonemapping | 13.181 | 13.273 | 18.924 | 0.99 | 0.003 |
| dlss_super_resolution | 8.003 | 8.059 | 18.578 | 0.99 | 0.193 |
| upscaling | 5.779 | 5.820 | 12.294 | 0.99 | 0.004 |
| bin_unpacking | 3.147 | 3.170 | 4.069 | 0.99 | 0.021 |
| water_snapshot | 1.455 | 1.465 | 1.701 | 0.99 | 0.001 |
| water_sspr | 1.454 | 1.464 | 1.973 | 0.99 | 0.003 |
| bloom | 1.345 | 1.354 | 4.161 | 0.99 | 0.063 |
| clouds_composite | 1.339 | 1.349 | 1.615 | 0.99 | 0.002 |
| atmosphere_luts | 1.106 | 1.114 | 1.406 | 0.99 | 0.003 |
| uniform_allocation | 0.976 | 0.983 | 1.371 | 0.99 | 0.031 |
| early_mesh_preprocessing | 0.970 | 0.977 | 1.733 | 0.99 | 0.004 |
| water_velocity | 0.776 | 0.782 | 1.143 | 0.99 | 0.001 |
| render_sky | 0.709 | 0.714 | 0.823 | 0.99 | 0.001 |
| ssao | 0.623 | 0.627 | 1.401 | 0.99 | 0.007 |
| main_transmissive_pass_3d | 0.557 | 0.561 | 1.079 | 0.99 | 0.008 |
| water_fft | 0.386 | 0.389 | 0.966 | 0.99 | 0.002 |
| water_mips | 0.241 | 0.243 | 0.849 | 0.99 | 0.002 |
| clouds_trace | 0.238 | 0.240 | 0.257 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.211 | 0.212 | 0.696 | 0.99 | 0.002 |
| clustering | 0.171 | 0.172 | 0.753 | 0.99 | 0.022 |
| lightprobe_irradiance_map | 0.132 | 1.050 | 1.180 | 0.13 | 0.001 |
| night_sky | 0.058 | 0.058 | 0.067 | 0.99 | 0.001 |
| clouds_resolve | 0.053 | 0.053 | 0.056 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.053 | 0.421 | 0.954 | 0.13 | 0.003 |
| main_transparent_pass_3d | 0.039 | 0.039 | 0.043 | 0.99 | 0.071 |
| atmos_froxel_integrate | 0.034 | 0.034 | 0.032 | 0.99 | 0.001 |
| atmosphere_environment | 0.019 | 0.019 | 0.014 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.018 | 0.018 | 0.023 | 0.99 | 0.005 |
| clouds_fog_light | 0.013 | 0.013 | 0.013 | 0.99 | 0.002 |
| sparse buffer updates | 0.010 | 0.010 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.005 | 0.005 | 0.006 | 0.99 | 0.003 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.017 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.05 | 0.009 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 1153.776 | 8243.937 | 26767.50 | 164990.0 |
| render/thread/ms | 104.281 | 112.049 | 127.44 | 14912.2 |
| render/acquire/ms | 93.872 | 101.254 | 106.30 | 13423.7 |
| pipelines/compile_ms | 13.773 | 136.557 | 183.74 | 1969.6 |
| pipelines/front_ms | 13.332 | 134.024 | 177.79 | 1906.4 |
| render/busy/ms | 9.687 | 11.731 | 24.17 | 1385.2 |
| main/busy/ms | 8.452 | 12.024 | 39.88 | 1208.7 |
| render/graph/ms | 5.561 | 6.352 | 16.47 | 795.2 |
| render/present/ms | 0.723 | 0.703 | 19.68 | 103.4 |
| pipelines/driver_ms | 0.442 | 4.342 | 6.49 | 63.2 |
| bigspace/total/ms | 0.335 | 0.513 | 0.80 | 47.9 |
| terrain/update/ms | 0.240 | 0.302 | 0.51 | 34.3 |
| stream/drive/ms | 0.213 | 0.354 | 0.59 | 30.5 |
| terrain/select/ms | 0.207 | 0.259 | 0.36 | 29.6 |
| stream/tick/ms | 0.184 | 0.313 | 0.55 | 26.3 |
| far/select/ms | 0.182 | 0.282 | 0.39 | 26.1 |
| controller/feed/ms | 0.175 | 0.253 | 0.34 | 25.0 |
| physics/step/ms | 0.144 | 0.394 | 0.96 | 20.6 |
| controller/move/ms | 0.118 | 0.226 | 0.33 | 16.8 |
| bigspace/high_precision/ms | 0.099 | 0.124 | 0.46 | 14.1 |
| bigspace/low_precision/ms | 0.081 | 0.118 | 0.43 | 11.6 |
| anim/evaluate_cpu/ms | 0.078 | 0.137 | 0.70 | 11.1 |
| anim/evaluate/ms | 0.074 | 0.117 | 0.28 | 10.6 |
| physics/insert/ms | 0.065 | 0.515 | 0.74 | 9.2 |
| weather/map/ms | 0.017 | 0.023 | 0.03 | 2.5 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 1.6 |
| script/update/ms | 0.009 | 0.014 | 0.07 | 1.4 |
| controller/avoid/ms | 0.006 | 0.009 | 0.01 | 0.9 |
| script/input/ms | 0.006 | 0.009 | 0.01 | 0.9 |
| anim/apply/ms | 0.006 | 0.010 | 0.04 | 0.8 |
| script/ui/ms | 0.005 | 0.008 | 0.01 | 0.7 |
| controller/write/ms | 0.002 | 0.003 | 0.00 | 0.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 0.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.1 |
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
| anim/a2/count | count | 0 | 38 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 131 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 43 | 8 |
| crowd/c0/joints/count | count | 272 | 1462 | 272 |
| crowd/c0/meshes/count | count | 48 | 258 | 48 |
| crowd/c1/count | count | 123 | 131 | 123 |
| crowd/c1/joints/count | count | 4880 | 5183 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1227 | 1088 |
| crowd/c2/count | count | 283 | 283 | 283 |
| crowd/c2/joints/count | count | 4916 | 4916 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1373 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 8218 | 2958 |
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
| frame/work/ms | ms | 94.27 | 99.09 | 92.82 |
| gpu/timed/ms | ms | 94.27 | 99.09 | 92.82 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1851 | 1851 | 1851 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 196.0 k | 196.0 k | 196.0 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 529.49 M | 529.49 M | 529.49 M |
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
| phases/opaque3d/batch_sets | count | 28 | 30 | 28 |
| phases/opaque3d/bins | count | 1303 | 1304 | 1303 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1303 | 1304 | 1303 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 24 | 24 | 24 |
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
| physics/pending | count | 0 | 636 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 197 | 197 | 197 |
| pipelines/waiting | count | 0 | 33 | 0 |
| preprocess/bind_groups | count | 0 | 202 | 0 |
| preprocess/bind_groups_created | count | 0 | 202 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 210 | 210 | 210 |
| process/cpu_usage | % | 10.79 | 23.32 | 8.52 |
| process/mem_usage | GiB | 5.00 | 5.07 | 5.06 |
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
| stream/latency/s | s | 0.09 | 0.09 | 0.09 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.66 | 69.85 | 26.28 |
| system/mem_usage | % | 72.81 | 72.91 | 72.91 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2478 | 2478 | 2478 |
| textures/images | count | 1917 | 1917 | 1917 |
| views/all/batch_sets | count | 94 | 98 | 94 |
| views/all/bins | count | 4151 | 4153 | 4151 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 90 | 94 | 90 |
| views/all/pipelines_distinct | count | 89 | 93 | 89 |
| views/all/work_items | count | 11.5 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 24 | 24 | 24 |
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
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 3166 | 3168 | 3166 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9300 | 9300 | 9300 |
| views/main/pipelines | count | 66 | 70 | 66 |
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
| anim/bones_written/count | count | 9.92 | 825 | 1419 |
| anim/events/count | count | 1 | 12 | 143 |
| assets/Image/added | count | 1.03 | 72 | 147 |
| assets/Image/modified | count | 0.03 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 2.69 | 191 | 384 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 1.15 | 80 | 164 |
| assets/StandardMaterial/modified | count | 11 | 11 | 1573 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 88.36 | 6335 | 12.6 k |
| physics/inserts | count | 4.65 | 130 | 665 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 3 | 3 | 429 |
| physics/steps_dropped | count | 3.32 | 5 | 475 |
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

## Hitches (> 266.8 ms; slowest 2, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2567.6 | main/busy/ms 2541, surface/advance_cpu_ms 0.76, surface/cell_upload_ms 0.35, stream/drive/ms 0.33, bigspace/total/ms 0.12, stream/tick/ms 0.08, bigspace/high_precision/ms 0.07, weather/eval/ms 0.04, weather/map/ms 0.04, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.39 | 825.6 | main/busy/ms 2575, render/thread/ms 784, render/busy/ms 662, render/acquire/ms 120, render/graph/ms 5.15, render/present/ms 2.45, stream/drive/ms 0.88, surface/advance_cpu_ms 0.76, terrain/update/ms 0.58, stream/tick/ms 0.45, terrain/select/ms 0.37, surface/cell_upload_ms 0.35 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_new.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lampsel_new\balmora_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 4%, CPU 20%
- during the run: other processes 24% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **99%** (min 81%) · 2580 MHz (min 2580) · memory 8001 MHz · 51.6 W · 68 °C (max 70) · VRAM 7773 MiB · throttled 100% of samples
