# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,shadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1960 frames over 15.0 s · **median 7.48 ms (134 fps)** · mean 7.66 · p95 9.21 · p99 10.35 (1% low 97 fps) · worst 24.51 ms · 12 spikes over 1.5× median · 32.0 stutters/min (work 32.0/min)
- warm-up: 293 frames over 7.4 s, worst 2589.2 ms
- hitches (> 25.0 ms): **0** measured, 30 in warm-up
- timed GPU passes: 5.77 ms of a 7.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 7.48 ms, limit 11 ms
- pass worst frame 24.51 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.905 | 2.071 | 2.313 | 0.92 | 0.182 |
| main_opaque_pass_3d | 1.544 | 1.678 | 2.018 | 0.92 | 0.065 |
| early prepass | 0.502 | 0.546 | 0.791 | 0.92 | 0.104 |
| atmosphere_luts | 0.372 | 0.405 | 0.611 | 0.92 | 0.004 |
| water_velocity | 0.243 | 0.264 | 0.265 | 0.92 | 0.002 |
| water_sspr | 0.205 | 0.223 | 0.221 | 0.92 | 0.002 |
| main_transmissive_pass_3d | 0.204 | 0.221 | 0.224 | 0.92 | 0.009 |
| water_fft | 0.141 | 0.153 | 0.376 | 0.92 | 0.003 |
| upscaling | 0.124 | 0.135 | 0.138 | 0.92 | 0.004 |
| tonemapping | 0.117 | 0.127 | 0.129 | 0.92 | 0.005 |
| uniform_allocation | 0.114 | 0.124 | 0.123 | 0.92 | 0.025 |
| bin_unpacking | 0.083 | 0.090 | 0.099 | 0.92 | 0.017 |
| clustering | 0.060 | 0.065 | 0.071 | 0.92 | 0.023 |
| render_sky | 0.050 | 0.054 | 0.054 | 0.92 | 0.002 |
| water_snapshot | 0.030 | 0.033 | 0.035 | 0.92 | 0.002 |
| early_mesh_preprocessing | 0.024 | 0.026 | 0.028 | 0.92 | 0.005 |
| water_mips | 0.022 | 0.024 | 0.024 | 0.92 | 0.002 |
| main_transparent_pass_3d | 0.018 | 0.020 | 0.020 | 0.92 | 0.058 |
| night_sky | 0.006 | 0.007 | 0.007 | 0.92 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.008 | 0.92 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.92 | 0.003 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.92 | 0.001 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.92 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.007 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 30.404 | 0.000 | 5895.59 | 59592.7 |
| render/thread/ms | 6.699 | 8.082 | 13.69 | 13129.8 |
| main/busy/ms | 6.371 | 7.694 | 18.04 | 12486.6 |
| render/busy/ms | 6.051 | 7.388 | 12.97 | 11859.0 |
| render/graph/ms | 3.397 | 4.392 | 5.59 | 6658.5 |
| pipelines/compile_ms | 0.744 | 0.000 | 131.36 | 1458.1 |
| pipelines/front_ms | 0.719 | 0.000 | 127.81 | 1409.1 |
| render/present/ms | 0.511 | 0.977 | 2.39 | 1002.0 |
| bigspace/total/ms | 0.447 | 1.144 | 2.00 | 876.6 |
| terrain/update/ms | 0.234 | 0.302 | 0.89 | 459.4 |
| stream/drive/ms | 0.221 | 0.316 | 0.61 | 433.1 |
| terrain/select/ms | 0.203 | 0.260 | 0.62 | 398.0 |
| stream/tick/ms | 0.191 | 0.280 | 0.58 | 373.4 |
| bigspace/high_precision/ms | 0.179 | 0.829 | 1.66 | 351.6 |
| far/select/ms | 0.179 | 0.263 | 0.51 | 351.1 |
| controller/feed/ms | 0.159 | 0.219 | 0.46 | 311.5 |
| anim/evaluate/ms | 0.156 | 0.706 | 1.74 | 306.3 |
| render/acquire/ms | 0.137 | 0.476 | 1.00 | 268.8 |
| controller/move/ms | 0.095 | 0.172 | 0.34 | 187.2 |
| bigspace/low_precision/ms | 0.086 | 0.122 | 1.49 | 167.7 |
| anim/evaluate_cpu/ms | 0.058 | 0.082 | 0.28 | 113.8 |
| physics/step/ms | 0.028 | 0.068 | 0.86 | 54.5 |
| pipelines/driver_ms | 0.025 | 0.000 | 4.77 | 49.1 |
| weather/eval/ms | 0.012 | 0.017 | 0.16 | 23.1 |
| physics/insert/ms | 0.009 | 0.000 | 0.79 | 18.6 |
| script/update/ms | 0.008 | 0.013 | 0.15 | 16.4 |
| controller/avoid/ms | 0.006 | 0.007 | 0.02 | 11.3 |
| script/input/ms | 0.005 | 0.007 | 0.04 | 10.3 |
| script/ui/ms | 0.005 | 0.007 | 0.06 | 9.4 |
| anim/apply/ms | 0.005 | 0.006 | 0.20 | 8.9 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 3.6 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 2.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.13 | 1.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 0.7 |
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
| anim/a2/count | count | 0 | 26 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 42 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 34 | 8 |
| crowd/c0/joints/count | count | 272 | 1156 | 272 |
| crowd/c0/meshes/count | count | 48 | 204 | 48 |
| crowd/c1/count | count | 122 | 125 | 123 |
| crowd/c1/joints/count | count | 4848 | 5086 | 4880 |
| crowd/c1/meshes/count | count | 1080 | 1159 | 1088 |
| crowd/c2/count | count | 284 | 284 | 283 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 109 | 109 | 109 |
| crowd/joints_extracted/count | count | 2697 | 5653 | 2697 |
| crowd/joints_uploaded/count | count | 2697 | 2697 | 2697 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.0 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 6.46 | 18.04 | 6.53 |
| gpu/timed/ms | ms | 6.30 | 7.19 | 6.53 |
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
| phases/opaque3d/bins | count | 1303 | 1304 | 1303 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d_prepass/bins | count | 1303 | 1304 | 1303 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3776 | 3776 | 3776 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1080 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 140 | 140 | 140 |
| pipelines/waiting | count | 0 | 23 | 0 |
| preprocess/bind_groups | count | 0 | 130 | 0 |
| preprocess/bind_groups_created | count | 0 | 130 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 134 | 134 |
| process/cpu_usage | % | 26.92 | 35.10 | 27.96 |
| process/mem_usage | GiB | 3.99 | 4.01 | 3.99 |
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
| started_load_count |  | 3530 | 3532 | 3532 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 392 | 392 | 392 |
| statics/materials_bound | count | 392 | 392 | 392 |
| statics/materials_unclassed | count | 6 | 6 | 6 |
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
| system/cpu_usage | % | 43.35 | 76.06 | 32.43 |
| system/mem_usage | % | 71.78 | 72.21 | 71.50 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2461 | 2461 | 2461 |
| textures/images | count | 1908 | 1908 | 1908 |
| views/all/batch_sets | count | 58 | 62 | 58 |
| views/all/bins | count | 3166 | 3168 | 3166 |
| views/all/count | count | 1 | 1 | 1 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 9300 | 9300 | 9300 |
| views/all/pipelines | count | 54 | 58 | 54 |
| views/all/pipelines_distinct | count | 54 | 58 | 54 |
| views/all/work_items | count | 9480 | 9480 | 9480 |
| views/cascade/batch_sets | count | 0 | 0 | 0 |
| views/cascade/bins | count | 0 | 0 | 0 |
| views/cascade/count | count | 0 | 0 | 0 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 0 | 0 | 0 |
| views/cascade/pipelines | count | 0 | 0 | 0 |
| views/cascade/work_items | count | 0 | 0 | 0 |
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
| views/main/bins | count | 3166 | 3168 | 3166 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9300 | 9300 | 9300 |
| views/main/pipelines | count | 54 | 58 | 54 |
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
| anim/bones_written/count | count | 0.44 | 594 | 858 |
| anim/events/count | count | 0.07 | 2 | 146 |
| assets/Image/added | count | 0.07 | 34 | 128 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.19 | 118 | 374 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.07 | 41 | 143 |
| assets/StandardMaterial/modified | count | 2.53 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 4.72 | 2681 | 9244 |
| physics/inserts | count | 0.55 | 151 | 1080 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.46 | 2 | 900 |
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

## Hitches (> 25.0 ms; slowest 30, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.59 | 2589.2 | main/busy/ms 2566, surface/advance_cpu_ms 0.73, surface/cell_upload_ms 0.29, stream/drive/ms 0.28, bigspace/total/ms 0.10, stream/tick/ms 0.07, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.55 | 958.2 | main/busy/ms 2596, render/thread/ms 925, render/busy/ms 760, render/acquire/ms 162, render/graph/ms 4.36, render/present/ms 3.23, stream/drive/ms 0.86, surface/advance_cpu_ms 0.73, terrain/update/ms 0.56, stream/tick/ms 0.45, terrain/select/ms 0.39, surface/cell_upload_ms 0.29 |
| warm-up | 3.82 | 88.5 | main/busy/ms 63.90, render/thread/ms 62.47, render/busy/ms 60.85, render/graph/ms 3.90, render/present/ms 1.37, stream/drive/ms 1.36, stream/tick/ms 0.85, terrain/update/ms 0.68, terrain/select/ms 0.53, render/acquire/ms 0.25, bigspace/total/ms 0.23, stream/spawn/ms 0.19 |
| warm-up | 3.95 | 28.5 | main/busy/ms 47.96, stream/drive/ms 35.86, render/thread/ms 32.16, render/busy/ms 30.60, render/graph/ms 4.04, render/present/ms 1.23, stream/tick/ms 0.95, terrain/update/ms 0.61, terrain/select/ms 0.48, render/acquire/ms 0.34, bigspace/total/ms 0.29, stream/spawn/ms 0.18 |
| warm-up | 4.00 | 49.5 | render/thread/ms 53.74, render/busy/ms 52.35, main/busy/ms 49.05, stream/drive/ms 35.75, render/graph/ms 4.17, render/present/ms 1.07, stream/tick/ms 0.75, terrain/update/ms 0.63, terrain/select/ms 0.50, render/acquire/ms 0.32, bigspace/total/ms 0.31, bigspace/high_precision/ms 0.20 |
| warm-up | 4.04 | 38.9 | main/busy/ms 29.01, render/thread/ms 28.93, render/busy/ms 26.82, stream/drive/ms 14.89, render/graph/ms 4.33, render/present/ms 1.75, stream/tick/ms 1.25, stream/spawn/ms 1.04, terrain/update/ms 0.68, terrain/select/ms 0.55, render/acquire/ms 0.36, bigspace/total/ms 0.27 |
| warm-up | 4.07 | 29.0 | pipelines/wait_ms 973, render/thread/ms 42.48, render/busy/ms 40.14, main/busy/ms 22.86, pipelines/compile_ms 9.27, pipelines/front_ms 8.42, render/graph/ms 5.03, stream/drive/ms 3.51, render/present/ms 1.98, stream/tick/ms 1.56, stream/spawn/ms 1.15, pipelines/driver_ms 0.85 |
| warm-up | 4.12 | 41.8 | pipelines/wait_ms 2811, pipelines/compile_ms 268, pipelines/front_ms 246, render/thread/ms 53.67, render/busy/ms 52.00, main/busy/ms 41.57, pipelines/driver_ms 21.21, stream/drive/ms 19.83, render/graph/ms 4.64, stream/spawn/ms 1.52, render/present/ms 1.40, stream/tick/ms 1.21 |
| warm-up | 4.16 | 41.9 | pipelines/wait_ms 1838, pipelines/compile_ms 258, pipelines/front_ms 238, render/thread/ms 67.20, render/busy/ms 65.87, main/busy/ms 43.72, stream/drive/ms 21.59, pipelines/driver_ms 20.36, render/graph/ms 3.74, stream/spawn/ms 3.23, render/present/ms 0.98, stream/tick/ms 0.93 |
| warm-up | 4.24 | 66.2 | main/busy/ms 54.22, render/thread/ms 40.40, render/busy/ms 38.60, stream/drive/ms 32.15, stream/spawn/ms 4.83, render/graph/ms 3.82, render/present/ms 1.51, stream/tick/ms 1.42, terrain/update/ms 1.10, terrain/select/ms 0.85, stream/activate/ms 0.64, bigspace/total/ms 0.46 |
| warm-up | 4.28 | 38.2 | pipelines/wait_ms 5217, pipelines/compile_ms 173, pipelines/front_ms 167, render/thread/ms 52.73, render/busy/ms 50.91, main/busy/ms 35.98, stream/drive/ms 7.19, pipelines/driver_ms 6.73, render/graph/ms 3.93, stream/spawn/ms 3.19, render/present/ms 1.50, stream/tick/ms 1.38 |
| warm-up | 4.30 | 28.5 | pipelines/wait_ms 7337, pipelines/compile_ms 201, pipelines/front_ms 192, render/thread/ms 40.10, render/busy/ms 39.01, main/busy/ms 28.13, pipelines/driver_ms 8.24, stream/drive/ms 3.27, render/graph/ms 2.38, terrain/update/ms 1.05, stream/activate/ms 1.05, render/present/ms 0.88 |
| warm-up | 4.37 | 48.5 | main/busy/ms 42.94, render/thread/ms 37.91, render/busy/ms 36.01, stream/drive/ms 29.11, render/graph/ms 5.83, terrain/update/ms 4.75, render/present/ms 1.66, stream/tick/ms 0.90, terrain/select/ms 0.53, bigspace/total/ms 0.33, stream/activate/ms 0.30, render/acquire/ms 0.24 |
| warm-up | 4.45 | 62.1 | pipelines/wait_ms 3036, pipelines/compile_ms 164, pipelines/front_ms 154, main/busy/ms 39.89, render/thread/ms 35.26, render/busy/ms 33.93, stream/drive/ms 25.77, pipelines/driver_ms 9.70, render/graph/ms 3.15, terrain/update/ms 1.42, terrain/select/ms 1.23, render/present/ms 1.08 |
| warm-up | 4.58 | 93.1 | pipelines/wait_ms 6594, pipelines/compile_ms 118, pipelines/front_ms 107, main/busy/ms 86.78, stream/drive/ms 73.95, render/thread/ms 25.07, render/busy/ms 24.03, pipelines/driver_ms 10.62, render/graph/ms 2.60, render/present/ms 0.84, stream/tick/ms 0.75, stream/spawn/ms 0.74 |
| warm-up | 4.79 | 34.0 | render/thread/ms 38.79, render/busy/ms 36.75, main/busy/ms 36.73, stream/drive/ms 15.81, render/graph/ms 5.75, stream/spawn/ms 5.38, render/present/ms 1.57, stream/tick/ms 1.16, terrain/update/ms 0.78, terrain/select/ms 0.62, stream/activate/ms 0.54, render/acquire/ms 0.48 |
| warm-up | 4.81 | 28.0 | render/thread/ms 38.37, render/busy/ms 36.56, main/busy/ms 32.81, stream/drive/ms 14.11, render/graph/ms 4.72, stream/spawn/ms 4.06, render/present/ms 1.48, terrain/update/ms 0.87, terrain/select/ms 0.70, stream/tick/ms 0.68, stream/activate/ms 0.64, bigspace/total/ms 0.49 |
| warm-up | 4.96 | 53.6 | main/busy/ms 49.63, render/thread/ms 45.93, render/busy/ms 43.98, render/graph/ms 6.25, stream/drive/ms 3.35, render/present/ms 1.65, stream/spawn/ms 1.57, stream/tick/ms 0.78, physics/insert/ms 0.75, bigspace/total/ms 0.70, terrain/update/ms 0.67, terrain/select/ms 0.53 |
| warm-up | 5.03 | 32.9 | render/thread/ms 41.67, render/busy/ms 39.69, main/busy/ms 26.80, render/graph/ms 6.19, stream/drive/ms 5.89, stream/spawn/ms 2.36, render/present/ms 1.69, stream/tick/ms 0.96, bigspace/total/ms 0.70, terrain/update/ms 0.56, terrain/select/ms 0.46, bigspace/high_precision/ms 0.34 |
| warm-up | 5.09 | 37.0 | render/thread/ms 51.10, render/busy/ms 49.41, main/busy/ms 25.69, render/graph/ms 5.33, stream/drive/ms 5.29, stream/spawn/ms 2.32, render/present/ms 1.44, stream/tick/ms 1.26, bigspace/total/ms 1.08, terrain/update/ms 0.54, terrain/select/ms 0.44, bigspace/high_precision/ms 0.38 |
| warm-up | 5.13 | 35.2 | pipelines/wait_ms 2385, pipelines/compile_ms 154, pipelines/front_ms 147, render/thread/ms 54.45, render/busy/ms 53.23, main/busy/ms 25.73, pipelines/driver_ms 7.30, stream/drive/ms 5.06, render/graph/ms 4.32, stream/spawn/ms 2.12, stream/tick/ms 1.26, bigspace/total/ms 1.10 |
| warm-up | 5.22 | 26.6 | pipelines/wait_ms 1536, pipelines/compile_ms 81.89, pipelines/front_ms 77.90, render/thread/ms 39.97, render/busy/ms 38.70, main/busy/ms 24.75, stream/drive/ms 6.19, render/graph/ms 4.46, pipelines/driver_ms 3.99, stream/spawn/ms 2.23, render/present/ms 1.04, stream/tick/ms 0.80 |
| warm-up | 5.29 | 32.5 | render/thread/ms 39.96, main/busy/ms 38.62, render/busy/ms 37.55, stream/drive/ms 18.59, render/graph/ms 7.92, stream/spawn/ms 2.30, render/present/ms 2.08, stream/tick/ms 1.31, bigspace/total/ms 0.78, terrain/update/ms 0.65, terrain/select/ms 0.55, bigspace/high_precision/ms 0.44 |
| warm-up | 5.33 | 40.2 | render/thread/ms 53.92, render/busy/ms 52.16, main/busy/ms 42.27, stream/drive/ms 18.93, render/graph/ms 6.36, stream/spawn/ms 2.38, render/present/ms 1.47, stream/tick/ms 1.28, bigspace/total/ms 1.05, terrain/update/ms 0.74, terrain/select/ms 0.63, bigspace/high_precision/ms 0.46 |
| warm-up | 5.44 | 28.5 | main/busy/ms 33.45, render/thread/ms 20.59, render/busy/ms 19.27, stream/drive/ms 16.39, render/graph/ms 5.95, stream/spawn/ms 2.08, render/present/ms 1.12, bigspace/total/ms 0.59, stream/tick/ms 0.57, terrain/update/ms 0.46, terrain/select/ms 0.39, bigspace/high_precision/ms 0.27 |
| warm-up | 5.61 | 28.4 | pipelines/wait_ms 1984, render/thread/ms 34.83, main/busy/ms 34.13, render/busy/ms 32.72, pipelines/compile_ms 15.08, pipelines/front_ms 12.77, render/graph/ms 7.42, stream/drive/ms 3.55, pipelines/driver_ms 2.31, stream/spawn/ms 2.05, render/present/ms 1.88, terrain/update/ms 0.78 |
| warm-up | 5.67 | 31.6 | render/thread/ms 43.30, render/busy/ms 42.10, main/busy/ms 39.82, render/graph/ms 4.65, stream/drive/ms 2.58, render/present/ms 0.91, stream/spawn/ms 0.80, stream/tick/ms 0.62, bigspace/total/ms 0.60, physics/insert/ms 0.51, terrain/update/ms 0.51, terrain/select/ms 0.42 |
| warm-up | 5.70 | 33.9 | render/thread/ms 50.99, render/busy/ms 49.78, main/busy/ms 38.89, render/graph/ms 4.94, stream/drive/ms 3.78, stream/spawn/ms 1.58, physics/insert/ms 0.99, render/present/ms 0.95, stream/tick/ms 0.76, bigspace/total/ms 0.54, terrain/update/ms 0.48, terrain/select/ms 0.40 |
| warm-up | 5.80 | 44.6 | pipelines/wait_ms 1611, main/busy/ms 38.66, render/thread/ms 37.84, render/busy/ms 36.22, pipelines/compile_ms 29.01, pipelines/front_ms 27.80, stream/drive/ms 13.79, render/graph/ms 5.39, stream/spawn/ms 2.84, render/present/ms 1.37, pipelines/driver_ms 1.21, stream/tick/ms 1.09 |
| warm-up | 7.13 | 37.8 | main/busy/ms 40.09, render/thread/ms 32.33, render/busy/ms 31.02, render/graph/ms 7.38, terrain/update/ms 4.56, render/present/ms 1.11, physics/insert/ms 1.06, stream/drive/ms 1.02, physics/step/ms 0.81, bigspace/total/ms 0.68, anim/evaluate_cpu/ms 0.44, stream/tick/ms 0.41 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_shadowoff\balmora_night@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_shadows-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 6%
- during the run: other processes 16% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **93%** (min 74%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.3 W · 77 °C (max 80) · VRAM 5191 MiB · throttled 100% of samples
