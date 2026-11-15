# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1446 frames over 15.0 s · **median 10.29 ms (97 fps)** · mean 10.37 · p95 11.57 · p99 12.34 (1% low 81 fps) · worst 18.74 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 570 frames over 9.9 s, worst 2452.2 ms
- hitches (> 25.7 ms): **0** measured, 35 in warm-up
- timed GPU passes: 8.68 ms of a 10.37 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.29 ms, limit 11 ms
- pass worst frame 18.74 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.742 | 2.786 | 3.194 | 0.98 | 0.060 |
| dlss_super_resolution | 2.125 | 2.160 | 2.497 | 0.98 | 0.187 |
| early prepass | 0.632 | 0.642 | 0.975 | 0.98 | 0.105 |
| atmosphere_luts | 0.352 | 0.358 | 0.485 | 0.98 | 0.003 |
| bloom | 0.334 | 0.340 | 0.494 | 0.98 | 0.064 |
| main_transmissive_pass_3d | 0.288 | 0.293 | 0.463 | 0.98 | 0.008 |
| water_velocity | 0.282 | 0.287 | 0.628 | 0.98 | 0.002 |
| water_sspr | 0.240 | 0.244 | 0.385 | 0.98 | 0.002 |
| ssao | 0.202 | 0.205 | 0.215 | 0.98 | 0.008 |
| uniform_allocation | 0.188 | 0.191 | 0.194 | 0.98 | 0.032 |
| water_fft | 0.144 | 0.147 | 0.140 | 0.98 | 0.003 |
| bin_unpacking | 0.143 | 0.145 | 0.153 | 0.98 | 0.023 |
| atmos_froxel_inject | 0.136 | 0.139 | 0.135 | 0.98 | 0.002 |
| upscaling | 0.131 | 0.133 | 0.163 | 0.98 | 0.004 |
| clouds_composite | 0.106 | 0.108 | 0.108 | 0.98 | 0.002 |
| tonemapping | 0.096 | 0.097 | 0.104 | 0.98 | 0.003 |
| clouds_trace | 0.068 | 0.069 | 0.076 | 0.98 | 0.001 |
| clustering | 0.065 | 0.066 | 0.076 | 0.98 | 0.022 |
| render_sky | 0.055 | 0.055 | 0.056 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.054 | 0.440 | 0.756 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.052 | 0.98 | 0.001 |
| water_snapshot | 0.039 | 0.040 | 0.052 | 0.98 | 0.002 |
| early_mesh_preprocessing | 0.033 | 0.034 | 0.034 | 0.98 | 0.005 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.033 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.030 | 0.245 | 0.560 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.025 | 0.026 | 0.027 | 0.98 | 0.075 |
| water_mips | 0.023 | 0.023 | 0.024 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.015 | 0.020 | 0.98 | 0.005 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.98 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.98 | 0.002 |
| night_sky | 0.007 | 0.007 | 0.008 | 0.98 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.016 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | — | — | — | - | — |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 21.509 | 0.000 | 8217.21 | 31102.0 |
| render/thread/ms | 9.466 | 10.625 | 17.93 | 13687.5 |
| render/busy/ms | 8.247 | 9.381 | 17.07 | 11925.4 |
| main/busy/ms | 7.217 | 8.579 | 12.40 | 10435.9 |
| render/graph/ms | 5.123 | 6.071 | 14.08 | 7407.3 |
| render/acquire/ms | 0.727 | 1.883 | 2.96 | 1051.0 |
| render/present/ms | 0.492 | 0.570 | 5.30 | 711.2 |
| bigspace/total/ms | 0.387 | 0.940 | 2.72 | 559.0 |
| pipelines/compile_ms | 0.295 | 0.000 | 133.55 | 426.7 |
| pipelines/front_ms | 0.286 | 0.000 | 130.59 | 413.7 |
| terrain/update/ms | 0.233 | 0.306 | 0.61 | 337.2 |
| terrain/select/ms | 0.202 | 0.262 | 0.57 | 292.4 |
| stream/drive/ms | 0.199 | 0.303 | 0.68 | 288.3 |
| far/select/ms | 0.171 | 0.252 | 0.51 | 247.5 |
| stream/tick/ms | 0.171 | 0.267 | 0.66 | 246.9 |
| controller/move/ms | 0.159 | 0.272 | 0.43 | 229.5 |
| controller/feed/ms | 0.155 | 0.209 | 0.54 | 224.1 |
| bigspace/high_precision/ms | 0.142 | 0.355 | 2.52 | 205.3 |
| bigspace/low_precision/ms | 0.101 | 0.196 | 2.02 | 145.6 |
| anim/evaluate/ms | 0.078 | 0.175 | 1.50 | 112.9 |
| anim/evaluate_cpu/ms | 0.063 | 0.078 | 0.41 | 90.7 |
| physics/step/ms | 0.030 | 0.058 | 0.23 | 43.5 |
| weather/map/ms | 0.017 | 0.022 | 0.24 | 24.0 |
| weather/eval/ms | 0.010 | 0.013 | 0.04 | 14.6 |
| pipelines/driver_ms | 0.009 | 0.000 | 3.88 | 12.9 |
| script/update/ms | 0.007 | 0.011 | 0.03 | 10.1 |
| controller/avoid/ms | 0.007 | 0.009 | 0.11 | 9.8 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.3 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 7.0 |
| anim/apply/ms | 0.004 | 0.006 | 0.08 | 6.2 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.04 | 0.4 |
| physics/insert/ms | 0.000 | 0.000 | 0.02 | 0.0 |
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
| anim/evaluated/count | count | 7 | 9 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 123 | 123 | 122 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1080 |
| crowd/c2/count | count | 283 | 284 | 284 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 2958 | 2958 |
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
| frame/work/ms | ms | 9.44 | 17.07 | 9.22 |
| gpu/timed/ms | ms | 9.43 | 11.09 | 9.22 |
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
| phases/opaque3d/batch_sets | count | 28 | 28 | 28 |
| phases/opaque3d/bins | count | 1303 | 1303 | 1303 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 27 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 28 | 28 |
| phases/opaque3d_prepass/bins | count | 1303 | 1303 | 1303 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 27 | 27 |
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
| physics/pending | count | 0 | 5 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 197 | 197 | 197 |
| pipelines/waiting | count | 0 | 7 | 0 |
| preprocess/bind_groups | count | 0 | 0 | 0 |
| preprocess/bind_groups_created | count | 0 | 0 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 210 | 210 | 210 |
| process/cpu_usage | % | 30.77 | 45.74 | 30.14 |
| process/mem_usage | GiB | 4.38 | 4.47 | 4.46 |
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
| system/cpu_usage | % | 38.48 | 48.69 | 36.51 |
| system/mem_usage | % | 61.25 | 61.70 | 61.61 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2478 | 2478 | 2478 |
| textures/images | count | 1917 | 1917 | 1917 |
| views/all/batch_sets | count | 94 | 94 | 94 |
| views/all/bins | count | 4151 | 4151 | 4151 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 90 | 90 | 90 |
| views/all/pipelines_distinct | count | 89 | 89 | 89 |
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
| views/main/batch_sets | count | 70 | 70 | 70 |
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
| anim/events/count | count | 0.10 | 3 | 141 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 5 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.42 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.10 | 148 | 147 |
| physics/inserts | count | 0.00 | 1 | 1 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.62 | 1 | 900 |
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

## Hitches (> 25.7 ms; slowest 35, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.45 | 2452.2 | main/busy/ms 2428, surface/advance_cpu_ms 0.74, surface/cell_upload_ms 0.29, stream/drive/ms 0.27, weather/map/ms 0.17, bigspace/total/ms 0.11, stream/tick/ms 0.08, bigspace/high_precision/ms 0.07, weather/eval/ms 0.03, script/update/ms 0.02, script/ui/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.28 | 825.1 | main/busy/ms 2461, render/thread/ms 790, render/busy/ms 653, render/acquire/ms 135, render/graph/ms 5.29, render/present/ms 2.12, stream/drive/ms 0.80, surface/advance_cpu_ms 0.74, terrain/update/ms 0.42, stream/tick/ms 0.41, surface/cell_upload_ms 0.29, terrain/select/ms 0.25 |
| warm-up | 3.35 | 68.5 | render/thread/ms 801, render/busy/ms 662, pipelines/wait_ms 438, render/acquire/ms 135, pipelines/compile_ms 75.31, pipelines/front_ms 69.27, main/busy/ms 49.39, terrain/update/ms 17.51, render/graph/ms 9.27, pipelines/driver_ms 6.04, render/present/ms 3.19, stream/drive/ms 0.92 |
| warm-up | 3.58 | 100.2 | pipelines/compile_ms 186, pipelines/front_ms 165, main/busy/ms 56.17, render/thread/ms 46.03, render/busy/ms 45.25, pipelines/wait_ms 35.21, pipelines/driver_ms 21.61, render/graph/ms 2.07, stream/drive/ms 1.59, stream/tick/ms 1.28, render/present/ms 0.63, terrain/update/ms 0.50 |
| warm-up | 3.61 | 27.8 | render/thread/ms 32.45, render/busy/ms 30.75, main/busy/ms 17.19, render/graph/ms 2.58, stream/drive/ms 1.69, render/present/ms 1.38, terrain/update/ms 0.84, stream/tick/ms 0.77, terrain/select/ms 0.62, stream/spawn/ms 0.52, bigspace/total/ms 0.50, render/acquire/ms 0.32 |
| warm-up | 3.68 | 30.8 | main/busy/ms 40.44, render/thread/ms 34.86, render/busy/ms 33.62, stream/drive/ms 30.04, render/graph/ms 3.30, render/present/ms 0.94, terrain/update/ms 0.82, stream/tick/ms 0.72, terrain/select/ms 0.69, render/acquire/ms 0.30, stream/spawn/ms 0.28, bigspace/total/ms 0.19 |
| warm-up | 3.72 | 39.1 | pipelines/wait_ms 707, render/thread/ms 55.52, render/busy/ms 54.18, main/busy/ms 41.64, stream/drive/ms 30.26, pipelines/compile_ms 7.82, pipelines/front_ms 7.03, render/graph/ms 4.41, render/present/ms 1.11, pipelines/driver_ms 0.79, terrain/update/ms 0.62, stream/tick/ms 0.62 |
| warm-up | 3.80 | 68.3 | pipelines/wait_ms 183, pipelines/compile_ms 179, pipelines/front_ms 168, main/busy/ms 46.08, render/thread/ms 40.95, render/busy/ms 39.07, stream/drive/ms 31.30, pipelines/driver_ms 11.86, render/graph/ms 5.61, render/present/ms 1.68, stream/spawn/ms 1.53, stream/tick/ms 0.93 |
| warm-up | 3.88 | 70.1 | render/thread/ms 61.32, render/busy/ms 59.19, main/busy/ms 25.42, render/graph/ms 6.72, stream/drive/ms 5.43, stream/spawn/ms 2.17, render/present/ms 1.83, stream/tick/ms 1.58, terrain/update/ms 0.76, terrain/select/ms 0.59, stream/activate/ms 0.34, render/acquire/ms 0.30 |
| warm-up | 3.93 | 42.2 | render/thread/ms 68.05, render/busy/ms 66.45, main/busy/ms 41.27, stream/drive/ms 24.95, render/graph/ms 5.88, render/present/ms 1.35, stream/tick/ms 1.19, stream/activate/ms 0.94, terrain/update/ms 0.80, terrain/select/ms 0.60, stream/spawn/ms 0.35, physics/insert/ms 0.33 |
| warm-up | 4.04 | 114.1 | pipelines/wait_ms 2825, pipelines/compile_ms 357, pipelines/front_ms 317, render/thread/ms 95.75, render/busy/ms 94.64, pipelines/driver_ms 40.70, main/busy/ms 40.01, stream/drive/ms 24.21, render/graph/ms 2.68, render/present/ms 0.90, stream/activate/ms 0.89, terrain/update/ms 0.64 |
| warm-up | 4.14 | 41.5 | main/busy/ms 36.89, render/thread/ms 33.02, render/busy/ms 31.43, stream/drive/ms 23.44, render/graph/ms 3.04, stream/spawn/ms 1.41, render/present/ms 1.37, stream/tick/ms 0.72, terrain/update/ms 0.58, terrain/select/ms 0.46, bigspace/total/ms 0.26, render/acquire/ms 0.21 |
| warm-up | 4.19 | 40.6 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 123, pipelines/front_ms 114, main/busy/ms 40.15, stream/drive/ms 30.21, render/thread/ms 22.62, render/busy/ms 21.13, pipelines/driver_ms 8.30, render/graph/ms 3.41, render/present/ms 1.29, stream/spawn/ms 1.15, stream/tick/ms 0.65 |
| warm-up | 4.28 | 67.3 | pipelines/wait_ms 3382, main/busy/ms 68.50, stream/drive/ms 50.93, pipelines/compile_ms 33.26, pipelines/front_ms 31.52, render/thread/ms 18.07, render/busy/ms 17.04, render/graph/ms 2.75, stream/spawn/ms 2.03, pipelines/driver_ms 1.74, render/present/ms 0.85, stream/tick/ms 0.70 |
| warm-up | 4.34 | 49.6 | pipelines/wait_ms 2787, pipelines/compile_ms 121, pipelines/front_ms 113, main/busy/ms 47.41, stream/drive/ms 35.28, render/thread/ms 28.32, render/busy/ms 26.92, pipelines/driver_ms 7.57, render/graph/ms 4.69, render/present/ms 1.13, stream/activate/ms 1.06, stream/spawn/ms 0.68 |
| warm-up | 4.49 | 30.4 | main/busy/ms 24.88, render/thread/ms 24.36, render/busy/ms 22.99, render/graph/ms 5.29, stream/drive/ms 4.76, bigspace/total/ms 2.60, stream/spawn/ms 2.44, bigspace/high_precision/ms 1.24, render/present/ms 1.19, bigspace/low_precision/ms 1.18, stream/tick/ms 0.96, terrain/update/ms 0.66 |
| warm-up | 4.52 | 30.8 | render/thread/ms 36.94, render/busy/ms 35.05, main/busy/ms 33.89, stream/drive/ms 11.12, render/graph/ms 6.48, bigspace/total/ms 2.74, stream/spawn/ms 2.60, render/present/ms 1.61, bigspace/high_precision/ms 1.24, stream/tick/ms 1.21, bigspace/low_precision/ms 1.10, terrain/update/ms 0.63 |
| warm-up | 4.56 | 31.2 | pipelines/wait_ms 4385, pipelines/compile_ms 153, pipelines/front_ms 142, render/thread/ms 41.07, render/busy/ms 39.35, main/busy/ms 29.55, pipelines/driver_ms 11.45, stream/drive/ms 10.10, render/graph/ms 4.72, stream/spawn/ms 2.20, render/present/ms 1.46, stream/tick/ms 1.01 |
| warm-up | 4.68 | 38.3 | render/thread/ms 53.59, render/busy/ms 33.06, render/present/ms 20.31, main/busy/ms 17.96, render/graph/ms 4.21, stream/drive/ms 2.56, stream/spawn/ms 1.30, terrain/update/ms 0.72, bigspace/total/ms 0.58, terrain/select/ms 0.58, stream/tick/ms 0.58, bigspace/high_precision/ms 0.34 |
| warm-up | 4.73 | 30.5 | pipelines/wait_ms 3143, pipelines/compile_ms 148, pipelines/front_ms 143, render/thread/ms 38.41, render/busy/ms 37.03, main/busy/ms 17.73, render/graph/ms 7.39, pipelines/driver_ms 5.05, stream/drive/ms 3.91, stream/spawn/ms 1.58, render/present/ms 1.11, terrain/update/ms 0.77 |
| warm-up | 4.78 | 43.5 | pipelines/wait_ms 7540, pipelines/compile_ms 61.44, pipelines/front_ms 55.23, render/thread/ms 47.92, render/busy/ms 45.85, main/busy/ms 19.87, render/graph/ms 8.74, pipelines/driver_ms 6.21, stream/drive/ms 3.49, stream/spawn/ms 2.29, render/present/ms 1.79, bigspace/total/ms 1.15 |
| warm-up | 4.84 | 40.5 | pipelines/wait_ms 4980, pipelines/compile_ms 57.57, pipelines/front_ms 54.76, render/thread/ms 44.59, render/busy/ms 43.05, main/busy/ms 27.11, render/graph/ms 5.26, stream/drive/ms 3.97, pipelines/driver_ms 2.81, stream/spawn/ms 2.04, render/present/ms 1.26, terrain/update/ms 0.82 |
| warm-up | 4.90 | 41.6 | render/thread/ms 46.65, render/busy/ms 45.23, main/busy/ms 40.90, render/graph/ms 5.48, stream/drive/ms 4.46, stream/spawn/ms 2.12, render/present/ms 1.21, physics/insert/ms 1.12, terrain/update/ms 0.63, bigspace/total/ms 0.63, stream/tick/ms 0.62, terrain/select/ms 0.51 |
| warm-up | 4.95 | 28.8 | render/thread/ms 43.34, render/busy/ms 41.79, main/busy/ms 26.23, stream/drive/ms 6.16, render/graph/ms 5.83, stream/spawn/ms 2.79, render/present/ms 1.25, terrain/update/ms 0.65, bigspace/total/ms 0.61, stream/tick/ms 0.61, terrain/select/ms 0.51, physics/insert/ms 0.44 |
| warm-up | 4.98 | 28.0 | render/thread/ms 44.41, render/busy/ms 42.10, main/busy/ms 29.47, render/graph/ms 7.78, stream/drive/ms 6.02, stream/spawn/ms 2.79, render/present/ms 1.99, bigspace/total/ms 0.81, stream/tick/ms 0.72, terrain/update/ms 0.67, terrain/select/ms 0.53, bigspace/high_precision/ms 0.39 |
| warm-up | 5.02 | 38.5 | render/thread/ms 40.08, render/busy/ms 38.15, main/busy/ms 26.88, render/graph/ms 5.92, stream/drive/ms 3.69, stream/spawn/ms 2.14, render/present/ms 1.68, stream/tick/ms 0.93, bigspace/total/ms 0.80, terrain/update/ms 0.68, terrain/select/ms 0.54, bigspace/high_precision/ms 0.39 |
| warm-up | 5.17 | 26.4 | pipelines/wait_ms 6147, pipelines/compile_ms 134, pipelines/front_ms 125, main/busy/ms 26.62, render/thread/ms 20.10, render/busy/ms 18.82, stream/drive/ms 14.24, pipelines/driver_ms 9.23, render/graph/ms 5.05, render/present/ms 1.01, stream/spawn/ms 0.81, stream/tick/ms 0.61 |
| warm-up | 5.27 | 26.7 | pipelines/wait_ms 1855, main/busy/ms 41.53, render/thread/ms 37.92, render/busy/ms 35.86, pipelines/compile_ms 15.01, pipelines/front_ms 14.06, render/graph/ms 8.64, stream/drive/ms 4.07, stream/spawn/ms 2.39, render/present/ms 1.80, pipelines/driver_ms 0.96, bigspace/total/ms 0.89 |
| warm-up | 5.31 | 36.8 | render/thread/ms 46.87, render/busy/ms 45.69, main/busy/ms 32.57, render/graph/ms 5.39, stream/drive/ms 3.28, stream/spawn/ms 1.76, render/present/ms 0.98, bigspace/total/ms 0.78, terrain/update/ms 0.74, stream/tick/ms 0.68, terrain/select/ms 0.60, anim/evaluate_cpu/ms 0.37 |
| warm-up | 5.43 | 29.7 | pipelines/wait_ms 2471, main/busy/ms 37.59, render/thread/ms 36.66, render/busy/ms 34.00, pipelines/compile_ms 31.85, pipelines/front_ms 30.04, stream/drive/ms 12.94, render/graph/ms 11.18, render/present/ms 2.43, pipelines/driver_ms 1.81, stream/tick/ms 1.50, stream/spawn/ms 1.49 |
| warm-up | 5.47 | 40.2 | render/thread/ms 45.25, render/busy/ms 43.38, main/busy/ms 37.81, stream/drive/ms 13.15, render/graph/ms 8.41, bigspace/total/ms 2.89, stream/spawn/ms 1.68, stream/tick/ms 1.61, render/present/ms 1.56, terrain/update/ms 0.73, terrain/select/ms 0.60, render/acquire/ms 0.31 |
| warm-up | 7.20 | 29.2 | pipelines/wait_ms 12.7 k, main/busy/ms 32.19, render/thread/ms 30.09, render/busy/ms 28.30, render/graph/ms 9.97, pipelines/compile_ms 9.36, pipelines/front_ms 7.97, render/present/ms 1.52, pipelines/driver_ms 1.40, bigspace/total/ms 1.02, physics/step/ms 0.80, terrain/update/ms 0.74 |
| warm-up | 7.23 | 28.4 | render/thread/ms 39.07, render/busy/ms 37.87, main/busy/ms 26.02, render/graph/ms 7.55, render/present/ms 0.95, terrain/update/ms 0.92, physics/step/ms 0.84, bigspace/total/ms 0.84, terrain/select/ms 0.73, stream/drive/ms 0.62, stream/tick/ms 0.55, far/select/ms 0.54 |
| warm-up | 7.27 | 27.6 | main/busy/ms 29.46, render/thread/ms 26.99, render/busy/ms 25.76, render/graph/ms 7.33, render/present/ms 1.06, bigspace/total/ms 0.52, terrain/update/ms 0.49, terrain/select/ms 0.41, stream/drive/ms 0.37, far/select/ms 0.33, stream/tick/ms 0.32, bigspace/high_precision/ms 0.21 |
| warm-up | 8.94 | 28.9 | render/thread/ms 31.21, render/busy/ms 29.63, main/busy/ms 27.87, render/graph/ms 12.15, render/present/ms 1.44, bigspace/total/ms 0.70, controller/move/ms 0.48, terrain/update/ms 0.46, terrain/select/ms 0.39, far/select/ms 0.38, stream/drive/ms 0.37, controller/feed/ms 0.34 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_old.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lampsel_old\balmora_night@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 6%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 93%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.1 W · 78 °C (max 80) · VRAM 4742 MiB · throttled 100% of samples
