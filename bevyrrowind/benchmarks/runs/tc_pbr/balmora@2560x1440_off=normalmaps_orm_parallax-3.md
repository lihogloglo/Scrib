# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 927 frames over 15.0 s · **median 16.11 ms (62 fps)** · mean 16.19 · p95 17.83 · p99 19.51 (1% low 51 fps) · worst 25.11 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 0.0/min)
- warm-up: 409 frames over 8.8 s, worst 2488.6 ms
- hitches (> 40.3 ms): **0** measured, 14 in warm-up
- timed GPU passes: 14.22 ms of a 16.19 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 16.11 ms, limit 11 ms
- pass worst frame 25.11 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 7.036 | 7.036 | 7.769 | 1.00 | 0.050 |
| early prepass | 1.003 | 1.003 | 1.643 | 1.00 | 0.090 |
| ssao | 0.845 | 0.845 | 1.446 | 1.00 | 0.006 |
| main_transmissive_pass_3d | 0.745 | 0.745 | 1.431 | 1.00 | 0.007 |
| clouds_composite | 0.490 | 0.490 | 1.097 | 1.00 | 0.001 |
| clouds_trace | 0.439 | 0.439 | 0.482 | 1.00 | 0.001 |
| taa | 0.418 | 0.418 | 0.769 | 1.00 | 0.002 |
| atmosphere_luts | 0.383 | 0.383 | 0.867 | 1.00 | 0.003 |
| bloom | 0.344 | 0.344 | 0.339 | 1.00 | 0.058 |
| water_sspr | 0.330 | 0.330 | 0.669 | 1.00 | 0.002 |
| water_velocity | 0.286 | 0.286 | 0.551 | 1.00 | 0.001 |
| render_sky | 0.223 | 0.223 | 0.207 | 1.00 | 0.001 |
| clouds_resolve | 0.218 | 0.218 | 0.215 | 1.00 | 0.001 |
| water_snapshot | 0.199 | 0.199 | 0.238 | 1.00 | 0.001 |
| uniform_allocation | 0.193 | 0.193 | 0.184 | 1.00 | 0.027 |
| atmos_froxel_inject | 0.163 | 0.163 | 0.155 | 1.00 | 0.001 |
| bin_unpacking | 0.148 | 0.148 | 0.149 | 1.00 | 0.021 |
| water_fft | 0.146 | 0.146 | 0.141 | 1.00 | 0.002 |
| upscaling | 0.128 | 0.128 | 0.132 | 1.00 | 0.004 |
| tonemapping | 0.097 | 0.097 | 0.094 | 1.00 | 0.002 |
| clustering | 0.067 | 0.067 | 0.074 | 1.00 | 0.020 |
| lightprobe_irradiance_map | 0.055 | 0.436 | 0.435 | 0.13 | 0.001 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.043 | 1.00 | 0.004 |
| atmos_froxel_integrate | 0.036 | 0.036 | 0.035 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.033 | 0.266 | 0.586 | 0.13 | 0.003 |
| clouds_shadow | 0.030 | 0.030 | 0.033 | 1.00 | 0.003 |
| main_transparent_pass_3d | 0.025 | 0.025 | 0.026 | 1.00 | 0.066 |
| water_mips | 0.024 | 0.024 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.023 | 0.030 | 1.00 | 0.006 |
| atmosphere_environment | 0.011 | 0.011 | 0.013 | 1.00 | 0.003 |
| clouds_fog_light | 0.011 | 0.011 | 0.011 | 1.00 | 0.002 |
| night_sky | 0.010 | 0.010 | 0.010 | 1.00 | 0.001 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 1.00 | 0.002 |
| auxiliary_views | 0.003 | 0.003 | 0.003 | 1.00 | 0.004 |
| lightprobe_downsampling_second_pass | 0.002 | 0.017 | 0.012 | 0.13 | 0.001 |
| lightprobe_copy | 0.002 | 0.016 | 0.017 | 0.13 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.015 | 0.13 | 0.001 |
| blas_build | — | — | — | - | — |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 15.279 | 16.936 | 24.15 | 14163.9 |
| render/acquire/ms | 7.741 | 9.253 | 15.42 | 7175.8 |
| render/busy/ms | 7.052 | 7.733 | 9.57 | 6537.6 |
| main/busy/ms | 5.940 | 6.489 | 8.98 | 5506.8 |
| pipelines/wait_ms | 3.907 | 0.000 | 1320.03 | 3621.7 |
| render/graph/ms | 3.759 | 4.370 | 5.28 | 3484.8 |
| render/present/ms | 0.486 | 0.583 | 10.86 | 450.5 |
| bigspace/total/ms | 0.275 | 0.354 | 0.55 | 254.7 |
| pipelines/compile_ms | 0.253 | 0.000 | 112.35 | 234.4 |
| pipelines/front_ms | 0.245 | 0.000 | 109.55 | 227.2 |
| terrain/update/ms | 0.225 | 0.253 | 0.45 | 208.8 |
| controller/move/ms | 0.203 | 0.493 | 0.72 | 187.8 |
| terrain/select/ms | 0.197 | 0.224 | 0.42 | 182.4 |
| stream/drive/ms | 0.173 | 0.196 | 0.53 | 160.1 |
| far/select/ms | 0.149 | 0.168 | 0.45 | 138.0 |
| stream/tick/ms | 0.147 | 0.168 | 0.51 | 136.5 |
| controller/feed/ms | 0.138 | 0.172 | 0.50 | 128.2 |
| bigspace/high_precision/ms | 0.090 | 0.111 | 0.31 | 83.8 |
| anim/evaluate_cpu/ms | 0.065 | 0.078 | 0.26 | 60.6 |
| bigspace/low_precision/ms | 0.062 | 0.079 | 0.21 | 57.4 |
| anim/evaluate/ms | 0.060 | 0.074 | 0.26 | 55.3 |
| physics/step/ms | 0.043 | 0.055 | 0.22 | 39.4 |
| weather/map/ms | 0.023 | 0.033 | 0.05 | 21.3 |
| weather/eval/ms | 0.010 | 0.014 | 0.03 | 9.6 |
| pipelines/driver_ms | 0.008 | 0.000 | 3.84 | 7.1 |
| script/update/ms | 0.006 | 0.007 | 0.04 | 5.6 |
| controller/avoid/ms | 0.005 | 0.007 | 0.22 | 5.0 |
| script/input/ms | 0.004 | 0.005 | 0.02 | 4.1 |
| anim/apply/ms | 0.004 | 0.005 | 0.15 | 3.7 |
| script/ui/ms | 0.003 | 0.005 | 0.02 | 2.9 |
| controller/write/ms | 0.002 | 0.002 | 0.00 | 1.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 0.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.3 |
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
| frame/work/ms | ms | 14.91 | 20.08 | 14.32 |
| gpu/timed/ms | ms | 14.91 | 20.08 | 14.32 |
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
| phases/shadow/batch_sets | count | 31 | 36 | 31 |
| phases/shadow/bins | count | 2028 | 2336 | 2029 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5345 | 5672 | 5346 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 31 | 36 | 31 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 184 | 184 | 184 |
| pipelines/waiting | count | 0 | 3 | 0 |
| preprocess/bind_groups | count | 0 | 47 | 0 |
| preprocess/bind_groups_created | count | 0 | 47 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 201 | 201 | 201 |
| process/cpu_usage | % | 19.18 | 44.71 | 11.54 |
| process/mem_usage | GiB | 4.04 | 4.16 | 4.16 |
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
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
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
| system/cpu_usage | % | 24.51 | 35.02 | 19.44 |
| system/mem_usage | % | 72.48 | 73.00 | 72.96 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 89 | 94 | 89 |
| views/all/bins | count | 5198 | 5506 | 5199 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 15.0 k | 14.6 k |
| views/all/pipelines | count | 85 | 90 | 85 |
| views/all/pipelines_distinct | count | 75 | 75 | 75 |
| views/all/work_items | count | 14.8 k | 15.1 k | 14.8 k |
| views/cascade/batch_sets | count | 31 | 36 | 31 |
| views/cascade/bins | count | 2028 | 2336 | 2029 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5345 | 5672 | 5346 |
| views/cascade/pipelines | count | 31 | 36 | 31 |
| views/cascade/work_items | count | 5345 | 5672 | 5346 |
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
| anim/events/count | count | 0.15 | 2 | 143 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 5.34 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 1 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.97 | 2 | 901 |
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
| surface/shelter_redraws | count | 0.00 | 1 | 1 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 0 | 0 | 0 |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 40.3 ms; slowest 14, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2488.6 | main/busy/ms 2466, surface/advance_cpu_ms 66.85, surface/cell_upload_ms 0.76, stream/drive/ms 0.35, weather/map/ms 0.14, stream/tick/ms 0.09, bigspace/total/ms 0.07, bigspace/high_precision/ms 0.04, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 2.93 | 440.5 | main/busy/ms 2499, render/thread/ms 408, render/busy/ms 388, surface/advance_cpu_ms 66.85, render/acquire/ms 18.18, render/graph/ms 12.09, render/present/ms 1.88, stream/drive/ms 1.15, surface/cell_upload_ms 0.76, stream/tick/ms 0.61, terrain/update/ms 0.52, terrain/select/ms 0.32 |
| warm-up | 3.16 | 229.6 | pipelines/wait_ms 1550, render/thread/ms 442, render/busy/ms 421, pipelines/compile_ms 229, pipelines/front_ms 214, main/busy/ms 49.21, render/graph/ms 38.36, terrain/update/ms 19.15, render/acquire/ms 18.38, pipelines/driver_ms 15.01, render/present/ms 3.09, stream/drive/ms 1.47 |
| warm-up | 3.37 | 93.0 | pipelines/wait_ms 252, pipelines/compile_ms 199, pipelines/front_ms 182, main/busy/ms 61.11, render/thread/ms 52.37, render/busy/ms 50.16, pipelines/driver_ms 16.77, render/graph/ms 4.65, stream/drive/ms 2.01, render/present/ms 1.90, stream/tick/ms 1.70, terrain/update/ms 0.56 |
| warm-up | 3.53 | 66.8 | render/thread/ms 53.57, render/busy/ms 51.66, main/busy/ms 46.40, stream/drive/ms 34.72, render/graph/ms 1.80, render/present/ms 1.73, terrain/update/ms 0.75, stream/tick/ms 0.61, terrain/select/ms 0.58, stream/spawn/ms 0.44, bigspace/total/ms 0.32, bigspace/high_precision/ms 0.20 |
| warm-up | 3.59 | 48.7 | pipelines/wait_ms 4080, pipelines/compile_ms 242, pipelines/front_ms 229, main/busy/ms 39.79, render/thread/ms 31.00, render/busy/ms 28.86, stream/drive/ms 24.00, pipelines/driver_ms 13.32, terrain/update/ms 4.66, render/graph/ms 4.23, render/present/ms 2.00, stream/tick/ms 0.69 |
| warm-up | 3.68 | 81.0 | pipelines/wait_ms 2126, main/busy/ms 53.68, pipelines/compile_ms 47.95, pipelines/front_ms 45.56, render/thread/ms 44.65, render/busy/ms 43.37, stream/drive/ms 36.92, pipelines/driver_ms 2.39, render/graph/ms 2.12, stream/spawn/ms 1.09, render/present/ms 1.07, terrain/update/ms 0.60 |
| warm-up | 3.75 | 56.4 | pipelines/wait_ms 1981, pipelines/compile_ms 75.82, pipelines/front_ms 72.51, render/thread/ms 41.69, render/busy/ms 39.93, main/busy/ms 34.81, stream/drive/ms 18.85, stream/spawn/ms 3.66, pipelines/driver_ms 3.31, render/graph/ms 1.95, render/present/ms 1.56, terrain/update/ms 0.82 |
| warm-up | 3.99 | 139.9 | pipelines/wait_ms 10.1 k, pipelines/compile_ms 292, pipelines/front_ms 276, render/thread/ms 116, render/busy/ms 115, main/busy/ms 51.84, stream/drive/ms 33.50, pipelines/driver_ms 15.51, render/graph/ms 3.72, render/present/ms 1.07, stream/tick/ms 1.02, terrain/update/ms 0.77 |
| warm-up | 4.12 | 75.5 | pipelines/wait_ms 13.1 k, pipelines/compile_ms 84.16, main/busy/ms 75.92, pipelines/front_ms 74.52, stream/drive/ms 55.43, render/thread/ms 41.49, render/busy/ms 40.14, pipelines/driver_ms 9.65, render/graph/ms 3.64, stream/tick/ms 1.20, render/present/ms 1.18, stream/spawn/ms 1.01 |
| warm-up | 4.18 | 55.6 | main/busy/ms 52.63, stream/drive/ms 37.68, render/thread/ms 19.82, render/busy/ms 18.85, render/graph/ms 2.44, stream/spawn/ms 2.14, stream/activate/ms 1.08, stream/tick/ms 0.87, render/present/ms 0.81, terrain/update/ms 0.77, terrain/select/ms 0.61, physics/insert/ms 0.33 |
| warm-up | 4.44 | 47.7 | pipelines/wait_ms 2394, pipelines/compile_ms 109, pipelines/front_ms 99.47, render/thread/ms 48.64, render/busy/ms 47.06, main/busy/ms 43.58, pipelines/driver_ms 9.55, stream/drive/ms 6.32, render/graph/ms 4.88, stream/spawn/ms 3.75, render/present/ms 1.28, stream/tick/ms 1.24 |
| warm-up | 4.64 | 43.7 | pipelines/wait_ms 1387, render/thread/ms 52.90, render/busy/ms 51.08, main/busy/ms 29.89, render/graph/ms 9.57, stream/drive/ms 4.84, stream/spawn/ms 2.84, pipelines/compile_ms 2.72, pipelines/front_ms 2.42, render/present/ms 1.56, stream/tick/ms 1.14, physics/insert/ms 1.04 |
| warm-up | 5.25 | 41.1 | pipelines/wait_ms 1849, pipelines/compile_ms 205, pipelines/front_ms 193, render/thread/ms 34.82, main/busy/ms 34.63, render/busy/ms 33.25, pipelines/driver_ms 12.66, stream/drive/ms 11.26, render/graph/ms 4.49, stream/spawn/ms 1.58, render/present/ms 1.37, stream/tick/ms 1.29 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\claude\3866c27c11752a1d\4ff90442-bcd4-4820-b03c-4dd8a81992a4\scratchpad\bin\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\tc_pbr\balmora@2560x1440_off=normalmaps_orm_parallax-3 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 3%, CPU 11%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 98%) · 2578 MHz (min 2565) · memory 8001 MHz · 98.3 W · 80 °C (max 83) · VRAM 4854 MiB · throttled 100% of samples
