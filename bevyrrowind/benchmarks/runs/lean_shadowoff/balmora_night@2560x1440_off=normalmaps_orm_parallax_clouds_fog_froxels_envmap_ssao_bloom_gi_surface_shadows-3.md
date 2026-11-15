# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: envmap,shadows,ssao,bloom,gi,clouds,fog,froxels,normalmaps,orm,parallax,surface · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1959 frames over 15.0 s · **median 7.52 ms (133 fps)** · mean 7.66 · p95 9.21 · p99 10.24 (1% low 98 fps) · worst 17.47 ms · 4 spikes over 1.5× median · 24.0 stutters/min (work 16.0/min)
- warm-up: 288 frames over 7.5 s, worst 2634.4 ms
- hitches (> 25.0 ms): **0** measured, 31 in warm-up
- timed GPU passes: 5.75 ms of a 7.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 7.52 ms, limit 11 ms
- pass worst frame 17.47 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.897 | 2.074 | 2.328 | 0.91 | 0.185 |
| main_opaque_pass_3d | 1.541 | 1.685 | 2.037 | 0.91 | 0.058 |
| early prepass | 0.497 | 0.544 | 0.794 | 0.91 | 0.105 |
| atmosphere_luts | 0.379 | 0.414 | 0.630 | 0.91 | 0.004 |
| water_velocity | 0.242 | 0.264 | 0.265 | 0.91 | 0.002 |
| water_sspr | 0.206 | 0.225 | 0.222 | 0.91 | 0.003 |
| main_transmissive_pass_3d | 0.203 | 0.222 | 0.223 | 0.91 | 0.009 |
| water_fft | 0.139 | 0.152 | 0.292 | 0.91 | 0.003 |
| upscaling | 0.123 | 0.134 | 0.139 | 0.91 | 0.004 |
| tonemapping | 0.115 | 0.125 | 0.129 | 0.91 | 0.005 |
| uniform_allocation | 0.110 | 0.121 | 0.117 | 0.91 | 0.024 |
| bin_unpacking | 0.083 | 0.090 | 0.096 | 0.91 | 0.016 |
| clustering | 0.059 | 0.065 | 0.071 | 0.91 | 0.022 |
| render_sky | 0.050 | 0.054 | 0.054 | 0.91 | 0.002 |
| water_snapshot | 0.030 | 0.033 | 0.035 | 0.91 | 0.002 |
| early_mesh_preprocessing | 0.025 | 0.027 | 0.029 | 0.91 | 0.004 |
| water_mips | 0.022 | 0.024 | 0.024 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.018 | 0.020 | 0.020 | 0.91 | 0.058 |
| night_sky | 0.006 | 0.007 | 0.007 | 0.91 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.91 | 0.003 |
| auxiliary_views | 0.001 | 0.001 | 0.002 | 0.91 | 0.004 |
| per_view_shadow_pass | 0.001 | 0.001 | 0.002 | 0.91 | 0.001 |
| atmosphere_environment | 0.001 | 0.001 | 0.002 | 0.91 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 28.034 | 0.000 | 9447.70 | 54918.0 |
| render/thread/ms | 6.692 | 8.013 | 11.42 | 13109.7 |
| main/busy/ms | 6.391 | 7.688 | 11.98 | 12520.1 |
| render/busy/ms | 6.031 | 7.383 | 10.63 | 11815.3 |
| render/graph/ms | 3.392 | 4.371 | 6.09 | 6645.7 |
| pipelines/compile_ms | 0.779 | 0.000 | 141.00 | 1525.8 |
| pipelines/front_ms | 0.753 | 0.000 | 138.06 | 1475.5 |
| render/present/ms | 0.512 | 0.991 | 1.78 | 1002.3 |
| bigspace/total/ms | 0.442 | 1.116 | 1.91 | 865.9 |
| terrain/update/ms | 0.235 | 0.304 | 0.64 | 460.5 |
| stream/drive/ms | 0.220 | 0.312 | 0.67 | 431.3 |
| terrain/select/ms | 0.204 | 0.264 | 0.60 | 400.1 |
| stream/tick/ms | 0.189 | 0.274 | 0.63 | 370.8 |
| far/select/ms | 0.174 | 0.248 | 0.53 | 341.2 |
| controller/feed/ms | 0.165 | 0.227 | 0.44 | 323.6 |
| bigspace/high_precision/ms | 0.163 | 0.473 | 1.71 | 318.5 |
| anim/evaluate/ms | 0.159 | 0.696 | 1.87 | 311.0 |
| render/acquire/ms | 0.149 | 0.557 | 1.21 | 292.2 |
| controller/move/ms | 0.140 | 0.268 | 0.59 | 273.8 |
| bigspace/low_precision/ms | 0.085 | 0.113 | 1.17 | 165.7 |
| anim/evaluate_cpu/ms | 0.061 | 0.084 | 0.34 | 119.7 |
| pipelines/driver_ms | 0.026 | 0.000 | 6.66 | 50.3 |
| physics/step/ms | 0.025 | 0.064 | 0.91 | 48.3 |
| weather/eval/ms | 0.012 | 0.016 | 0.23 | 22.9 |
| script/update/ms | 0.008 | 0.013 | 0.30 | 16.2 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 12.0 |
| script/input/ms | 0.005 | 0.007 | 0.30 | 10.4 |
| script/ui/ms | 0.005 | 0.008 | 0.14 | 10.0 |
| anim/apply/ms | 0.004 | 0.006 | 0.04 | 8.8 |
| physics/insert/ms | 0.004 | 0.000 | 0.70 | 8.1 |
| controller/write/ms | 0.002 | 0.003 | 0.21 | 4.2 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 2.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 1.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 0.8 |
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
| anim/a2/count | count | 0 | 15 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 34 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 23 | 8 |
| crowd/c0/joints/count | count | 272 | 782 | 272 |
| crowd/c0/meshes/count | count | 48 | 138 | 48 |
| crowd/c1/count | count | 123 | 124 | 123 |
| crowd/c1/joints/count | count | 4880 | 4914 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 109 | 109 | 109 |
| crowd/joints_extracted/count | count | 2697 | 6331 | 2697 |
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
| frame/work/ms | ms | 6.49 | 11.98 | 6.27 |
| gpu/timed/ms | ms | 6.31 | 7.35 | 6.27 |
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
| physics/pending | count | 0 | 434 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 145 | 145 | 145 |
| pipelines/waiting | count | 0 | 22 | 0 |
| preprocess/bind_groups | count | 0 | 130 | 0 |
| preprocess/bind_groups_created | count | 0 | 130 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 134 | 134 | 134 |
| process/cpu_usage | % | 25.40 | 44.71 | 30.77 |
| process/mem_usage | GiB | 3.94 | 4.03 | 4.03 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 42.36 | 67.19 | 51.39 |
| system/mem_usage | % | 71.62 | 71.91 | 71.91 |
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
| anim/bones_written/count | count | 0.25 | 264 | 495 |
| anim/events/count | count | 0.08 | 3 | 157 |
| assets/Image/added | count | 0.01 | 13 | 20 |
| assets/Image/modified | count | 0.00 | 1 | 2 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.02 | 25 | 48 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 16 | 24 |
| assets/StandardMaterial/modified | count | 2.53 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.75 | 1917 | 3430 |
| physics/inserts | count | 0.22 | 206 | 434 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.46 | 1 | 901 |
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

## Hitches (> 25.0 ms; slowest 31, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.63 | 2634.4 | main/busy/ms 2608, surface/advance_cpu_ms 1.20, stream/drive/ms 0.41, surface/cell_upload_ms 0.29, stream/tick/ms 0.15, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.02, script/update/ms 0.02, far/select/ms 0.01, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.56 | 927.7 | main/busy/ms 2644, render/thread/ms 890, render/busy/ms 716, render/acquire/ms 170, render/graph/ms 5.65, render/present/ms 3.72, surface/advance_cpu_ms 1.20, stream/drive/ms 0.92, terrain/update/ms 0.57, stream/tick/ms 0.46, terrain/select/ms 0.37, surface/cell_upload_ms 0.29 |
| warm-up | 3.84 | 100.4 | render/thread/ms 83.03, render/busy/ms 75.64, main/busy/ms 71.26, render/graph/ms 5.89, render/acquire/ms 5.63, stream/drive/ms 2.12, render/present/ms 1.76, stream/tick/ms 1.67, terrain/update/ms 0.66, terrain/select/ms 0.51, bigspace/total/ms 0.39, bigspace/high_precision/ms 0.20 |
| warm-up | 3.89 | 32.4 | pipelines/compile_ms 251, pipelines/front_ms 225, pipelines/wait_ms 53.28, render/thread/ms 43.14, render/busy/ms 41.07, main/busy/ms 28.63, pipelines/driver_ms 26.10, stream/drive/ms 5.36, render/graph/ms 4.23, stream/tick/ms 2.46, render/present/ms 1.77, stream/spawn/ms 1.74 |
| warm-up | 4.04 | 73.2 | main/busy/ms 52.62, render/thread/ms 49.38, render/busy/ms 47.96, stream/drive/ms 40.44, render/graph/ms 2.39, stream/tick/ms 1.18, render/present/ms 1.08, terrain/update/ms 0.61, terrain/select/ms 0.49, render/acquire/ms 0.34, bigspace/total/ms 0.19, bigspace/high_precision/ms 0.09 |
| warm-up | 4.08 | 35.0 | pipelines/wait_ms 1703, pipelines/compile_ms 241, pipelines/front_ms 226, main/busy/ms 33.78, render/thread/ms 30.24, render/busy/ms 28.16, stream/drive/ms 15.47, pipelines/driver_ms 14.67, render/graph/ms 3.62, stream/spawn/ms 1.51, render/present/ms 1.48, stream/tick/ms 1.24 |
| warm-up | 4.11 | 27.9 | pipelines/wait_ms 2191, pipelines/compile_ms 272, pipelines/front_ms 256, main/busy/ms 57.18, render/thread/ms 41.78, render/busy/ms 39.36, stream/drive/ms 34.42, pipelines/driver_ms 16.03, render/graph/ms 4.98, render/present/ms 1.94, stream/tick/ms 1.80, stream/spawn/ms 1.71 |
| warm-up | 4.20 | 88.2 | pipelines/wait_ms 5492, pipelines/compile_ms 124, pipelines/front_ms 115, render/thread/ms 64.59, render/busy/ms 62.88, main/busy/ms 59.18, stream/drive/ms 40.54, pipelines/driver_ms 8.39, render/graph/ms 3.57, render/present/ms 1.50, stream/tick/ms 1.34, terrain/update/ms 0.72 |
| warm-up | 4.25 | 26.4 | main/busy/ms 37.30, render/thread/ms 26.63, render/busy/ms 25.54, stream/drive/ms 18.23, render/graph/ms 2.97, stream/spawn/ms 1.40, render/present/ms 0.92, stream/tick/ms 0.68, terrain/update/ms 0.68, terrain/select/ms 0.48, bigspace/total/ms 0.33, stream/activate/ms 0.27 |
| warm-up | 4.33 | 44.3 | pipelines/wait_ms 1072, pipelines/compile_ms 139, pipelines/front_ms 134, main/busy/ms 36.59, render/thread/ms 28.00, render/busy/ms 26.24, stream/drive/ms 20.26, pipelines/driver_ms 5.21, render/graph/ms 3.96, stream/spawn/ms 2.21, render/present/ms 1.53, stream/tick/ms 1.32 |
| warm-up | 4.36 | 29.7 | pipelines/wait_ms 1072, pipelines/compile_ms 139, pipelines/front_ms 134, render/thread/ms 38.76, render/busy/ms 36.39, main/busy/ms 24.43, render/graph/ms 5.32, pipelines/driver_ms 5.21, stream/drive/ms 4.85, render/present/ms 2.09, stream/spawn/ms 2.06, stream/tick/ms 1.63 |
| warm-up | 4.43 | 46.3 | render/thread/ms 38.94, render/busy/ms 37.51, main/busy/ms 30.20, stream/drive/ms 7.26, stream/spawn/ms 4.01, render/graph/ms 3.03, stream/tick/ms 1.29, render/present/ms 1.19, terrain/update/ms 0.82, stream/activate/ms 0.80, terrain/select/ms 0.64, physics/insert/ms 0.45 |
| warm-up | 4.47 | 42.6 | pipelines/wait_ms 737, pipelines/compile_ms 111, pipelines/front_ms 107, main/busy/ms 49.93, render/thread/ms 49.13, render/busy/ms 47.80, stream/drive/ms 27.04, pipelines/driver_ms 3.77, stream/spawn/ms 3.24, render/graph/ms 3.12, stream/tick/ms 1.38, render/present/ms 1.16 |
| warm-up | 4.58 | 62.8 | pipelines/wait_ms 1498, pipelines/compile_ms 115, pipelines/front_ms 111, main/busy/ms 74.52, stream/drive/ms 53.05, render/thread/ms 16.99, render/busy/ms 15.76, terrain/update/ms 6.02, pipelines/driver_ms 3.28, render/graph/ms 2.83, stream/tick/ms 1.09, render/present/ms 1.00 |
| warm-up | 4.67 | 73.6 | pipelines/wait_ms 8693, pipelines/compile_ms 102, pipelines/front_ms 95.70, main/busy/ms 83.09, stream/drive/ms 47.62, render/thread/ms 27.85, render/busy/ms 26.24, pipelines/driver_ms 6.03, render/graph/ms 3.94, terrain/update/ms 3.70, render/present/ms 1.40, physics/insert/ms 1.07 |
| warm-up | 4.71 | 39.6 | pipelines/wait_ms 8693, pipelines/compile_ms 102, pipelines/front_ms 95.70, main/busy/ms 51.82, render/thread/ms 42.11, render/busy/ms 39.89, stream/drive/ms 8.77, render/graph/ms 6.06, pipelines/driver_ms 6.03, render/present/ms 1.93, stream/spawn/ms 1.88, physics/insert/ms 1.07 |
| warm-up | 4.85 | 28.8 | main/busy/ms 30.56, render/thread/ms 24.87, render/busy/ms 23.20, stream/drive/ms 12.42, render/graph/ms 5.02, stream/spawn/ms 3.72, render/present/ms 1.48, stream/tick/ms 0.87, terrain/update/ms 0.78, terrain/select/ms 0.63, bigspace/total/ms 0.32, render/acquire/ms 0.18 |
| warm-up | 5.07 | 38.6 | pipelines/wait_ms 3982, pipelines/compile_ms 158, pipelines/front_ms 143, render/thread/ms 54.33, render/busy/ms 51.90, main/busy/ms 19.22, pipelines/driver_ms 14.98, render/graph/ms 6.34, stream/drive/ms 3.81, stream/spawn/ms 2.44, render/present/ms 2.00, stream/tick/ms 0.65 |
| warm-up | 5.13 | 41.8 | pipelines/wait_ms 6332, pipelines/compile_ms 69.30, pipelines/front_ms 62.09, render/thread/ms 45.41, render/busy/ms 44.06, main/busy/ms 30.09, pipelines/driver_ms 7.20, render/graph/ms 3.88, stream/drive/ms 3.85, stream/spawn/ms 2.17, render/present/ms 1.02, bigspace/total/ms 0.79 |
| warm-up | 5.16 | 35.2 | pipelines/wait_ms 16.3 k, pipelines/compile_ms 78.43, pipelines/front_ms 67.53, render/thread/ms 60.33, render/busy/ms 58.96, main/busy/ms 39.50, pipelines/driver_ms 10.91, stream/drive/ms 4.25, render/graph/ms 4.22, stream/spawn/ms 2.13, physics/insert/ms 1.13, render/present/ms 1.07 |
| warm-up | 5.19 | 29.7 | pipelines/wait_ms 10.5 k, render/thread/ms 52.85, render/busy/ms 51.66, main/busy/ms 38.53, pipelines/compile_ms 32.27, pipelines/front_ms 27.69, stream/drive/ms 4.58, pipelines/driver_ms 4.57, render/graph/ms 4.05, stream/spawn/ms 2.17, physics/insert/ms 1.20, bigspace/total/ms 1.05 |
| warm-up | 5.23 | 31.7 | pipelines/wait_ms 3193, pipelines/compile_ms 78.04, pipelines/front_ms 73.09, render/thread/ms 48.76, render/busy/ms 47.54, main/busy/ms 30.43, pipelines/driver_ms 4.95, render/graph/ms 4.83, stream/drive/ms 4.37, stream/spawn/ms 2.23, physics/insert/ms 1.16, physics/step/ms 1.02 |
| warm-up | 5.29 | 47.3 | pipelines/wait_ms 4893, render/thread/ms 46.61, render/busy/ms 44.18, pipelines/compile_ms 40.68, pipelines/front_ms 37.13, main/busy/ms 35.15, stream/drive/ms 8.26, render/graph/ms 8.25, pipelines/driver_ms 3.55, stream/spawn/ms 3.49, render/present/ms 2.09, physics/insert/ms 1.39 |
| warm-up | 5.32 | 35.1 | pipelines/wait_ms 4376, render/thread/ms 55.06, render/busy/ms 53.22, main/busy/ms 31.66, pipelines/compile_ms 24.17, pipelines/front_ms 21.77, render/graph/ms 7.02, stream/drive/ms 4.30, stream/spawn/ms 2.42, pipelines/driver_ms 2.40, render/present/ms 1.61, stream/tick/ms 1.20 |
| warm-up | 5.35 | 25.0 | render/thread/ms 37.88, render/busy/ms 36.68, main/busy/ms 35.96, stream/drive/ms 15.82, render/graph/ms 3.47, stream/spawn/ms 2.09, render/present/ms 1.03, physics/insert/ms 0.93, stream/tick/ms 0.91, terrain/update/ms 0.84, bigspace/total/ms 0.82, physics/step/ms 0.77 |
| warm-up | 5.47 | 28.9 | pipelines/wait_ms 1802, pipelines/compile_ms 68.66, pipelines/front_ms 65.41, main/busy/ms 31.73, render/thread/ms 18.96, stream/drive/ms 18.83, render/busy/ms 17.77, render/graph/ms 3.97, pipelines/driver_ms 3.25, stream/spawn/ms 1.55, render/present/ms 0.99, stream/tick/ms 0.60 |
| warm-up | 5.67 | 27.4 | pipelines/wait_ms 2043, render/thread/ms 30.54, render/busy/ms 28.24, main/busy/ms 22.37, pipelines/compile_ms 15.68, pipelines/front_ms 14.66, render/graph/ms 7.06, stream/drive/ms 2.89, render/present/ms 2.06, stream/spawn/ms 1.63, pipelines/driver_ms 1.02, bigspace/total/ms 0.79 |
| warm-up | 5.72 | 29.4 | render/thread/ms 31.59, render/busy/ms 30.28, main/busy/ms 27.49, stream/drive/ms 8.77, render/graph/ms 4.29, stream/tick/ms 1.19, render/present/ms 1.09, stream/spawn/ms 1.00, bigspace/total/ms 0.90, terrain/update/ms 0.73, terrain/select/ms 0.60, bigspace/high_precision/ms 0.32 |
| warm-up | 6.22 | 26.8 | main/busy/ms 36.29, render/thread/ms 20.96, render/busy/ms 19.12, render/graph/ms 6.69, render/present/ms 1.66, stream/drive/ms 1.42, physics/insert/ms 0.58, bigspace/total/ms 0.55, stream/tick/ms 0.55, terrain/update/ms 0.45, stream/spawn/ms 0.39, terrain/select/ms 0.38 |
| warm-up | 6.59 | 30.2 | pipelines/wait_ms 10.7 k, main/busy/ms 47.17, pipelines/compile_ms 46.17, pipelines/front_ms 39.83, render/thread/ms 27.45, render/busy/ms 25.34, render/graph/ms 8.24, pipelines/driver_ms 6.34, terrain/update/ms 2.92, render/present/ms 1.84, stream/drive/ms 1.16, physics/insert/ms 0.84 |
| warm-up | 6.63 | 39.2 | pipelines/wait_ms 14.0 k, pipelines/compile_ms 91.56, pipelines/front_ms 82.54, main/busy/ms 46.53, render/thread/ms 36.88, render/busy/ms 35.28, pipelines/driver_ms 9.02, render/graph/ms 7.63, terrain/update/ms 7.24, render/present/ms 1.36, stream/drive/ms 1.30, physics/insert/ms 1.17 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_shadowoff\balmora_night@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels_envmap_ssao_bloom_gi_surface_shadows-3 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels,envmap,ssao,bloom,gi,surface,shadows
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 14%
- during the run: other processes 17% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **93%** (min 69%) · 2580 MHz (min 2580) · memory 8001 MHz · 91.8 W · 77 °C (max 80) · VRAM 5192 MiB · throttled 100% of samples
