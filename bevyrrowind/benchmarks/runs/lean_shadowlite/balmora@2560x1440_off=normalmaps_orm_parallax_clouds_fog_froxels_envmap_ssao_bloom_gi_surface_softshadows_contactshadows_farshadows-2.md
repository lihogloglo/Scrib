# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,farshadows,softshadows,contactshadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1738 frames over 15.0 s · **median 8.49 ms (118 fps)** · mean 8.63 · p95 10.37 · p99 11.31 (1% low 88 fps) · worst 14.20 ms · 3 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 290 frames over 7.9 s, worst 2686.6 ms
- hitches (> 25.0 ms): **0** measured, 41 in warm-up
- timed GPU passes: 5.77 ms of a 8.63 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.49 ms, limit 11 ms
- pass worst frame 14.20 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.898 | 2.072 | 2.224 | 0.92 | 0.187 |
| main_opaque_pass_3d | 1.501 | 1.639 | 1.807 | 0.92 | 0.071 |
| early prepass | 0.492 | 0.538 | 0.774 | 0.92 | 0.099 |
| atmosphere_luts | 0.318 | 0.347 | 0.434 | 0.92 | 0.004 |
| water_velocity | 0.222 | 0.242 | 0.265 | 0.92 | 0.002 |
| water_sspr | 0.203 | 0.222 | 0.220 | 0.92 | 0.003 |
| main_transmissive_pass_3d | 0.198 | 0.216 | 0.229 | 0.92 | 0.009 |
| uniform_allocation | 0.165 | 0.180 | 0.191 | 0.92 | 0.038 |
| water_fft | 0.127 | 0.139 | 0.139 | 0.92 | 0.003 |
| bin_unpacking | 0.121 | 0.132 | 0.146 | 0.92 | 0.026 |
| upscaling | 0.120 | 0.131 | 0.135 | 0.92 | 0.005 |
| tonemapping | 0.111 | 0.122 | 0.123 | 0.92 | 0.005 |
| clustering | 0.100 | 0.109 | 0.333 | 0.92 | 0.023 |
| render_sky | 0.049 | 0.054 | 0.054 | 0.92 | 0.002 |
| early_mesh_preprocessing | 0.036 | 0.040 | 0.041 | 0.92 | 0.007 |
| water_snapshot | 0.030 | 0.033 | 0.035 | 0.92 | 0.002 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.018 | 0.020 | 0.022 | 0.92 | 0.008 |
| main_transparent_pass_3d | 0.017 | 0.019 | 0.019 | 0.92 | 0.060 |
| sparse buffer updates | 0.008 | 0.009 | 0.008 | 0.92 | 0.004 |
| auxiliary_views | 0.007 | 0.008 | 0.004 | 0.92 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.004 | 0.92 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.001 | 0.92 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 14.859 | 0.000 | 4231.42 | 25824.7 |
| render/thread/ms | 7.650 | 9.252 | 12.32 | 13295.8 |
| render/busy/ms | 7.026 | 8.572 | 10.47 | 12211.5 |
| main/busy/ms | 6.859 | 8.473 | 11.07 | 11921.0 |
| render/graph/ms | 3.854 | 4.872 | 6.65 | 6697.9 |
| pipelines/compile_ms | 0.594 | 0.000 | 127.26 | 1032.2 |
| pipelines/front_ms | 0.573 | 0.000 | 123.03 | 996.1 |
| render/present/ms | 0.534 | 1.060 | 2.83 | 928.4 |
| bigspace/total/ms | 0.480 | 1.403 | 2.27 | 833.9 |
| terrain/update/ms | 0.245 | 0.318 | 0.55 | 426.3 |
| stream/drive/ms | 0.234 | 0.325 | 0.57 | 407.2 |
| terrain/select/ms | 0.212 | 0.273 | 0.50 | 368.9 |
| bigspace/high_precision/ms | 0.205 | 0.963 | 1.90 | 356.0 |
| stream/tick/ms | 0.203 | 0.288 | 0.54 | 353.0 |
| far/select/ms | 0.176 | 0.260 | 0.45 | 306.3 |
| controller/move/ms | 0.168 | 0.383 | 0.66 | 291.4 |
| controller/feed/ms | 0.166 | 0.228 | 0.42 | 288.5 |
| bigspace/low_precision/ms | 0.137 | 0.523 | 1.76 | 238.8 |
| render/acquire/ms | 0.090 | 0.129 | 0.72 | 155.9 |
| anim/evaluate/ms | 0.086 | 0.231 | 1.50 | 149.0 |
| anim/evaluate_cpu/ms | 0.063 | 0.081 | 0.27 | 109.4 |
| physics/step/ms | 0.028 | 0.067 | 0.45 | 49.0 |
| pipelines/driver_ms | 0.021 | 0.000 | 4.79 | 36.1 |
| weather/eval/ms | 0.013 | 0.018 | 0.18 | 22.1 |
| script/update/ms | 0.009 | 0.014 | 0.22 | 15.5 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 10.7 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 9.2 |
| anim/apply/ms | 0.005 | 0.006 | 0.03 | 7.9 |
| script/ui/ms | 0.004 | 0.006 | 0.03 | 6.7 |
| controller/write/ms | 0.002 | 0.003 | 0.08 | 3.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.08 | 1.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 1.4 |
| physics/insert/ms | 0.001 | 0.000 | 0.54 | 1.1 |
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
| anim/a2/count | count | 0 | 0 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 7 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 8 | 8 |
| crowd/c0/joints/count | count | 272 | 272 | 272 |
| crowd/c0/meshes/count | count | 48 | 48 | 48 |
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
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
| frame/work/ms | ms | 7.09 | 11.07 | 6.94 |
| gpu/timed/ms | ms | 6.30 | 7.14 | 6.37 |
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
| physics/pending | count | 0 | 39 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 169 | 169 | 169 |
| pipelines/waiting | count | 0 | 18 | 0 |
| preprocess/bind_groups | count | 0 | 0 | 0 |
| preprocess/bind_groups_created | count | 0 | 0 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 27.40 | 38.10 | 32.21 |
| process/mem_usage | GiB | 4.00 | 4.02 | 3.99 |
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
| system/cpu_usage | % | 38.56 | 69.87 | 38.72 |
| system/mem_usage | % | 71.38 | 71.53 | 71.31 |
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
| anim/events/count | count | 0.09 | 3 | 152 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.85 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| physics/inserts | count | 0.04 | 35 | 67 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.52 | 1 | 900 |
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

## Hitches (> 25.0 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.69 | 2686.6 | main/busy/ms 2655, surface/advance_cpu_ms 67.44, surface/cell_upload_ms 0.75, stream/drive/ms 0.32, stream/tick/ms 0.10, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.05, script/update/ms 0.03, weather/eval/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.48 | 797.4 | main/busy/ms 2687, render/thread/ms 765, render/busy/ms 598, render/acquire/ms 164, surface/advance_cpu_ms 67.44, render/graph/ms 6.82, render/present/ms 3.58, stream/drive/ms 0.87, surface/cell_upload_ms 0.75, stream/tick/ms 0.48, terrain/update/ms 0.47, terrain/select/ms 0.33 |
| warm-up | 3.61 | 121.6 | render/thread/ms 780, render/busy/ms 611, pipelines/wait_ms 290, render/acquire/ms 164, pipelines/compile_ms 119, pipelines/front_ms 112, main/busy/ms 51.15, terrain/update/ms 17.91, render/graph/ms 9.75, pipelines/driver_ms 7.73, render/present/ms 4.54, stream/drive/ms 0.95 |
| warm-up | 3.65 | 32.4 | render/thread/ms 34.66, render/busy/ms 32.27, main/busy/ms 23.21, terrain/update/ms 8.61, render/graph/ms 3.60, render/present/ms 1.98, stream/drive/ms 1.36, stream/tick/ms 1.10, terrain/select/ms 0.47, render/acquire/ms 0.41, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.13 |
| warm-up | 3.69 | 25.7 | render/thread/ms 34.33, render/busy/ms 31.67, main/busy/ms 27.38, terrain/update/ms 4.67, render/graph/ms 3.08, render/present/ms 2.12, stream/drive/ms 1.01, stream/tick/ms 0.84, render/acquire/ms 0.54, terrain/select/ms 0.47, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.14 |
| warm-up | 3.77 | 51.6 | render/thread/ms 65.53, render/busy/ms 53.98, render/graph/ms 36.87, main/busy/ms 16.03, render/acquire/ms 9.70, terrain/update/ms 4.82, render/present/ms 1.85, stream/drive/ms 1.15, stream/tick/ms 0.98, terrain/select/ms 0.68, bigspace/total/ms 0.19, bigspace/high_precision/ms 0.09 |
| warm-up | 3.92 | 91.1 | main/busy/ms 57.79, render/thread/ms 53.31, render/busy/ms 51.89, render/graph/ms 4.08, stream/drive/ms 1.48, render/present/ms 1.12, stream/tick/ms 1.11, terrain/update/ms 0.73, terrain/select/ms 0.56, render/acquire/ms 0.30, bigspace/total/ms 0.16, bigspace/high_precision/ms 0.08 |
| warm-up | 4.02 | 29.3 | pipelines/wait_ms 738, pipelines/compile_ms 61.22, pipelines/front_ms 56.32, main/busy/ms 48.93, render/thread/ms 41.91, render/busy/ms 40.35, stream/drive/ms 35.98, pipelines/driver_ms 4.90, render/graph/ms 4.33, render/present/ms 1.18, stream/tick/ms 1.08, terrain/update/ms 0.77 |
| warm-up | 4.07 | 47.4 | pipelines/wait_ms 1914, pipelines/compile_ms 76.74, pipelines/front_ms 70.75, main/busy/ms 60.93, render/thread/ms 60.15, render/busy/ms 58.77, stream/drive/ms 45.98, pipelines/driver_ms 6.00, render/graph/ms 4.90, render/present/ms 1.15, stream/tick/ms 1.05, stream/spawn/ms 0.65 |
| warm-up | 4.11 | 44.5 | pipelines/wait_ms 1176, render/thread/ms 72.40, render/busy/ms 70.23, main/busy/ms 43.68, stream/drive/ms 25.99, pipelines/compile_ms 15.52, pipelines/front_ms 14.43, render/graph/ms 7.77, render/present/ms 1.76, stream/spawn/ms 1.46, pipelines/driver_ms 1.09, stream/tick/ms 0.95 |
| warm-up | 4.14 | 27.4 | pipelines/wait_ms 884, pipelines/compile_ms 114, pipelines/front_ms 103, render/thread/ms 55.09, render/busy/ms 53.00, main/busy/ms 26.01, pipelines/driver_ms 10.95, render/graph/ms 6.29, stream/drive/ms 4.06, render/present/ms 1.72, stream/tick/ms 1.69, stream/spawn/ms 1.03 |
| warm-up | 4.25 | 106.0 | pipelines/wait_ms 5253, pipelines/compile_ms 234, pipelines/front_ms 213, render/thread/ms 101, render/busy/ms 99.37, main/busy/ms 33.58, pipelines/driver_ms 20.56, stream/drive/ms 11.41, render/graph/ms 3.05, stream/tick/ms 1.77, render/present/ms 1.17, terrain/update/ms 0.92 |
| warm-up | 4.29 | 47.5 | pipelines/wait_ms 4369, render/thread/ms 121, pipelines/compile_ms 120, render/busy/ms 120, pipelines/front_ms 110, main/busy/ms 30.99, stream/drive/ms 11.78, pipelines/driver_ms 9.61, render/graph/ms 2.66, stream/spawn/ms 1.38, render/present/ms 1.03, stream/tick/ms 1.00 |
| warm-up | 4.32 | 27.1 | render/thread/ms 61.54, render/busy/ms 59.63, main/busy/ms 53.67, stream/drive/ms 36.84, render/graph/ms 4.13, stream/spawn/ms 2.47, render/present/ms 1.66, terrain/update/ms 1.09, stream/tick/ms 0.91, terrain/select/ms 0.79, bigspace/total/ms 0.27, render/acquire/ms 0.25 |
| warm-up | 4.38 | 61.4 | pipelines/wait_ms 1757, pipelines/compile_ms 117, pipelines/front_ms 111, main/busy/ms 53.42, render/thread/ms 48.44, render/busy/ms 46.64, stream/drive/ms 36.45, pipelines/driver_ms 6.36, render/graph/ms 4.27, render/present/ms 1.55, stream/spawn/ms 1.42, stream/tick/ms 1.16 |
| warm-up | 4.44 | 46.3 | pipelines/wait_ms 735, pipelines/compile_ms 59.52, pipelines/front_ms 56.26, main/busy/ms 41.63, render/thread/ms 27.66, render/busy/ms 26.08, stream/drive/ms 21.33, render/graph/ms 4.26, pipelines/driver_ms 3.26, stream/spawn/ms 2.42, render/present/ms 1.26, stream/tick/ms 1.14 |
| warm-up | 4.46 | 28.2 | pipelines/wait_ms 4543, pipelines/compile_ms 119, pipelines/front_ms 113, main/busy/ms 56.91, render/thread/ms 37.68, render/busy/ms 36.22, stream/drive/ms 31.59, pipelines/driver_ms 6.21, render/graph/ms 4.56, stream/spawn/ms 3.67, render/present/ms 1.19, stream/tick/ms 1.13 |
| warm-up | 4.52 | 50.8 | pipelines/wait_ms 4623, pipelines/compile_ms 61.67, pipelines/front_ms 58.40, main/busy/ms 50.47, render/thread/ms 32.84, render/busy/ms 31.56, stream/drive/ms 29.99, pipelines/driver_ms 3.27, render/graph/ms 3.05, stream/spawn/ms 2.83, render/present/ms 1.08, terrain/update/ms 0.83 |
| warm-up | 4.54 | 25.2 | pipelines/wait_ms 816, render/thread/ms 35.26, render/busy/ms 33.56, main/busy/ms 19.56, render/graph/ms 4.71, stream/drive/ms 3.76, pipelines/compile_ms 2.46, pipelines/front_ms 2.14, render/present/ms 1.43, stream/tick/ms 1.05, stream/activate/ms 0.82, terrain/update/ms 0.76 |
| warm-up | 4.65 | 62.5 | pipelines/wait_ms 9074, pipelines/compile_ms 161, pipelines/front_ms 154, main/busy/ms 97.50, stream/drive/ms 86.56, render/thread/ms 19.03, render/busy/ms 17.83, pipelines/driver_ms 7.11, render/graph/ms 3.59, stream/spawn/ms 1.04, render/present/ms 1.03, terrain/update/ms 0.69 |
| warm-up | 4.70 | 48.6 | pipelines/wait_ms 9074, pipelines/compile_ms 161, pipelines/front_ms 154, main/busy/ms 52.83, stream/drive/ms 38.38, render/thread/ms 22.23, render/busy/ms 21.18, pipelines/driver_ms 7.11, render/graph/ms 3.35, stream/spawn/ms 2.13, render/present/ms 0.88, stream/tick/ms 0.71 |
| warm-up | 4.93 | 29.8 | pipelines/wait_ms 4376, render/thread/ms 36.16, render/busy/ms 34.17, pipelines/compile_ms 33.88, pipelines/front_ms 26.54, main/busy/ms 26.06, pipelines/driver_ms 7.34, render/graph/ms 6.06, stream/drive/ms 5.53, stream/spawn/ms 3.22, render/present/ms 1.68, stream/tick/ms 1.21 |
| warm-up | 4.96 | 28.8 | main/busy/ms 38.06, render/thread/ms 37.93, render/busy/ms 36.67, stream/drive/ms 10.16, render/graph/ms 4.45, terrain/update/ms 2.93, stream/spawn/ms 2.15, render/present/ms 1.02, stream/tick/ms 0.95, bigspace/total/ms 0.59, terrain/select/ms 0.57, physics/insert/ms 0.39 |
| warm-up | 4.99 | 36.2 | pipelines/wait_ms 3630, pipelines/compile_ms 85.23, pipelines/front_ms 80.49, main/busy/ms 50.49, render/thread/ms 46.62, render/busy/ms 45.44, stream/drive/ms 9.82, terrain/update/ms 7.36, pipelines/driver_ms 4.74, render/graph/ms 4.70, stream/spawn/ms 2.07, render/present/ms 0.96 |
| warm-up | 5.10 | 36.5 | pipelines/wait_ms 3185, pipelines/compile_ms 130, pipelines/front_ms 126, render/thread/ms 49.17, render/busy/ms 47.85, main/busy/ms 28.75, render/graph/ms 5.88, stream/drive/ms 5.63, pipelines/driver_ms 4.28, stream/spawn/ms 1.92, render/present/ms 1.09, stream/tick/ms 1.04 |
| warm-up | 5.17 | 29.5 | pipelines/wait_ms 4471, pipelines/compile_ms 69.89, pipelines/front_ms 66.30, render/thread/ms 43.54, render/busy/ms 41.49, main/busy/ms 30.28, render/graph/ms 6.39, stream/drive/ms 4.76, pipelines/driver_ms 3.59, stream/spawn/ms 2.56, render/present/ms 1.71, stream/tick/ms 1.08 |
| warm-up | 5.21 | 37.6 | pipelines/wait_ms 3803, pipelines/compile_ms 80.97, pipelines/front_ms 77.58, render/thread/ms 51.78, render/busy/ms 50.11, main/busy/ms 30.33, render/graph/ms 6.19, stream/drive/ms 4.75, pipelines/driver_ms 3.40, stream/spawn/ms 2.32, render/present/ms 1.31, stream/tick/ms 1.14 |
| warm-up | 5.24 | 27.4 | pipelines/wait_ms 8240, pipelines/compile_ms 67.94, pipelines/front_ms 63.99, render/thread/ms 47.66, render/busy/ms 46.40, main/busy/ms 27.17, render/graph/ms 4.67, stream/drive/ms 4.51, pipelines/driver_ms 3.95, stream/spawn/ms 2.33, render/present/ms 1.02, bigspace/total/ms 0.75 |
| warm-up | 5.26 | 27.4 | pipelines/wait_ms 7847, render/thread/ms 46.34, render/busy/ms 45.11, pipelines/compile_ms 43.48, pipelines/front_ms 39.60, main/busy/ms 24.70, render/graph/ms 4.29, stream/drive/ms 4.21, pipelines/driver_ms 3.88, stream/spawn/ms 2.12, render/present/ms 1.04, terrain/update/ms 0.74 |
| warm-up | 5.33 | 27.7 | render/thread/ms 30.77, main/busy/ms 29.43, render/busy/ms 28.41, render/graph/ms 7.09, stream/drive/ms 6.69, stream/spawn/ms 2.78, render/present/ms 2.11, bigspace/total/ms 0.92, stream/tick/ms 0.83, terrain/update/ms 0.73, terrain/select/ms 0.59, bigspace/high_precision/ms 0.41 |
| warm-up | 5.36 | 31.3 | render/thread/ms 39.39, render/busy/ms 36.93, main/busy/ms 26.99, render/graph/ms 7.49, stream/drive/ms 4.21, stream/spawn/ms 2.70, render/present/ms 2.12, bigspace/total/ms 0.87, terrain/update/ms 0.75, stream/tick/ms 0.71, terrain/select/ms 0.61, bigspace/high_precision/ms 0.45 |
| warm-up | 5.39 | 27.9 | pipelines/wait_ms 2579, pipelines/compile_ms 141, pipelines/front_ms 137, render/thread/ms 39.45, render/busy/ms 37.35, main/busy/ms 33.99, stream/drive/ms 14.40, render/graph/ms 7.02, pipelines/driver_ms 3.64, stream/spawn/ms 2.14, render/present/ms 1.76, terrain/update/ms 0.80 |
| warm-up | 5.42 | 32.0 | pipelines/wait_ms 4341, pipelines/compile_ms 151, pipelines/front_ms 146, render/thread/ms 37.04, render/busy/ms 35.50, main/busy/ms 35.16, stream/drive/ms 14.23, render/graph/ms 5.64, pipelines/driver_ms 4.07, stream/spawn/ms 2.04, render/present/ms 1.30, bigspace/total/ms 0.95 |
| warm-up | 5.54 | 31.0 | pipelines/wait_ms 6407, pipelines/compile_ms 152, pipelines/front_ms 145, main/busy/ms 36.94, render/thread/ms 22.98, render/busy/ms 21.54, stream/drive/ms 19.69, pipelines/driver_ms 7.79, render/graph/ms 5.56, stream/spawn/ms 2.35, render/present/ms 1.24, physics/insert/ms 0.90 |
| warm-up | 5.70 | 27.7 | pipelines/wait_ms 2783, render/thread/ms 43.48, render/busy/ms 41.48, main/busy/ms 36.90, pipelines/compile_ms 30.75, pipelines/front_ms 28.91, render/graph/ms 7.74, stream/drive/ms 2.92, pipelines/driver_ms 1.84, render/present/ms 1.66, stream/spawn/ms 1.39, bigspace/total/ms 0.69 |
| warm-up | 5.75 | 25.6 | render/thread/ms 38.44, render/busy/ms 37.22, main/busy/ms 27.09, render/graph/ms 4.81, stream/drive/ms 4.58, stream/spawn/ms 2.69, render/present/ms 1.04, stream/tick/ms 0.86, physics/insert/ms 0.84, terrain/update/ms 0.65, terrain/select/ms 0.53, bigspace/total/ms 0.50 |
| warm-up | 5.86 | 53.0 | main/busy/ms 42.09, render/thread/ms 40.97, render/busy/ms 39.46, stream/drive/ms 14.85, render/graph/ms 5.39, stream/spawn/ms 3.32, stream/tick/ms 1.97, render/present/ms 1.22, bigspace/total/ms 0.84, terrain/update/ms 0.80, physics/insert/ms 0.71, terrain/select/ms 0.67 |
| warm-up | 7.01 | 25.2 | main/busy/ms 30.77, render/thread/ms 28.25, render/busy/ms 26.88, render/graph/ms 9.10, render/present/ms 1.13, stream/drive/ms 1.03, bigspace/total/ms 0.78, physics/step/ms 0.70, stream/tick/ms 0.43, terrain/update/ms 0.42, far/select/ms 0.39, physics/insert/ms 0.38 |
| warm-up | 7.04 | 25.9 | render/thread/ms 36.55, render/busy/ms 35.21, main/busy/ms 29.10, render/graph/ms 6.89, stream/drive/ms 1.24, render/present/ms 1.11, bigspace/total/ms 0.79, terrain/update/ms 0.61, terrain/select/ms 0.52, stream/tick/ms 0.48, stream/spawn/ms 0.38, far/select/ms 0.36 |
| warm-up | 7.55 | 37.7 | pipelines/wait_ms 2235, main/busy/ms 61.67, render/thread/ms 43.07, render/busy/ms 41.55, pipelines/compile_ms 33.58, pipelines/front_ms 32.19, render/graph/ms 8.62, pipelines/driver_ms 1.39, stream/drive/ms 1.22, physics/step/ms 1.19, anim/evaluate_cpu/ms 1.15, render/present/ms 1.14 |
| warm-up | 7.59 | 45.8 | pipelines/wait_ms 481, main/busy/ms 48.57, render/thread/ms 47.80, render/busy/ms 46.60, pipelines/compile_ms 20.87, pipelines/front_ms 19.85, render/graph/ms 7.40, physics/step/ms 1.39, stream/drive/ms 1.26, physics/insert/ms 1.10, pipelines/driver_ms 1.03, bigspace/total/ms 1.02 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_shadowlite\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_softshadows_contactshadows_farshadows-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,softshadows,contactshadows,farshadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **89%** (min 39%) · 2580 MHz (min 2580) · memory 8001 MHz · 87.3 W · 73 °C (max 78) · VRAM 5199 MiB · throttled 100% of samples
