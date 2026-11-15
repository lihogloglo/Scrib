# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1606 frames over 15.0 s · **median 9.25 ms (108 fps)** · mean 9.34 · p95 10.56 · p99 12.00 (1% low 83 fps) · worst 20.16 ms · 4 spikes over 1.5× median · 12.0 stutters/min (work 12.0/min)
- warm-up: 313 frames over 7.8 s, worst 2563.2 ms
- hitches (> 25.0 ms): **0** measured, 27 in warm-up
- timed GPU passes: 7.39 ms of a 9.34 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.25 ms, limit 11 ms
- pass worst frame 20.16 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.100 | 2.142 | 2.186 | 0.98 | 0.056 |
| dlss_super_resolution | 2.021 | 2.062 | 2.084 | 0.98 | 0.184 |
| early prepass | 0.814 | 0.831 | 1.128 | 0.98 | 0.098 |
| atmosphere_luts | 0.345 | 0.352 | 0.468 | 0.98 | 0.003 |
| bloom | 0.316 | 0.323 | 0.339 | 0.98 | 0.066 |
| water_velocity | 0.238 | 0.243 | 0.269 | 0.98 | 0.002 |
| main_transmissive_pass_3d | 0.225 | 0.229 | 0.250 | 0.98 | 0.008 |
| water_sspr | 0.218 | 0.222 | 0.225 | 0.98 | 0.002 |
| ssao | 0.191 | 0.195 | 0.202 | 0.98 | 0.008 |
| uniform_allocation | 0.173 | 0.177 | 0.185 | 0.98 | 0.034 |
| bin_unpacking | 0.134 | 0.137 | 0.143 | 0.98 | 0.025 |
| water_fft | 0.131 | 0.134 | 0.137 | 0.98 | 0.003 |
| upscaling | 0.121 | 0.124 | 0.128 | 0.98 | 0.004 |
| tonemapping | 0.086 | 0.088 | 0.092 | 0.98 | 0.004 |
| clustering | 0.071 | 0.073 | 0.079 | 0.98 | 0.023 |
| render_sky | 0.053 | 0.054 | 0.056 | 0.98 | 0.002 |
| early_mesh_preprocessing | 0.039 | 0.040 | 0.042 | 0.98 | 0.006 |
| water_snapshot | 0.033 | 0.034 | 0.035 | 0.98 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.029 | 0.98 | 0.007 |
| main_transparent_pass_3d | 0.020 | 0.020 | 0.022 | 0.98 | 0.056 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.98 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.006 | 0.98 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.004 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.98 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.008 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 36.785 | 0.000 | 5810.98 | 59076.6 |
| render/thread/ms | 8.453 | 9.652 | 18.84 | 13576.0 |
| render/busy/ms | 7.343 | 8.696 | 18.21 | 11793.6 |
| main/busy/ms | 7.149 | 8.279 | 17.53 | 11481.7 |
| render/graph/ms | 4.203 | 5.318 | 15.20 | 6750.4 |
| pipelines/compile_ms | 0.746 | 0.000 | 120.88 | 1198.2 |
| pipelines/front_ms | 0.722 | 0.000 | 117.46 | 1159.3 |
| render/acquire/ms | 0.622 | 1.456 | 2.47 | 999.1 |
| render/present/ms | 0.488 | 0.624 | 2.08 | 783.3 |
| bigspace/total/ms | 0.417 | 1.226 | 2.06 | 669.4 |
| terrain/update/ms | 0.236 | 0.310 | 0.62 | 378.8 |
| stream/drive/ms | 0.205 | 0.322 | 0.61 | 329.3 |
| terrain/select/ms | 0.205 | 0.267 | 0.59 | 329.2 |
| far/select/ms | 0.187 | 0.284 | 0.50 | 300.7 |
| stream/tick/ms | 0.176 | 0.281 | 0.58 | 282.4 |
| bigspace/high_precision/ms | 0.171 | 0.815 | 1.83 | 274.3 |
| controller/feed/ms | 0.170 | 0.223 | 0.54 | 273.3 |
| controller/move/ms | 0.170 | 0.284 | 0.54 | 273.2 |
| bigspace/low_precision/ms | 0.116 | 0.395 | 1.84 | 185.7 |
| anim/evaluate/ms | 0.093 | 0.226 | 1.25 | 149.4 |
| anim/evaluate_cpu/ms | 0.063 | 0.078 | 0.25 | 100.7 |
| physics/step/ms | 0.029 | 0.061 | 0.44 | 46.7 |
| pipelines/driver_ms | 0.024 | 0.000 | 4.28 | 38.9 |
| weather/eval/ms | 0.012 | 0.017 | 0.06 | 19.6 |
| script/update/ms | 0.007 | 0.012 | 0.04 | 11.8 |
| controller/avoid/ms | 0.006 | 0.008 | 0.02 | 9.5 |
| script/input/ms | 0.005 | 0.006 | 0.23 | 8.4 |
| script/ui/ms | 0.005 | 0.006 | 0.02 | 8.0 |
| anim/apply/ms | 0.004 | 0.006 | 0.07 | 7.0 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 3.4 |
| physics/insert/ms | 0.001 | 0.000 | 0.77 | 2.1 |
| weather/fx/ms | 0.001 | 0.002 | 0.18 | 1.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 1.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
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
| anim/a2/count | count | 0 | 7 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 20 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 15 | 8 |
| crowd/c0/joints/count | count | 272 | 510 | 272 |
| crowd/c0/meshes/count | count | 48 | 90 | 48 |
| crowd/c1/count | count | 123 | 131 | 123 |
| crowd/c1/joints/count | count | 4880 | 5176 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1207 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 4212 | 4212 |
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
| frame/work/ms | ms | 7.67 | 18.21 | 7.86 |
| gpu/timed/ms | ms | 7.61 | 8.94 | 7.86 |
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
| phases/opaque3d/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 22 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 21 | 21 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 33 | 38 | 33 |
| phases/shadow/bins | count | 2039 | 2347 | 2039 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5403 | 5730 | 5403 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 38 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 76 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 167 | 167 | 167 |
| pipelines/waiting | count | 0 | 21 | 0 |
| preprocess/bind_groups | count | 0 | 12 | 0 |
| preprocess/bind_groups_created | count | 0 | 12 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 30.62 | 41.83 | 33.65 |
| process/mem_usage | GiB | 4.17 | 4.24 | 4.24 |
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
| started_load_count |  | 3532 | 3534 | 3534 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.91 | 61.73 | 45.16 |
| system/mem_usage | % | 61.70 | 62.08 | 62.07 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2463 | 2463 | 2463 |
| textures/images | count | 1910 | 1910 | 1910 |
| views/all/batch_sets | count | 91 | 96 | 91 |
| views/all/bins | count | 5209 | 5517 | 5209 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 15.0 k | 14.7 k |
| views/all/pipelines | count | 87 | 92 | 87 |
| views/all/pipelines_distinct | count | 77 | 77 | 77 |
| views/all/work_items | count | 14.9 k | 15.2 k | 14.9 k |
| views/cascade/batch_sets | count | 33 | 38 | 33 |
| views/cascade/bins | count | 2039 | 2347 | 2039 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5403 | 5730 | 5403 |
| views/cascade/pipelines | count | 33 | 38 | 33 |
| views/cascade/work_items | count | 5403 | 5730 | 5403 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 58 | 58 | 58 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 54 | 54 |
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
| anim/bones_written/count | count | 0.12 | 198 | 198 |
| anim/events/count | count | 0.07 | 3 | 105 |
| assets/Image/added | count | 0.02 | 26 | 31 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.07 | 98 | 111 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.02 | 27 | 32 |
| assets/StandardMaterial/modified | count | 3.08 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.68 | 991 | 2705 |
| physics/inserts | count | 0.05 | 21 | 76 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.56 | 2 | 900 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 2 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2563.2 | main/busy/ms 2539, surface/advance_cpu_ms 68.81, surface/cell_upload_ms 0.71, stream/drive/ms 0.34, bigspace/total/ms 0.10, stream/tick/ms 0.10, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.43 | 865.7 | main/busy/ms 2576, render/thread/ms 828, render/busy/ms 723, render/acquire/ms 103, surface/advance_cpu_ms 68.81, render/graph/ms 6.88, render/present/ms 2.29, stream/drive/ms 0.93, surface/cell_upload_ms 0.71, stream/tick/ms 0.50, terrain/update/ms 0.44, terrain/select/ms 0.31 |
| warm-up | 3.56 | 130.2 | render/thread/ms 840, render/busy/ms 733, pipelines/wait_ms 410, pipelines/compile_ms 134, pipelines/front_ms 124, render/acquire/ms 103, main/busy/ms 54.55, terrain/update/ms 24.02, render/graph/ms 11.00, pipelines/driver_ms 9.80, render/present/ms 3.75, stream/drive/ms 1.19 |
| warm-up | 3.79 | 87.2 | pipelines/compile_ms 207, pipelines/front_ms 189, main/busy/ms 60.49, render/thread/ms 52.05, render/busy/ms 50.20, pipelines/driver_ms 17.67, render/graph/ms 4.97, render/present/ms 1.61, stream/drive/ms 1.39, stream/tick/ms 1.13, terrain/update/ms 0.65, terrain/select/ms 0.51 |
| warm-up | 3.92 | 54.1 | main/busy/ms 45.93, render/thread/ms 44.82, render/busy/ms 43.18, stream/drive/ms 32.24, render/graph/ms 3.41, render/present/ms 1.43, stream/tick/ms 1.20, terrain/update/ms 0.73, terrain/select/ms 0.57, bigspace/total/ms 0.30, render/acquire/ms 0.21, bigspace/high_precision/ms 0.13 |
| warm-up | 3.95 | 28.1 | render/thread/ms 59.72, render/busy/ms 58.24, main/busy/ms 48.50, stream/drive/ms 35.47, render/graph/ms 4.61, render/present/ms 1.15, stream/tick/ms 1.00, terrain/update/ms 0.61, terrain/select/ms 0.47, render/acquire/ms 0.34, bigspace/total/ms 0.28, stream/spawn/ms 0.17 |
| warm-up | 4.07 | 72.3 | pipelines/wait_ms 3074, pipelines/compile_ms 285, pipelines/front_ms 267, render/thread/ms 63.79, render/busy/ms 61.08, main/busy/ms 39.22, stream/drive/ms 20.85, pipelines/driver_ms 17.43, render/graph/ms 7.30, render/present/ms 2.31, stream/spawn/ms 1.66, stream/tick/ms 0.95 |
| warm-up | 4.10 | 33.8 | pipelines/wait_ms 3552, pipelines/compile_ms 320, pipelines/front_ms 301, render/thread/ms 65.51, render/busy/ms 63.27, main/busy/ms 54.81, stream/drive/ms 34.06, pipelines/driver_ms 18.76, render/graph/ms 5.78, stream/spawn/ms 2.28, render/present/ms 1.91, stream/tick/ms 1.07 |
| warm-up | 4.23 | 134.6 | pipelines/wait_ms 4441, pipelines/compile_ms 179, pipelines/front_ms 170, render/thread/ms 99.23, render/busy/ms 97.95, main/busy/ms 69.90, stream/drive/ms 46.70, pipelines/driver_ms 9.62, stream/spawn/ms 3.67, render/graph/ms 2.57, render/present/ms 1.09, stream/activate/ms 0.96 |
| warm-up | 4.31 | 47.0 | pipelines/wait_ms 2496, render/thread/ms 46.45, render/busy/ms 44.79, pipelines/compile_ms 38.94, pipelines/front_ms 36.61, main/busy/ms 33.98, stream/drive/ms 15.03, render/graph/ms 6.65, stream/spawn/ms 2.36, pipelines/driver_ms 2.32, render/present/ms 1.40, terrain/update/ms 0.81 |
| warm-up | 4.35 | 39.0 | pipelines/wait_ms 1226, pipelines/compile_ms 54.25, pipelines/front_ms 51.93, main/busy/ms 40.90, stream/drive/ms 27.74, render/thread/ms 17.17, render/busy/ms 15.86, render/graph/ms 2.63, pipelines/driver_ms 2.32, render/present/ms 1.14, stream/spawn/ms 0.82, terrain/update/ms 0.72 |
| warm-up | 4.46 | 64.8 | pipelines/wait_ms 5423, pipelines/compile_ms 110, pipelines/front_ms 103, main/busy/ms 95.54, stream/drive/ms 83.52, render/thread/ms 25.95, render/busy/ms 24.73, pipelines/driver_ms 7.47, render/graph/ms 3.27, stream/tick/ms 1.03, render/present/ms 0.97, terrain/update/ms 0.61 |
| warm-up | 4.50 | 34.3 | pipelines/wait_ms 3056, main/busy/ms 51.94, stream/drive/ms 37.04, pipelines/compile_ms 35.71, pipelines/front_ms 33.43, render/thread/ms 20.42, render/busy/ms 19.28, render/graph/ms 2.94, pipelines/driver_ms 2.27, stream/spawn/ms 1.03, render/present/ms 0.89, stream/tick/ms 0.88 |
| warm-up | 4.70 | 34.3 | pipelines/wait_ms 2159, main/busy/ms 50.30, pipelines/compile_ms 40.68, pipelines/front_ms 39.38, render/thread/ms 38.93, render/busy/ms 37.06, terrain/update/ms 6.53, render/graph/ms 5.22, stream/drive/ms 4.06, stream/spawn/ms 1.93, render/present/ms 1.57, pipelines/driver_ms 1.30 |
| warm-up | 4.74 | 35.6 | pipelines/wait_ms 2159, main/busy/ms 49.06, render/thread/ms 44.00, render/busy/ms 42.10, pipelines/compile_ms 40.68, pipelines/front_ms 39.38, stream/drive/ms 10.20, terrain/update/ms 9.14, render/graph/ms 6.02, stream/spawn/ms 2.73, render/present/ms 1.56, pipelines/driver_ms 1.30 |
| warm-up | 4.93 | 28.7 | pipelines/wait_ms 807, render/thread/ms 44.95, render/busy/ms 43.00, main/busy/ms 21.06, pipelines/compile_ms 20.95, pipelines/front_ms 19.94, render/graph/ms 6.44, stream/drive/ms 4.61, stream/spawn/ms 2.24, render/present/ms 1.63, pipelines/driver_ms 1.01, stream/tick/ms 0.68 |
| warm-up | 4.96 | 31.6 | render/thread/ms 52.66, render/busy/ms 50.94, main/busy/ms 24.96, render/graph/ms 6.02, stream/drive/ms 4.31, stream/spawn/ms 2.68, render/present/ms 1.43, stream/tick/ms 0.76, terrain/update/ms 0.67, bigspace/total/ms 0.60, terrain/select/ms 0.54, bigspace/high_precision/ms 0.30 |
| warm-up | 4.99 | 37.1 | render/thread/ms 49.05, render/busy/ms 47.89, main/busy/ms 22.89, render/graph/ms 4.20, stream/drive/ms 4.13, stream/spawn/ms 2.10, render/present/ms 0.95, stream/tick/ms 0.81, terrain/update/ms 0.76, terrain/select/ms 0.62, bigspace/total/ms 0.52, bigspace/high_precision/ms 0.29 |
| warm-up | 5.17 | 26.9 | main/busy/ms 32.14, render/thread/ms 31.39, render/busy/ms 29.60, stream/drive/ms 14.40, render/graph/ms 5.17, stream/spawn/ms 2.38, render/present/ms 1.59, terrain/update/ms 0.81, stream/tick/ms 0.72, terrain/select/ms 0.68, bigspace/total/ms 0.60, bigspace/high_precision/ms 0.32 |
| warm-up | 5.20 | 30.6 | pipelines/wait_ms 6198, pipelines/compile_ms 180, pipelines/front_ms 173, render/thread/ms 33.82, main/busy/ms 33.68, render/busy/ms 31.80, stream/drive/ms 14.16, pipelines/driver_ms 6.95, render/graph/ms 4.61, stream/spawn/ms 2.10, render/present/ms 1.81, bigspace/total/ms 0.75 |
| warm-up | 5.25 | 35.2 | pipelines/wait_ms 3252, main/busy/ms 39.59, render/thread/ms 31.66, render/busy/ms 29.92, stream/drive/ms 8.63, render/graph/ms 6.78, pipelines/compile_ms 5.98, pipelines/front_ms 5.22, stream/spawn/ms 2.38, render/present/ms 1.46, physics/insert/ms 1.08, stream/tick/ms 0.82 |
| warm-up | 5.35 | 27.5 | pipelines/wait_ms 7400, pipelines/compile_ms 149, pipelines/front_ms 142, main/busy/ms 32.24, render/thread/ms 20.23, render/busy/ms 18.86, stream/drive/ms 16.95, pipelines/driver_ms 6.74, render/graph/ms 4.22, stream/spawn/ms 1.94, render/present/ms 1.14, stream/tick/ms 0.83 |
| warm-up | 5.42 | 26.7 | main/busy/ms 30.63, render/thread/ms 29.70, render/busy/ms 28.03, render/graph/ms 6.12, stream/drive/ms 2.88, stream/spawn/ms 1.51, render/present/ms 1.29, stream/tick/ms 0.75, bigspace/total/ms 0.67, terrain/update/ms 0.47, terrain/select/ms 0.38, render/acquire/ms 0.38 |
| warm-up | 5.47 | 25.3 | pipelines/wait_ms 8812, pipelines/compile_ms 109, pipelines/front_ms 103, main/busy/ms 43.72, render/thread/ms 38.38, render/busy/ms 37.08, stream/drive/ms 7.47, pipelines/driver_ms 5.93, render/graph/ms 4.69, bigspace/total/ms 1.11, stream/spawn/ms 1.08, render/present/ms 1.04 |
| warm-up | 5.50 | 30.6 | pipelines/wait_ms 7948, pipelines/compile_ms 54.55, pipelines/front_ms 51.40, main/busy/ms 41.67, render/thread/ms 40.76, render/busy/ms 39.42, render/graph/ms 4.79, pipelines/driver_ms 3.15, stream/drive/ms 3.06, stream/spawn/ms 1.81, render/present/ms 1.09, bigspace/total/ms 1.07 |
| warm-up | 5.69 | 38.4 | pipelines/wait_ms 1548, render/thread/ms 36.91, main/busy/ms 35.52, render/busy/ms 35.34, pipelines/compile_ms 22.09, pipelines/front_ms 20.80, stream/drive/ms 10.51, render/graph/ms 5.67, stream/spawn/ms 1.80, render/present/ms 1.29, pipelines/driver_ms 1.29, bigspace/total/ms 1.04 |
| warm-up | 7.62 | 32.1 | render/thread/ms 36.18, render/busy/ms 34.84, main/busy/ms 34.45, render/graph/ms 10.67, render/present/ms 1.11, bigspace/total/ms 0.73, physics/insert/ms 0.60, anim/evaluate_cpu/ms 0.49, stream/drive/ms 0.45, terrain/update/ms 0.44, far/select/ms 0.41, stream/tick/ms 0.39 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_envmap\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap-3 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 8%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **96%** (min 83%) · 2574 MHz (min 2565) · memory 8001 MHz · 94.5 W · 78 °C (max 80) · VRAM 5427 MiB · throttled 100% of samples
