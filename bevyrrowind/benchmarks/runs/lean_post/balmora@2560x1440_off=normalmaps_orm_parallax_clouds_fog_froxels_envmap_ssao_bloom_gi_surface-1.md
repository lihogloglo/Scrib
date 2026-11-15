# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1675 frames over 15.0 s · **median 8.73 ms (115 fps)** · mean 8.96 · p95 10.88 · p99 11.80 (1% low 85 fps) · worst 16.69 ms · 2 spikes over 1.5× median · 16.0 stutters/min (work 20.0/min)
- warm-up: 294 frames over 8.1 s, worst 2747.8 ms
- hitches (> 25.0 ms): **0** measured, 35 in warm-up
- timed GPU passes: 6.24 ms of a 8.96 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.73 ms, limit 11 ms
- pass worst frame 16.69 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.929 | 2.070 | 2.243 | 0.93 | 0.190 |
| main_opaque_pass_3d | 1.866 | 2.003 | 2.245 | 0.93 | 0.069 |
| early prepass | 0.522 | 0.560 | 0.778 | 0.93 | 0.102 |
| atmosphere_luts | 0.329 | 0.354 | 0.481 | 0.93 | 0.004 |
| water_velocity | 0.226 | 0.242 | 0.265 | 0.93 | 0.002 |
| main_transmissive_pass_3d | 0.220 | 0.236 | 0.248 | 0.93 | 0.010 |
| water_sspr | 0.206 | 0.221 | 0.222 | 0.93 | 0.003 |
| uniform_allocation | 0.166 | 0.178 | 0.188 | 0.93 | 0.040 |
| water_fft | 0.128 | 0.137 | 0.140 | 0.93 | 0.003 |
| bin_unpacking | 0.125 | 0.134 | 0.143 | 0.93 | 0.027 |
| upscaling | 0.123 | 0.133 | 0.144 | 0.93 | 0.005 |
| tonemapping | 0.111 | 0.120 | 0.124 | 0.93 | 0.005 |
| clustering | 0.090 | 0.097 | 0.331 | 0.93 | 0.024 |
| render_sky | 0.050 | 0.054 | 0.054 | 0.93 | 0.002 |
| early_mesh_preprocessing | 0.037 | 0.039 | 0.042 | 0.93 | 0.008 |
| water_snapshot | 0.030 | 0.032 | 0.034 | 0.93 | 0.002 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.93 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.021 | 0.023 | 0.93 | 0.008 |
| main_transparent_pass_3d | 0.019 | 0.020 | 0.020 | 0.93 | 0.062 |
| sparse buffer updates | 0.009 | 0.010 | 0.008 | 0.93 | 0.004 |
| auxiliary_views | 0.005 | 0.005 | 0.004 | 0.93 | 0.004 |
| night_sky | 0.003 | 0.003 | 0.003 | 0.93 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.001 | 0.93 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 33.610 | 0.000 | 12909.23 | 56296.8 |
| render/thread/ms | 7.998 | 9.737 | 12.75 | 13396.3 |
| render/busy/ms | 7.294 | 9.067 | 12.08 | 12216.8 |
| main/busy/ms | 7.195 | 8.853 | 13.47 | 12051.1 |
| render/graph/ms | 3.976 | 5.143 | 6.44 | 6660.1 |
| render/present/ms | 0.549 | 1.149 | 2.00 | 918.8 |
| pipelines/compile_ms | 0.515 | 0.000 | 122.62 | 862.9 |
| pipelines/front_ms | 0.498 | 0.000 | 117.88 | 834.6 |
| bigspace/total/ms | 0.493 | 1.379 | 2.11 | 825.3 |
| terrain/update/ms | 0.249 | 0.324 | 0.69 | 417.6 |
| stream/drive/ms | 0.241 | 0.332 | 0.56 | 403.1 |
| terrain/select/ms | 0.216 | 0.279 | 0.65 | 361.5 |
| bigspace/high_precision/ms | 0.213 | 1.055 | 1.88 | 356.9 |
| stream/tick/ms | 0.208 | 0.295 | 0.53 | 348.2 |
| far/select/ms | 0.189 | 0.287 | 0.65 | 316.4 |
| controller/feed/ms | 0.173 | 0.230 | 0.53 | 289.4 |
| render/acquire/ms | 0.156 | 0.638 | 1.38 | 260.7 |
| bigspace/low_precision/ms | 0.141 | 0.591 | 1.57 | 235.8 |
| controller/move/ms | 0.127 | 0.222 | 0.43 | 212.4 |
| anim/evaluate/ms | 0.076 | 0.187 | 1.09 | 127.9 |
| anim/evaluate_cpu/ms | 0.066 | 0.085 | 0.31 | 111.0 |
| physics/step/ms | 0.033 | 0.072 | 0.78 | 55.2 |
| pipelines/driver_ms | 0.017 | 0.000 | 4.74 | 28.2 |
| weather/eval/ms | 0.013 | 0.018 | 0.06 | 21.7 |
| script/update/ms | 0.009 | 0.014 | 0.05 | 15.4 |
| physics/insert/ms | 0.008 | 0.000 | 0.92 | 14.2 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 10.2 |
| script/input/ms | 0.006 | 0.007 | 0.03 | 9.3 |
| anim/apply/ms | 0.005 | 0.007 | 0.04 | 8.1 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 7.4 |
| controller/write/ms | 0.002 | 0.002 | 0.33 | 3.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.04 | 1.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.4 |
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
| crowd/joints_extracted/count | count | 4212 | 8244 | 4212 |
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
| frame/work/ms | ms | 7.36 | 13.47 | 7.55 |
| gpu/timed/ms | ms | 6.69 | 7.96 | 6.77 |
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
| phases/opaque3d/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 22 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 22 | 21 |
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
| physics/pending | count | 0 | 1364 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 164 | 164 | 164 |
| pipelines/waiting | count | 0 | 15 | 0 |
| preprocess/bind_groups | count | 0 | 156 | 0 |
| preprocess/bind_groups_created | count | 0 | 156 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 28.65 | 41.35 | 24.52 |
| process/mem_usage | GiB | 4.04 | 4.06 | 4.01 |
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
| system/cpu_usage | % | 46.67 | 73.72 | 41.21 |
| system/mem_usage | % | 65.87 | 66.62 | 65.79 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2463 | 2463 | 2463 |
| textures/images | count | 1910 | 1910 | 1910 |
| views/all/batch_sets | count | 91 | 93 | 91 |
| views/all/bins | count | 5210 | 5210 | 5210 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 14.7 k | 14.7 k |
| views/all/pipelines | count | 87 | 89 | 87 |
| views/all/pipelines_distinct | count | 77 | 79 | 77 |
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
| views/main/batch_sets | count | 58 | 60 | 58 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 56 | 54 |
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
| anim/events/count | count | 0.09 | 3 | 153 |
| assets/Image/added | count | 0.01 | 13 | 14 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.04 | 75 | 75 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 14 | 14 |
| assets/StandardMaterial/modified | count | 2.96 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.26 | 327 | 430 |
| physics/inserts | count | 0.68 | 140 | 1140 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.54 | 1 | 901 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 2 | 4 |
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

## Hitches (> 25.0 ms; slowest 35, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.75 | 2747.8 | main/busy/ms 2724, surface/advance_cpu_ms 76.92, surface/cell_upload_ms 0.79, stream/drive/ms 0.34, bigspace/total/ms 0.12, stream/tick/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.63 | 878.0 | main/busy/ms 2755, render/thread/ms 846, render/busy/ms 679, render/acquire/ms 165, surface/advance_cpu_ms 76.92, render/graph/ms 4.76, render/present/ms 2.14, stream/drive/ms 1.27, surface/cell_upload_ms 0.79, stream/tick/ms 0.72, terrain/update/ms 0.51, terrain/select/ms 0.36 |
| warm-up | 3.87 | 244.2 | pipelines/wait_ms 3794, render/thread/ms 858, render/busy/ms 690, pipelines/compile_ms 255, pipelines/front_ms 236, render/acquire/ms 165, main/busy/ms 49.55, pipelines/driver_ms 18.45, terrain/update/ms 16.67, render/graph/ms 9.02, render/present/ms 3.08, stream/drive/ms 1.59 |
| warm-up | 4.17 | 89.9 | pipelines/compile_ms 263, pipelines/front_ms 245, pipelines/wait_ms 208, main/busy/ms 58.12, render/thread/ms 54.03, render/busy/ms 52.34, pipelines/driver_ms 18.09, render/graph/ms 3.63, stream/drive/ms 2.50, render/present/ms 1.49, stream/tick/ms 1.10, stream/spawn/ms 0.70 |
| warm-up | 4.25 | 25.1 | main/busy/ms 44.03, render/thread/ms 34.13, render/busy/ms 32.53, stream/drive/ms 31.94, render/graph/ms 5.00, render/present/ms 1.19, stream/tick/ms 0.62, terrain/update/ms 0.57, bigspace/total/ms 0.56, terrain/select/ms 0.46, render/acquire/ms 0.41, bigspace/low_precision/ms 0.38 |
| warm-up | 4.29 | 43.7 | render/thread/ms 48.41, render/busy/ms 46.47, main/busy/ms 43.58, stream/drive/ms 33.10, render/graph/ms 4.22, render/present/ms 1.71, stream/tick/ms 0.76, terrain/update/ms 0.62, terrain/select/ms 0.49, stream/spawn/ms 0.48, render/acquire/ms 0.23, bigspace/total/ms 0.23 |
| warm-up | 4.36 | 55.2 | pipelines/wait_ms 686, pipelines/compile_ms 223, pipelines/front_ms 215, render/thread/ms 37.40, render/busy/ms 35.36, main/busy/ms 32.92, stream/drive/ms 19.59, pipelines/driver_ms 8.40, render/graph/ms 7.22, render/present/ms 1.74, stream/spawn/ms 1.17, stream/tick/ms 0.64 |
| warm-up | 4.47 | 93.6 | pipelines/wait_ms 2049, pipelines/compile_ms 134, pipelines/front_ms 128, render/thread/ms 66.29, render/busy/ms 64.58, main/busy/ms 56.78, stream/drive/ms 25.12, pipelines/driver_ms 5.89, terrain/update/ms 5.10, render/graph/ms 4.01, stream/spawn/ms 2.65, stream/tick/ms 2.49 |
| warm-up | 4.51 | 41.8 | pipelines/wait_ms 2049, pipelines/compile_ms 134, pipelines/front_ms 128, render/thread/ms 78.17, render/busy/ms 76.73, main/busy/ms 74.16, stream/drive/ms 51.60, pipelines/driver_ms 5.89, terrain/update/ms 5.22, render/graph/ms 3.86, stream/spawn/ms 2.35, stream/tick/ms 1.48 |
| warm-up | 4.64 | 122.8 | pipelines/wait_ms 3106, pipelines/compile_ms 199, pipelines/front_ms 168, render/thread/ms 95.42, render/busy/ms 77.83, main/busy/ms 71.62, stream/drive/ms 51.14, pipelines/driver_ms 31.09, render/present/ms 17.19, render/graph/ms 5.05, stream/spawn/ms 2.84, terrain/update/ms 0.83 |
| warm-up | 4.67 | 30.2 | pipelines/wait_ms 7836, pipelines/compile_ms 243, pipelines/front_ms 206, render/thread/ms 89.80, render/busy/ms 72.36, pipelines/driver_ms 37.30, main/busy/ms 34.00, render/present/ms 17.14, stream/drive/ms 12.13, render/graph/ms 4.78, stream/spawn/ms 3.41, stream/tick/ms 0.81 |
| warm-up | 4.71 | 46.4 | pipelines/wait_ms 4730, render/thread/ms 52.60, render/busy/ms 51.32, pipelines/compile_ms 44.07, pipelines/front_ms 37.87, main/busy/ms 34.51, stream/drive/ms 11.71, pipelines/driver_ms 6.21, render/graph/ms 4.48, stream/spawn/ms 2.66, stream/activate/ms 1.16, render/present/ms 1.05 |
| warm-up | 4.76 | 35.9 | pipelines/wait_ms 2174, pipelines/compile_ms 96.78, pipelines/front_ms 90.49, main/busy/ms 40.05, stream/drive/ms 27.19, render/thread/ms 21.91, render/busy/ms 20.58, pipelines/driver_ms 6.29, render/graph/ms 4.20, render/present/ms 1.02, stream/tick/ms 0.67, terrain/update/ms 0.61 |
| warm-up | 4.88 | 69.7 | pipelines/wait_ms 1855, main/busy/ms 67.52, stream/drive/ms 55.90, pipelines/compile_ms 54.81, pipelines/front_ms 52.96, render/thread/ms 26.30, render/busy/ms 24.99, render/graph/ms 3.40, pipelines/driver_ms 1.85, render/present/ms 1.13, stream/tick/ms 0.90, terrain/update/ms 0.52 |
| warm-up | 4.93 | 51.5 | main/busy/ms 50.28, stream/drive/ms 35.42, render/thread/ms 15.87, render/busy/ms 14.72, render/graph/ms 2.80, stream/spawn/ms 2.27, render/present/ms 0.99, stream/tick/ms 0.93, terrain/update/ms 0.62, terrain/select/ms 0.47, bigspace/total/ms 0.28, render/acquire/ms 0.16 |
| warm-up | 4.98 | 29.0 | pipelines/wait_ms 667, pipelines/compile_ms 117, pipelines/front_ms 110, main/busy/ms 28.21, render/thread/ms 23.11, render/busy/ms 21.92, stream/drive/ms 11.10, pipelines/driver_ms 7.01, render/graph/ms 3.40, stream/spawn/ms 2.58, stream/tick/ms 1.15, render/present/ms 1.02 |
| warm-up | 5.12 | 33.2 | pipelines/wait_ms 15.7 k, pipelines/compile_ms 67.01, pipelines/front_ms 60.18, render/thread/ms 39.28, render/busy/ms 37.52, main/busy/ms 29.43, pipelines/driver_ms 6.83, render/graph/ms 5.50, stream/drive/ms 4.99, stream/spawn/ms 2.57, render/present/ms 1.46, stream/tick/ms 0.86 |
| warm-up | 5.23 | 85.2 | pipelines/wait_ms 16.8 k, pipelines/compile_ms 93.58, pipelines/front_ms 80.40, main/busy/ms 73.18, render/thread/ms 52.99, render/busy/ms 50.97, stream/drive/ms 14.02, pipelines/driver_ms 13.17, render/graph/ms 7.45, stream/spawn/ms 4.31, render/present/ms 1.80, stream/tick/ms 0.76 |
| warm-up | 5.26 | 29.1 | pipelines/wait_ms 16.8 k, pipelines/compile_ms 93.58, pipelines/front_ms 80.40, render/thread/ms 61.67, render/busy/ms 60.23, main/busy/ms 57.33, pipelines/driver_ms 13.17, render/graph/ms 6.26, stream/drive/ms 4.04, terrain/update/ms 2.39, stream/spawn/ms 2.27, render/present/ms 1.16 |
| warm-up | 5.29 | 33.0 | pipelines/wait_ms 1321, pipelines/compile_ms 78.72, pipelines/front_ms 74.45, render/thread/ms 47.01, render/busy/ms 45.56, main/busy/ms 36.73, render/graph/ms 5.97, pipelines/driver_ms 4.27, terrain/update/ms 3.94, stream/drive/ms 3.80, stream/spawn/ms 2.10, render/present/ms 1.15 |
| warm-up | 5.43 | 35.2 | pipelines/wait_ms 1245, pipelines/compile_ms 158, pipelines/front_ms 151, render/thread/ms 47.59, render/busy/ms 45.19, main/busy/ms 28.42, render/graph/ms 11.50, stream/drive/ms 8.93, pipelines/driver_ms 6.23, render/present/ms 2.18, stream/spawn/ms 2.08, bigspace/total/ms 0.66 |
| warm-up | 5.47 | 35.2 | pipelines/wait_ms 1245, pipelines/compile_ms 158, pipelines/front_ms 151, render/thread/ms 60.00, render/busy/ms 58.09, main/busy/ms 25.17, render/graph/ms 10.49, pipelines/driver_ms 6.23, stream/drive/ms 4.08, stream/spawn/ms 2.30, render/present/ms 1.68, bigspace/total/ms 0.81 |
| warm-up | 5.51 | 36.9 | render/thread/ms 53.66, render/busy/ms 52.47, main/busy/ms 23.27, render/graph/ms 5.75, stream/drive/ms 4.23, stream/spawn/ms 2.26, render/present/ms 1.00, terrain/update/ms 0.86, bigspace/total/ms 0.82, stream/tick/ms 0.78, terrain/select/ms 0.71, bigspace/high_precision/ms 0.44 |
| warm-up | 5.57 | 29.9 | pipelines/wait_ms 2073, pipelines/compile_ms 137, pipelines/front_ms 132, render/thread/ms 42.20, render/busy/ms 41.03, main/busy/ms 19.98, render/graph/ms 5.87, pipelines/driver_ms 4.93, stream/drive/ms 3.29, stream/spawn/ms 1.63, render/present/ms 0.94, bigspace/total/ms 0.63 |
| warm-up | 5.60 | 26.7 | pipelines/wait_ms 3766, pipelines/compile_ms 163, pipelines/front_ms 157, render/thread/ms 51.03, render/busy/ms 49.12, main/busy/ms 37.04, render/graph/ms 8.90, stream/drive/ms 6.26, pipelines/driver_ms 6.03, stream/spawn/ms 1.71, render/present/ms 1.64, bigspace/total/ms 1.04 |
| warm-up | 5.65 | 53.0 | pipelines/wait_ms 2879, pipelines/compile_ms 70.77, pipelines/front_ms 67.74, render/thread/ms 65.64, render/busy/ms 63.70, main/busy/ms 42.99, render/graph/ms 8.82, stream/drive/ms 6.96, pipelines/driver_ms 3.03, stream/spawn/ms 2.48, render/present/ms 1.63, physics/insert/ms 1.18 |
| warm-up | 5.68 | 31.2 | pipelines/wait_ms 1185, render/thread/ms 63.91, render/busy/ms 62.02, pipelines/compile_ms 45.07, pipelines/front_ms 43.14, main/busy/ms 39.83, render/graph/ms 8.31, stream/drive/ms 4.04, stream/spawn/ms 2.43, pipelines/driver_ms 1.93, render/present/ms 1.55, physics/insert/ms 1.36 |
| warm-up | 5.73 | 43.5 | render/thread/ms 49.88, main/busy/ms 48.46, render/busy/ms 48.15, stream/drive/ms 14.52, render/graph/ms 8.09, terrain/update/ms 5.63, stream/spawn/ms 2.29, render/present/ms 1.48, physics/insert/ms 1.31, bigspace/total/ms 0.77, stream/tick/ms 0.77, terrain/select/ms 0.57 |
| warm-up | 5.76 | 29.9 | render/thread/ms 48.67, render/busy/ms 47.50, main/busy/ms 39.09, stream/drive/ms 14.25, terrain/update/ms 7.08, render/graph/ms 5.89, stream/spawn/ms 2.28, physics/insert/ms 1.06, render/present/ms 0.94, bigspace/total/ms 0.70, stream/tick/ms 0.66, physics/step/ms 0.60 |
| warm-up | 5.88 | 27.4 | main/busy/ms 29.82, render/thread/ms 23.29, render/busy/ms 22.14, stream/drive/ms 15.56, render/graph/ms 7.34, stream/spawn/ms 1.12, render/present/ms 0.94, stream/tick/ms 0.62, bigspace/total/ms 0.55, terrain/update/ms 0.46, terrain/select/ms 0.40, physics/insert/ms 0.31 |
| warm-up | 6.12 | 42.5 | render/thread/ms 52.91, render/busy/ms 51.71, render/graph/ms 30.32, main/busy/ms 18.13, stream/drive/ms 2.44, stream/spawn/ms 1.32, render/present/ms 1.05, stream/tick/ms 0.54, bigspace/total/ms 0.49, terrain/update/ms 0.44, terrain/select/ms 0.38, bigspace/high_precision/ms 0.23 |
| warm-up | 6.20 | 43.1 | pipelines/wait_ms 1059, pipelines/compile_ms 127, pipelines/front_ms 121, render/thread/ms 39.05, main/busy/ms 38.03, render/busy/ms 37.35, stream/drive/ms 13.58, render/graph/ms 6.65, pipelines/driver_ms 5.62, stream/spawn/ms 2.65, render/present/ms 1.45, stream/tick/ms 1.34 |
| warm-up | 6.39 | 32.9 | pipelines/wait_ms 4356, pipelines/compile_ms 58.92, pipelines/front_ms 56.53, main/busy/ms 43.25, render/thread/ms 34.68, render/busy/ms 32.92, render/graph/ms 8.57, pipelines/driver_ms 2.40, stream/drive/ms 1.53, render/present/ms 1.48, bigspace/total/ms 0.92, anim/evaluate_cpu/ms 0.76 |
| warm-up | 6.43 | 40.7 | pipelines/wait_ms 3252, main/busy/ms 56.92, render/thread/ms 50.96, render/busy/ms 49.51, pipelines/compile_ms 38.85, pipelines/front_ms 37.45, render/graph/ms 9.28, pipelines/driver_ms 1.40, stream/drive/ms 1.39, render/present/ms 1.13, bigspace/total/ms 0.83, anim/evaluate_cpu/ms 0.78 |
| warm-up | 6.47 | 42.6 | render/thread/ms 56.30, render/busy/ms 55.13, main/busy/ms 52.38, render/graph/ms 8.17, stream/drive/ms 1.58, render/present/ms 0.95, bigspace/total/ms 0.93, stream/tick/ms 0.66, stream/spawn/ms 0.47, terrain/update/ms 0.45, terrain/select/ms 0.39, controller/feed/ms 0.37 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_post\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface-1 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 18% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **90%** (min 76%) · 2580 MHz (min 2580) · memory 8001 MHz · 89.1 W · 77 °C (max 79) · VRAM 5416 MiB · throttled 100% of samples
