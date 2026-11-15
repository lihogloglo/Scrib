# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1722 frames over 15.0 s · **median 8.61 ms (116 fps)** · mean 8.71 · p95 10.20 · p99 11.06 (1% low 90 fps) · worst 12.28 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 369 frames over 8.7 s, worst 2804.0 ms
- hitches (> 25.0 ms): **0** measured, 29 in warm-up
- timed GPU passes: 6.17 ms of a 8.71 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 8.61 ms, limit 11 ms
- pass worst frame 12.28 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.894 | 2.068 | 2.216 | 0.92 | 0.184 |
| main_opaque_pass_3d | 1.843 | 2.013 | 2.270 | 0.92 | 0.063 |
| early prepass | 0.534 | 0.583 | 0.778 | 0.92 | 0.098 |
| atmosphere_luts | 0.328 | 0.358 | 0.493 | 0.92 | 0.004 |
| water_velocity | 0.223 | 0.244 | 0.266 | 0.92 | 0.002 |
| main_transmissive_pass_3d | 0.215 | 0.235 | 0.248 | 0.92 | 0.009 |
| water_sspr | 0.209 | 0.228 | 0.229 | 0.92 | 0.003 |
| uniform_allocation | 0.163 | 0.178 | 0.188 | 0.92 | 0.037 |
| water_fft | 0.129 | 0.140 | 0.141 | 0.92 | 0.003 |
| bin_unpacking | 0.123 | 0.135 | 0.146 | 0.92 | 0.026 |
| upscaling | 0.121 | 0.133 | 0.138 | 0.92 | 0.005 |
| tonemapping | 0.110 | 0.120 | 0.124 | 0.92 | 0.005 |
| clustering | 0.087 | 0.095 | 0.330 | 0.92 | 0.023 |
| render_sky | 0.049 | 0.054 | 0.054 | 0.92 | 0.002 |
| early_mesh_preprocessing | 0.036 | 0.039 | 0.042 | 0.92 | 0.007 |
| water_snapshot | 0.029 | 0.032 | 0.034 | 0.92 | 0.002 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.92 | 0.002 |
| per_view_shadow_pass | 0.019 | 0.021 | 0.022 | 0.92 | 0.007 |
| main_transparent_pass_3d | 0.018 | 0.020 | 0.020 | 0.92 | 0.059 |
| sparse buffer updates | 0.009 | 0.010 | 0.008 | 0.92 | 0.003 |
| auxiliary_views | 0.005 | 0.006 | 0.003 | 0.92 | 0.004 |
| night_sky | 0.003 | 0.003 | 0.004 | 0.92 | 0.002 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.92 | 0.001 |
| blas_build | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 8.814 | 0.000 | 3264.92 | 15177.0 |
| render/thread/ms | 7.761 | 9.010 | 11.00 | 13363.6 |
| render/busy/ms | 7.081 | 8.339 | 10.24 | 12193.9 |
| main/busy/ms | 6.978 | 8.239 | 9.89 | 12016.1 |
| render/graph/ms | 3.886 | 4.822 | 6.15 | 6692.2 |
| render/present/ms | 0.542 | 1.160 | 2.40 | 934.0 |
| bigspace/total/ms | 0.478 | 1.395 | 1.99 | 823.1 |
| pipelines/compile_ms | 0.300 | 0.000 | 129.69 | 516.8 |
| pipelines/front_ms | 0.293 | 0.000 | 126.60 | 503.7 |
| terrain/update/ms | 0.245 | 0.313 | 0.62 | 421.2 |
| stream/drive/ms | 0.230 | 0.317 | 0.59 | 396.7 |
| bigspace/high_precision/ms | 0.219 | 1.083 | 1.79 | 377.3 |
| terrain/select/ms | 0.212 | 0.270 | 0.59 | 364.7 |
| stream/tick/ms | 0.199 | 0.280 | 0.55 | 342.5 |
| far/select/ms | 0.182 | 0.265 | 0.43 | 313.7 |
| controller/feed/ms | 0.163 | 0.215 | 0.44 | 280.2 |
| controller/move/ms | 0.139 | 0.289 | 0.41 | 239.3 |
| render/acquire/ms | 0.137 | 0.437 | 1.19 | 235.7 |
| bigspace/low_precision/ms | 0.126 | 0.478 | 1.74 | 216.2 |
| anim/evaluate/ms | 0.079 | 0.220 | 1.53 | 136.3 |
| anim/evaluate_cpu/ms | 0.064 | 0.080 | 0.25 | 110.7 |
| physics/step/ms | 0.027 | 0.063 | 0.20 | 46.9 |
| weather/eval/ms | 0.013 | 0.017 | 0.18 | 22.1 |
| script/update/ms | 0.009 | 0.015 | 0.20 | 15.9 |
| pipelines/driver_ms | 0.008 | 0.000 | 3.74 | 13.1 |
| controller/avoid/ms | 0.006 | 0.007 | 0.03 | 9.8 |
| script/input/ms | 0.006 | 0.007 | 0.03 | 9.6 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 8.5 |
| anim/apply/ms | 0.005 | 0.006 | 0.16 | 8.0 |
| controller/write/ms | 0.002 | 0.003 | 0.32 | 3.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.17 | 1.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.6 |
| physics/insert/ms | 0.000 | 0.000 | 0.05 | 0.0 |
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
| frame/work/ms | ms | 7.25 | 10.24 | 6.86 |
| gpu/timed/ms | ms | 6.77 | 7.90 | 6.84 |
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
| phases/shadow/bins | count | 2039 | 2040 | 2040 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5403 | 5404 | 5404 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 33 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 169 | 169 | 169 |
| pipelines/waiting | count | 0 | 4 | 0 |
| preprocess/bind_groups | count | 0 | 1 | 0 |
| preprocess/bind_groups_created | count | 0 | 1 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 26.92 | 41.35 | 25.48 |
| process/mem_usage | GiB | 4.05 | 4.06 | 4.06 |
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
| system/cpu_usage | % | 43.46 | 63.60 | 36.95 |
| system/mem_usage | % | 65.87 | 65.99 | 65.87 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2463 | 2463 | 2463 |
| textures/images | count | 1910 | 1910 | 1910 |
| views/all/batch_sets | count | 91 | 91 | 91 |
| views/all/bins | count | 5209 | 5210 | 5210 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 14.7 k | 14.7 k |
| views/all/pipelines | count | 87 | 87 | 87 |
| views/all/pipelines_distinct | count | 77 | 77 | 77 |
| views/all/work_items | count | 14.9 k | 14.9 k | 14.9 k |
| views/cascade/batch_sets | count | 33 | 33 | 33 |
| views/cascade/bins | count | 2039 | 2040 | 2040 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5403 | 5404 | 5404 |
| views/cascade/pipelines | count | 33 | 33 | 33 |
| views/cascade/work_items | count | 5403 | 5404 | 5404 |
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
| anim/events/count | count | 0.09 | 3 | 147 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 2.87 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 4 | 7 |
| physics/inserts | count | 0.00 | 6 | 6 |
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

## Hitches (> 25.0 ms; slowest 29, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.80 | 2804.0 | main/busy/ms 2777, surface/advance_cpu_ms 70.18, surface/cell_upload_ms 0.86, stream/drive/ms 0.33, bigspace/total/ms 0.10, stream/tick/ms 0.08, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.02, anim/evaluate/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.64 | 838.6 | main/busy/ms 2811, render/thread/ms 801, render/busy/ms 631, render/acquire/ms 166, surface/advance_cpu_ms 70.18, render/graph/ms 6.50, render/present/ms 3.79, stream/drive/ms 0.91, surface/cell_upload_ms 0.86, terrain/update/ms 0.56, stream/tick/ms 0.47, terrain/select/ms 0.36 |
| warm-up | 3.73 | 89.7 | render/thread/ms 816, render/busy/ms 645, pipelines/wait_ms 633, render/acquire/ms 166, pipelines/compile_ms 85.20, pipelines/front_ms 78.26, main/busy/ms 54.20, terrain/update/ms 19.60, render/graph/ms 10.19, pipelines/driver_ms 6.94, render/present/ms 4.90, stream/drive/ms 1.00 |
| warm-up | 4.00 | 87.8 | pipelines/compile_ms 212, pipelines/front_ms 192, render/thread/ms 74.68, main/busy/ms 69.92, render/busy/ms 62.63, pipelines/driver_ms 19.85, render/acquire/ms 10.32, render/graph/ms 5.41, stream/drive/ms 2.57, stream/tick/ms 2.08, render/present/ms 1.73, pipelines/wait_ms 1.32 |
| warm-up | 4.17 | 77.8 | pipelines/wait_ms 597, pipelines/compile_ms 253, pipelines/front_ms 240, render/thread/ms 49.52, render/busy/ms 48.38, main/busy/ms 44.54, stream/drive/ms 32.41, pipelines/driver_ms 12.20, render/graph/ms 3.57, render/present/ms 0.95, stream/tick/ms 0.92, terrain/update/ms 0.58 |
| warm-up | 4.23 | 44.4 | pipelines/wait_ms 1745, pipelines/compile_ms 63.14, pipelines/front_ms 60.19, main/busy/ms 38.95, render/thread/ms 27.49, render/busy/ms 25.82, stream/drive/ms 18.46, terrain/update/ms 5.82, render/graph/ms 4.53, pipelines/driver_ms 2.96, render/present/ms 1.46, stream/tick/ms 1.44 |
| warm-up | 4.26 | 35.6 | pipelines/wait_ms 1373, pipelines/compile_ms 48.84, pipelines/front_ms 46.91, render/thread/ms 44.60, render/busy/ms 43.00, main/busy/ms 24.58, terrain/update/ms 5.78, render/graph/ms 4.64, stream/drive/ms 4.56, pipelines/driver_ms 1.93, stream/tick/ms 1.75, stream/spawn/ms 1.61 |
| warm-up | 4.30 | 40.2 | pipelines/wait_ms 477, render/thread/ms 55.97, render/busy/ms 54.24, main/busy/ms 43.33, stream/drive/ms 19.28, pipelines/compile_ms 5.64, render/graph/ms 4.99, pipelines/front_ms 4.97, stream/spawn/ms 1.88, render/present/ms 1.46, stream/tick/ms 1.36, terrain/update/ms 0.91 |
| warm-up | 4.39 | 85.6 | render/thread/ms 102, render/busy/ms 100, main/busy/ms 46.33, stream/drive/ms 22.79, render/graph/ms 6.15, stream/spawn/ms 4.51, render/present/ms 1.57, stream/tick/ms 1.57, terrain/update/ms 0.87, terrain/select/ms 0.71, stream/activate/ms 0.56, render/acquire/ms 0.34 |
| warm-up | 4.43 | 38.8 | pipelines/wait_ms 381, pipelines/compile_ms 165, pipelines/front_ms 162, render/thread/ms 99.30, render/busy/ms 97.76, main/busy/ms 72.06, stream/drive/ms 54.27, render/graph/ms 4.46, stream/spawn/ms 3.40, pipelines/driver_ms 3.38, stream/tick/ms 1.61, render/present/ms 1.19 |
| warm-up | 4.58 | 149.8 | pipelines/wait_ms 5914, pipelines/compile_ms 382, pipelines/front_ms 344, render/thread/ms 116, render/busy/ms 115, main/busy/ms 66.57, stream/drive/ms 50.82, pipelines/driver_ms 37.98, render/graph/ms 2.86, stream/spawn/ms 1.23, render/present/ms 1.05, stream/tick/ms 0.82 |
| warm-up | 4.66 | 35.2 | main/busy/ms 38.75, stream/drive/ms 26.02, render/thread/ms 17.61, render/busy/ms 16.55, render/graph/ms 3.05, stream/tick/ms 0.97, render/present/ms 0.88, terrain/update/ms 0.65, terrain/select/ms 0.51, stream/activate/ms 0.48, bigspace/total/ms 0.33, physics/insert/ms 0.28 |
| warm-up | 4.82 | 103.8 | pipelines/wait_ms 18.9 k, pipelines/compile_ms 212, pipelines/front_ms 195, main/busy/ms 94.79, stream/drive/ms 85.08, render/thread/ms 23.12, render/busy/ms 21.73, pipelines/driver_ms 16.75, render/graph/ms 3.75, render/present/ms 1.20, stream/tick/ms 0.98, terrain/update/ms 0.57 |
| warm-up | 4.88 | 28.6 | main/busy/ms 27.63, render/thread/ms 21.73, render/busy/ms 20.59, stream/drive/ms 11.49, render/graph/ms 2.76, stream/spawn/ms 2.21, stream/tick/ms 1.18, render/present/ms 0.96, terrain/update/ms 0.54, terrain/select/ms 0.42, bigspace/total/ms 0.35, render/acquire/ms 0.19 |
| warm-up | 5.03 | 35.3 | pipelines/wait_ms 1167, pipelines/compile_ms 99.16, pipelines/front_ms 91.14, render/thread/ms 40.18, render/busy/ms 38.61, main/busy/ms 29.70, stream/drive/ms 12.71, pipelines/driver_ms 8.02, render/graph/ms 6.30, stream/spawn/ms 2.48, render/present/ms 1.26, stream/tick/ms 0.84 |
| warm-up | 5.27 | 34.8 | render/thread/ms 51.96, render/busy/ms 50.02, main/busy/ms 21.57, render/graph/ms 5.43, stream/drive/ms 3.71, stream/spawn/ms 2.14, render/present/ms 1.60, stream/tick/ms 0.83, bigspace/total/ms 0.77, terrain/update/ms 0.70, terrain/select/ms 0.55, bigspace/high_precision/ms 0.37 |
| warm-up | 5.29 | 25.1 | render/thread/ms 52.50, render/busy/ms 50.52, main/busy/ms 20.85, render/graph/ms 6.49, stream/drive/ms 3.74, stream/spawn/ms 2.03, render/present/ms 1.64, terrain/update/ms 0.73, bigspace/total/ms 0.71, stream/tick/ms 0.67, terrain/select/ms 0.57, bigspace/high_precision/ms 0.39 |
| warm-up | 5.33 | 33.6 | pipelines/wait_ms 3580, pipelines/compile_ms 157, pipelines/front_ms 153, render/thread/ms 43.14, render/busy/ms 41.90, main/busy/ms 22.10, render/graph/ms 4.58, pipelines/driver_ms 4.11, stream/drive/ms 3.86, stream/spawn/ms 2.12, render/present/ms 1.03, terrain/update/ms 0.68 |
| warm-up | 5.36 | 29.7 | pipelines/wait_ms 3580, pipelines/compile_ms 157, pipelines/front_ms 153, render/thread/ms 50.00, render/busy/ms 48.52, main/busy/ms 24.94, render/graph/ms 5.23, stream/drive/ms 4.71, pipelines/driver_ms 4.11, stream/spawn/ms 2.19, render/present/ms 1.24, stream/tick/ms 0.67 |
| warm-up | 5.43 | 53.3 | pipelines/wait_ms 10.1 k, pipelines/compile_ms 127, pipelines/front_ms 113, render/thread/ms 55.37, render/busy/ms 53.85, main/busy/ms 48.45, pipelines/driver_ms 14.00, render/graph/ms 4.93, stream/drive/ms 4.48, stream/spawn/ms 2.43, render/present/ms 1.24, stream/tick/ms 0.83 |
| warm-up | 5.51 | 65.6 | pipelines/wait_ms 7438, pipelines/compile_ms 91.09, pipelines/front_ms 86.18, render/thread/ms 61.09, render/busy/ms 59.15, main/busy/ms 42.90, render/graph/ms 7.14, stream/drive/ms 6.82, pipelines/driver_ms 4.91, stream/spawn/ms 2.92, render/present/ms 1.59, physics/insert/ms 1.30 |
| warm-up | 5.55 | 38.3 | pipelines/wait_ms 9011, pipelines/compile_ms 96.04, pipelines/front_ms 90.92, render/thread/ms 65.68, render/busy/ms 64.10, main/busy/ms 30.16, render/graph/ms 6.19, pipelines/driver_ms 5.11, stream/drive/ms 4.14, stream/spawn/ms 2.40, render/present/ms 1.33, physics/insert/ms 1.23 |
| warm-up | 5.61 | 62.1 | pipelines/wait_ms 3242, render/thread/ms 75.13, render/busy/ms 51.92, main/busy/ms 34.58, render/present/ms 22.98, stream/drive/ms 13.91, pipelines/compile_ms 8.55, pipelines/front_ms 8.04, render/graph/ms 4.16, stream/spawn/ms 2.06, physics/insert/ms 1.20, stream/tick/ms 0.82 |
| warm-up | 5.72 | 28.4 | main/busy/ms 38.59, render/thread/ms 29.72, render/busy/ms 28.29, render/graph/ms 5.11, stream/drive/ms 3.86, terrain/update/ms 2.44, stream/spawn/ms 2.40, render/present/ms 1.10, physics/insert/ms 1.07, physics/step/ms 0.91, bigspace/total/ms 0.77, stream/tick/ms 0.57 |
| warm-up | 5.81 | 69.6 | pipelines/wait_ms 1442, pipelines/compile_ms 140, pipelines/front_ms 134, main/busy/ms 76.89, render/thread/ms 56.50, render/busy/ms 54.55, stream/drive/ms 19.63, render/graph/ms 8.14, pipelines/driver_ms 6.04, stream/spawn/ms 2.12, render/present/ms 1.61, stream/tick/ms 0.94 |
| warm-up | 5.85 | 33.6 | pipelines/wait_ms 4855, pipelines/compile_ms 188, pipelines/front_ms 180, main/busy/ms 62.05, render/thread/ms 57.93, render/busy/ms 56.52, stream/drive/ms 18.87, pipelines/driver_ms 7.92, render/graph/ms 6.56, stream/spawn/ms 1.62, render/present/ms 1.12, physics/insert/ms 0.83 |
| warm-up | 6.16 | 32.0 | render/thread/ms 38.09, render/busy/ms 36.21, main/busy/ms 30.33, render/graph/ms 6.24, stream/drive/ms 4.33, stream/spawn/ms 2.31, render/present/ms 1.42, stream/tick/ms 1.27, bigspace/total/ms 0.89, terrain/update/ms 0.74, terrain/select/ms 0.63, render/acquire/ms 0.46 |
| warm-up | 6.19 | 28.1 | pipelines/wait_ms 5789, pipelines/compile_ms 174, pipelines/front_ms 169, render/thread/ms 39.62, render/busy/ms 38.38, main/busy/ms 30.51, pipelines/driver_ms 5.00, stream/drive/ms 4.34, render/graph/ms 4.11, stream/spawn/ms 2.77, stream/tick/ms 0.97, bigspace/total/ms 0.92 |
| warm-up | 6.98 | 26.8 | pipelines/wait_ms 1991, main/busy/ms 35.87, render/thread/ms 32.13, render/busy/ms 30.79, pipelines/compile_ms 18.07, pipelines/front_ms 17.03, render/graph/ms 11.74, stream/drive/ms 1.17, render/present/ms 1.10, pipelines/driver_ms 1.03, bigspace/total/ms 0.71, physics/insert/ms 0.56 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_post\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface-2 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 17% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **94%** (min 80%) · 2580 MHz (min 2580) · memory 8001 MHz · 90.8 W · 77 °C (max 81) · VRAM 5415 MiB · throttled 100% of samples
