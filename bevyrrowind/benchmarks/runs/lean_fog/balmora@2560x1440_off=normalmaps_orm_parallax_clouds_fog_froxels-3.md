# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,fog,froxels,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1508 frames over 15.0 s · **median 9.83 ms (102 fps)** · mean 9.95 · p95 11.34 · p99 12.14 (1% low 82 fps) · worst 27.34 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 12.0/min)
- warm-up: 369 frames over 8.3 s, worst 2543.5 ms
- hitches (> 25.0 ms): **1** measured, 30 in warm-up
- timed GPU passes: 8.16 ms of a 9.95 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 9.83 ms, limit 11 ms
- pass worst frame 27.34 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.574 | 2.607 | 3.215 | 0.99 | 0.067 |
| dlss_super_resolution | 2.203 | 2.231 | 2.788 | 0.99 | 0.188 |
| early prepass | 0.608 | 0.615 | 0.950 | 0.99 | 0.098 |
| atmosphere_luts | 0.358 | 0.362 | 0.485 | 0.99 | 0.004 |
| bloom | 0.341 | 0.346 | 0.513 | 0.99 | 0.067 |
| main_transmissive_pass_3d | 0.295 | 0.299 | 0.461 | 0.99 | 0.009 |
| water_velocity | 0.265 | 0.268 | 0.432 | 0.99 | 0.002 |
| water_sspr | 0.233 | 0.236 | 0.325 | 0.99 | 0.002 |
| ssao | 0.207 | 0.210 | 0.216 | 0.99 | 0.009 |
| uniform_allocation | 0.188 | 0.190 | 0.187 | 0.99 | 0.035 |
| water_fft | 0.143 | 0.145 | 0.139 | 0.99 | 0.003 |
| bin_unpacking | 0.141 | 0.142 | 0.147 | 0.99 | 0.024 |
| upscaling | 0.132 | 0.134 | 0.150 | 0.99 | 0.005 |
| tonemapping | 0.098 | 0.099 | 0.097 | 0.99 | 0.004 |
| clustering | 0.071 | 0.072 | 0.076 | 0.99 | 0.023 |
| render_sky | 0.056 | 0.057 | 0.057 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.055 | 0.445 | 0.750 | 0.12 | 0.001 |
| early_mesh_preprocessing | 0.040 | 0.041 | 0.042 | 0.99 | 0.007 |
| water_snapshot | 0.029 | 0.029 | 0.048 | 0.99 | 0.002 |
| lightprobe_radiance_map | 0.028 | 0.227 | 0.232 | 0.12 | 0.005 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.022 | 0.022 | 0.024 | 0.99 | 0.057 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.026 | 0.99 | 0.008 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.99 | 0.003 |
| night_sky | 0.003 | 0.003 | 0.004 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.014 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.007 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 51.632 | 0.000 | 9777.07 | 77860.6 |
| render/thread/ms | 9.051 | 10.420 | 19.32 | 13648.3 |
| render/busy/ms | 7.696 | 8.906 | 18.57 | 11605.0 |
| main/busy/ms | 7.222 | 8.199 | 17.78 | 10890.8 |
| render/graph/ms | 4.501 | 5.409 | 7.30 | 6786.8 |
| pipelines/compile_ms | 1.281 | 0.000 | 136.54 | 1932.2 |
| pipelines/front_ms | 1.240 | 0.000 | 132.33 | 1870.4 |
| render/acquire/ms | 0.874 | 1.992 | 3.30 | 1318.2 |
| render/present/ms | 0.481 | 0.580 | 3.37 | 725.1 |
| bigspace/total/ms | 0.364 | 0.787 | 2.11 | 548.6 |
| terrain/update/ms | 0.232 | 0.304 | 0.64 | 349.8 |
| terrain/select/ms | 0.201 | 0.263 | 0.60 | 303.7 |
| stream/drive/ms | 0.201 | 0.293 | 0.48 | 303.4 |
| far/select/ms | 0.174 | 0.260 | 0.60 | 262.8 |
| stream/tick/ms | 0.173 | 0.257 | 0.45 | 260.5 |
| controller/feed/ms | 0.165 | 0.226 | 0.54 | 248.5 |
| bigspace/high_precision/ms | 0.134 | 0.310 | 1.85 | 202.4 |
| controller/move/ms | 0.132 | 0.212 | 0.54 | 198.4 |
| bigspace/low_precision/ms | 0.092 | 0.198 | 1.83 | 138.1 |
| anim/evaluate/ms | 0.076 | 0.183 | 1.10 | 114.6 |
| anim/evaluate_cpu/ms | 0.060 | 0.080 | 0.34 | 90.3 |
| pipelines/driver_ms | 0.041 | 0.000 | 4.41 | 61.8 |
| physics/step/ms | 0.034 | 0.061 | 0.84 | 50.7 |
| physics/insert/ms | 0.018 | 0.136 | 0.88 | 27.5 |
| weather/eval/ms | 0.012 | 0.016 | 0.17 | 18.1 |
| script/update/ms | 0.008 | 0.011 | 0.11 | 11.8 |
| controller/avoid/ms | 0.006 | 0.008 | 0.12 | 9.1 |
| script/input/ms | 0.006 | 0.007 | 0.03 | 8.3 |
| script/ui/ms | 0.005 | 0.006 | 0.02 | 6.9 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.7 |
| controller/write/ms | 0.002 | 0.003 | 0.01 | 2.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.06 | 1.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.12 | 0.6 |
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
| anim/a2/count | count | 0 | 17 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 58 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 24 | 8 |
| crowd/c0/joints/count | count | 272 | 816 | 272 |
| crowd/c0/meshes/count | count | 48 | 144 | 48 |
| crowd/c1/count | count | 122 | 124 | 122 |
| crowd/c1/joints/count | count | 4848 | 4848 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1100 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 14.1 k | 4212 |
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
| frame/work/ms | ms | 8.84 | 18.57 | 9.68 |
| gpu/timed/ms | ms | 8.82 | 10.54 | 9.68 |
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
| phases/opaque3d/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 22 |
| phases/opaque3d_prepass/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 21 |
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
| physics/pending | count | 0 | 1552 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 176 | 176 | 176 |
| pipelines/waiting | count | 0 | 20 | 0 |
| preprocess/bind_groups | count | 0 | 207 | 0 |
| preprocess/bind_groups_created | count | 0 | 207 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 205 | 205 | 205 |
| process/cpu_usage | % | 24.04 | 33.65 | 21.15 |
| process/mem_usage | GiB | 4.27 | 4.29 | 4.29 |
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
| stream/latency/s | s | 0.03 | 0.03 | 0.03 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 40.04 | 61.51 | 48.62 |
| system/mem_usage | % | 61.78 | 62.13 | 62.06 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2465 | 2465 | 2465 |
| textures/images | count | 1913 | 1913 | 1913 |
| views/all/batch_sets | count | 91 | 96 | 91 |
| views/all/bins | count | 5209 | 5517 | 5209 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.7 k | 15.0 k | 14.7 k |
| views/all/pipelines | count | 87 | 92 | 87 |
| views/all/pipelines_distinct | count | 77 | 81 | 77 |
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
| views/main/batch_sets | count | 58 | 62 | 58 |
| views/main/bins | count | 3170 | 3172 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 54 | 58 | 54 |
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
| anim/bones_written/count | count | 0.18 | 264 | 264 |
| anim/events/count | count | 0.11 | 3 | 160 |
| assets/Image/added | count | 0.03 | 23 | 43 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.12 | 107 | 182 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.03 | 24 | 44 |
| assets/StandardMaterial/modified | count | 3.28 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.99 | 2916 | 4506 |
| physics/inserts | count | 0.90 | 177 | 1360 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.60 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.01 | 5 | 12 |
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

## Hitches (> 25.0 ms; slowest 31, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.54 | 2543.5 | main/busy/ms 2517, surface/advance_cpu_ms 69.73, surface/cell_upload_ms 0.69, stream/drive/ms 0.37, bigspace/total/ms 0.10, stream/tick/ms 0.09, bigspace/high_precision/ms 0.05, weather/eval/ms 0.04, script/update/ms 0.03, anim/evaluate/ms 0.01, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.24 | 700.9 | main/busy/ms 2555, render/thread/ms 659, render/busy/ms 533, render/acquire/ms 124, surface/advance_cpu_ms 69.73, render/graph/ms 5.51, render/present/ms 2.20, stream/drive/ms 1.14, terrain/update/ms 0.76, surface/cell_upload_ms 0.69, stream/tick/ms 0.51, terrain/select/ms 0.51 |
| warm-up | 3.53 | 282.3 | pipelines/wait_ms 3402, render/thread/ms 672, render/busy/ms 545, pipelines/compile_ms 299, pipelines/front_ms 280, render/acquire/ms 124, main/busy/ms 57.03, terrain/update/ms 20.49, pipelines/driver_ms 18.61, render/graph/ms 10.18, render/present/ms 3.64, stream/drive/ms 1.33 |
| warm-up | 3.79 | 115.1 | pipelines/compile_ms 225, pipelines/front_ms 205, render/thread/ms 73.07, render/busy/ms 70.69, main/busy/ms 64.63, pipelines/driver_ms 19.41, render/graph/ms 7.41, stream/drive/ms 2.28, render/present/ms 2.07, stream/tick/ms 1.55, terrain/update/ms 0.66, terrain/select/ms 0.53 |
| warm-up | 3.89 | 32.5 | main/busy/ms 49.40, render/thread/ms 42.25, render/busy/ms 40.72, stream/drive/ms 38.43, render/graph/ms 5.55, render/present/ms 1.26, stream/tick/ms 0.72, terrain/update/ms 0.55, terrain/select/ms 0.44, stream/spawn/ms 0.30, render/acquire/ms 0.27, bigspace/total/ms 0.20 |
| warm-up | 3.94 | 45.2 | render/thread/ms 58.85, render/busy/ms 57.38, main/busy/ms 48.99, stream/drive/ms 39.12, render/graph/ms 5.31, stream/tick/ms 1.19, render/present/ms 1.17, terrain/update/ms 0.49, terrain/select/ms 0.39, render/acquire/ms 0.29, bigspace/total/ms 0.22, bigspace/high_precision/ms 0.14 |
| warm-up | 3.99 | 45.3 | render/thread/ms 35.83, main/busy/ms 34.84, render/busy/ms 33.26, stream/drive/ms 19.82, render/graph/ms 5.80, render/present/ms 1.95, stream/spawn/ms 1.45, stream/tick/ms 1.31, render/acquire/ms 0.62, terrain/update/ms 0.59, terrain/select/ms 0.45, bigspace/total/ms 0.30 |
| warm-up | 4.02 | 26.6 | render/thread/ms 45.47, render/busy/ms 43.21, main/busy/ms 23.14, render/graph/ms 6.09, stream/drive/ms 4.59, stream/tick/ms 1.97, render/present/ms 1.96, stream/spawn/ms 1.49, terrain/update/ms 0.64, terrain/select/ms 0.48, render/acquire/ms 0.30, bigspace/total/ms 0.27 |
| warm-up | 4.07 | 51.7 | pipelines/wait_ms 303, pipelines/compile_ms 211, pipelines/front_ms 203, main/busy/ms 60.20, render/thread/ms 57.04, render/busy/ms 54.87, stream/drive/ms 41.68, pipelines/driver_ms 8.33, render/graph/ms 7.01, render/present/ms 1.96, stream/tick/ms 1.42, terrain/update/ms 0.67 |
| warm-up | 4.14 | 70.5 | pipelines/wait_ms 1838, pipelines/compile_ms 279, pipelines/front_ms 267, render/thread/ms 74.62, render/busy/ms 72.96, main/busy/ms 60.85, stream/drive/ms 42.16, pipelines/driver_ms 11.16, render/graph/ms 5.66, render/present/ms 1.47, stream/tick/ms 1.18, stream/spawn/ms 1.08 |
| warm-up | 4.21 | 67.3 | pipelines/wait_ms 1536, render/thread/ms 89.63, pipelines/compile_ms 67.13, pipelines/front_ms 64.30, render/busy/ms 58.38, main/busy/ms 54.33, stream/drive/ms 32.90, render/present/ms 31.08, render/graph/ms 3.49, pipelines/driver_ms 2.82, stream/spawn/ms 2.26, stream/tick/ms 1.24 |
| warm-up | 4.24 | 29.7 | pipelines/wait_ms 679, render/thread/ms 73.51, main/busy/ms 50.62, render/busy/ms 42.19, stream/drive/ms 32.02, render/present/ms 31.13, render/graph/ms 3.83, pipelines/compile_ms 2.94, pipelines/front_ms 2.60, stream/spawn/ms 2.13, terrain/update/ms 1.31, terrain/select/ms 1.09 |
| warm-up | 4.37 | 72.2 | pipelines/wait_ms 4357, pipelines/compile_ms 126, pipelines/front_ms 121, main/busy/ms 55.09, render/thread/ms 43.16, render/busy/ms 41.25, stream/drive/ms 33.74, pipelines/driver_ms 5.15, render/graph/ms 4.57, stream/spawn/ms 1.61, render/present/ms 1.55, stream/tick/ms 1.38 |
| warm-up | 4.52 | 95.1 | pipelines/wait_ms 1505, pipelines/compile_ms 148, pipelines/front_ms 142, main/busy/ms 93.26, stream/drive/ms 81.40, render/thread/ms 28.21, render/busy/ms 27.11, pipelines/driver_ms 5.71, render/graph/ms 2.98, bigspace/total/ms 1.15, bigspace/high_precision/ms 1.02, stream/activate/ms 0.94 |
| warm-up | 4.58 | 26.8 | main/busy/ms 27.98, render/thread/ms 19.62, render/busy/ms 18.57, stream/drive/ms 10.76, render/graph/ms 2.92, stream/spawn/ms 2.05, render/present/ms 0.89, stream/tick/ms 0.80, terrain/update/ms 0.59, terrain/select/ms 0.47, bigspace/total/ms 0.32, render/acquire/ms 0.17 |
| warm-up | 4.72 | 39.8 | pipelines/wait_ms 961, pipelines/compile_ms 220, pipelines/front_ms 207, main/busy/ms 31.49, render/thread/ms 30.28, render/busy/ms 28.76, pipelines/driver_ms 13.36, stream/drive/ms 12.30, stream/spawn/ms 3.65, render/graph/ms 3.38, render/present/ms 1.35, stream/tick/ms 0.88 |
| warm-up | 4.75 | 26.7 | pipelines/wait_ms 961, pipelines/compile_ms 220, pipelines/front_ms 207, render/thread/ms 36.77, render/busy/ms 35.56, main/busy/ms 21.83, pipelines/driver_ms 13.36, stream/drive/ms 4.34, render/graph/ms 3.03, stream/spawn/ms 2.19, render/present/ms 1.01, terrain/update/ms 0.72 |
| warm-up | 4.93 | 34.1 | pipelines/wait_ms 4027, pipelines/compile_ms 86.34, pipelines/front_ms 80.13, render/thread/ms 53.86, render/busy/ms 51.89, main/busy/ms 22.98, render/graph/ms 6.32, pipelines/driver_ms 6.21, stream/drive/ms 4.59, stream/spawn/ms 2.54, render/present/ms 1.60, stream/tick/ms 1.15 |
| warm-up | 4.96 | 27.7 | pipelines/wait_ms 7940, pipelines/compile_ms 112, pipelines/front_ms 104, render/thread/ms 52.97, render/busy/ms 51.15, main/busy/ms 22.14, pipelines/driver_ms 7.81, render/graph/ms 6.17, stream/drive/ms 4.10, stream/spawn/ms 2.38, render/present/ms 1.50, stream/tick/ms 0.79 |
| warm-up | 4.99 | 30.5 | pipelines/wait_ms 6722, pipelines/compile_ms 42.52, render/thread/ms 41.59, render/busy/ms 40.38, pipelines/front_ms 40.02, main/busy/ms 19.45, stream/drive/ms 4.03, render/graph/ms 3.71, pipelines/driver_ms 2.50, stream/spawn/ms 2.26, render/present/ms 0.98, stream/tick/ms 0.78 |
| warm-up | 5.02 | 29.5 | pipelines/wait_ms 2809, render/thread/ms 47.41, render/busy/ms 45.84, main/busy/ms 19.10, pipelines/compile_ms 16.79, pipelines/front_ms 15.88, render/graph/ms 4.98, stream/drive/ms 4.37, stream/spawn/ms 2.33, render/present/ms 1.32, pipelines/driver_ms 0.91, stream/tick/ms 0.66 |
| warm-up | 5.07 | 39.5 | render/thread/ms 42.31, render/busy/ms 40.42, main/busy/ms 21.43, render/graph/ms 7.15, stream/drive/ms 5.55, stream/spawn/ms 1.93, render/present/ms 1.66, stream/tick/ms 0.69, bigspace/total/ms 0.58, terrain/update/ms 0.50, terrain/select/ms 0.39, bigspace/high_precision/ms 0.28 |
| warm-up | 5.14 | 56.2 | pipelines/wait_ms 10.5 k, pipelines/compile_ms 190, pipelines/front_ms 169, render/thread/ms 51.74, render/busy/ms 50.12, main/busy/ms 28.54, pipelines/driver_ms 20.65, render/graph/ms 5.56, stream/drive/ms 5.23, stream/spawn/ms 2.97, render/present/ms 1.37, stream/tick/ms 1.09 |
| warm-up | 5.18 | 39.1 | pipelines/wait_ms 18.3 k, pipelines/compile_ms 240, pipelines/front_ms 215, render/thread/ms 63.69, render/busy/ms 62.20, main/busy/ms 43.12, pipelines/driver_ms 25.33, stream/drive/ms 14.93, render/graph/ms 6.99, stream/spawn/ms 2.37, render/present/ms 1.22, stream/tick/ms 1.10 |
| warm-up | 5.21 | 25.2 | pipelines/wait_ms 9158, pipelines/compile_ms 58.80, pipelines/front_ms 53.40, render/thread/ms 47.29, render/busy/ms 45.92, main/busy/ms 38.12, stream/drive/ms 13.57, render/graph/ms 6.63, pipelines/driver_ms 5.41, terrain/update/ms 5.33, stream/spawn/ms 2.02, render/present/ms 1.15 |
| warm-up | 5.55 | 30.3 | pipelines/wait_ms 4609, render/thread/ms 35.16, render/busy/ms 33.13, pipelines/compile_ms 23.06, pipelines/front_ms 21.38, main/busy/ms 20.35, render/graph/ms 9.30, stream/drive/ms 3.49, stream/spawn/ms 2.08, render/present/ms 1.76, pipelines/driver_ms 1.68, bigspace/total/ms 0.70 |
| warm-up | 5.61 | 48.2 | pipelines/wait_ms 12.5 k, pipelines/compile_ms 78.24, pipelines/front_ms 68.13, render/thread/ms 41.94, render/busy/ms 40.09, main/busy/ms 33.56, stream/drive/ms 11.96, pipelines/driver_ms 10.10, render/graph/ms 5.66, stream/spawn/ms 2.03, render/present/ms 1.59, stream/tick/ms 1.13 |
| warm-up | 6.80 | 32.6 | pipelines/wait_ms 9104, pipelines/compile_ms 87.34, pipelines/front_ms 82.50, main/busy/ms 45.94, render/thread/ms 37.82, render/busy/ms 36.06, render/graph/ms 7.75, pipelines/driver_ms 4.84, render/present/ms 1.38, stream/drive/ms 1.26, bigspace/total/ms 0.88, terrain/update/ms 0.75 |
| warm-up | 7.67 | 27.1 | pipelines/wait_ms 2535, render/thread/ms 40.05, main/busy/ms 39.45, render/busy/ms 38.51, pipelines/compile_ms 23.22, pipelines/front_ms 22.43, render/graph/ms 11.68, bigspace/total/ms 1.43, render/present/ms 1.16, pipelines/driver_ms 0.78, terrain/update/ms 0.72, terrain/select/ms 0.61 |
| warm-up | 7.77 | 51.5 | pipelines/wait_ms 3553, pipelines/compile_ms 145, pipelines/front_ms 142, render/thread/ms 39.39, render/busy/ms 37.91, main/busy/ms 36.07, render/graph/ms 9.22, pipelines/driver_ms 3.61, render/present/ms 1.26, bigspace/total/ms 0.62, terrain/update/ms 0.47, stream/drive/ms 0.44 |
| measure | 0.82 | 27.3 | pipelines/wait_ms 5997, pipelines/compile_ms 42.88, pipelines/front_ms 40.27, main/busy/ms 33.03, render/thread/ms 30.02, render/busy/ms 28.55, render/graph/ms 10.00, pipelines/driver_ms 2.60, physics/step/ms 1.07, render/present/ms 1.07, physics/insert/ms 1.01, bigspace/total/ms 0.72 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_fog\balmora@2560x1440_off=normalmaps_orm_parallax_clouds_fog_froxels-3 --bench-baseline none --off normalmaps,orm,parallax,clouds,fog,froxels
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 18%
- during the run: other processes 16% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 92%) · 2580 MHz (min 2580) · memory 8001 MHz · 93.3 W · 77 °C (max 79) · VRAM 5752 MiB · throttled 100% of samples
