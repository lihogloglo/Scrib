# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1621 frames over 15.0 s · **median 9.17 ms (109 fps)** · mean 9.25 · p95 10.57 · p99 11.33 (1% low 88 fps) · worst 12.93 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 490 frames over 9.4 s, worst 2584.3 ms
- hitches (> 25.0 ms): **0** measured, 36 in warm-up
- timed GPU passes: 7.17 ms of a 9.25 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.17 ms, limit 11 ms
- pass worst frame 12.93 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.047 | 2.148 | 2.177 | 0.95 | 0.067 |
| dlss_super_resolution | 1.963 | 2.060 | 2.081 | 0.95 | 0.179 |
| early prepass | 0.771 | 0.809 | 0.964 | 0.95 | 0.100 |
| atmosphere_luts | 0.339 | 0.355 | 0.488 | 0.95 | 0.004 |
| bloom | 0.307 | 0.323 | 0.338 | 0.95 | 0.064 |
| water_velocity | 0.230 | 0.242 | 0.265 | 0.95 | 0.002 |
| main_transmissive_pass_3d | 0.222 | 0.233 | 0.250 | 0.95 | 0.009 |
| water_sspr | 0.211 | 0.221 | 0.224 | 0.95 | 0.003 |
| ssao | 0.186 | 0.195 | 0.205 | 0.95 | 0.008 |
| uniform_allocation | 0.167 | 0.175 | 0.185 | 0.95 | 0.037 |
| water_fft | 0.129 | 0.135 | 0.137 | 0.95 | 0.003 |
| bin_unpacking | 0.126 | 0.132 | 0.143 | 0.95 | 0.024 |
| upscaling | 0.114 | 0.119 | 0.128 | 0.95 | 0.004 |
| tonemapping | 0.084 | 0.088 | 0.092 | 0.95 | 0.004 |
| clustering | 0.075 | 0.078 | 0.079 | 0.95 | 0.022 |
| render_sky | 0.051 | 0.054 | 0.054 | 0.95 | 0.002 |
| early_mesh_preprocessing | 0.038 | 0.040 | 0.041 | 0.95 | 0.007 |
| water_snapshot | 0.032 | 0.034 | 0.035 | 0.95 | 0.002 |
| water_mips | 0.022 | 0.023 | 0.024 | 0.95 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.029 | 0.95 | 0.007 |
| main_transparent_pass_3d | 0.020 | 0.021 | 0.029 | 0.95 | 0.058 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.95 | 0.004 |
| night_sky | 0.003 | 0.003 | 0.004 | 0.95 | 0.002 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.95 | 0.003 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.95 | 0.001 |
| blas_build | — | — | — | - | — |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.382 | 9.569 | 11.93 | 13586.6 |
| render/busy/ms | 7.406 | 8.603 | 11.23 | 12005.0 |
| main/busy/ms | 7.058 | 8.078 | 10.06 | 11441.4 |
| render/graph/ms | 4.254 | 5.223 | 7.20 | 6895.1 |
| pipelines/wait_ms | 3.201 | 0.000 | 5188.69 | 5188.7 |
| render/present/ms | 0.506 | 1.220 | 1.96 | 820.1 |
| render/acquire/ms | 0.470 | 1.392 | 1.93 | 761.5 |
| bigspace/total/ms | 0.402 | 1.279 | 2.06 | 651.6 |
| terrain/update/ms | 0.238 | 0.318 | 0.58 | 385.7 |
| stream/drive/ms | 0.219 | 0.318 | 0.71 | 354.2 |
| terrain/select/ms | 0.207 | 0.275 | 0.44 | 334.7 |
| far/select/ms | 0.197 | 0.282 | 0.45 | 320.1 |
| stream/tick/ms | 0.187 | 0.277 | 0.68 | 303.5 |
| controller/feed/ms | 0.168 | 0.217 | 0.45 | 272.5 |
| bigspace/high_precision/ms | 0.164 | 0.711 | 1.84 | 265.6 |
| controller/move/ms | 0.119 | 0.188 | 0.45 | 193.5 |
| bigspace/low_precision/ms | 0.105 | 0.267 | 1.76 | 170.2 |
| pipelines/compile_ms | 0.076 | 0.000 | 123.56 | 123.6 |
| anim/evaluate/ms | 0.076 | 0.181 | 1.41 | 123.0 |
| pipelines/front_ms | 0.075 | 0.000 | 120.87 | 120.9 |
| anim/evaluate_cpu/ms | 0.067 | 0.079 | 0.44 | 108.7 |
| physics/step/ms | 0.027 | 0.056 | 0.91 | 43.0 |
| weather/eval/ms | 0.012 | 0.017 | 0.06 | 20.0 |
| script/update/ms | 0.008 | 0.013 | 0.17 | 13.3 |
| controller/avoid/ms | 0.006 | 0.008 | 0.20 | 10.2 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 8.4 |
| anim/apply/ms | 0.005 | 0.006 | 0.03 | 7.4 |
| script/ui/ms | 0.004 | 0.005 | 0.02 | 6.6 |
| physics/insert/ms | 0.004 | 0.000 | 0.81 | 6.1 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 3.2 |
| pipelines/driver_ms | 0.002 | 0.000 | 2.69 | 2.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 1.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.3 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.6 |
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
| anim/evaluated/count | count | 7 | 7 | 7 |
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
| frame/work/ms | ms | 7.65 | 11.23 | 7.68 |
| gpu/timed/ms | ms | 7.59 | 8.78 | 7.68 |
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
| phases/shadow/batch_sets | count | 33 | 33 | 33 |
| phases/shadow/bins | count | 2040 | 2040 | 2040 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5404 | 5404 | 5404 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 33 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 188 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 174 | 174 | 174 |
| pipelines/waiting | count | 0 | 1 | 0 |
| preprocess/bind_groups | count | 0 | 0 | 0 |
| preprocess/bind_groups_created | count | 0 | 0 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 31.33 | 44.23 | 27.11 |
| process/mem_usage | GiB | 4.25 | 4.29 | 4.29 |
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
| system/cpu_usage | % | 39.25 | 56.12 | 34.25 |
| system/mem_usage | % | 61.90 | 62.03 | 61.98 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2463 | 2463 | 2463 |
| textures/images | count | 1910 | 1910 | 1910 |
| views/all/batch_sets | count | 91 | 91 | 91 |
| views/all/bins | count | 5210 | 5210 | 5210 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 14.7 k | 14.7 k |
| views/all/pipelines | count | 87 | 87 | 87 |
| views/all/pipelines_distinct | count | 77 | 77 | 77 |
| views/all/work_items | count | 14.9 k | 14.9 k | 14.9 k |
| views/cascade/batch_sets | count | 33 | 33 | 33 |
| views/cascade/bins | count | 2040 | 2040 | 2040 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5404 | 5404 | 5404 |
| views/cascade/pipelines | count | 33 | 33 | 33 |
| views/cascade/work_items | count | 5404 | 5404 | 5404 |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.08 | 2 | 137 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.05 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0 | 1 | 0 |
| physics/inserts | count | 0.13 | 80 | 216 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.56 | 1 | 900 |
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
| surface/shelter_redraws | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 36, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.58 | 2584.3 | main/busy/ms 2558, surface/advance_cpu_ms 67.68, surface/cell_upload_ms 0.69, stream/drive/ms 0.39, stream/tick/ms 0.12, bigspace/total/ms 0.10, bigspace/high_precision/ms 0.05, weather/eval/ms 0.03, script/update/ms 0.03, script/input/ms 0.01, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.40 | 820.5 | main/busy/ms 2594, render/thread/ms 782, render/busy/ms 686, render/acquire/ms 93.61, surface/advance_cpu_ms 67.68, render/graph/ms 4.71, render/present/ms 2.14, stream/drive/ms 1.07, surface/cell_upload_ms 0.69, stream/tick/ms 0.60, terrain/update/ms 0.54, terrain/select/ms 0.36 |
| warm-up | 3.67 | 89.3 | pipelines/compile_ms 278, pipelines/front_ms 253, pipelines/wait_ms 203, main/busy/ms 61.50, render/thread/ms 57.66, render/busy/ms 55.56, pipelines/driver_ms 25.10, render/graph/ms 5.22, render/present/ms 1.70, stream/drive/ms 1.09, stream/tick/ms 0.82, terrain/update/ms 0.74 |
| warm-up | 3.81 | 59.7 | render/thread/ms 50.32, main/busy/ms 49.57, render/busy/ms 48.82, stream/drive/ms 37.51, render/graph/ms 3.41, render/present/ms 1.29, stream/tick/ms 1.00, terrain/update/ms 0.57, terrain/select/ms 0.43, stream/spawn/ms 0.24, bigspace/total/ms 0.22, render/acquire/ms 0.21 |
| warm-up | 3.89 | 63.0 | render/thread/ms 45.61, render/busy/ms 43.59, main/busy/ms 43.24, stream/drive/ms 27.94, render/graph/ms 5.47, render/present/ms 1.75, stream/spawn/ms 1.12, stream/tick/ms 1.01, terrain/update/ms 0.66, terrain/select/ms 0.52, render/acquire/ms 0.26, bigspace/total/ms 0.23 |
| warm-up | 3.99 | 74.2 | pipelines/wait_ms 5652, pipelines/compile_ms 341, pipelines/front_ms 319, render/thread/ms 60.86, render/busy/ms 58.32, main/busy/ms 33.36, pipelines/driver_ms 22.42, stream/drive/ms 11.34, render/graph/ms 5.91, render/present/ms 1.94, stream/tick/ms 1.31, terrain/update/ms 0.80 |
| warm-up | 4.02 | 38.1 | pipelines/wait_ms 7072, pipelines/compile_ms 120, pipelines/front_ms 111, render/thread/ms 69.92, render/busy/ms 68.22, main/busy/ms 44.28, stream/drive/ms 25.75, pipelines/driver_ms 9.66, render/graph/ms 4.26, render/present/ms 1.33, terrain/update/ms 0.88, stream/tick/ms 0.82 |
| warm-up | 4.08 | 54.0 | pipelines/wait_ms 1673, render/thread/ms 50.30, render/busy/ms 49.00, main/busy/ms 45.22, stream/drive/ms 26.83, pipelines/compile_ms 15.78, pipelines/front_ms 14.43, render/graph/ms 2.56, bigspace/total/ms 1.40, pipelines/driver_ms 1.35, bigspace/low_precision/ms 1.23, stream/spawn/ms 1.05 |
| warm-up | 4.15 | 60.7 | pipelines/wait_ms 333, pipelines/compile_ms 141, pipelines/front_ms 134, main/busy/ms 54.99, render/thread/ms 36.89, render/busy/ms 34.66, stream/drive/ms 32.43, pipelines/driver_ms 6.53, render/graph/ms 5.62, stream/spawn/ms 3.73, render/present/ms 1.96, stream/tick/ms 0.93 |
| warm-up | 4.19 | 34.6 | pipelines/wait_ms 1037, pipelines/compile_ms 167, pipelines/front_ms 159, main/busy/ms 58.53, render/thread/ms 45.63, render/busy/ms 43.37, stream/drive/ms 33.39, pipelines/driver_ms 7.53, render/graph/ms 4.70, stream/spawn/ms 3.88, render/present/ms 1.99, stream/tick/ms 0.90 |
| warm-up | 4.25 | 61.4 | pipelines/wait_ms 2236, pipelines/compile_ms 118, pipelines/front_ms 106, main/busy/ms 54.56, render/thread/ms 48.77, render/busy/ms 47.46, stream/drive/ms 31.68, pipelines/driver_ms 11.73, render/graph/ms 3.06, stream/spawn/ms 2.58, render/present/ms 1.08, stream/activate/ms 0.69 |
| warm-up | 4.29 | 26.0 | render/thread/ms 30.61, render/busy/ms 28.92, main/busy/ms 19.97, render/graph/ms 4.10, stream/drive/ms 3.40, render/present/ms 1.40, stream/activate/ms 1.04, bigspace/total/ms 1.02, terrain/update/ms 0.74, stream/tick/ms 0.71, terrain/select/ms 0.55, physics/insert/ms 0.49 |
| warm-up | 4.35 | 43.6 | main/busy/ms 51.40, stream/drive/ms 38.85, render/thread/ms 19.57, render/busy/ms 18.07, render/graph/ms 3.03, render/present/ms 1.30, stream/activate/ms 1.00, stream/tick/ms 0.73, terrain/update/ms 0.57, stream/spawn/ms 0.49, terrain/select/ms 0.46, physics/insert/ms 0.46 |
| warm-up | 4.44 | 69.7 | pipelines/wait_ms 3485, pipelines/compile_ms 209, pipelines/front_ms 200, main/busy/ms 63.72, stream/drive/ms 48.91, render/thread/ms 29.02, render/busy/ms 26.76, pipelines/driver_ms 8.63, render/graph/ms 3.92, render/present/ms 1.99, stream/tick/ms 1.08, stream/spawn/ms 1.06 |
| warm-up | 4.51 | 26.5 | pipelines/wait_ms 2078, main/busy/ms 31.29, render/thread/ms 24.93, render/busy/ms 23.00, pipelines/compile_ms 16.99, pipelines/front_ms 14.54, stream/drive/ms 9.79, render/graph/ms 3.80, terrain/update/ms 3.36, pipelines/driver_ms 2.45, render/present/ms 1.67, stream/spawn/ms 1.60 |
| warm-up | 4.57 | 26.5 | main/busy/ms 27.20, render/thread/ms 23.11, render/busy/ms 21.18, stream/drive/ms 12.79, render/graph/ms 4.76, render/present/ms 1.67, stream/spawn/ms 1.27, stream/activate/ms 1.07, terrain/update/ms 0.65, stream/tick/ms 0.62, physics/insert/ms 0.60, terrain/select/ms 0.53 |
| warm-up | 4.62 | 25.0 | pipelines/wait_ms 2362, render/thread/ms 32.65, render/busy/ms 31.16, main/busy/ms 25.48, pipelines/compile_ms 6.75, pipelines/front_ms 6.09, stream/drive/ms 4.26, render/graph/ms 4.12, stream/spawn/ms 1.69, render/present/ms 1.27, stream/tick/ms 1.02, terrain/update/ms 0.80 |
| warm-up | 4.87 | 26.8 | pipelines/wait_ms 1156, render/thread/ms 41.21, render/busy/ms 39.50, main/busy/ms 35.42, pipelines/compile_ms 18.20, pipelines/front_ms 17.41, render/graph/ms 6.40, stream/drive/ms 5.04, stream/spawn/ms 2.18, render/present/ms 1.46, stream/tick/ms 0.83, pipelines/driver_ms 0.79 |
| warm-up | 4.91 | 41.3 | pipelines/wait_ms 9631, pipelines/compile_ms 65.99, render/thread/ms 61.93, pipelines/front_ms 61.87, render/busy/ms 59.75, main/busy/ms 36.14, render/graph/ms 8.33, stream/drive/ms 6.36, pipelines/driver_ms 4.12, stream/spawn/ms 3.07, render/present/ms 1.79, physics/insert/ms 1.46 |
| warm-up | 4.95 | 30.9 | pipelines/wait_ms 11.1 k, pipelines/compile_ms 102, pipelines/front_ms 95.91, render/thread/ms 61.43, render/busy/ms 59.43, main/busy/ms 33.51, render/graph/ms 8.76, pipelines/driver_ms 5.73, stream/drive/ms 5.12, stream/spawn/ms 3.11, render/present/ms 1.58, physics/insert/ms 1.56 |
| warm-up | 4.99 | 40.2 | pipelines/wait_ms 8801, pipelines/compile_ms 65.43, pipelines/front_ms 60.48, render/thread/ms 51.67, render/busy/ms 49.83, main/busy/ms 44.83, render/graph/ms 6.55, pipelines/driver_ms 4.95, stream/drive/ms 4.03, stream/spawn/ms 2.10, render/present/ms 1.56, physics/insert/ms 1.20 |
| warm-up | 5.02 | 37.2 | pipelines/wait_ms 12.4 k, pipelines/compile_ms 91.71, pipelines/front_ms 85.31, render/thread/ms 57.80, render/busy/ms 56.04, main/busy/ms 42.72, pipelines/driver_ms 6.40, render/graph/ms 5.48, stream/drive/ms 4.45, stream/spawn/ms 2.30, render/present/ms 1.54, physics/insert/ms 1.38 |
| warm-up | 5.11 | 26.2 | pipelines/wait_ms 1591, pipelines/compile_ms 80.57, pipelines/front_ms 74.58, render/thread/ms 42.27, render/busy/ms 39.97, main/busy/ms 31.19, stream/drive/ms 7.94, render/graph/ms 7.06, pipelines/driver_ms 6.00, stream/spawn/ms 2.39, render/present/ms 2.03, stream/tick/ms 1.15 |
| warm-up | 5.15 | 38.9 | pipelines/wait_ms 2704, pipelines/compile_ms 107, pipelines/front_ms 99.57, render/thread/ms 51.41, render/busy/ms 49.15, main/busy/ms 30.58, pipelines/driver_ms 7.05, render/graph/ms 6.41, stream/drive/ms 5.47, stream/spawn/ms 3.02, render/present/ms 1.96, stream/tick/ms 1.46 |
| warm-up | 5.18 | 31.1 | pipelines/wait_ms 1113, render/thread/ms 47.87, render/busy/ms 46.01, main/busy/ms 38.53, pipelines/compile_ms 26.05, pipelines/front_ms 24.99, stream/drive/ms 18.57, render/graph/ms 5.15, stream/spawn/ms 2.79, render/present/ms 1.57, pipelines/driver_ms 1.06, stream/tick/ms 1.00 |
| warm-up | 5.21 | 32.3 | render/thread/ms 39.81, main/busy/ms 38.28, render/busy/ms 38.08, stream/drive/ms 18.26, render/graph/ms 5.39, stream/spawn/ms 2.55, render/present/ms 1.48, physics/insert/ms 1.14, stream/tick/ms 0.99, bigspace/total/ms 0.78, terrain/update/ms 0.64, terrain/select/ms 0.51 |
| warm-up | 5.32 | 25.7 | pipelines/wait_ms 1281, main/busy/ms 39.55, render/thread/ms 30.75, render/busy/ms 28.72, pipelines/compile_ms 21.89, pipelines/front_ms 20.54, render/graph/ms 9.42, stream/drive/ms 3.52, stream/spawn/ms 2.14, render/present/ms 1.62, pipelines/driver_ms 1.35, physics/insert/ms 0.70 |
| warm-up | 5.35 | 33.4 | main/busy/ms 55.90, render/thread/ms 38.92, render/busy/ms 37.02, stream/drive/ms 16.53, render/graph/ms 8.45, stream/spawn/ms 2.70, render/present/ms 1.61, bigspace/total/ms 0.70, physics/insert/ms 0.64, stream/tick/ms 0.57, terrain/update/ms 0.49, terrain/select/ms 0.41 |
| warm-up | 5.38 | 34.8 | pipelines/wait_ms 1853, pipelines/compile_ms 87.55, pipelines/front_ms 84.44, main/busy/ms 42.50, render/thread/ms 36.24, render/busy/ms 34.90, stream/drive/ms 16.26, render/graph/ms 6.02, pipelines/driver_ms 3.11, stream/spawn/ms 2.43, render/present/ms 1.16, bigspace/total/ms 0.72 |
| warm-up | 5.64 | 33.4 | main/busy/ms 35.32, render/thread/ms 27.78, render/busy/ms 25.35, stream/drive/ms 10.64, render/graph/ms 7.37, stream/spawn/ms 2.54, render/present/ms 2.24, stream/tick/ms 0.98, physics/insert/ms 0.89, bigspace/total/ms 0.88, terrain/update/ms 0.60, terrain/select/ms 0.50 |
| warm-up | 5.67 | 32.9 | render/thread/ms 38.74, render/busy/ms 36.81, main/busy/ms 31.67, render/graph/ms 7.34, stream/drive/ms 4.85, stream/spawn/ms 2.92, render/present/ms 1.61, stream/tick/ms 1.06, terrain/update/ms 0.84, physics/insert/ms 0.83, bigspace/total/ms 0.78, terrain/select/ms 0.71 |
| warm-up | 6.27 | 28.2 | pipelines/wait_ms 6285, pipelines/compile_ms 56.69, pipelines/front_ms 54.06, main/busy/ms 34.07, render/thread/ms 33.12, render/busy/ms 31.80, render/graph/ms 5.85, pipelines/driver_ms 2.63, stream/drive/ms 1.27, render/present/ms 1.10, bigspace/total/ms 0.75, physics/insert/ms 0.66 |
| warm-up | 6.66 | 36.2 | main/busy/ms 54.46, render/thread/ms 27.59, render/busy/ms 26.12, render/graph/ms 9.71, stream/drive/ms 1.43, render/present/ms 1.25, bigspace/total/ms 0.75, stream/tick/ms 0.61, physics/insert/ms 0.59, anim/evaluate_cpu/ms 0.55, terrain/update/ms 0.47, stream/spawn/ms 0.46 |
| warm-up | 6.72 | 53.1 | main/busy/ms 58.34, render/thread/ms 48.44, render/busy/ms 46.68, render/graph/ms 9.26, render/present/ms 1.45, stream/drive/ms 1.41, bigspace/total/ms 1.18, anim/evaluate_cpu/ms 0.73, physics/insert/ms 0.61, stream/tick/ms 0.61, anim/evaluate/ms 0.52, bigspace/low_precision/ms 0.51 |
| warm-up | 6.75 | 29.9 | pipelines/wait_ms 7576, pipelines/compile_ms 165, pipelines/front_ms 161, render/thread/ms 50.55, render/busy/ms 49.11, main/busy/ms 39.40, render/graph/ms 7.26, pipelines/driver_ms 3.88, stream/drive/ms 1.28, render/present/ms 1.17, bigspace/total/ms 1.01, physics/insert/ms 0.69 |
| warm-up | 7.38 | 25.4 | pipelines/wait_ms 2511, pipelines/compile_ms 40.12, pipelines/front_ms 37.53, render/thread/ms 34.90, render/busy/ms 33.51, main/busy/ms 28.81, render/graph/ms 9.00, pipelines/driver_ms 2.59, bigspace/total/ms 1.25, render/present/ms 1.19, physics/insert/ms 0.98, stream/drive/ms 0.96 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_envmap\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **97%** (min 87%) · 2580 MHz (min 2580) · memory 8001 MHz · 95.8 W · 78 °C (max 80) · VRAM 5461 MiB · throttled 100% of samples
