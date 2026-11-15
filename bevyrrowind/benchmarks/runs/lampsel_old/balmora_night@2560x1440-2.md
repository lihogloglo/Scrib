# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1459 frames over 15.0 s · **median 10.21 ms (98 fps)** · mean 10.28 · p95 11.50 · p99 12.74 (1% low 79 fps) · worst 27.91 ms · 5 spikes over 1.5× median · 24.0 stutters/min (work 16.0/min)
- warm-up: 298 frames over 7.6 s, worst 2462.9 ms
- hitches (> 25.5 ms): **1** measured, 35 in warm-up
- timed GPU passes: 8.60 ms of a 10.28 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.21 ms, limit 11 ms
- pass worst frame 27.91 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.695 | 2.738 | 3.144 | 0.98 | 0.059 |
| dlss_super_resolution | 2.126 | 2.160 | 2.524 | 0.98 | 0.185 |
| early prepass | 0.628 | 0.638 | 0.973 | 0.98 | 0.102 |
| atmosphere_luts | 0.358 | 0.364 | 0.678 | 0.98 | 0.003 |
| bloom | 0.334 | 0.339 | 0.488 | 0.98 | 0.066 |
| main_transmissive_pass_3d | 0.286 | 0.291 | 0.290 | 0.98 | 0.009 |
| water_velocity | 0.277 | 0.281 | 0.271 | 0.98 | 0.001 |
| water_sspr | 0.229 | 0.232 | 0.235 | 0.98 | 0.002 |
| ssao | 0.197 | 0.201 | 0.209 | 0.98 | 0.008 |
| uniform_allocation | 0.187 | 0.190 | 0.193 | 0.98 | 0.031 |
| bin_unpacking | 0.142 | 0.145 | 0.151 | 0.98 | 0.023 |
| water_fft | 0.140 | 0.142 | 0.138 | 0.98 | 0.003 |
| atmos_froxel_inject | 0.135 | 0.137 | 0.135 | 0.98 | 0.002 |
| upscaling | 0.122 | 0.124 | 0.155 | 0.98 | 0.005 |
| clouds_composite | 0.106 | 0.108 | 0.108 | 0.98 | 0.002 |
| tonemapping | 0.098 | 0.099 | 0.111 | 0.98 | 0.003 |
| clouds_trace | 0.069 | 0.070 | 0.090 | 0.98 | 0.001 |
| clustering | 0.066 | 0.067 | 0.081 | 0.98 | 0.022 |
| render_sky | 0.054 | 0.055 | 0.056 | 0.98 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.430 | 0.433 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.053 | 0.98 | 0.001 |
| water_snapshot | 0.041 | 0.041 | 0.055 | 0.98 | 0.002 |
| early_mesh_preprocessing | 0.035 | 0.036 | 0.035 | 0.98 | 0.005 |
| atmos_froxel_integrate | 0.030 | 0.030 | 0.032 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.029 | 0.238 | 0.364 | 0.12 | 0.004 |
| main_transparent_pass_3d | 0.024 | 0.025 | 0.027 | 0.98 | 0.074 |
| water_mips | 0.024 | 0.025 | 0.024 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.015 | 0.016 | 0.020 | 0.98 | 0.005 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.98 | 0.004 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.98 | 0.002 |
| night_sky | 0.007 | 0.007 | 0.008 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.98 | 0.003 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.016 | 0.015 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.004 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.006 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 136.995 | 0.000 | 12688.39 | 199876.3 |
| render/thread/ms | 9.363 | 10.627 | 21.92 | 13661.0 |
| render/busy/ms | 7.802 | 9.153 | 16.13 | 11382.7 |
| main/busy/ms | 7.014 | 8.345 | 19.36 | 10234.1 |
| render/graph/ms | 4.649 | 5.874 | 12.55 | 6782.9 |
| pipelines/compile_ms | 2.026 | 0.000 | 133.88 | 2955.5 |
| pipelines/front_ms | 1.954 | 0.000 | 129.06 | 2851.6 |
| render/acquire/ms | 1.059 | 2.293 | 3.55 | 1544.9 |
| render/present/ms | 0.503 | 0.609 | 12.65 | 733.4 |
| bigspace/total/ms | 0.352 | 0.629 | 2.56 | 513.7 |
| terrain/update/ms | 0.230 | 0.290 | 0.53 | 336.1 |
| terrain/select/ms | 0.200 | 0.252 | 0.50 | 291.6 |
| stream/drive/ms | 0.189 | 0.276 | 0.53 | 276.0 |
| far/select/ms | 0.167 | 0.234 | 0.52 | 243.7 |
| stream/tick/ms | 0.162 | 0.238 | 0.49 | 236.4 |
| controller/feed/ms | 0.144 | 0.199 | 0.43 | 210.7 |
| controller/move/ms | 0.129 | 0.265 | 1.28 | 188.8 |
| bigspace/high_precision/ms | 0.120 | 0.277 | 1.66 | 174.5 |
| bigspace/low_precision/ms | 0.088 | 0.180 | 2.20 | 127.7 |
| pipelines/driver_ms | 0.071 | 0.000 | 4.82 | 104.0 |
| anim/evaluate_cpu/ms | 0.067 | 0.085 | 1.13 | 97.4 |
| anim/evaluate/ms | 0.059 | 0.166 | 1.61 | 85.9 |
| physics/step/ms | 0.034 | 0.060 | 1.61 | 49.3 |
| weather/map/ms | 0.016 | 0.022 | 0.08 | 23.6 |
| weather/eval/ms | 0.010 | 0.013 | 0.08 | 14.4 |
| physics/insert/ms | 0.008 | 0.035 | 0.61 | 12.2 |
| script/update/ms | 0.007 | 0.010 | 0.03 | 9.9 |
| controller/avoid/ms | 0.006 | 0.008 | 0.11 | 9.4 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 7.5 |
| script/ui/ms | 0.005 | 0.007 | 0.04 | 7.0 |
| anim/apply/ms | 0.004 | 0.006 | 0.13 | 6.3 |
| controller/write/ms | 0.002 | 0.003 | 0.09 | 3.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 1.0 |
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
| anim/a2/count | count | 0 | 23 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 95 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 31 | 8 |
| crowd/c0/joints/count | count | 272 | 1054 | 272 |
| crowd/c0/meshes/count | count | 48 | 186 | 48 |
| crowd/c1/count | count | 123 | 142 | 122 |
| crowd/c1/joints/count | count | 4880 | 5213 | 4848 |
| crowd/c1/meshes/count | count | 1088 | 1226 | 1080 |
| crowd/c2/count | count | 283 | 284 | 284 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 118 | 118 | 118 |
| crowd/joints_extracted/count | count | 2958 | 3935 | 2958 |
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
| frame/work/ms | ms | 9.35 | 19.36 | 9.61 |
| gpu/timed/ms | ms | 9.34 | 10.25 | 9.61 |
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
| phases/opaque3d/batch_sets | count | 28 | 29 | 28 |
| phases/opaque3d/bins | count | 1303 | 1304 | 1303 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 28 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 29 | 28 |
| phases/opaque3d_prepass/bins | count | 1303 | 1304 | 1303 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 28 | 27 |
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
| physics/pending | count | 0 | 950 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 197 | 197 | 197 |
| pipelines/waiting | count | 0 | 54 | 0 |
| preprocess/bind_groups | count | 0 | 208 | 0 |
| preprocess/bind_groups_created | count | 0 | 208 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 210 | 210 | 210 |
| process/cpu_usage | % | 29.26 | 47.76 | 31.25 |
| process/mem_usage | GiB | 4.40 | 4.46 | 4.46 |
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
| stream/latency/s | s | 0.06 | 0.06 | 0.06 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 34.67 | 46.22 | 38.18 |
| system/mem_usage | % | 61.44 | 61.63 | 61.60 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2478 | 2478 | 2478 |
| textures/images | count | 1917 | 1917 | 1917 |
| views/all/batch_sets | count | 94 | 96 | 94 |
| views/all/bins | count | 4151 | 4151 | 4151 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.3 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 90 | 92 | 90 |
| views/all/pipelines_distinct | count | 89 | 91 | 89 |
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
| views/main/batch_sets | count | 70 | 72 | 70 |
| views/main/bins | count | 3166 | 3168 | 3166 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9300 | 9300 | 9300 |
| views/main/pipelines | count | 66 | 68 | 66 |
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
| anim/bones_written/count | count | 0.43 | 363 | 627 |
| anim/events/count | count | 0.09 | 3 | 130 |
| assets/Image/added | count | 0.05 | 34 | 79 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.19 | 126 | 281 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.06 | 43 | 92 |
| assets/StandardMaterial/modified | count | 3.39 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 5.31 | 4289 | 7742 |
| physics/inserts | count | 0.57 | 153 | 826 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.62 | 2 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 3 | 3 |
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

## Hitches (> 25.5 ms; slowest 36, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.46 | 2462.9 | main/busy/ms 2438, surface/advance_cpu_ms 0.73, stream/drive/ms 0.32, surface/cell_upload_ms 0.28, stream/tick/ms 0.08, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, weather/map/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.02 | 558.4 | main/busy/ms 2473, render/thread/ms 523, render/busy/ms 411, render/acquire/ms 109, render/graph/ms 5.18, render/present/ms 2.41, stream/drive/ms 0.79, surface/advance_cpu_ms 0.73, terrain/update/ms 0.67, terrain/select/ms 0.46, stream/tick/ms 0.39, surface/cell_upload_ms 0.28 |
| warm-up | 3.37 | 351.5 | pipelines/wait_ms 2959, render/thread/ms 534, render/busy/ms 422, pipelines/compile_ms 370, pipelines/front_ms 346, render/acquire/ms 109, main/busy/ms 50.41, pipelines/driver_ms 23.95, terrain/update/ms 19.34, render/graph/ms 9.48, render/present/ms 3.36, stream/drive/ms 0.87 |
| warm-up | 3.62 | 111.1 | pipelines/wait_ms 282, pipelines/compile_ms 212, pipelines/front_ms 194, main/busy/ms 75.07, render/thread/ms 55.16, render/busy/ms 52.46, pipelines/driver_ms 18.03, render/graph/ms 5.71, render/present/ms 2.15, stream/drive/ms 1.63, stream/tick/ms 1.37, terrain/update/ms 0.61 |
| warm-up | 3.67 | 27.8 | render/thread/ms 40.70, render/busy/ms 39.56, main/busy/ms 18.43, render/graph/ms 2.26, stream/drive/ms 1.46, render/present/ms 0.95, stream/tick/ms 0.87, terrain/update/ms 0.79, terrain/select/ms 0.62, stream/spawn/ms 0.30, bigspace/total/ms 0.19, render/acquire/ms 0.19 |
| warm-up | 3.79 | 70.4 | pipelines/wait_ms 222, pipelines/compile_ms 177, pipelines/front_ms 169, main/busy/ms 46.10, render/thread/ms 44.91, render/busy/ms 43.80, stream/drive/ms 35.56, pipelines/driver_ms 8.64, render/graph/ms 2.54, render/present/ms 0.93, stream/tick/ms 0.92, terrain/update/ms 0.49 |
| warm-up | 3.84 | 32.8 | main/busy/ms 29.50, render/thread/ms 29.07, render/busy/ms 27.24, stream/drive/ms 16.43, render/graph/ms 4.39, render/present/ms 1.53, stream/spawn/ms 1.31, stream/tick/ms 1.26, terrain/update/ms 0.50, terrain/select/ms 0.40, render/acquire/ms 0.30, bigspace/total/ms 0.23 |
| warm-up | 3.89 | 30.8 | render/thread/ms 36.28, render/busy/ms 34.78, main/busy/ms 32.66, stream/drive/ms 16.38, render/graph/ms 3.92, render/present/ms 1.18, stream/tick/ms 1.01, terrain/update/ms 0.63, terrain/select/ms 0.49, render/acquire/ms 0.32, stream/spawn/ms 0.31, bigspace/total/ms 0.24 |
| warm-up | 3.96 | 66.0 | pipelines/wait_ms 555, render/thread/ms 58.19, render/busy/ms 56.90, main/busy/ms 37.52, stream/drive/ms 17.14, render/graph/ms 3.07, render/present/ms 1.09, pipelines/compile_ms 1.09, stream/spawn/ms 1.07, stream/tick/ms 0.85, pipelines/front_ms 0.83, terrain/update/ms 0.62 |
| warm-up | 3.99 | 30.6 | pipelines/wait_ms 962, pipelines/compile_ms 188, pipelines/front_ms 181, main/busy/ms 64.42, render/thread/ms 59.62, render/busy/ms 58.29, stream/drive/ms 43.87, pipelines/driver_ms 6.92, render/graph/ms 3.55, stream/spawn/ms 2.11, stream/tick/ms 1.14, render/present/ms 1.11 |
| warm-up | 4.08 | 94.2 | pipelines/wait_ms 3538, pipelines/compile_ms 311, pipelines/front_ms 298, main/busy/ms 75.47, render/thread/ms 58.72, render/busy/ms 56.31, stream/drive/ms 54.74, pipelines/driver_ms 13.26, render/graph/ms 7.00, stream/spawn/ms 3.00, render/present/ms 2.06, stream/tick/ms 0.91 |
| warm-up | 4.13 | 47.7 | pipelines/wait_ms 4511, pipelines/compile_ms 151, pipelines/front_ms 141, render/thread/ms 66.91, render/busy/ms 64.49, main/busy/ms 44.72, stream/drive/ms 22.22, pipelines/driver_ms 10.03, render/graph/ms 6.69, stream/spawn/ms 3.02, render/present/ms 2.10, stream/tick/ms 0.92 |
| warm-up | 4.17 | 43.9 | pipelines/wait_ms 1380, render/thread/ms 52.02, render/busy/ms 50.67, main/busy/ms 29.78, pipelines/compile_ms 26.28, pipelines/front_ms 22.85, stream/drive/ms 11.44, render/graph/ms 3.64, pipelines/driver_ms 3.43, stream/spawn/ms 2.13, stream/tick/ms 1.14, render/present/ms 1.14 |
| warm-up | 4.23 | 40.5 | main/busy/ms 41.41, stream/drive/ms 25.71, render/thread/ms 19.80, render/busy/ms 16.13, render/present/ms 3.50, render/graph/ms 2.80, stream/tick/ms 0.60, stream/spawn/ms 0.53, terrain/update/ms 0.49, bigspace/total/ms 0.40, terrain/select/ms 0.39, stream/activate/ms 0.37 |
| warm-up | 4.38 | 96.5 | pipelines/wait_ms 2648, pipelines/compile_ms 109, pipelines/front_ms 102, main/busy/ms 93.36, stream/drive/ms 83.05, render/thread/ms 23.80, render/busy/ms 22.77, pipelines/driver_ms 6.90, terrain/update/ms 5.25, render/graph/ms 3.77, stream/tick/ms 0.85, render/present/ms 0.84 |
| warm-up | 4.55 | 28.9 | pipelines/wait_ms 5366, render/thread/ms 33.20, render/busy/ms 31.28, main/busy/ms 28.68, pipelines/compile_ms 24.58, pipelines/front_ms 22.02, stream/drive/ms 10.38, render/graph/ms 5.32, pipelines/driver_ms 2.56, stream/spawn/ms 2.42, render/present/ms 1.52, stream/tick/ms 0.84 |
| warm-up | 4.58 | 28.9 | render/thread/ms 42.31, render/busy/ms 39.80, main/busy/ms 33.62, stream/drive/ms 12.03, render/graph/ms 8.13, stream/spawn/ms 3.85, render/present/ms 2.12, stream/tick/ms 1.01, terrain/update/ms 0.53, stream/activate/ms 0.50, bigspace/total/ms 0.45, render/acquire/ms 0.40 |
| warm-up | 4.61 | 27.8 | render/thread/ms 37.99, render/busy/ms 36.16, main/busy/ms 27.27, render/graph/ms 6.19, stream/drive/ms 5.71, stream/spawn/ms 3.21, render/present/ms 1.53, stream/tick/ms 1.38, terrain/update/ms 0.80, terrain/select/ms 0.62, bigspace/total/ms 0.44, render/acquire/ms 0.31 |
| warm-up | 4.79 | 36.9 | render/thread/ms 48.91, render/busy/ms 47.29, main/busy/ms 22.50, render/graph/ms 5.51, stream/drive/ms 4.54, stream/spawn/ms 2.20, render/present/ms 1.29, stream/tick/ms 1.15, terrain/update/ms 0.61, bigspace/total/ms 0.61, terrain/select/ms 0.49, render/acquire/ms 0.33 |
| warm-up | 4.82 | 26.6 | render/thread/ms 54.19, render/busy/ms 52.78, main/busy/ms 22.34, render/graph/ms 4.70, stream/drive/ms 4.29, stream/spawn/ms 2.13, render/present/ms 1.08, stream/tick/ms 0.89, terrain/update/ms 0.82, bigspace/total/ms 0.66, terrain/select/ms 0.64, render/acquire/ms 0.33 |
| warm-up | 4.85 | 27.8 | render/thread/ms 36.50, render/busy/ms 35.14, main/busy/ms 19.80, render/graph/ms 3.94, stream/drive/ms 3.75, stream/spawn/ms 2.08, render/present/ms 1.15, terrain/update/ms 0.85, bigspace/total/ms 0.73, stream/tick/ms 0.69, terrain/select/ms 0.68, bigspace/high_precision/ms 0.42 |
| warm-up | 4.88 | 29.4 | render/thread/ms 43.90, render/busy/ms 42.47, main/busy/ms 18.50, render/graph/ms 4.54, stream/drive/ms 3.96, stream/spawn/ms 2.16, render/present/ms 1.20, bigspace/total/ms 0.64, stream/tick/ms 0.62, terrain/update/ms 0.62, terrain/select/ms 0.50, bigspace/high_precision/ms 0.38 |
| warm-up | 4.92 | 39.0 | pipelines/wait_ms 10.3 k, pipelines/compile_ms 260, pipelines/front_ms 250, render/thread/ms 40.59, render/busy/ms 39.25, main/busy/ms 21.32, pipelines/driver_ms 10.66, stream/drive/ms 6.11, render/graph/ms 4.56, stream/spawn/ms 1.42, render/present/ms 1.11, stream/tick/ms 0.87 |
| warm-up | 4.98 | 41.3 | render/thread/ms 47.17, render/busy/ms 45.87, main/busy/ms 25.96, stream/drive/ms 5.02, render/graph/ms 4.87, stream/spawn/ms 2.73, stream/tick/ms 1.24, render/present/ms 1.08, bigspace/total/ms 0.93, terrain/update/ms 0.71, terrain/select/ms 0.56, bigspace/high_precision/ms 0.47 |
| warm-up | 5.03 | 28.0 | pipelines/wait_ms 6350, pipelines/compile_ms 94.59, pipelines/front_ms 86.63, main/busy/ms 35.21, render/thread/ms 28.20, render/busy/ms 26.09, stream/drive/ms 13.75, pipelines/driver_ms 7.96, render/graph/ms 6.66, stream/spawn/ms 2.42, render/present/ms 1.87, terrain/update/ms 0.83 |
| warm-up | 5.07 | 41.0 | pipelines/wait_ms 13.0 k, pipelines/compile_ms 56.70, pipelines/front_ms 51.84, main/busy/ms 38.47, render/thread/ms 36.54, render/busy/ms 35.15, stream/drive/ms 13.73, pipelines/driver_ms 4.87, render/graph/ms 4.60, stream/spawn/ms 2.38, render/present/ms 1.21, physics/insert/ms 1.12 |
| warm-up | 5.11 | 28.0 | pipelines/wait_ms 1646, main/busy/ms 46.44, render/thread/ms 30.11, render/busy/ms 27.96, pipelines/compile_ms 27.85, pipelines/front_ms 26.66, render/graph/ms 7.77, stream/drive/ms 6.67, stream/spawn/ms 2.16, render/present/ms 1.91, pipelines/driver_ms 1.19, physics/insert/ms 1.08 |
| warm-up | 5.16 | 45.2 | pipelines/wait_ms 7933, pipelines/compile_ms 71.66, pipelines/front_ms 68.33, render/thread/ms 45.55, render/busy/ms 43.52, main/busy/ms 38.89, render/graph/ms 12.02, stream/drive/ms 6.70, pipelines/driver_ms 3.33, stream/spawn/ms 2.16, render/present/ms 1.76, physics/insert/ms 1.12 |
| warm-up | 5.25 | 26.5 | main/busy/ms 29.89, render/thread/ms 21.62, render/busy/ms 20.31, stream/drive/ms 16.30, render/graph/ms 6.38, stream/spawn/ms 1.72, render/present/ms 1.09, bigspace/total/ms 0.61, stream/tick/ms 0.60, terrain/update/ms 0.51, terrain/select/ms 0.41, bigspace/high_precision/ms 0.36 |
| warm-up | 5.37 | 25.8 | render/thread/ms 34.68, render/busy/ms 33.25, main/busy/ms 16.55, render/graph/ms 6.30, stream/drive/ms 2.38, render/present/ms 1.21, stream/spawn/ms 1.00, stream/tick/ms 0.65, terrain/update/ms 0.59, terrain/select/ms 0.47, bigspace/total/ms 0.46, render/acquire/ms 0.23 |
| warm-up | 5.54 | 50.9 | pipelines/wait_ms 11.7 k, pipelines/compile_ms 84.85, pipelines/front_ms 78.54, render/thread/ms 44.00, main/busy/ms 43.95, render/busy/ms 42.23, stream/drive/ms 13.68, render/graph/ms 7.87, pipelines/driver_ms 6.31, stream/spawn/ms 2.41, render/present/ms 1.51, stream/tick/ms 1.06 |
| warm-up | 5.57 | 30.9 | pipelines/wait_ms 9767, pipelines/compile_ms 81.97, pipelines/front_ms 76.56, render/thread/ms 45.29, render/busy/ms 43.99, main/busy/ms 35.77, pipelines/driver_ms 5.41, render/graph/ms 4.81, stream/drive/ms 4.66, terrain/update/ms 3.52, stream/spawn/ms 2.98, render/present/ms 1.08 |
| warm-up | 5.90 | 36.5 | pipelines/wait_ms 4725, pipelines/compile_ms 81.29, main/busy/ms 48.30, pipelines/front_ms 47.20, render/thread/ms 42.35, render/busy/ms 40.61, pipelines/driver_ms 34.08, render/graph/ms 12.90, stream/drive/ms 1.67, render/present/ms 1.51, bigspace/total/ms 1.00, anim/evaluate_cpu/ms 0.84 |
| warm-up | 5.93 | 32.7 | pipelines/wait_ms 793, render/thread/ms 46.18, render/busy/ms 44.80, main/busy/ms 44.45, pipelines/compile_ms 10.65, pipelines/front_ms 10.29, render/graph/ms 7.67, stream/drive/ms 1.36, render/present/ms 1.13, bigspace/total/ms 0.72, stream/tick/ms 0.65, terrain/update/ms 0.58 |
| warm-up | 7.55 | 27.7 | main/busy/ms 32.41, render/thread/ms 26.09, render/busy/ms 24.96, render/graph/ms 10.56, render/present/ms 0.95, physics/insert/ms 0.92, physics/step/ms 0.70, stream/drive/ms 0.54, bigspace/total/ms 0.51, terrain/update/ms 0.43, terrain/select/ms 0.38, stream/tick/ms 0.37 |
| measure | 1.44 | 27.9 | main/busy/ms 31.99, render/thread/ms 24.86, render/busy/ms 23.59, render/graph/ms 8.68, render/present/ms 1.00, bigspace/total/ms 0.98, physics/insert/ms 0.56, anim/evaluate_cpu/ms 0.53, terrain/update/ms 0.50, terrain/select/ms 0.43, bigspace/low_precision/ms 0.41, anim/evaluate/ms 0.38 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_old.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lampsel_old\balmora_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 15%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 91%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.1 W · 77 °C (max 80) · VRAM 4742 MiB · throttled 100% of samples
