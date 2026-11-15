# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1756 frames over 15.0 s · **median 8.42 ms (119 fps)** · mean 8.54 · p95 9.99 · p99 10.86 (1% low 92 fps) · worst 11.96 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 389 frames over 8.8 s, worst 2793.8 ms
- hitches (> 25.0 ms): **0** measured, 32 in warm-up
- timed GPU passes: 6.31 ms of a 8.54 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.42 ms, limit 11 ms
- pass worst frame 11.96 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.942 | 2.068 | 2.221 | 0.94 | 0.181 |
| main_opaque_pass_3d | 1.874 | 1.996 | 2.237 | 0.94 | 0.055 |
| early prepass | 0.574 | 0.611 | 0.786 | 0.94 | 0.100 |
| atmosphere_luts | 0.332 | 0.354 | 0.476 | 0.94 | 0.004 |
| water_velocity | 0.228 | 0.243 | 0.265 | 0.94 | 0.002 |
| main_transmissive_pass_3d | 0.220 | 0.235 | 0.248 | 0.94 | 0.008 |
| water_sspr | 0.207 | 0.220 | 0.222 | 0.94 | 0.002 |
| uniform_allocation | 0.164 | 0.175 | 0.186 | 0.94 | 0.034 |
| water_fft | 0.129 | 0.137 | 0.139 | 0.94 | 0.003 |
| bin_unpacking | 0.126 | 0.135 | 0.142 | 0.94 | 0.023 |
| upscaling | 0.124 | 0.132 | 0.137 | 0.94 | 0.004 |
| tonemapping | 0.112 | 0.120 | 0.125 | 0.94 | 0.005 |
| clustering | 0.080 | 0.085 | 0.327 | 0.94 | 0.023 |
| render_sky | 0.050 | 0.054 | 0.054 | 0.94 | 0.002 |
| early_mesh_preprocessing | 0.037 | 0.040 | 0.042 | 0.94 | 0.006 |
| water_snapshot | 0.031 | 0.033 | 0.034 | 0.94 | 0.002 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.94 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.021 | 0.022 | 0.94 | 0.007 |
| main_transparent_pass_3d | 0.019 | 0.020 | 0.022 | 0.94 | 0.057 |
| sparse buffer updates | 0.008 | 0.009 | 0.008 | 0.94 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.008 | 0.94 | 0.002 |
| auxiliary_views | 0.003 | 0.003 | 0.003 | 0.94 | 0.003 |
| atmosphere_environment | 0.001 | 0.001 | 0.001 | 0.94 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 18.221 | 0.000 | 8625.66 | 31996.3 |
| render/thread/ms | 7.612 | 8.792 | 10.78 | 13367.1 |
| render/busy/ms | 6.864 | 8.057 | 10.04 | 12052.9 |
| main/busy/ms | 6.845 | 8.071 | 10.56 | 12019.2 |
| render/graph/ms | 3.750 | 4.710 | 6.19 | 6585.6 |
| render/present/ms | 0.502 | 1.090 | 1.97 | 882.1 |
| bigspace/total/ms | 0.455 | 1.357 | 2.02 | 799.3 |
| pipelines/compile_ms | 0.263 | 0.000 | 138.86 | 461.1 |
| pipelines/front_ms | 0.255 | 0.000 | 135.19 | 448.1 |
| render/acquire/ms | 0.246 | 0.832 | 1.33 | 432.1 |
| terrain/update/ms | 0.237 | 0.302 | 0.65 | 416.0 |
| stream/drive/ms | 0.222 | 0.311 | 0.61 | 389.6 |
| terrain/select/ms | 0.205 | 0.262 | 0.61 | 360.8 |
| bigspace/high_precision/ms | 0.204 | 1.030 | 1.79 | 359.0 |
| stream/tick/ms | 0.191 | 0.275 | 0.57 | 335.1 |
| far/select/ms | 0.180 | 0.256 | 0.48 | 316.7 |
| controller/feed/ms | 0.170 | 0.221 | 0.45 | 298.3 |
| controller/move/ms | 0.123 | 0.189 | 0.43 | 215.9 |
| bigspace/low_precision/ms | 0.122 | 0.413 | 1.56 | 213.4 |
| anim/evaluate/ms | 0.095 | 0.227 | 1.31 | 166.0 |
| anim/evaluate_cpu/ms | 0.059 | 0.074 | 0.23 | 104.4 |
| physics/step/ms | 0.029 | 0.064 | 1.24 | 51.5 |
| weather/eval/ms | 0.013 | 0.016 | 0.26 | 22.1 |
| script/update/ms | 0.008 | 0.014 | 0.19 | 14.9 |
| pipelines/driver_ms | 0.007 | 0.000 | 3.85 | 13.1 |
| controller/avoid/ms | 0.006 | 0.008 | 0.08 | 10.5 |
| script/ui/ms | 0.006 | 0.007 | 0.10 | 9.7 |
| script/input/ms | 0.005 | 0.007 | 0.06 | 9.4 |
| physics/insert/ms | 0.005 | 0.000 | 0.79 | 9.2 |
| anim/apply/ms | 0.004 | 0.006 | 0.02 | 7.3 |
| controller/write/ms | 0.002 | 0.003 | 0.08 | 3.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.12 | 1.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 1.4 |
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
| crowd/c1/count | count | 122 | 123 | 122 |
| crowd/c1/joints/count | count | 4848 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1088 | 1080 |
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
| frame/work/ms | ms | 7.05 | 10.56 | 6.64 |
| gpu/timed/ms | ms | 6.75 | 7.86 | 6.50 |
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
| physics/pending | count | 0 | 489 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 167 | 167 | 167 |
| pipelines/waiting | count | 0 | 8 | 0 |
| preprocess/bind_groups | count | 0 | 26 | 0 |
| preprocess/bind_groups_created | count | 0 | 26 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 25.79 | 40.38 | 28.71 |
| process/mem_usage | GiB | 4.03 | 4.05 | 4.05 |
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
| system/cpu_usage | % | 40.55 | 66.14 | 45.53 |
| system/mem_usage | % | 65.91 | 66.14 | 66.10 |
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
| anim/events/count | count | 0.08 | 3 | 142 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.82 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.01 | 10 | 15 |
| physics/inserts | count | 0.27 | 114 | 479 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.51 | 1 | 900 |
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

## Hitches (> 25.0 ms; slowest 32, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.79 | 2793.8 | main/busy/ms 2770, surface/advance_cpu_ms 71.85, surface/cell_upload_ms 0.99, stream/drive/ms 0.33, bigspace/total/ms 0.18, stream/tick/ms 0.08, bigspace/high_precision/ms 0.08, script/update/ms 0.03, weather/eval/ms 0.03, anim/evaluate/ms 0.01, weather/apply/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.70 | 908.9 | main/busy/ms 2803, render/thread/ms 871, render/busy/ms 697, render/acquire/ms 172, surface/advance_cpu_ms 71.85, render/graph/ms 4.56, render/present/ms 2.74, surface/cell_upload_ms 0.99, stream/drive/ms 0.94, stream/tick/ms 0.47, terrain/update/ms 0.44, terrain/select/ms 0.31 |
| warm-up | 4.01 | 120.4 | pipelines/compile_ms 204, pipelines/front_ms 190, render/thread/ms 103, render/busy/ms 88.64, main/busy/ms 74.80, pipelines/driver_ms 14.13, render/acquire/ms 12.42, render/graph/ms 4.90, stream/drive/ms 1.78, render/present/ms 1.48, stream/tick/ms 1.29, pipelines/wait_ms 1.00 |
| warm-up | 4.05 | 29.9 | render/thread/ms 38.93, render/busy/ms 37.43, main/busy/ms 21.41, render/graph/ms 2.55, stream/drive/ms 2.25, render/present/ms 1.20, stream/tick/ms 0.97, terrain/update/ms 0.82, terrain/select/ms 0.68, stream/spawn/ms 0.67, bigspace/total/ms 0.35, render/acquire/ms 0.30 |
| warm-up | 4.12 | 32.9 | main/busy/ms 52.73, stream/drive/ms 39.45, render/thread/ms 36.50, render/busy/ms 34.79, render/graph/ms 5.69, render/present/ms 1.45, stream/tick/ms 0.82, terrain/update/ms 0.64, terrain/select/ms 0.51, stream/spawn/ms 0.34, render/acquire/ms 0.26, bigspace/total/ms 0.18 |
| warm-up | 4.19 | 64.8 | pipelines/wait_ms 623, pipelines/compile_ms 285, pipelines/front_ms 274, render/thread/ms 66.57, render/busy/ms 64.95, main/busy/ms 52.97, stream/drive/ms 39.07, pipelines/driver_ms 11.56, render/graph/ms 4.50, render/present/ms 1.37, stream/tick/ms 0.73, terrain/update/ms 0.68 |
| warm-up | 4.23 | 37.3 | pipelines/wait_ms 2033, pipelines/compile_ms 37.12, pipelines/front_ms 33.75, main/busy/ms 30.99, render/thread/ms 26.49, render/busy/ms 24.77, stream/drive/ms 19.01, render/graph/ms 4.39, pipelines/driver_ms 3.36, render/present/ms 1.46, stream/spawn/ms 1.01, stream/tick/ms 0.69 |
| warm-up | 4.40 | 139.7 | pipelines/wait_ms 4736, pipelines/compile_ms 110, main/busy/ms 104, pipelines/front_ms 104, stream/drive/ms 87.73, render/thread/ms 65.84, render/busy/ms 64.16, pipelines/driver_ms 6.59, render/graph/ms 4.00, stream/tick/ms 1.40, render/present/ms 1.33, terrain/update/ms 0.53 |
| warm-up | 4.43 | 26.3 | main/busy/ms 25.72, render/thread/ms 20.24, render/busy/ms 18.37, stream/drive/ms 13.21, render/graph/ms 5.30, render/present/ms 1.59, stream/spawn/ms 1.01, stream/tick/ms 0.66, terrain/update/ms 0.58, terrain/select/ms 0.44, bigspace/total/ms 0.29, render/acquire/ms 0.28 |
| warm-up | 4.47 | 28.0 | pipelines/wait_ms 4965, pipelines/compile_ms 144, pipelines/front_ms 135, render/thread/ms 24.71, main/busy/ms 24.47, render/busy/ms 23.43, pipelines/driver_ms 8.88, stream/drive/ms 8.40, render/graph/ms 2.39, stream/tick/ms 1.07, stream/spawn/ms 1.03, render/present/ms 0.95 |
| warm-up | 4.52 | 42.4 | pipelines/wait_ms 2002, pipelines/compile_ms 64.24, pipelines/front_ms 61.58, main/busy/ms 41.13, stream/drive/ms 28.40, render/thread/ms 27.13, render/busy/ms 25.80, render/graph/ms 3.90, pipelines/driver_ms 2.66, render/present/ms 1.15, stream/activate/ms 1.02, stream/tick/ms 0.75 |
| warm-up | 4.56 | 30.3 | pipelines/wait_ms 2355, main/busy/ms 29.71, render/thread/ms 19.77, render/busy/ms 18.14, stream/drive/ms 17.46, pipelines/compile_ms 6.74, pipelines/front_ms 5.38, render/graph/ms 4.19, render/present/ms 1.37, pipelines/driver_ms 1.36, stream/tick/ms 1.03, terrain/update/ms 0.82 |
| warm-up | 4.61 | 26.2 | render/thread/ms 35.63, render/busy/ms 33.41, main/busy/ms 24.99, render/graph/ms 5.70, stream/drive/ms 5.01, stream/spawn/ms 2.17, render/present/ms 1.85, stream/tick/ms 1.27, terrain/update/ms 0.67, bigspace/total/ms 0.59, terrain/select/ms 0.53, render/acquire/ms 0.37 |
| warm-up | 4.64 | 33.2 | render/thread/ms 42.51, render/busy/ms 40.24, main/busy/ms 25.71, stream/drive/ms 6.78, render/graph/ms 5.82, stream/spawn/ms 3.81, render/present/ms 1.97, stream/tick/ms 1.56, terrain/update/ms 0.81, terrain/select/ms 0.64, bigspace/total/ms 0.33, render/acquire/ms 0.30 |
| warm-up | 4.68 | 44.6 | pipelines/wait_ms 945, render/thread/ms 46.32, render/busy/ms 44.69, main/busy/ms 27.27, stream/drive/ms 8.32, stream/spawn/ms 4.49, render/graph/ms 3.67, pipelines/compile_ms 3.36, pipelines/front_ms 2.98, stream/tick/ms 2.02, render/present/ms 1.40, terrain/update/ms 0.73 |
| warm-up | 4.74 | 51.8 | pipelines/wait_ms 2414, pipelines/compile_ms 173, pipelines/front_ms 164, render/thread/ms 58.72, render/busy/ms 56.99, main/busy/ms 56.59, stream/drive/ms 29.60, pipelines/driver_ms 8.85, render/graph/ms 4.46, stream/spawn/ms 2.77, render/present/ms 1.53, stream/tick/ms 1.50 |
| warm-up | 4.76 | 28.2 | pipelines/wait_ms 1470, pipelines/compile_ms 169, pipelines/front_ms 161, main/busy/ms 51.30, render/thread/ms 47.52, render/busy/ms 45.71, stream/drive/ms 27.12, pipelines/driver_ms 8.47, render/graph/ms 4.35, render/present/ms 1.59, stream/spawn/ms 1.29, stream/activate/ms 1.20 |
| warm-up | 4.93 | 31.3 | render/thread/ms 38.97, render/busy/ms 37.55, main/busy/ms 25.51, stream/drive/ms 6.50, render/graph/ms 3.62, stream/spawn/ms 2.73, stream/tick/ms 1.46, stream/activate/ms 1.04, render/present/ms 1.02, terrain/update/ms 0.79, physics/insert/ms 0.66, terrain/select/ms 0.60 |
| warm-up | 4.98 | 31.6 | pipelines/wait_ms 5691, pipelines/compile_ms 163, pipelines/front_ms 153, render/thread/ms 31.73, render/busy/ms 30.01, main/busy/ms 28.09, stream/drive/ms 11.14, pipelines/driver_ms 9.74, render/graph/ms 4.03, stream/spawn/ms 2.93, render/present/ms 1.56, stream/tick/ms 0.88 |
| warm-up | 5.14 | 27.7 | pipelines/wait_ms 1031, render/thread/ms 44.73, render/busy/ms 42.73, main/busy/ms 39.20, pipelines/compile_ms 12.13, pipelines/front_ms 11.23, render/graph/ms 6.53, stream/drive/ms 5.49, stream/spawn/ms 2.55, render/present/ms 1.72, physics/insert/ms 1.46, bigspace/total/ms 1.25 |
| warm-up | 5.18 | 42.7 | pipelines/wait_ms 1355, pipelines/compile_ms 61.49, render/thread/ms 60.35, pipelines/front_ms 59.38, render/busy/ms 58.58, main/busy/ms 48.33, render/graph/ms 6.11, stream/drive/ms 5.21, stream/spawn/ms 2.39, pipelines/driver_ms 2.11, render/present/ms 1.45, physics/insert/ms 1.41 |
| warm-up | 5.23 | 42.9 | pipelines/wait_ms 1355, pipelines/compile_ms 61.49, render/thread/ms 61.18, render/busy/ms 59.95, pipelines/front_ms 59.38, main/busy/ms 39.98, stream/drive/ms 4.79, render/graph/ms 4.23, stream/spawn/ms 2.20, pipelines/driver_ms 2.11, stream/tick/ms 1.11, render/present/ms 1.00 |
| warm-up | 5.35 | 38.4 | render/thread/ms 52.50, render/busy/ms 50.44, main/busy/ms 30.63, stream/drive/ms 7.15, render/graph/ms 6.62, stream/spawn/ms 2.48, render/present/ms 1.74, physics/insert/ms 1.11, bigspace/total/ms 0.73, stream/tick/ms 0.72, terrain/update/ms 0.63, terrain/select/ms 0.51 |
| warm-up | 5.41 | 44.2 | pipelines/wait_ms 7917, pipelines/compile_ms 131, pipelines/front_ms 124, render/thread/ms 41.61, render/busy/ms 39.93, main/busy/ms 24.53, pipelines/driver_ms 7.05, render/graph/ms 5.09, stream/drive/ms 3.74, stream/spawn/ms 2.03, render/present/ms 1.44, physics/insert/ms 1.09 |
| warm-up | 5.57 | 32.7 | pipelines/wait_ms 11.3 k, pipelines/compile_ms 47.68, main/busy/ms 47.59, pipelines/front_ms 42.66, render/thread/ms 30.14, render/busy/ms 27.85, stream/drive/ms 14.84, render/graph/ms 7.97, pipelines/driver_ms 5.02, render/present/ms 2.04, physics/insert/ms 1.33, stream/tick/ms 0.69 |
| warm-up | 5.61 | 33.5 | pipelines/wait_ms 15.1 k, pipelines/compile_ms 56.85, pipelines/front_ms 51.53, main/busy/ms 44.51, render/thread/ms 35.56, render/busy/ms 34.06, stream/drive/ms 14.50, render/graph/ms 5.44, pipelines/driver_ms 5.31, render/present/ms 1.24, physics/insert/ms 1.09, stream/tick/ms 0.73 |
| warm-up | 5.66 | 27.4 | main/busy/ms 44.09, render/thread/ms 24.58, render/busy/ms 22.50, render/graph/ms 7.75, stream/drive/ms 2.90, terrain/update/ms 2.51, render/present/ms 1.90, stream/spawn/ms 1.78, physics/insert/ms 1.11, bigspace/total/ms 0.67, stream/tick/ms 0.58, terrain/select/ms 0.40 |
| warm-up | 5.70 | 43.7 | main/busy/ms 41.53, render/thread/ms 37.29, render/busy/ms 35.48, render/graph/ms 5.82, terrain/update/ms 2.50, stream/drive/ms 2.47, render/present/ms 1.60, stream/spawn/ms 1.28, bigspace/total/ms 0.86, physics/insert/ms 0.84, physics/step/ms 0.61, stream/tick/ms 0.59 |
| warm-up | 5.75 | 30.8 | pipelines/wait_ms 3156, pipelines/compile_ms 130, pipelines/front_ms 125, main/busy/ms 40.33, render/thread/ms 21.26, render/busy/ms 20.02, stream/drive/ms 8.47, pipelines/driver_ms 5.22, render/graph/ms 4.37, stream/spawn/ms 1.82, render/present/ms 1.04, bigspace/total/ms 0.84 |
| warm-up | 5.89 | 32.9 | render/thread/ms 41.59, render/busy/ms 39.92, main/busy/ms 30.19, render/graph/ms 6.81, stream/drive/ms 4.20, stream/spawn/ms 1.65, render/present/ms 1.38, stream/tick/ms 1.25, terrain/update/ms 0.85, bigspace/total/ms 0.85, terrain/select/ms 0.65, physics/insert/ms 0.52 |
| warm-up | 5.92 | 27.3 | pipelines/wait_ms 2037, pipelines/compile_ms 145, pipelines/front_ms 139, render/thread/ms 43.54, render/busy/ms 42.44, main/busy/ms 25.83, pipelines/driver_ms 5.90, render/graph/ms 4.43, stream/drive/ms 4.13, stream/tick/ms 1.34, stream/spawn/ms 1.33, render/present/ms 0.88 |
| warm-up | 7.17 | 36.2 | main/busy/ms 39.23, render/thread/ms 31.86, render/busy/ms 30.14, render/graph/ms 9.18, render/present/ms 1.52, bigspace/total/ms 1.33, stream/drive/ms 0.90, physics/insert/ms 0.82, bigspace/low_precision/ms 0.57, anim/evaluate_cpu/ms 0.57, terrain/update/ms 0.47, anim/evaluate/ms 0.44 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_post\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface-3 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 15% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **95%** (min 85%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.3 W · 77 °C (max 80) · VRAM 5198 MiB · throttled 100% of samples
