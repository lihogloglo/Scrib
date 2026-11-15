# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1335 frames over 15.0 s · **median 11.14 ms (90 fps)** · mean 11.24 · p95 12.59 · p99 13.67 (1% low 73 fps) · worst 25.02 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 0.0/min)
- warm-up: 531 frames over 9.8 s, worst 2475.6 ms
- hitches (> 27.8 ms): **0** measured, 26 in warm-up
- timed GPU passes: 8.99 ms of a 11.24 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.14 ms, limit 11 ms
- pass worst frame 25.02 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.908 | 2.984 | 3.363 | 0.97 | 0.081 |
| dlss_super_resolution | 2.116 | 2.171 | 2.444 | 0.97 | 0.184 |
| early prepass | 0.621 | 0.638 | 0.974 | 0.97 | 0.120 |
| atmosphere_luts | 0.352 | 0.361 | 0.674 | 0.97 | 0.004 |
| bloom | 0.329 | 0.338 | 0.653 | 0.97 | 0.066 |
| main_transmissive_pass_3d | 0.293 | 0.301 | 0.306 | 0.97 | 0.008 |
| water_velocity | 0.248 | 0.254 | 0.271 | 0.97 | 0.002 |
| water_sspr | 0.244 | 0.250 | 0.331 | 0.97 | 0.002 |
| uniform_allocation | 0.237 | 0.243 | 0.248 | 0.97 | 0.047 |
| ssao | 0.199 | 0.204 | 0.211 | 0.97 | 0.008 |
| bin_unpacking | 0.187 | 0.191 | 0.201 | 0.97 | 0.031 |
| atmos_froxel_inject | 0.148 | 0.152 | 0.153 | 0.97 | 0.002 |
| clouds_trace | 0.146 | 0.150 | 0.173 | 0.97 | 0.001 |
| water_fft | 0.135 | 0.139 | 0.136 | 0.97 | 0.003 |
| upscaling | 0.121 | 0.124 | 0.155 | 0.97 | 0.004 |
| clouds_composite | 0.114 | 0.117 | 0.117 | 0.97 | 0.002 |
| tonemapping | 0.087 | 0.089 | 0.091 | 0.97 | 0.004 |
| clustering | 0.068 | 0.070 | 0.076 | 0.97 | 0.022 |
| render_sky | 0.056 | 0.058 | 0.060 | 0.97 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.428 | 0.424 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.051 | 0.053 | 0.97 | 0.001 |
| water_snapshot | 0.042 | 0.043 | 0.055 | 0.97 | 0.002 |
| early_mesh_preprocessing | 0.040 | 0.041 | 0.042 | 0.97 | 0.007 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.97 | 0.004 |
| main_transparent_pass_3d | 0.029 | 0.030 | 0.043 | 0.97 | 0.074 |
| lightprobe_radiance_map | 0.028 | 0.237 | 0.247 | 0.12 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.031 | 0.97 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.97 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.023 | 0.023 | 0.97 | 0.007 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.97 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.97 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.97 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.005 | 0.97 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.002 | 0.013 | 0.011 | 0.12 | 0.001 |
| auxiliary_views | 0.001 | 0.002 | 0.003 | 0.97 | 0.003 |
| blas_build | — | — | — | - | — |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.324 | 11.640 | 23.73 | 13782.8 |
| render/busy/ms | 9.307 | 10.441 | 14.11 | 12424.4 |
| main/busy/ms | 7.556 | 9.223 | 11.00 | 10086.7 |
| pipelines/wait_ms | 6.805 | 0.000 | 4602.08 | 9084.1 |
| render/graph/ms | 5.771 | 6.704 | 9.77 | 7703.9 |
| render/acquire/ms | 0.512 | 1.499 | 3.11 | 683.2 |
| render/present/ms | 0.506 | 0.606 | 11.87 | 675.2 |
| bigspace/total/ms | 0.339 | 0.548 | 2.79 | 452.3 |
| terrain/update/ms | 0.222 | 0.280 | 0.54 | 296.4 |
| stream/drive/ms | 0.207 | 0.301 | 0.55 | 276.6 |
| terrain/select/ms | 0.192 | 0.242 | 0.42 | 256.9 |
| stream/tick/ms | 0.177 | 0.264 | 0.51 | 237.0 |
| far/select/ms | 0.175 | 0.256 | 0.46 | 233.0 |
| controller/feed/ms | 0.151 | 0.198 | 0.52 | 201.4 |
| bigspace/high_precision/ms | 0.113 | 0.240 | 2.56 | 150.7 |
| controller/move/ms | 0.112 | 0.168 | 0.29 | 148.9 |
| pipelines/compile_ms | 0.106 | 0.000 | 122.75 | 141.6 |
| pipelines/front_ms | 0.103 | 0.000 | 120.05 | 137.9 |
| anim/evaluate/ms | 0.086 | 0.187 | 1.02 | 114.5 |
| bigspace/low_precision/ms | 0.085 | 0.187 | 2.30 | 114.0 |
| anim/evaluate_cpu/ms | 0.060 | 0.072 | 0.39 | 79.7 |
| physics/step/ms | 0.035 | 0.062 | 0.30 | 46.5 |
| weather/map/ms | 0.025 | 0.035 | 0.13 | 33.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 14.5 |
| script/update/ms | 0.008 | 0.011 | 0.16 | 10.3 |
| controller/avoid/ms | 0.006 | 0.008 | 0.03 | 8.5 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 6.7 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 5.7 |
| script/ui/ms | 0.004 | 0.005 | 0.13 | 5.1 |
| pipelines/driver_ms | 0.003 | 0.000 | 2.70 | 3.7 |
| controller/write/ms | 0.002 | 0.003 | 0.16 | 2.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 1.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 1.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.4 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| frame/work/ms | ms | 9.88 | 14.11 | 10.03 |
| gpu/timed/ms | ms | 9.78 | 11.33 | 10.03 |
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
| phases/opaque3d/batch_sets | count | 33 | 33 | 33 |
| phases/opaque3d/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 28 | 28 | 28 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 33 | 33 | 33 |
| phases/opaque3d_prepass/bins | count | 1305 | 1305 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 28 | 28 | 28 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 44 | 44 | 44 |
| phases/shadow/bins | count | 2029 | 2029 | 2029 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5346 | 5346 | 5346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 37 | 37 | 37 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 197 | 197 | 197 |
| pipelines/waiting | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 0 | 0 | 0 |
| preprocess/bind_groups_created | count | 0 | 0 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 271 | 271 | 271 |
| process/cpu_usage | % | 24.77 | 36.06 | 18.27 |
| process/mem_usage | GiB | 3.66 | 3.75 | 3.75 |
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
| statics/material_keys | count | 4 | 4 | 4 |
| statics/material_pipelines | count | 4 | 4 | 4 |
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
| system/cpu_usage | % | 32.40 | 54.26 | 28.15 |
| system/mem_usage | % | 59.34 | 59.59 | 59.59 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 1490 | 1490 | 1490 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 124 | 124 | 124 |
| views/all/bins | count | 5199 | 5199 | 5199 |
| views/all/count | count | 4 | 4 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.6 k | 14.6 k |
| views/all/pipelines | count | 105 | 105 | 105 |
| views/all/pipelines_distinct | count | 93 | 93 | 93 |
| views/all/work_items | count | 14.8 k | 14.8 k | 14.8 k |
| views/cascade/batch_sets | count | 44 | 44 | 44 |
| views/cascade/bins | count | 2029 | 2029 | 2029 |
| views/cascade/count | count | 3 | 3 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5346 | 5346 | 5346 |
| views/cascade/pipelines | count | 37 | 37 | 37 |
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
| views/main/batch_sets | count | 80 | 80 | 80 |
| views/main/bins | count | 3170 | 3170 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 68 | 68 | 68 |
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
| anim/events/count | count | 0.10 | 3 | 133 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.71 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | -0.00 | 1 | -1 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.67 | 2 | 901 |
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

## Hitches (> 27.8 ms; slowest 26, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.48 | 2475.6 | main/busy/ms 2451, surface/advance_cpu_ms 81.10, surface/cell_upload_ms 0.70, stream/drive/ms 0.32, weather/map/ms 0.20, bigspace/total/ms 0.09, stream/tick/ms 0.08, bigspace/high_precision/ms 0.06, weather/eval/ms 0.02, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 2.94 | 463.7 | main/busy/ms 2484, render/thread/ms 426, render/busy/ms 318, render/acquire/ms 106, surface/advance_cpu_ms 81.10, render/graph/ms 5.38, render/present/ms 2.63, stream/drive/ms 0.94, surface/cell_upload_ms 0.70, terrain/update/ms 0.65, stream/tick/ms 0.50, terrain/select/ms 0.44 |
| warm-up | 3.15 | 215.4 | pipelines/wait_ms 1725, render/thread/ms 440, render/busy/ms 330, pipelines/compile_ms 229, pipelines/front_ms 215, render/acquire/ms 106, main/busy/ms 50.55, terrain/update/ms 18.02, pipelines/driver_ms 13.90, render/graph/ms 8.82, render/present/ms 3.47, stream/drive/ms 1.03 |
| warm-up | 3.29 | 45.7 | render/thread/ms 69.60, render/busy/ms 45.28, main/busy/ms 38.35, render/acquire/ms 22.70, render/graph/ms 3.53, stream/drive/ms 2.83, bigspace/total/ms 1.70, render/present/ms 1.62, stream/tick/ms 1.44, bigspace/high_precision/ms 1.05, terrain/update/ms 0.59, terrain/select/ms 0.41 |
| warm-up | 3.35 | 30.3 | main/busy/ms 73.40, render/thread/ms 54.17, render/busy/ms 32.53, render/acquire/ms 20.41, terrain/update/ms 11.26, render/graph/ms 3.61, stream/drive/ms 3.04, stream/tick/ms 2.09, terrain/select/ms 1.70, render/present/ms 1.23, bigspace/total/ms 0.24, weather/eval/ms 0.21 |
| warm-up | 3.47 | 122.0 | pipelines/compile_ms 337, pipelines/front_ms 317, pipelines/wait_ms 255, render/thread/ms 115, render/busy/ms 103, main/busy/ms 80.72, pipelines/driver_ms 20.88, render/acquire/ms 10.94, render/graph/ms 3.99, stream/drive/ms 1.95, stream/tick/ms 1.53, render/present/ms 1.23 |
| warm-up | 3.61 | 56.7 | main/busy/ms 42.60, render/thread/ms 38.36, render/busy/ms 36.99, stream/drive/ms 31.09, render/graph/ms 2.58, render/present/ms 1.07, stream/tick/ms 0.82, terrain/update/ms 0.50, terrain/select/ms 0.39, render/acquire/ms 0.30, bigspace/total/ms 0.19, stream/spawn/ms 0.16 |
| warm-up | 3.66 | 32.4 | pipelines/wait_ms 3577, pipelines/compile_ms 205, pipelines/front_ms 187, render/thread/ms 41.98, render/busy/ms 40.15, main/busy/ms 30.45, pipelines/driver_ms 18.12, stream/drive/ms 15.23, render/graph/ms 5.11, render/present/ms 1.58, stream/spawn/ms 1.42, terrain/update/ms 0.83 |
| warm-up | 3.73 | 48.4 | pipelines/wait_ms 1947, render/thread/ms 55.90, render/busy/ms 54.10, main/busy/ms 36.32, pipelines/compile_ms 27.96, pipelines/front_ms 25.81, stream/drive/ms 15.51, render/graph/ms 5.52, pipelines/driver_ms 2.16, render/present/ms 1.51, stream/tick/ms 1.09, stream/spawn/ms 1.05 |
| warm-up | 3.77 | 35.0 | main/busy/ms 67.70, render/thread/ms 66.85, render/busy/ms 65.47, stream/drive/ms 46.23, render/graph/ms 5.19, stream/spawn/ms 2.82, terrain/update/ms 1.36, stream/tick/ms 1.21, terrain/select/ms 1.19, render/present/ms 1.15, bigspace/total/ms 0.30, stream/activate/ms 0.27 |
| warm-up | 3.91 | 141.9 | pipelines/wait_ms 1812, pipelines/compile_ms 179, pipelines/front_ms 170, render/thread/ms 116, render/busy/ms 81.68, main/busy/ms 67.92, stream/drive/ms 44.98, render/present/ms 34.52, pipelines/driver_ms 8.44, render/graph/ms 5.25, stream/spawn/ms 2.63, stream/tick/ms 1.34 |
| warm-up | 3.96 | 56.2 | pipelines/wait_ms 3967, pipelines/compile_ms 247, pipelines/front_ms 235, render/thread/ms 133, render/busy/ms 98.30, main/busy/ms 36.37, render/present/ms 34.43, pipelines/driver_ms 12.37, stream/drive/ms 10.44, render/graph/ms 4.32, stream/spawn/ms 2.00, stream/tick/ms 1.24 |
| warm-up | 4.02 | 32.8 | main/busy/ms 35.75, stream/drive/ms 25.00, render/thread/ms 15.13, render/busy/ms 14.04, render/graph/ms 2.62, render/present/ms 0.89, terrain/update/ms 0.68, stream/tick/ms 0.66, stream/activate/ms 0.54, terrain/select/ms 0.54, bigspace/total/ms 0.23, render/acquire/ms 0.19 |
| warm-up | 4.15 | 88.6 | pipelines/wait_ms 10.1 k, pipelines/compile_ms 113, pipelines/front_ms 105, main/busy/ms 94.01, stream/drive/ms 83.41, render/thread/ms 25.59, render/busy/ms 24.35, pipelines/driver_ms 7.66, render/graph/ms 3.06, render/present/ms 1.05, stream/tick/ms 0.91, terrain/update/ms 0.49 |
| warm-up | 4.27 | 32.3 | pipelines/wait_ms 1025, pipelines/compile_ms 43.84, pipelines/front_ms 42.02, main/busy/ms 37.46, render/thread/ms 18.81, render/busy/ms 17.66, stream/drive/ms 14.46, render/graph/ms 2.70, pipelines/driver_ms 1.82, stream/activate/ms 1.18, stream/spawn/ms 1.10, render/present/ms 0.94 |
| warm-up | 4.35 | 34.9 | main/busy/ms 29.92, render/thread/ms 29.21, render/busy/ms 27.71, stream/drive/ms 12.25, render/graph/ms 3.84, stream/spawn/ms 3.77, render/present/ms 1.27, stream/tick/ms 0.91, terrain/update/ms 0.68, stream/activate/ms 0.62, terrain/select/ms 0.54, physics/insert/ms 0.43 |
| warm-up | 4.57 | 50.2 | pipelines/wait_ms 4866, render/thread/ms 50.78, render/busy/ms 49.38, main/busy/ms 40.77, pipelines/compile_ms 25.75, pipelines/front_ms 23.08, stream/drive/ms 9.83, stream/spawn/ms 7.46, render/graph/ms 5.81, pipelines/driver_ms 2.67, stream/tick/ms 1.39, render/present/ms 1.21 |
| warm-up | 4.60 | 32.8 | render/thread/ms 61.59, render/busy/ms 60.20, main/busy/ms 30.65, render/graph/ms 5.30, stream/drive/ms 4.55, stream/spawn/ms 2.12, render/present/ms 1.15, stream/tick/ms 1.11, physics/insert/ms 0.85, bigspace/total/ms 0.76, terrain/update/ms 0.61, terrain/select/ms 0.45 |
| warm-up | 4.75 | 35.5 | render/thread/ms 55.03, render/busy/ms 53.34, main/busy/ms 28.01, render/graph/ms 8.44, stream/drive/ms 6.10, stream/spawn/ms 2.36, render/present/ms 1.41, stream/tick/ms 1.18, bigspace/total/ms 0.83, terrain/update/ms 0.70, terrain/select/ms 0.54, bigspace/high_precision/ms 0.45 |
| warm-up | 5.16 | 41.7 | pipelines/wait_ms 10.4 k, pipelines/compile_ms 82.39, pipelines/front_ms 77.51, render/thread/ms 39.70, render/busy/ms 38.22, main/busy/ms 32.18, stream/drive/ms 8.18, render/graph/ms 6.81, pipelines/driver_ms 4.88, render/present/ms 1.25, stream/tick/ms 0.88, physics/insert/ms 0.83 |
| warm-up | 5.24 | 30.0 | render/thread/ms 33.63, main/busy/ms 33.53, render/busy/ms 31.96, render/graph/ms 6.79, render/present/ms 1.45, stream/drive/ms 1.19, bigspace/total/ms 0.99, terrain/update/ms 0.57, stream/tick/ms 0.52, terrain/select/ms 0.45, physics/step/ms 0.42, physics/insert/ms 0.39 |
| warm-up | 5.27 | 29.3 | render/thread/ms 41.02, render/busy/ms 39.71, main/busy/ms 26.30, render/graph/ms 5.65, stream/drive/ms 1.32, render/present/ms 1.12, bigspace/total/ms 0.74, terrain/update/ms 0.68, stream/tick/ms 0.58, terrain/select/ms 0.53, stream/spawn/ms 0.35, bigspace/high_precision/ms 0.22 |
| warm-up | 5.32 | 32.9 | pipelines/wait_ms 2149, main/busy/ms 32.33, render/thread/ms 31.73, render/busy/ms 30.24, render/graph/ms 7.74, stream/drive/ms 1.33, render/present/ms 1.31, pipelines/compile_ms 0.67, pipelines/driver_ms 0.67, stream/tick/ms 0.62, terrain/update/ms 0.48, bigspace/total/ms 0.47 |
| warm-up | 5.40 | 30.4 | pipelines/wait_ms 2149, pipelines/compile_ms 61.97, pipelines/front_ms 60.43, main/busy/ms 46.95, render/thread/ms 37.42, render/busy/ms 35.70, render/graph/ms 8.70, terrain/update/ms 2.04, pipelines/driver_ms 1.55, stream/drive/ms 1.51, render/present/ms 1.43, bigspace/total/ms 0.81 |
| warm-up | 5.43 | 33.8 | pipelines/wait_ms 2149, pipelines/compile_ms 61.97, pipelines/front_ms 60.43, render/thread/ms 40.01, render/busy/ms 38.68, main/busy/ms 38.65, render/graph/ms 7.53, pipelines/driver_ms 1.55, stream/drive/ms 1.31, render/present/ms 1.09, physics/insert/ms 0.93, bigspace/total/ms 0.72 |
| warm-up | 7.53 | 28.1 | render/thread/ms 50.23, render/acquire/ms 34.46, render/busy/ms 14.85, main/busy/ms 10.44, render/graph/ms 7.94, render/present/ms 0.92, terrain/update/ms 0.44, bigspace/total/ms 0.42, terrain/select/ms 0.37, stream/drive/ms 0.32, far/select/ms 0.29, stream/tick/ms 0.27 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --texture-cap 512 --resolution 2560x1440 --bench-out target/perf\lean_cap512\balmora@2560x1440_--texture-cap_512-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 7%
- during the run: other processes 8% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **97%** (min 87%) · 2580 MHz (min 2580) · memory 8001 MHz · 92.4 W · 77 °C (max 79) · VRAM 4731 MiB · throttled 100% of samples
