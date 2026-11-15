# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5192 frames over 65.7 s · **median 12.08 ms (83 fps)** · mean 12.64 · p95 16.46 · p99 18.96 (1% low 53 fps) · worst 36.98 ms · 85 spikes over 1.5× median · 26.5 stutters/min (work 31.1/min)
- warm-up: 338 frames over 8.3 s, worst 2638.4 ms
- hitches (> 30.2 ms): **3** measured, 19 in warm-up
- timed GPU passes: 7.06 ms of a 12.64 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.08 ms, limit 11 ms
- **FAIL** frame_ms: 29 stutters (> 1.5x moving median of 60) in 1.09 min = 26.50/min, limit 1/min
- **FAIL** worst frame 36.98 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.080 | 2.082 | 2.238 | 1.00 | 0.181 |
| main_opaque_pass_3d | 1.331 | 1.332 | 1.694 | 1.00 | 0.071 |
| early prepass | 0.528 | 0.528 | 0.754 | 1.00 | 0.107 |
| bloom | 0.324 | 0.324 | 0.329 | 1.00 | 0.067 |
| atmosphere_luts | 0.319 | 0.319 | 0.382 | 1.00 | 0.004 |
| water_velocity | 0.274 | 0.274 | 0.358 | 1.00 | 0.002 |
| auxiliary_views | 0.213 | 0.214 | 0.897 | 1.00 | 0.194 |
| water_sspr | 0.211 | 0.211 | 0.211 | 1.00 | 0.002 |
| uniform_allocation | 0.209 | 0.209 | 0.222 | 1.00 | 0.037 |
| ssao | 0.193 | 0.193 | 0.196 | 1.00 | 0.008 |
| bin_unpacking | 0.166 | 0.166 | 0.179 | 1.00 | 0.026 |
| water_fft | 0.133 | 0.133 | 0.138 | 1.00 | 0.003 |
| atmos_froxel_inject | 0.130 | 0.130 | 0.131 | 1.00 | 0.001 |
| upscaling | 0.125 | 0.125 | 0.132 | 1.00 | 0.004 |
| clouds_composite | 0.107 | 0.107 | 0.110 | 1.00 | 0.002 |
| tonemapping | 0.088 | 0.088 | 0.091 | 1.00 | 0.004 |
| main_transmissive_pass_3d | 0.081 | 0.081 | 0.142 | 1.00 | 0.009 |
| clustering | 0.065 | 0.065 | 0.069 | 1.00 | 0.020 |
| clouds_trace | 0.055 | 0.055 | 0.075 | 1.00 | 0.001 |
| render_sky | 0.053 | 0.053 | 0.054 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.421 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.051 | 1.00 | 0.001 |
| surface_trail | 0.045 | 0.057 | 0.057 | 0.79 | 0.002 |
| early_mesh_preprocessing | 0.034 | 0.034 | 0.038 | 1.00 | 0.005 |
| water_snapshot | 0.033 | 0.033 | 0.036 | 1.00 | 0.002 |
| lightprobe_radiance_map | 0.028 | 0.225 | 0.234 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.026 | 0.026 | 0.029 | 1.00 | 0.001 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.033 | 1.00 | 0.114 |
| night_sky | 0.016 | 0.016 | 0.037 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.014 | 0.014 | 0.016 | 1.00 | 0.005 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| clouds_shadow | 0.003 | 0.003 | 0.004 | 1.00 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 57.007 | 0.000 | 27370.64 | 295978.6 |
| render/thread/ms | 11.356 | 15.015 | 34.58 | 58958.1 |
| render/busy/ms | 10.779 | 14.369 | 33.77 | 55965.9 |
| main/busy/ms | 9.479 | 12.146 | 29.26 | 49213.2 |
| render/graph/ms | 6.450 | 8.986 | 16.64 | 33487.0 |
| pipelines/compile_ms | 0.850 | 0.000 | 185.15 | 4414.1 |
| pipelines/front_ms | 0.820 | 0.000 | 177.83 | 4257.6 |
| bigspace/total/ms | 0.508 | 0.955 | 3.80 | 2639.1 |
| render/present/ms | 0.484 | 0.578 | 5.44 | 2512.8 |
| terrain/update/ms | 0.303 | 0.391 | 1.81 | 1572.8 |
| terrain/select/ms | 0.269 | 0.346 | 0.92 | 1398.7 |
| controller/feed/ms | 0.268 | 0.366 | 0.83 | 1392.3 |
| stream/drive/ms | 0.250 | 0.341 | 20.78 | 1297.2 |
| bigspace/low_precision/ms | 0.200 | 0.362 | 3.48 | 1036.1 |
| far/select/ms | 0.196 | 0.274 | 0.64 | 1018.6 |
| stream/tick/ms | 0.191 | 0.261 | 0.63 | 993.4 |
| anim/evaluate/ms | 0.163 | 0.301 | 2.28 | 847.9 |
| anim/evaluate_cpu/ms | 0.148 | 0.197 | 0.66 | 769.6 |
| bigspace/high_precision/ms | 0.133 | 0.257 | 3.17 | 693.1 |
| controller/move/ms | 0.121 | 0.253 | 0.45 | 627.9 |
| render/acquire/ms | 0.092 | 0.122 | 0.35 | 479.4 |
| physics/insert/ms | 0.062 | 0.459 | 0.88 | 322.9 |
| physics/step/ms | 0.054 | 0.089 | 0.73 | 278.0 |
| pipelines/driver_ms | 0.030 | 0.000 | 7.32 | 156.5 |
| weather/map/ms | 0.015 | 0.020 | 0.09 | 79.3 |
| anim/apply/ms | 0.011 | 0.021 | 0.15 | 56.5 |
| controller/avoid/ms | 0.010 | 0.014 | 0.05 | 53.2 |
| weather/eval/ms | 0.010 | 0.013 | 0.13 | 49.9 |
| script/update/ms | 0.007 | 0.010 | 0.24 | 37.9 |
| stream/despawn/ms | 0.006 | 0.000 | 1.06 | 29.1 |
| script/input/ms | 0.005 | 0.007 | 0.15 | 27.9 |
| script/ui/ms | 0.005 | 0.007 | 0.29 | 26.5 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 25.9 |
| controller/write/ms | 0.003 | 0.004 | 0.04 | 17.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.64 | 13.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 3.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 33 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 25 | 43 | 12 |
| crowd/c0/joints/count | count | 1247 | 2262 | 528 |
| crowd/c0/meshes/count | count | 349 | 646 | 135 |
| crowd/c1/count | count | 145 | 155 | 149 |
| crowd/c1/joints/count | count | 5475 | 5936 | 5668 |
| crowd/c1/meshes/count | count | 1166 | 1268 | 1206 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 157 | 215 | 202 |
| crowd/joints_extracted/count | count | 3597 | 8765 | 5619 |
| crowd/joints_uploaded/count | count | 3532 | 6129 | 5619 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.38 | 33.77 | 9.88 |
| gpu/timed/ms | ms | 7.75 | 10.52 | 8.02 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2523 | 2580 | 2580 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 278.4 k | 278.4 k | 278.4 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 806 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 806 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d/bins | count | 1292 | 1399 | 1349 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3399 | 5135 | 3184 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d_prepass/bins | count | 1292 | 1399 | 1349 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3399 | 5135 | 3184 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 35 | 259 | 29 |
| phases/shadow/bins | count | 3636 | 4316 | 1136 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 4990 | 7352 | 2441 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 248 | 310 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2641 | 3268 | 2347 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 1883 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 223 | 228 | 228 |
| pipelines/waiting | count | 0 | 68 | 0 |
| preprocess/bind_groups | count | 10 | 714 | 8 |
| preprocess/bind_groups_created | count | 10 | 714 | 8 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 219 | 374 | 216 |
| process/cpu_usage | % | 25.48 | 45.28 | 20.19 |
| process/mem_usage | GiB | 5.04 | 5.20 | 5.20 |
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
| started_load_count |  | 4765 | 4860 | 4860 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 331 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.81 | 67.29 | 29.59 |
| system/mem_usage | % | 63.46 | 64.40 | 64.06 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2821 | 2936 | 2936 |
| textures/images | count | 2589 | 2647 | 2647 |
| views/all/batch_sets | count | 104 | 325 | 101 |
| views/all/bins | count | 6139 | 7222 | 4628 |
| views/all/count | count | 51 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 13.7 k | 19.4 k | 10.0 k |
| views/all/pipelines | count | 314 | 378 | 97 |
| views/all/pipelines_distinct | count | 101 | 111 | 92 |
| views/all/work_items | count | 11.3 k | 17.0 k | 10.4 k |
| views/cascade/batch_sets | count | 33 | 41 | 29 |
| views/cascade/bins | count | 1180 | 1540 | 1136 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2734 | 4298 | 2441 |
| views/cascade/pipelines | count | 33 | 41 | 29 |
| views/cascade/work_items | count | 2734 | 4298 | 2441 |
| views/lamp_face/batch_sets | count | 0 | 224 | 0 |
| views/lamp_face/bins | count | 2540 | 2909 | 0 |
| views/lamp_face/count | count | 48 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2842 | 3221 | 0 |
| views/lamp_face/pipelines | count | 214 | 270 | 0 |
| views/lamp_face/work_items | count | 0 | 2865 | 0 |
| views/main/batch_sets | count | 70 | 76 | 72 |
| views/main/bins | count | 2878 | 3574 | 3492 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8398 | 13.0 k | 7604 |
| views/main/pipelines | count | 66 | 72 | 68 |
| views/main/work_items | count | 8568 | 13.3 k | 7912 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.5 k | 37.8 k | 30.7 k |
| world/point_lights | count | 299 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 44.31 | 267 | 230.0 k |
| anim/events/count | count | 0.11 | 3 | 552 |
| assets/Image/added | count | 0.12 | 43 | 604 |
| assets/Image/modified | count | 0.00 | 9 | 23 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.80 | 152 | 4139 |
| assets/Mesh/removed | count | 0.94 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 469 |
| assets/StandardMaterial/modified | count | 4.17 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 394 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.26 | 1386 | 6525 |
| physics/inserts | count | 0.72 | 140 | 3756 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.45 | 161 | 2335 |
| physics/steps | count | 0.76 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 15 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6520 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1429 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.2 ms; slowest 22, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.64 | 2638.4 | main/busy/ms 2614, surface/advance_cpu_ms 0.73, surface/cell_upload_ms 0.31, stream/drive/ms 0.31, bigspace/total/ms 0.10, stream/tick/ms 0.10, bigspace/high_precision/ms 0.07, weather/map/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, script/input/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.64 | 999.2 | main/busy/ms 2658, render/thread/ms 965, render/busy/ms 799, render/acquire/ms 164, render/graph/ms 5.29, render/present/ms 1.81, stream/drive/ms 0.91, surface/advance_cpu_ms 0.73, stream/tick/ms 0.46, terrain/update/ms 0.40, surface/cell_upload_ms 0.31, bigspace/total/ms 0.29 |
| warm-up | 3.89 | 90.7 | pipelines/compile_ms 194, pipelines/front_ms 179, main/busy/ms 62.58, render/thread/ms 53.10, render/busy/ms 51.03, pipelines/wait_ms 39.46, pipelines/driver_ms 14.57, render/graph/ms 3.99, render/present/ms 1.79, stream/drive/ms 1.54, stream/tick/ms 1.07, terrain/update/ms 0.81 |
| warm-up | 4.05 | 78.6 | render/thread/ms 52.87, render/busy/ms 51.48, main/busy/ms 51.44, stream/drive/ms 35.88, render/graph/ms 3.29, stream/tick/ms 1.39, render/present/ms 1.23, terrain/update/ms 0.86, terrain/select/ms 0.69, stream/spawn/ms 0.44, bigspace/total/ms 0.27, render/acquire/ms 0.16 |
| warm-up | 4.12 | 52.8 | pipelines/wait_ms 650, pipelines/compile_ms 222, pipelines/front_ms 214, render/thread/ms 42.00, render/busy/ms 40.56, main/busy/ms 29.87, stream/drive/ms 11.38, pipelines/driver_ms 8.48, render/graph/ms 3.57, render/present/ms 1.19, terrain/update/ms 1.18, stream/tick/ms 1.00 |
| warm-up | 4.18 | 40.7 | render/thread/ms 44.34, render/busy/ms 42.76, main/busy/ms 36.03, stream/drive/ms 18.19, render/graph/ms 4.08, stream/spawn/ms 2.55, render/present/ms 1.27, stream/tick/ms 0.94, terrain/update/ms 0.93, terrain/select/ms 0.73, stream/activate/ms 0.45, bigspace/total/ms 0.36 |
| warm-up | 4.22 | 33.2 | render/thread/ms 53.18, render/busy/ms 51.98, main/busy/ms 24.17, stream/drive/ms 5.12, render/graph/ms 2.98, stream/spawn/ms 2.26, render/present/ms 0.94, stream/tick/ms 0.88, terrain/update/ms 0.79, terrain/select/ms 0.63, stream/activate/ms 0.45, bigspace/total/ms 0.40 |
| warm-up | 4.28 | 45.5 | pipelines/wait_ms 1573, pipelines/compile_ms 186, pipelines/front_ms 176, main/busy/ms 42.25, render/thread/ms 39.12, render/busy/ms 37.34, stream/drive/ms 27.58, pipelines/driver_ms 9.61, render/graph/ms 5.94, render/present/ms 1.51, stream/activate/ms 1.13, stream/tick/ms 0.93 |
| warm-up | 4.32 | 33.5 | pipelines/wait_ms 1573, pipelines/compile_ms 186, pipelines/front_ms 176, render/thread/ms 53.86, render/busy/ms 52.26, main/busy/ms 17.00, render/graph/ms 11.00, pipelines/driver_ms 9.61, stream/drive/ms 3.47, render/present/ms 1.32, stream/tick/ms 1.12, terrain/update/ms 1.05 |
| warm-up | 4.41 | 33.5 | pipelines/wait_ms 1814, pipelines/compile_ms 160, pipelines/front_ms 151, main/busy/ms 77.10, stream/drive/ms 56.64, render/thread/ms 37.81, render/busy/ms 35.73, pipelines/driver_ms 8.46, render/graph/ms 4.71, stream/spawn/ms 2.96, stream/tick/ms 2.38, render/present/ms 1.52 |
| warm-up | 4.53 | 120.1 | pipelines/wait_ms 3359, pipelines/compile_ms 228, pipelines/front_ms 216, render/thread/ms 80.27, main/busy/ms 75.84, stream/drive/ms 53.71, render/busy/ms 50.83, render/present/ms 29.09, pipelines/driver_ms 12.11, render/graph/ms 3.41, stream/tick/ms 1.53, terrain/update/ms 1.15 |
| warm-up | 4.63 | 86.6 | pipelines/wait_ms 5902, pipelines/compile_ms 161, pipelines/front_ms 144, main/busy/ms 65.94, stream/drive/ms 47.24, render/thread/ms 40.65, render/busy/ms 39.39, pipelines/driver_ms 16.98, render/graph/ms 12.61, stream/spawn/ms 2.16, render/present/ms 1.09, stream/activate/ms 0.77 |
| warm-up | 4.69 | 41.4 | pipelines/wait_ms 4756, pipelines/compile_ms 75.27, pipelines/front_ms 72.02, main/busy/ms 46.59, render/thread/ms 36.01, render/busy/ms 34.24, stream/drive/ms 32.72, render/graph/ms 5.00, pipelines/driver_ms 3.25, render/present/ms 1.55, stream/spawn/ms 1.53, stream/tick/ms 0.83 |
| warm-up | 4.73 | 36.3 | pipelines/wait_ms 9747, pipelines/compile_ms 50.57, pipelines/front_ms 46.99, main/busy/ms 36.51, stream/drive/ms 25.98, render/thread/ms 16.88, render/busy/ms 15.67, pipelines/driver_ms 3.59, render/graph/ms 3.12, stream/activate/ms 1.40, render/present/ms 1.04, stream/spawn/ms 0.98 |
| warm-up | 4.87 | 31.5 | render/thread/ms 39.58, render/busy/ms 38.54, main/busy/ms 23.18, stream/drive/ms 8.23, render/graph/ms 4.00, render/present/ms 0.88, terrain/update/ms 0.83, stream/tick/ms 0.73, terrain/select/ms 0.69, stream/spawn/ms 0.57, bigspace/total/ms 0.53, bigspace/high_precision/ms 0.33 |
| warm-up | 5.87 | 30.4 | pipelines/wait_ms 2133, render/thread/ms 37.86, main/busy/ms 36.62, render/busy/ms 36.32, pipelines/compile_ms 33.61, pipelines/front_ms 31.56, stream/drive/ms 19.61, render/graph/ms 4.82, stream/spawn/ms 2.07, pipelines/driver_ms 2.06, render/present/ms 1.33, stream/tick/ms 0.92 |
| warm-up | 6.97 | 31.2 | main/busy/ms 43.72, render/thread/ms 39.41, render/busy/ms 37.70, render/graph/ms 7.07, render/present/ms 1.39, stream/drive/ms 1.25, bigspace/total/ms 1.11, anim/evaluate_cpu/ms 0.71, physics/insert/ms 0.69, terrain/update/ms 0.67, terrain/select/ms 0.58, stream/tick/ms 0.56 |
| warm-up | 7.01 | 34.8 | main/busy/ms 58.59, render/thread/ms 45.46, render/busy/ms 44.05, render/graph/ms 5.76, stream/drive/ms 1.44, anim/evaluate_cpu/ms 1.35, bigspace/total/ms 1.35, render/present/ms 1.05, bigspace/low_precision/ms 0.75, physics/insert/ms 0.70, terrain/update/ms 0.68, stream/tick/ms 0.62 |
| warm-up | 7.04 | 35.0 | main/busy/ms 43.74, render/thread/ms 40.67, render/busy/ms 39.50, render/graph/ms 5.04, stream/drive/ms 1.32, bigspace/total/ms 1.28, physics/insert/ms 0.99, render/present/ms 0.91, anim/evaluate_cpu/ms 0.76, terrain/update/ms 0.69, bigspace/low_precision/ms 0.65, terrain/select/ms 0.63 |
| measure | 2.74 | 35.9 | render/thread/ms 46.03, render/busy/ms 44.70, render/graph/ms 22.97, main/busy/ms 19.12, render/present/ms 1.14, bigspace/total/ms 0.72, physics/insert/ms 0.72, far/select/ms 0.64, terrain/update/ms 0.55, stream/drive/ms 0.52, terrain/select/ms 0.49, stream/tick/ms 0.45 |
| measure | 59.07 | 37.0 | render/thread/ms 42.00, render/busy/ms 40.59, main/busy/ms 25.55, render/graph/ms 16.48, bigspace/total/ms 1.60, render/present/ms 1.18, controller/feed/ms 1.13, terrain/update/ms 0.85, stream/drive/ms 0.82, anim/evaluate/ms 0.79, stream/tick/ms 0.74, terrain/select/ms 0.74 |
| measure | 59.10 | 34.7 | render/thread/ms 40.73, render/busy/ms 39.42, main/busy/ms 17.58, render/graph/ms 11.08, render/present/ms 1.15, bigspace/total/ms 1.03, controller/feed/ms 0.88, terrain/update/ms 0.81, terrain/select/ms 0.71, far/select/ms 0.45, stream/drive/ms 0.40, anim/evaluate/ms 0.35 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_new.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_new\balmora_walk_night@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **70%** (min 39%) · 2580 MHz (min 2580) · memory 8001 MHz · 71.6 W · 74 °C (max 81) · VRAM 5519 MiB · throttled 100% of samples
