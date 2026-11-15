# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1404 frames over 15.0 s · **median 10.61 ms (94 fps)** · mean 10.69 · p95 11.83 · p99 12.69 (1% low 79 fps) · worst 26.24 ms · 3 spikes over 1.5× median · 12.0 stutters/min (work 4.0/min)
- warm-up: 429 frames over 9.0 s, worst 2640.4 ms
- hitches (> 26.5 ms): **0** measured, 27 in warm-up
- timed GPU passes: 8.68 ms of a 10.69 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.61 ms, limit 11 ms
- pass worst frame 26.24 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.628 | 2.666 | 3.046 | 0.99 | 0.056 |
| dlss_super_resolution | 2.148 | 2.179 | 2.448 | 0.99 | 0.186 |
| early prepass | 0.638 | 0.647 | 0.979 | 0.99 | 0.102 |
| atmosphere_luts | 0.353 | 0.358 | 0.543 | 0.99 | 0.003 |
| bloom | 0.331 | 0.335 | 0.349 | 0.99 | 0.065 |
| main_transmissive_pass_3d | 0.287 | 0.291 | 0.301 | 0.99 | 0.009 |
| water_velocity | 0.254 | 0.258 | 0.273 | 0.99 | 0.002 |
| water_sspr | 0.239 | 0.242 | 0.251 | 0.99 | 0.002 |
| ssao | 0.200 | 0.203 | 0.213 | 0.99 | 0.008 |
| uniform_allocation | 0.189 | 0.192 | 0.196 | 0.99 | 0.032 |
| atmos_froxel_inject | 0.152 | 0.155 | 0.154 | 0.99 | 0.002 |
| clouds_trace | 0.148 | 0.150 | 0.181 | 0.99 | 0.001 |
| bin_unpacking | 0.147 | 0.149 | 0.156 | 0.99 | 0.024 |
| water_fft | 0.136 | 0.138 | 0.138 | 0.99 | 0.003 |
| upscaling | 0.120 | 0.122 | 0.136 | 0.99 | 0.004 |
| clouds_composite | 0.114 | 0.115 | 0.117 | 0.99 | 0.002 |
| tonemapping | 0.093 | 0.094 | 0.096 | 0.99 | 0.003 |
| clustering | 0.068 | 0.069 | 0.079 | 0.99 | 0.022 |
| render_sky | 0.056 | 0.057 | 0.056 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.054 | 0.436 | 0.753 | 0.12 | 0.001 |
| clouds_resolve | 0.051 | 0.052 | 0.054 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.041 | 0.042 | 0.041 | 0.99 | 0.006 |
| water_snapshot | 0.040 | 0.040 | 0.054 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.032 | 0.036 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.032 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.225 | 0.235 | 0.12 | 0.004 |
| water_mips | 0.024 | 0.025 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.024 | 0.024 | 0.026 | 0.99 | 0.073 |
| per_view_shadow_pass | 0.023 | 0.023 | 0.029 | 0.99 | 0.007 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.004 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.015 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | — | — | — | - | — |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 43.803 | 0.000 | 9557.48 | 61499.5 |
| render/thread/ms | 9.774 | 10.930 | 25.32 | 13722.3 |
| render/busy/ms | 8.182 | 9.638 | 24.50 | 11486.9 |
| main/busy/ms | 7.285 | 8.738 | 11.55 | 10227.6 |
| render/graph/ms | 4.957 | 6.137 | 21.10 | 6959.2 |
| render/acquire/ms | 1.115 | 2.264 | 3.40 | 1565.5 |
| pipelines/compile_ms | 1.024 | 0.000 | 157.64 | 1438.3 |
| pipelines/front_ms | 0.992 | 0.000 | 154.73 | 1392.3 |
| render/present/ms | 0.477 | 0.563 | 10.57 | 669.9 |
| bigspace/total/ms | 0.381 | 0.762 | 2.77 | 534.8 |
| terrain/update/ms | 0.228 | 0.294 | 0.67 | 320.5 |
| terrain/select/ms | 0.198 | 0.255 | 0.62 | 278.5 |
| stream/drive/ms | 0.188 | 0.284 | 0.49 | 264.4 |
| far/select/ms | 0.164 | 0.236 | 0.44 | 229.6 |
| stream/tick/ms | 0.161 | 0.248 | 0.37 | 225.7 |
| controller/feed/ms | 0.150 | 0.204 | 0.38 | 210.3 |
| controller/move/ms | 0.135 | 0.219 | 0.37 | 189.5 |
| bigspace/high_precision/ms | 0.130 | 0.268 | 2.53 | 181.9 |
| bigspace/low_precision/ms | 0.098 | 0.207 | 2.10 | 137.5 |
| anim/evaluate/ms | 0.078 | 0.196 | 1.09 | 108.9 |
| anim/evaluate_cpu/ms | 0.066 | 0.083 | 0.26 | 92.6 |
| physics/step/ms | 0.037 | 0.064 | 0.80 | 52.2 |
| pipelines/driver_ms | 0.033 | 0.000 | 4.94 | 46.0 |
| weather/map/ms | 0.024 | 0.034 | 0.16 | 34.4 |
| physics/insert/ms | 0.013 | 0.059 | 0.80 | 17.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 15.1 |
| script/update/ms | 0.007 | 0.011 | 0.11 | 9.6 |
| controller/avoid/ms | 0.006 | 0.008 | 0.16 | 8.5 |
| script/input/ms | 0.005 | 0.006 | 0.04 | 6.7 |
| anim/apply/ms | 0.004 | 0.006 | 0.17 | 6.2 |
| script/ui/ms | 0.004 | 0.007 | 0.06 | 6.1 |
| controller/write/ms | 0.002 | 0.003 | 0.02 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.03 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.4 |
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
| anim/evaluated/count | count | 7 | 10 | 7 |
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
| crowd/joints_extracted/count | count | 4212 | 4620 | 4212 |
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
| frame/work/ms | ms | 9.38 | 24.50 | 9.26 |
| gpu/timed/ms | ms | 9.36 | 10.88 | 9.26 |
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
| phases/opaque3d/batch_sets | count | 25 | 25 | 25 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 24 | 24 | 24 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 25 | 25 | 25 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 24 | 24 | 24 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 33 | 33 | 33 |
| phases/shadow/bins | count | 2029 | 2029 | 2029 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5346 | 5346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 33 | 33 | 33 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 1122 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 191 | 191 | 191 |
| pipelines/waiting | count | 0 | 21 | 0 |
| preprocess/bind_groups | count | 0 | 26 | 0 |
| preprocess/bind_groups_created | count | 0 | 26 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 217 | 217 | 217 |
| process/cpu_usage | % | 27.64 | 41.44 | 25.48 |
| process/mem_usage | GiB | 4.35 | 4.38 | 4.38 |
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
| started_load_count |  | 3547 | 3549 | 3549 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
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
| system/cpu_usage | % | 32.76 | 59.66 | 32.34 |
| system/mem_usage | % | 72.78 | 72.95 | 72.94 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 97 | 97 | 97 |
| views/all/bins | count | 5199 | 5199 | 5199 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.6 k | 14.6 k |
| views/all/pipelines | count | 93 | 93 | 93 |
| views/all/pipelines_distinct | count | 83 | 83 | 83 |
| views/all/work_items | count | 14.8 k | 14.8 k | 14.8 k |
| views/cascade/batch_sets | count | 33 | 33 | 33 |
| views/cascade/bins | count | 2029 | 2029 | 2029 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5346 | 5346 |
| views/cascade/pipelines | count | 33 | 33 | 33 |
| views/cascade/work_items | count | 5346 | 5346 | 5346 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 64 | 64 | 64 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 60 | 60 | 60 |
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
| anim/events/count | count | 0.09 | 2 | 128 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.53 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.08 | 102 | 110 |
| physics/inserts | count | 0.75 | 141 | 1054 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 2 | 900 |
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

## Hitches (> 26.5 ms; slowest 27, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.64 | 2640.4 | main/busy/ms 2615, surface/advance_cpu_ms 74.95, surface/cell_upload_ms 0.77, stream/drive/ms 0.35, weather/map/ms 0.19, stream/tick/ms 0.11, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.04, script/update/ms 0.03, weather/eval/ms 0.03, anim/evaluate/ms 0.02, far/select/ms 0.01 |
| warm-up | 3.37 | 727.1 | main/busy/ms 2650, render/thread/ms 689, render/busy/ms 546, render/acquire/ms 140, surface/advance_cpu_ms 74.95, render/graph/ms 5.96, render/present/ms 2.07, stream/drive/ms 1.11, surface/cell_upload_ms 0.77, terrain/update/ms 0.61, stream/tick/ms 0.53, terrain/select/ms 0.41 |
| warm-up | 3.58 | 211.8 | pipelines/wait_ms 1605, render/thread/ms 700, render/busy/ms 557, pipelines/compile_ms 222, pipelines/front_ms 205, render/acquire/ms 140, main/busy/ms 50.83, terrain/update/ms 19.17, pipelines/driver_ms 16.86, render/graph/ms 9.07, render/present/ms 3.10, stream/drive/ms 1.47 |
| warm-up | 3.82 | 108.2 | pipelines/compile_ms 235, pipelines/front_ms 220, pipelines/wait_ms 218, render/thread/ms 67.82, main/busy/ms 66.68, render/busy/ms 65.57, pipelines/driver_ms 14.54, render/graph/ms 5.56, stream/drive/ms 2.32, render/present/ms 1.97, stream/tick/ms 1.61, terrain/update/ms 0.51 |
| warm-up | 3.87 | 34.1 | render/thread/ms 40.58, render/busy/ms 38.91, main/busy/ms 30.43, terrain/update/ms 5.82, render/graph/ms 3.54, stream/drive/ms 2.12, render/present/ms 1.42, stream/tick/ms 0.83, stream/spawn/ms 0.80, terrain/select/ms 0.67, render/acquire/ms 0.25, bigspace/total/ms 0.23 |
| warm-up | 3.97 | 43.5 | render/thread/ms 49.27, render/busy/ms 47.83, main/busy/ms 38.86, stream/drive/ms 29.21, render/graph/ms 3.69, render/present/ms 1.22, stream/tick/ms 0.60, terrain/update/ms 0.51, terrain/select/ms 0.41, render/acquire/ms 0.22, bigspace/total/ms 0.19, stream/spawn/ms 0.13 |
| warm-up | 4.02 | 43.8 | pipelines/wait_ms 1838, pipelines/compile_ms 240, pipelines/front_ms 220, render/thread/ms 35.93, render/busy/ms 33.79, main/busy/ms 30.37, pipelines/driver_ms 20.12, stream/drive/ms 17.14, render/graph/ms 5.95, render/present/ms 1.89, stream/spawn/ms 1.00, stream/tick/ms 1.00 |
| warm-up | 4.05 | 28.4 | pipelines/wait_ms 2698, pipelines/compile_ms 274, pipelines/front_ms 251, main/busy/ms 57.34, render/thread/ms 49.10, render/busy/ms 46.42, stream/drive/ms 38.41, pipelines/driver_ms 22.23, render/graph/ms 7.29, render/present/ms 2.39, stream/tick/ms 1.21, stream/spawn/ms 0.78 |
| warm-up | 4.19 | 137.3 | pipelines/wait_ms 4374, pipelines/compile_ms 168, pipelines/front_ms 159, render/thread/ms 93.46, render/busy/ms 91.64, main/busy/ms 60.22, stream/drive/ms 38.60, pipelines/driver_ms 9.10, render/graph/ms 4.12, render/present/ms 1.53, stream/spawn/ms 1.15, stream/tick/ms 1.15 |
| warm-up | 4.25 | 63.2 | pipelines/wait_ms 5664, pipelines/compile_ms 179, pipelines/front_ms 169, render/thread/ms 117, render/busy/ms 114, main/busy/ms 56.06, stream/drive/ms 34.89, render/graph/ms 15.45, pipelines/driver_ms 10.68, render/present/ms 2.59, stream/spawn/ms 2.18, stream/tick/ms 0.90 |
| warm-up | 4.40 | 67.8 | pipelines/wait_ms 2662, pipelines/compile_ms 155, pipelines/front_ms 149, main/busy/ms 56.86, render/thread/ms 38.67, render/busy/ms 36.95, stream/drive/ms 35.64, pipelines/driver_ms 6.55, render/graph/ms 3.75, stream/spawn/ms 1.78, stream/tick/ms 1.70, render/present/ms 1.49 |
| warm-up | 4.51 | 58.9 | main/busy/ms 60.28, stream/drive/ms 49.97, render/thread/ms 22.58, render/busy/ms 21.05, render/graph/ms 2.97, render/present/ms 1.29, stream/tick/ms 1.04, stream/spawn/ms 0.73, terrain/update/ms 0.50, terrain/select/ms 0.40, render/acquire/ms 0.24, bigspace/total/ms 0.20 |
| warm-up | 4.57 | 55.5 | main/busy/ms 52.62, stream/drive/ms 37.61, render/thread/ms 19.81, render/busy/ms 18.46, render/graph/ms 2.85, stream/spawn/ms 2.40, render/present/ms 1.09, stream/tick/ms 0.78, terrain/update/ms 0.58, terrain/select/ms 0.45, stream/activate/ms 0.42, bigspace/total/ms 0.38 |
| warm-up | 4.78 | 36.7 | render/thread/ms 29.48, main/busy/ms 28.95, render/busy/ms 28.21, stream/drive/ms 10.64, render/graph/ms 3.65, stream/spawn/ms 3.05, render/present/ms 1.10, stream/tick/ms 0.89, terrain/update/ms 0.80, terrain/select/ms 0.63, stream/activate/ms 0.49, bigspace/total/ms 0.37 |
| warm-up | 4.80 | 26.9 | pipelines/wait_ms 2179, pipelines/compile_ms 208, pipelines/front_ms 197, render/thread/ms 36.40, render/busy/ms 35.39, main/busy/ms 22.44, pipelines/driver_ms 11.07, stream/drive/ms 4.31, render/graph/ms 2.53, stream/spawn/ms 2.17, stream/tick/ms 0.89, render/present/ms 0.83 |
| warm-up | 4.99 | 34.1 | render/thread/ms 51.26, render/busy/ms 49.80, main/busy/ms 24.69, render/graph/ms 6.21, stream/drive/ms 4.74, stream/spawn/ms 2.04, stream/tick/ms 1.39, render/present/ms 1.21, terrain/update/ms 0.86, terrain/select/ms 0.67, bigspace/total/ms 0.67, bigspace/high_precision/ms 0.35 |
| warm-up | 5.02 | 31.3 | render/thread/ms 49.36, render/busy/ms 48.12, main/busy/ms 22.94, stream/drive/ms 4.71, render/graph/ms 4.14, stream/spawn/ms 2.08, stream/tick/ms 1.22, render/present/ms 1.04, terrain/update/ms 0.69, bigspace/total/ms 0.59, terrain/select/ms 0.56, bigspace/high_precision/ms 0.32 |
| warm-up | 5.11 | 31.2 | pipelines/wait_ms 5113, pipelines/compile_ms 170, pipelines/front_ms 161, render/thread/ms 46.10, render/busy/ms 44.91, main/busy/ms 27.64, pipelines/driver_ms 8.54, stream/drive/ms 6.09, render/graph/ms 3.96, stream/tick/ms 1.07, stream/spawn/ms 0.99, render/present/ms 0.94 |
| warm-up | 5.23 | 48.2 | main/busy/ms 43.72, render/thread/ms 43.34, render/busy/ms 42.20, stream/drive/ms 19.62, render/graph/ms 3.95, stream/spawn/ms 2.60, stream/tick/ms 1.07, bigspace/total/ms 1.07, render/present/ms 0.97, terrain/update/ms 0.79, terrain/select/ms 0.61, bigspace/high_precision/ms 0.38 |
| warm-up | 5.64 | 45.5 | pipelines/wait_ms 9342, pipelines/compile_ms 63.11, pipelines/front_ms 58.75, main/busy/ms 37.39, render/thread/ms 35.98, render/busy/ms 34.16, stream/drive/ms 13.73, render/graph/ms 5.03, pipelines/driver_ms 4.36, stream/spawn/ms 2.31, render/present/ms 1.56, stream/tick/ms 1.15 |
| warm-up | 6.18 | 29.5 | pipelines/wait_ms 2484, pipelines/compile_ms 59.62, pipelines/front_ms 57.76, main/busy/ms 32.29, render/thread/ms 24.37, render/busy/ms 22.68, render/graph/ms 5.83, pipelines/driver_ms 1.86, render/present/ms 1.53, stream/drive/ms 1.24, bigspace/total/ms 1.16, terrain/update/ms 0.62 |
| warm-up | 6.52 | 27.2 | pipelines/wait_ms 2866, main/busy/ms 35.68, render/thread/ms 32.29, render/busy/ms 30.49, pipelines/compile_ms 16.13, pipelines/front_ms 15.24, render/graph/ms 9.97, render/present/ms 1.55, stream/drive/ms 1.39, bigspace/total/ms 1.15, pipelines/driver_ms 0.89, terrain/update/ms 0.76 |
| warm-up | 6.55 | 30.7 | main/busy/ms 44.30, render/thread/ms 41.78, render/busy/ms 40.05, render/graph/ms 6.95, render/present/ms 1.40, stream/drive/ms 1.38, bigspace/total/ms 1.28, terrain/update/ms 0.82, anim/evaluate_cpu/ms 0.76, terrain/select/ms 0.65, stream/tick/ms 0.60, bigspace/low_precision/ms 0.51 |
| warm-up | 6.58 | 26.9 | pipelines/wait_ms 2897, pipelines/compile_ms 82.83, pipelines/front_ms 79.07, render/thread/ms 40.13, render/busy/ms 38.69, main/busy/ms 33.78, render/graph/ms 5.76, pipelines/driver_ms 3.76, stream/drive/ms 1.56, render/present/ms 1.17, bigspace/total/ms 0.95, stream/tick/ms 0.68 |
| warm-up | 6.63 | 31.9 | pipelines/wait_ms 8505, main/busy/ms 52.30, pipelines/compile_ms 29.03, render/thread/ms 28.88, pipelines/front_ms 27.04, render/busy/ms 26.56, render/graph/ms 9.63, render/present/ms 2.02, pipelines/driver_ms 2.00, stream/drive/ms 1.27, bigspace/total/ms 0.72, anim/evaluate_cpu/ms 0.56 |
| warm-up | 6.68 | 52.2 | pipelines/wait_ms 24.3 k, main/busy/ms 58.95, pipelines/compile_ms 55.06, pipelines/front_ms 52.52, render/thread/ms 45.95, render/busy/ms 44.17, render/graph/ms 8.58, pipelines/driver_ms 2.54, render/present/ms 1.46, stream/drive/ms 1.14, anim/evaluate_cpu/ms 1.03, physics/insert/ms 0.90 |
| warm-up | 6.71 | 31.6 | pipelines/wait_ms 24.3 k, pipelines/compile_ms 55.06, pipelines/front_ms 52.52, render/thread/ms 49.77, render/busy/ms 48.51, main/busy/ms 43.31, render/graph/ms 5.71, pipelines/driver_ms 2.54, physics/insert/ms 1.11, stream/drive/ms 1.06, render/present/ms 1.02, bigspace/total/ms 0.82 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lean_pom\balmora@2560x1440_off=parallax-2 --bench-baseline none --off parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 95%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.9 W · 76 °C (max 78) · VRAM 5699 MiB · throttled 100% of samples
