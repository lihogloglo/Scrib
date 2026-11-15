# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 479 frames over 15.0 s · **median 35.28 ms (28 fps)** · mean 31.38 · p95 46.05 · p99 49.30 (1% low 20 fps) · worst 95.60 ms · 4 spikes over 1.5× median · 115.8 stutters/min (work 91.8/min)
- warm-up: 689 frames over 15.8 s, worst 3468.9 ms
- hitches (> 88.2 ms): **3** measured, 8 in warm-up
- timed GPU passes: 22.66 ms of a 31.38 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 35.28 ms, limit 11 ms
- **FAIL** worst frame 95.60 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 5.688 | 5.748 | 7.939 | 0.99 | 0.261 |
| bin_unpacking | 4.233 | 4.278 | 6.655 | 0.99 | 0.035 |
| main_opaque_pass_3d | 3.798 | 3.838 | 8.830 | 0.99 | 0.052 |
| early prepass | 1.565 | 1.582 | 3.747 | 0.99 | 0.088 |
| early_mesh_preprocessing | 1.479 | 1.495 | 2.408 | 0.99 | 0.007 |
| atmosphere_luts | 0.797 | 0.977 | 1.664 | 0.82 | 0.005 |
| water_velocity | 0.762 | 0.906 | 1.633 | 0.84 | 0.002 |
| uniform_allocation | 0.681 | 1.279 | 2.204 | 0.53 | 0.041 |
| water_sspr | 0.466 | 0.471 | 1.132 | 0.99 | 0.003 |
| ssao | 0.438 | 0.545 | 1.227 | 0.80 | 0.010 |
| main_transmissive_pass_3d | 0.379 | 0.383 | 1.275 | 0.99 | 0.007 |
| upscaling | 0.357 | 0.361 | 1.100 | 0.99 | 0.005 |
| bloom | 0.330 | 0.837 | 1.464 | 0.39 | 0.083 |
| clustering | 0.302 | 0.552 | 1.205 | 0.55 | 0.031 |
| tonemapping | 0.276 | 0.279 | 1.017 | 0.99 | 0.005 |
| atmos_froxel_inject | 0.141 | 0.265 | 0.887 | 0.53 | 0.002 |
| clouds_composite | 0.141 | 0.142 | 0.118 | 0.99 | 0.003 |
| lightprobe_irradiance_map | 0.139 | 1.336 | 2.615 | 0.10 | 0.001 |
| render_sky | 0.127 | 0.129 | 0.670 | 0.99 | 0.002 |
| water_mips | 0.083 | 0.211 | 0.825 | 0.39 | 0.003 |
| lightprobe_radiance_map | 0.079 | 0.756 | 1.357 | 0.10 | 0.008 |
| water_fft | 0.072 | 0.182 | 0.713 | 0.39 | 0.003 |
| water_snapshot | 0.061 | 0.077 | 0.565 | 0.80 | 0.002 |
| atmos_froxel_integrate | 0.048 | 0.090 | 0.591 | 0.53 | 0.001 |
| clouds_trace | 0.038 | 0.071 | 0.077 | 0.53 | 0.001 |
| per_view_shadow_pass | 0.034 | 0.035 | 0.026 | 0.99 | 0.005 |
| clouds_resolve | 0.029 | 0.054 | 0.058 | 0.53 | 0.001 |
| atmosphere_environment | 0.027 | 0.027 | 0.014 | 0.99 | 0.005 |
| night_sky | 0.023 | 0.023 | 0.028 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.022 | 0.023 | 0.040 | 0.99 | 0.066 |
| sparse buffer updates | 0.009 | 0.011 | 0.007 | 0.83 | 0.004 |
| lightprobe_copy | 0.009 | 0.177 | 0.687 | 0.05 | 0.005 |
| auxiliary_views | 0.008 | 0.008 | 0.004 | 0.99 | 0.005 |
| clouds_fog_light | 0.006 | 0.011 | 0.011 | 0.53 | 0.004 |
| clouds_shadow | 0.005 | 0.005 | 0.006 | 0.99 | 0.004 |
| lightprobe_downsampling_first_pass | 0.003 | 0.068 | 0.035 | 0.05 | 0.001 |
| lightprobe_downsampling_second_pass | 0.002 | 0.041 | 0.012 | 0.05 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.01 | 0.007 |
| surface_trail | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 3299.036 | 23714.866 | 65545.25 | 1580238.3 |
| pipelines/compile_ms | 116.954 | 1132.285 | 4739.66 | 56020.8 |
| pipelines/driver_ms | 105.529 | 1020.327 | 4523.90 | 50548.2 |
| render/thread/ms | 30.041 | 44.842 | 94.51 | 14389.5 |
| render/acquire/ms | 19.000 | 31.976 | 83.66 | 9101.2 |
| pipelines/front_ms | 11.425 | 127.110 | 279.16 | 5472.6 |
| render/busy/ms | 10.188 | 12.878 | 24.14 | 4880.2 |
| main/busy/ms | 9.024 | 11.919 | 22.13 | 4322.5 |
| render/graph/ms | 5.370 | 7.533 | 14.31 | 2572.4 |
| render/present/ms | 0.852 | 1.077 | 22.28 | 408.2 |
| bigspace/total/ms | 0.368 | 0.539 | 3.01 | 176.2 |
| terrain/update/ms | 0.285 | 0.358 | 0.69 | 136.5 |
| stream/drive/ms | 0.259 | 0.401 | 0.86 | 123.9 |
| terrain/select/ms | 0.244 | 0.307 | 0.58 | 116.7 |
| far/select/ms | 0.232 | 0.358 | 0.81 | 110.9 |
| stream/tick/ms | 0.223 | 0.357 | 0.69 | 106.8 |
| controller/feed/ms | 0.183 | 0.252 | 0.41 | 87.8 |
| physics/insert/ms | 0.134 | 0.400 | 3.93 | 64.0 |
| bigspace/high_precision/ms | 0.130 | 0.185 | 2.18 | 62.3 |
| physics/step/ms | 0.075 | 0.265 | 4.54 | 36.0 |
| bigspace/low_precision/ms | 0.047 | 0.076 | 1.38 | 22.4 |
| anim/evaluate/ms | 0.028 | 0.072 | 0.38 | 13.5 |
| weather/map/ms | 0.021 | 0.027 | 0.06 | 10.0 |
| weather/eval/ms | 0.013 | 0.017 | 0.06 | 6.4 |
| script/update/ms | 0.011 | 0.016 | 0.17 | 5.4 |
| controller/move/ms | 0.010 | 0.008 | 1.03 | 4.9 |
| script/input/ms | 0.008 | 0.011 | 0.07 | 3.6 |
| script/ui/ms | 0.007 | 0.010 | 0.02 | 3.5 |
| anim/apply/ms | 0.005 | 0.007 | 0.04 | 2.4 |
| anim/evaluate_cpu/ms | 0.005 | 0.036 | 0.29 | 2.3 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 0.7 |
| controller/avoid/ms | 0.001 | 0.001 | 0.30 | 0.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.13 | 0.6 |
| weather/apply/ms | 0.001 | 0.001 | 0.06 | 0.3 |
| controller/write/ms | 0.000 | 0.000 | 0.00 | 0.1 |
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
| anim/a2/count | count | 0 | 2 | 0 |
| anim/a3/count | count | 382 | 414 | 414 |
| anim/characters/count | count | 382 | 414 | 414 |
| anim/evaluated/count | count | 0 | 17 | 7 |
| controller/movers/count | count | 0 | 7 | 7 |
| crowd/c0/count | count | 6 | 9 | 8 |
| crowd/c0/joints/count | count | 204 | 306 | 272 |
| crowd/c0/meshes/count | count | 36 | 54 | 48 |
| crowd/c1/count | count | 117 | 123 | 122 |
| crowd/c1/joints/count | count | 4618 | 4880 | 4848 |
| crowd/c1/meshes/count | count | 1026 | 1088 | 1080 |
| crowd/c2/count | count | 259 | 284 | 284 |
| crowd/c2/joints/count | count | 4549 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1267 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 105 | 118 | 118 |
| crowd/joints_extracted/count | count | 2676 | 12.1 k | 2958 |
| crowd/joints_uploaded/count | count | 2676 | 2958 | 2958 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 45.6 k | 47.1 k | 47.1 k |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.09 M | 189.09 M | 189.09 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 26.88 | 84.77 | 32.05 |
| gpu/timed/ms | ms | 26.88 | 84.77 | 32.05 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1838 | 1851 | 1851 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 193.5 k | 196.0 k | 196.0 k |
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
| phases/opaque3d/batch_sets | count | 28 | 30 | 28 |
| phases/opaque3d/bins | count | 1274 | 1303 | 1303 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3710 | 3776 | 3776 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1274 | 1303 | 1303 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3710 | 3776 | 3776 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 24 | 24 | 24 |
| phases/shadow/bins | count | 958 | 985 | 985 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 1951 | 2006 | 2006 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 24 | 24 | 24 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 142 | 2070 | 2070 |
| physics/items | count | 0 | 5 | 5 |
| physics/pending | count | 2207 | 2394 | 14 |
| physics/terrain/chunks | count | 0 | 52 | 52 |
| pipelines/total | count | 192 | 192 | 192 |
| pipelines/waiting | count | 66 | 96 | 3 |
| preprocess/bind_groups | count | 0 | 216 | 0 |
| preprocess/bind_groups_created | count | 0 | 216 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 210 | 210 | 210 |
| process/cpu_usage | % | 38.99 | 44.32 | 36.61 |
| process/mem_usage | GiB | 4.79 | 4.84 | 4.84 |
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
| started_load_count |  | 3534 | 3550 | 3550 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 62.50 | 86.06 | 56.72 |
| system/mem_usage | % | 81.06 | 81.25 | 81.25 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2472 | 2478 | 2478 |
| textures/images | count | 1904 | 1917 | 1917 |
| views/all/batch_sets | count | 94 | 98 | 94 |
| views/all/bins | count | 4066 | 4151 | 4151 |
| views/all/count | count | 3 | 3 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.1 k | 11.3 k | 11.3 k |
| views/all/pipelines | count | 90 | 94 | 90 |
| views/all/pipelines_distinct | count | 89 | 93 | 89 |
| views/all/work_items | count | 11.3 k | 11.5 k | 11.5 k |
| views/cascade/batch_sets | count | 24 | 24 | 24 |
| views/cascade/bins | count | 958 | 985 | 985 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 1951 | 2006 | 2006 |
| views/cascade/pipelines | count | 24 | 24 | 24 |
| views/cascade/work_items | count | 1951 | 2006 | 2006 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 3108 | 3166 | 3166 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9168 | 9300 | 9300 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 9348 | 9480 | 9480 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 24.5 k | 24.7 k | 24.7 k |
| world/point_lights | count | 222 | 222 | 222 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0.28 | 66 | 132 |
| anim/events/count | count | 0.01 | 2 | 7 |
| assets/Image/added | count | 0.13 | 25 | 62 |
| assets/Image/modified | count | 0.01 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.50 | 107 | 239 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.13 | 26 | 63 |
| assets/StandardMaterial/modified | count | 8.86 | 11 | 4246 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 2251 | 3918 | 1.08 M |
| ecs/entities_delta | count | 9.34 | 1149 | 4475 |
| physics/inserts | count | 4.28 | 148 | 2051 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 1.86 | 3 | 891 |
| physics/steps_dropped | count | 0.02 | 3 | 9 |
| pipelines/after_reveal | count | 0.05 | 15 | 23 |
| pipelines/created | count | 0.05 | 15 | 23 |
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

## Hitches (> 88.2 ms; slowest 11, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.47 | 3468.9 | main/busy/ms 3380, stream/drive/ms 2.37, stream/tick/ms 1.38, surface/advance_cpu_ms 0.77, surface/cell_upload_ms 0.73, bigspace/total/ms 0.63, bigspace/high_precision/ms 0.45, weather/map/ms 0.26, weather/eval/ms 0.04, script/update/ms 0.04, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 4.35 | 878.8 | main/busy/ms 3442, render/thread/ms 829, render/busy/ms 657, render/acquire/ms 169, render/graph/ms 18.97, stream/drive/ms 4.38, render/present/ms 4.04, stream/tick/ms 2.30, terrain/update/ms 1.56, surface/advance_cpu_ms 0.77, surface/cell_upload_ms 0.73, bigspace/total/ms 0.73 |
| warm-up | 4.71 | 146.3 | pipelines/wait_ms 1122, main/busy/ms 91.82, pipelines/compile_ms 78.51, render/thread/ms 77.80, render/busy/ms 74.74, pipelines/driver_ms 48.48, pipelines/front_ms 30.03, render/graph/ms 8.26, render/present/ms 2.42, stream/drive/ms 2.28, stream/tick/ms 1.90, terrain/update/ms 0.91 |
| warm-up | 5.06 | 88.4 | main/busy/ms 51.22, render/thread/ms 49.50, render/busy/ms 47.12, stream/drive/ms 32.40, render/graph/ms 6.22, render/present/ms 1.89, stream/spawn/ms 1.55, stream/tick/ms 0.81, terrain/update/ms 0.77, terrain/select/ms 0.60, bigspace/total/ms 0.50, render/acquire/ms 0.49 |
| warm-up | 5.25 | 154.9 | render/thread/ms 95.84, render/busy/ms 91.94, main/busy/ms 73.10, stream/drive/ms 36.91, render/graph/ms 13.35, render/present/ms 3.56, stream/spawn/ms 2.75, stream/tick/ms 1.89, terrain/update/ms 0.87, terrain/select/ms 0.67, bigspace/total/ms 0.35, render/acquire/ms 0.34 |
| warm-up | 5.61 | 89.6 | main/busy/ms 72.65, render/thread/ms 68.96, render/busy/ms 66.58, stream/drive/ms 7.89, render/graph/ms 7.30, stream/spawn/ms 3.28, bigspace/total/ms 3.15, render/present/ms 2.07, stream/tick/ms 1.54, stream/activate/ms 1.05, terrain/update/ms 0.89, terrain/select/ms 0.67 |
| warm-up | 5.81 | 144.0 | pipelines/compile_ms 1023, pipelines/driver_ms 919, pipelines/wait_ms 676, main/busy/ms 141, stream/drive/ms 122, pipelines/front_ms 104, render/thread/ms 38.91, render/busy/ms 36.93, render/graph/ms 4.25, render/present/ms 1.58, stream/tick/ms 1.25, stream/activate/ms 1.07 |
| warm-up | 8.59 | 106.3 | pipelines/wait_ms 4053, render/thread/ms 113, render/acquire/ms 94.33, pipelines/compile_ms 65.44, pipelines/driver_ms 48.08, main/busy/ms 18.16, pipelines/front_ms 17.36, render/busy/ms 17.00, render/graph/ms 5.31, render/present/ms 2.15, stream/drive/ms 1.41, terrain/update/ms 0.67 |
| measure | 4.45 | 95.3 | render/thread/ms 118, render/acquire/ms 95.11, render/busy/ms 21.63, main/busy/ms 19.59, render/graph/ms 11.89, render/present/ms 1.29, stream/drive/ms 0.83, bigspace/total/ms 0.80, stream/tick/ms 0.76, terrain/update/ms 0.54, controller/feed/ms 0.49, terrain/select/ms 0.47 |
| measure | 4.55 | 95.6 | pipelines/wait_ms 31.5 k, render/thread/ms 189, pipelines/compile_ms 168, render/acquire/ms 167, pipelines/driver_ms 150, render/busy/ms 20.27, main/busy/ms 20.22, pipelines/front_ms 17.90, render/graph/ms 11.11, render/present/ms 1.28, stream/drive/ms 1.04, stream/tick/ms 0.98 |
| measure | 4.64 | 89.1 | pipelines/wait_ms 31.5 k, render/thread/ms 182, pipelines/compile_ms 168, render/acquire/ms 160, pipelines/driver_ms 150, main/busy/ms 21.81, render/busy/ms 20.80, pipelines/front_ms 17.90, render/graph/ms 11.55, render/present/ms 1.42, stream/drive/ms 0.83, terrain/update/ms 0.77 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_old.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\lampsel_old\balmora_night@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 24% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 88%) · 2580 MHz (min 2580) · memory 8001 MHz · 55.6 W · 70 °C (max 71) · VRAM 7840 MiB · throttled 100% of samples
