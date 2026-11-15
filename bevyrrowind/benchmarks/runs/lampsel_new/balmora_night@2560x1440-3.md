# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1442 frames over 15.0 s · **median 10.28 ms (97 fps)** · mean 10.40 · p95 11.86 · p99 13.60 (1% low 74 fps) · worst 32.80 ms · 7 spikes over 1.5× median · 20.0 stutters/min (work 16.0/min)
- warm-up: 448 frames over 9.2 s, worst 2471.3 ms
- hitches (> 25.7 ms): **2** measured, 31 in warm-up
- timed GPU passes: 8.49 ms of a 10.40 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.28 ms, limit 11 ms
- pass worst frame 32.80 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.654 | 2.742 | 3.123 | 0.97 | 0.067 |
| dlss_super_resolution | 2.092 | 2.161 | 2.521 | 0.97 | 0.191 |
| early prepass | 0.624 | 0.644 | 0.974 | 0.97 | 0.105 |
| atmosphere_luts | 0.347 | 0.358 | 0.651 | 0.97 | 0.004 |
| bloom | 0.331 | 0.342 | 0.510 | 0.97 | 0.065 |
| main_transmissive_pass_3d | 0.280 | 0.289 | 0.321 | 0.97 | 0.009 |
| water_velocity | 0.277 | 0.286 | 0.446 | 0.97 | 0.002 |
| water_sspr | 0.229 | 0.236 | 0.342 | 0.97 | 0.003 |
| ssao | 0.198 | 0.205 | 0.210 | 0.97 | 0.008 |
| uniform_allocation | 0.183 | 0.189 | 0.193 | 0.97 | 0.033 |
| water_fft | 0.142 | 0.147 | 0.140 | 0.97 | 0.003 |
| bin_unpacking | 0.141 | 0.146 | 0.153 | 0.97 | 0.024 |
| atmos_froxel_inject | 0.134 | 0.138 | 0.136 | 0.97 | 0.002 |
| upscaling | 0.127 | 0.132 | 0.163 | 0.97 | 0.004 |
| clouds_composite | 0.107 | 0.110 | 0.109 | 0.97 | 0.002 |
| tonemapping | 0.097 | 0.100 | 0.105 | 0.97 | 0.004 |
| clouds_trace | 0.068 | 0.070 | 0.081 | 0.97 | 0.001 |
| clustering | 0.067 | 0.069 | 0.076 | 0.97 | 0.022 |
| render_sky | 0.054 | 0.056 | 0.056 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.437 | 0.749 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.051 | 0.052 | 0.97 | 0.001 |
| water_snapshot | 0.040 | 0.041 | 0.056 | 0.97 | 0.002 |
| early_mesh_preprocessing | 0.033 | 0.034 | 0.035 | 0.97 | 0.006 |
| atmos_froxel_integrate | 0.030 | 0.031 | 0.033 | 0.97 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.233 | 0.239 | 0.12 | 0.005 |
| main_transparent_pass_3d | 0.024 | 0.025 | 0.026 | 0.97 | 0.071 |
| water_mips | 0.024 | 0.025 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.016 | 0.020 | 0.97 | 0.006 |
| atmosphere_environment | 0.010 | 0.011 | 0.013 | 0.97 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| night_sky | 0.007 | 0.007 | 0.008 | 0.97 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.97 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 0.97 | 0.004 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.97 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 79.419 | 0.000 | 11494.09 | 114521.6 |
| render/thread/ms | 9.472 | 10.902 | 32.02 | 13659.2 |
| render/busy/ms | 8.433 | 9.483 | 31.43 | 12159.8 |
| main/busy/ms | 7.286 | 8.532 | 17.59 | 10505.9 |
| render/graph/ms | 5.179 | 6.053 | 27.87 | 7468.5 |
| pipelines/compile_ms | 1.331 | 0.000 | 149.17 | 1918.8 |
| pipelines/front_ms | 1.287 | 0.000 | 143.89 | 1856.0 |
| render/present/ms | 0.522 | 0.632 | 5.41 | 752.7 |
| render/acquire/ms | 0.518 | 1.561 | 3.00 | 746.7 |
| bigspace/total/ms | 0.370 | 0.781 | 2.75 | 532.9 |
| terrain/update/ms | 0.236 | 0.306 | 0.72 | 340.6 |
| stream/drive/ms | 0.210 | 0.303 | 0.67 | 303.4 |
| terrain/select/ms | 0.206 | 0.264 | 0.58 | 296.7 |
| stream/tick/ms | 0.180 | 0.266 | 0.62 | 260.3 |
| far/select/ms | 0.178 | 0.263 | 0.51 | 256.7 |
| controller/feed/ms | 0.162 | 0.209 | 0.51 | 233.5 |
| bigspace/high_precision/ms | 0.128 | 0.278 | 2.33 | 185.2 |
| controller/move/ms | 0.109 | 0.168 | 0.32 | 157.2 |
| bigspace/low_precision/ms | 0.093 | 0.200 | 2.34 | 134.7 |
| anim/evaluate/ms | 0.076 | 0.169 | 1.53 | 109.7 |
| anim/evaluate_cpu/ms | 0.061 | 0.078 | 0.36 | 87.8 |
| pipelines/driver_ms | 0.044 | 0.000 | 5.29 | 62.8 |
| physics/step/ms | 0.034 | 0.061 | 0.90 | 49.1 |
| weather/map/ms | 0.016 | 0.022 | 0.21 | 23.2 |
| weather/eval/ms | 0.010 | 0.013 | 0.22 | 14.8 |
| script/update/ms | 0.008 | 0.011 | 0.05 | 10.9 |
| physics/insert/ms | 0.007 | 0.029 | 0.57 | 10.4 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 8.6 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 7.9 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 7.7 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.2 |
| controller/write/ms | 0.002 | 0.003 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
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
| anim/a2/count | count | 0 | 20 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 37 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 27 | 8 |
| crowd/c0/joints/count | count | 272 | 918 | 272 |
| crowd/c0/meshes/count | count | 48 | 162 | 48 |
| crowd/c1/count | count | 122 | 122 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1087 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 5265 | 2958 |
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
| frame/work/ms | ms | 9.37 | 31.43 | 9.19 |
| gpu/timed/ms | ms | 9.34 | 11.57 | 9.19 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1851 | 1851 | 1851 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 196.0 k | 196.0 k | 196.0 k |
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
| physics/pending | count | 0 | 624 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 197 | 197 | 197 |
| pipelines/waiting | count | 0 | 29 | 0 |
| preprocess/bind_groups | count | 0 | 202 | 0 |
| preprocess/bind_groups_created | count | 0 | 202 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 210 | 210 | 210 |
| process/cpu_usage | % | 33.50 | 45.45 | 37.02 |
| process/mem_usage | GiB | 4.41 | 4.49 | 4.49 |
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
| stream/latency/s | s | 0.07 | 0.07 | 0.07 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.83 | 51.70 | 51.65 |
| system/mem_usage | % | 56.61 | 57.01 | 57.01 |
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
| anim/bones_written/count | count | 0.50 | 264 | 726 |
| anim/events/count | count | 0.07 | 3 | 106 |
| assets/Image/added | count | 0.03 | 19 | 44 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.05 | 35 | 77 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.03 | 23 | 49 |
| assets/StandardMaterial/modified | count | 3.43 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 3.75 | 2361 | 5409 |
| physics/inserts | count | 0.33 | 90 | 478 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.62 | 2 | 900 |
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

## Hitches (> 25.7 ms; slowest 33, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.47 | 2471.3 | main/busy/ms 2446, surface/advance_cpu_ms 0.73, surface/cell_upload_ms 0.34, stream/drive/ms 0.26, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06, stream/tick/ms 0.06, weather/map/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.13 | 658.8 | main/busy/ms 2484, render/thread/ms 615, render/busy/ms 501, render/acquire/ms 111, render/graph/ms 4.60, render/present/ms 2.05, stream/drive/ms 0.98, surface/advance_cpu_ms 0.73, terrain/update/ms 0.58, stream/tick/ms 0.54, terrain/select/ms 0.38, surface/cell_upload_ms 0.34 |
| warm-up | 3.35 | 216.3 | pipelines/wait_ms 1813, render/thread/ms 626, render/busy/ms 512, pipelines/compile_ms 229, pipelines/front_ms 215, render/acquire/ms 112, main/busy/ms 54.61, terrain/update/ms 22.60, pipelines/driver_ms 13.94, render/graph/ms 8.37, render/present/ms 2.91, stream/drive/ms 1.15 |
| warm-up | 3.61 | 115.0 | pipelines/compile_ms 216, pipelines/front_ms 190, main/busy/ms 81.98, render/thread/ms 72.58, render/busy/ms 69.91, pipelines/wait_ms 34.07, pipelines/driver_ms 26.02, render/graph/ms 5.16, render/present/ms 1.88, stream/drive/ms 1.73, stream/tick/ms 1.44, render/acquire/ms 0.79 |
| warm-up | 3.76 | 60.8 | main/busy/ms 41.46, render/thread/ms 40.15, render/busy/ms 38.86, stream/drive/ms 29.98, render/graph/ms 2.50, render/present/ms 1.09, stream/tick/ms 0.66, terrain/update/ms 0.64, terrain/select/ms 0.51, stream/spawn/ms 0.29, bigspace/total/ms 0.26, render/acquire/ms 0.19 |
| warm-up | 3.85 | 73.3 | pipelines/wait_ms 659, pipelines/compile_ms 225, pipelines/front_ms 212, render/thread/ms 64.09, render/busy/ms 61.86, main/busy/ms 48.94, stream/drive/ms 31.81, pipelines/driver_ms 12.95, render/graph/ms 6.43, render/present/ms 1.89, stream/spawn/ms 1.70, stream/tick/ms 1.53 |
| warm-up | 3.88 | 28.0 | pipelines/wait_ms 224, pipelines/compile_ms 223, pipelines/front_ms 211, render/thread/ms 64.66, render/busy/ms 62.61, main/busy/ms 30.53, pipelines/driver_ms 12.49, render/graph/ms 5.91, stream/drive/ms 5.62, stream/spawn/ms 2.41, stream/tick/ms 2.01, render/present/ms 1.74 |
| warm-up | 3.98 | 102.0 | pipelines/wait_ms 615, render/thread/ms 88.12, render/busy/ms 62.82, main/busy/ms 31.89, render/present/ms 25.08, pipelines/compile_ms 10.67, pipelines/driver_ms 7.46, terrain/update/ms 6.84, stream/drive/ms 3.94, render/graph/ms 3.30, pipelines/front_ms 3.22, stream/spawn/ms 1.27 |
| warm-up | 4.04 | 61.2 | pipelines/wait_ms 1642, pipelines/compile_ms 229, pipelines/front_ms 208, render/thread/ms 118, render/busy/ms 92.78, main/busy/ms 29.15, render/present/ms 25.25, pipelines/driver_ms 20.94, terrain/update/ms 7.20, render/graph/ms 4.79, stream/drive/ms 3.88, stream/spawn/ms 1.69 |
| warm-up | 4.18 | 113.3 | pipelines/wait_ms 3463, pipelines/compile_ms 176, pipelines/front_ms 163, render/thread/ms 79.67, render/busy/ms 52.69, main/busy/ms 51.92, stream/drive/ms 36.80, render/present/ms 26.79, pipelines/driver_ms 13.51, render/graph/ms 4.55, stream/spawn/ms 1.11, stream/tick/ms 0.73 |
| warm-up | 4.24 | 46.5 | pipelines/wait_ms 5233, pipelines/compile_ms 132, pipelines/front_ms 82.07, pipelines/driver_ms 49.56, main/busy/ms 35.59, render/thread/ms 23.19, stream/drive/ms 21.59, render/busy/ms 20.96, render/graph/ms 6.84, stream/spawn/ms 2.06, render/present/ms 2.04, stream/activate/ms 0.89 |
| warm-up | 4.30 | 45.4 | pipelines/wait_ms 6606, pipelines/compile_ms 62.15, pipelines/front_ms 54.80, main/busy/ms 52.54, stream/drive/ms 32.08, render/thread/ms 26.44, render/busy/ms 24.85, pipelines/driver_ms 7.34, render/graph/ms 3.91, render/present/ms 1.34, stream/tick/ms 1.17, terrain/update/ms 0.61 |
| warm-up | 4.40 | 60.8 | pipelines/wait_ms 2689, pipelines/compile_ms 85.65, pipelines/front_ms 82.74, main/busy/ms 65.53, stream/drive/ms 53.38, render/thread/ms 21.10, render/busy/ms 19.82, render/graph/ms 2.99, pipelines/driver_ms 2.92, render/present/ms 1.04, stream/tick/ms 0.77, terrain/update/ms 0.66 |
| warm-up | 4.47 | 56.6 | pipelines/wait_ms 3120, main/busy/ms 48.16, pipelines/compile_ms 41.79, pipelines/front_ms 39.42, stream/drive/ms 37.20, render/thread/ms 21.55, render/busy/ms 20.21, render/graph/ms 4.09, pipelines/driver_ms 2.37, render/present/ms 1.12, stream/tick/ms 1.02, terrain/update/ms 0.57 |
| warm-up | 4.52 | 30.0 | main/busy/ms 29.84, render/thread/ms 22.23, render/busy/ms 20.75, stream/drive/ms 9.36, render/graph/ms 3.48, stream/spawn/ms 2.07, render/present/ms 1.22, terrain/update/ms 0.73, stream/tick/ms 0.60, terrain/select/ms 0.57, bigspace/total/ms 0.52, render/acquire/ms 0.25 |
| warm-up | 4.83 | 30.4 | pipelines/wait_ms 4015, render/thread/ms 39.29, render/busy/ms 37.88, pipelines/compile_ms 29.24, pipelines/front_ms 27.32, main/busy/ms 17.86, render/graph/ms 5.05, stream/drive/ms 4.37, stream/spawn/ms 2.05, pipelines/driver_ms 1.92, render/present/ms 1.21, bigspace/total/ms 0.60 |
| warm-up | 4.89 | 41.9 | pipelines/wait_ms 4104, render/thread/ms 51.16, render/busy/ms 49.59, pipelines/compile_ms 39.71, pipelines/front_ms 37.72, main/busy/ms 26.43, stream/drive/ms 4.98, render/graph/ms 4.71, stream/spawn/ms 2.87, pipelines/driver_ms 1.98, render/present/ms 1.24, stream/tick/ms 0.93 |
| warm-up | 4.95 | 31.9 | render/thread/ms 39.48, render/busy/ms 38.19, main/busy/ms 19.67, render/graph/ms 4.13, stream/drive/ms 3.62, stream/spawn/ms 2.13, render/present/ms 1.05, terrain/update/ms 0.71, stream/tick/ms 0.62, bigspace/total/ms 0.58, terrain/select/ms 0.55, bigspace/high_precision/ms 0.29 |
| warm-up | 4.97 | 27.2 | render/thread/ms 45.48, render/busy/ms 44.34, main/busy/ms 21.54, stream/drive/ms 4.25, render/graph/ms 4.06, stream/spawn/ms 2.11, render/present/ms 0.91, stream/tick/ms 0.74, bigspace/total/ms 0.70, terrain/update/ms 0.62, terrain/select/ms 0.49, bigspace/high_precision/ms 0.28 |
| warm-up | 5.02 | 34.0 | render/thread/ms 41.26, render/busy/ms 39.73, main/busy/ms 22.69, stream/drive/ms 5.36, render/graph/ms 4.84, stream/spawn/ms 2.06, render/present/ms 1.29, bigspace/total/ms 0.62, terrain/update/ms 0.57, stream/tick/ms 0.55, terrain/select/ms 0.45, bigspace/high_precision/ms 0.32 |
| warm-up | 5.05 | 27.6 | render/thread/ms 54.81, render/busy/ms 52.86, main/busy/ms 28.26, render/graph/ms 6.31, stream/drive/ms 5.87, stream/spawn/ms 2.43, render/present/ms 1.62, bigspace/total/ms 0.77, terrain/update/ms 0.65, stream/tick/ms 0.60, terrain/select/ms 0.51, bigspace/high_precision/ms 0.42 |
| warm-up | 5.08 | 34.3 | render/thread/ms 50.08, render/busy/ms 48.30, main/busy/ms 26.95, render/graph/ms 5.76, stream/drive/ms 4.12, stream/spawn/ms 2.55, render/present/ms 1.54, terrain/update/ms 0.83, bigspace/total/ms 0.75, stream/tick/ms 0.72, terrain/select/ms 0.64, bigspace/high_precision/ms 0.43 |
| warm-up | 5.12 | 40.8 | pipelines/wait_ms 1494, pipelines/compile_ms 251, pipelines/front_ms 231, render/thread/ms 51.90, render/busy/ms 49.83, main/busy/ms 42.81, pipelines/driver_ms 20.34, stream/drive/ms 17.83, render/graph/ms 7.31, stream/spawn/ms 2.25, render/present/ms 1.84, stream/tick/ms 1.30 |
| warm-up | 5.18 | 53.9 | pipelines/wait_ms 1494, pipelines/compile_ms 251, pipelines/front_ms 231, render/thread/ms 62.62, render/busy/ms 60.64, main/busy/ms 47.21, pipelines/driver_ms 20.34, stream/drive/ms 18.53, render/graph/ms 7.13, stream/spawn/ms 2.13, stream/tick/ms 1.99, render/present/ms 1.73 |
| warm-up | 5.32 | 32.2 | pipelines/wait_ms 1944, main/busy/ms 36.06, render/thread/ms 23.40, render/busy/ms 21.93, stream/drive/ms 16.80, terrain/update/ms 4.97, render/graph/ms 4.08, stream/spawn/ms 2.32, render/present/ms 1.27, bigspace/total/ms 0.68, stream/tick/ms 0.62, pipelines/compile_ms 0.52 |
| warm-up | 5.60 | 42.1 | render/thread/ms 35.35, main/busy/ms 35.15, render/busy/ms 33.52, stream/drive/ms 11.88, render/graph/ms 4.99, stream/spawn/ms 2.58, render/present/ms 1.58, stream/tick/ms 0.85, terrain/update/ms 0.83, bigspace/total/ms 0.80, terrain/select/ms 0.67, bigspace/high_precision/ms 0.31 |
| warm-up | 5.75 | 30.7 | pipelines/wait_ms 11.7 k, main/busy/ms 38.36, pipelines/compile_ms 33.92, pipelines/front_ms 31.34, render/thread/ms 23.13, render/busy/ms 21.61, render/graph/ms 6.35, pipelines/driver_ms 2.58, stream/drive/ms 1.36, render/present/ms 1.16, stream/tick/ms 0.63, physics/insert/ms 0.58 |
| warm-up | 6.67 | 30.9 | pipelines/wait_ms 3221, pipelines/compile_ms 51.89, main/busy/ms 50.08, pipelines/front_ms 49.66, render/thread/ms 42.92, render/busy/ms 41.38, render/graph/ms 9.26, pipelines/driver_ms 2.23, render/present/ms 1.23, stream/drive/ms 1.18, bigspace/total/ms 0.81, anim/evaluate_cpu/ms 0.57 |
| warm-up | 6.72 | 44.0 | pipelines/wait_ms 3221, main/busy/ms 56.98, pipelines/compile_ms 51.89, render/thread/ms 50.47, pipelines/front_ms 49.66, render/busy/ms 48.99, render/graph/ms 9.15, pipelines/driver_ms 2.23, bigspace/total/ms 1.22, render/present/ms 1.18, stream/drive/ms 1.10, physics/insert/ms 0.73 |
| warm-up | 6.75 | 32.8 | render/thread/ms 48.12, render/busy/ms 46.92, main/busy/ms 38.11, render/graph/ms 7.22, stream/drive/ms 1.22, physics/insert/ms 1.09, bigspace/total/ms 1.05, render/present/ms 0.98, terrain/update/ms 0.53, stream/spawn/ms 0.47, stream/tick/ms 0.44, terrain/select/ms 0.44 |
| warm-up | 7.63 | 31.6 | main/busy/ms 32.38, render/thread/ms 26.44, render/busy/ms 25.24, render/graph/ms 10.63, render/present/ms 1.04, physics/insert/ms 0.92, physics/step/ms 0.77, bigspace/total/ms 0.57, stream/drive/ms 0.56, terrain/update/ms 0.51, terrain/select/ms 0.43, far/select/ms 0.34 |
| measure | 0.84 | 26.0 | pipelines/wait_ms 5481, main/busy/ms 34.98, render/thread/ms 29.70, render/busy/ms 28.11, pipelines/compile_ms 15.50, pipelines/front_ms 14.45, render/graph/ms 12.56, render/present/ms 1.33, bigspace/total/ms 1.15, pipelines/driver_ms 1.05, far/select/ms 0.58, bigspace/low_precision/ms 0.52 |
| measure | 12.75 | 32.8 | render/thread/ms 41.41, render/busy/ms 40.28, render/graph/ms 33.68, main/busy/ms 15.50, bigspace/total/ms 3.01, bigspace/low_precision/ms 2.41, render/present/ms 0.94, stream/drive/ms 0.49, terrain/update/ms 0.44, stream/tick/ms 0.41, terrain/select/ms 0.38, far/select/ms 0.38 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_new.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lampsel_new\balmora_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 19%
- during the run: other processes 4% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **97%** (min 85%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.3 W · 76 °C (max 79) · VRAM 4715 MiB · throttled 100% of samples
