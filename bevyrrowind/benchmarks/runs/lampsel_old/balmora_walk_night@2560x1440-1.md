# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5612 frames over 65.7 s · **median 11.26 ms (89 fps)** · mean 11.70 · p95 14.46 · p99 16.61 (1% low 60 fps) · worst 47.49 ms · 53 spikes over 1.5× median · 29.2 stutters/min (work 25.6/min)
- warm-up: 371 frames over 8.4 s, worst 2460.3 ms
- hitches (> 28.1 ms): **6** measured, 26 in warm-up
- timed GPU passes: 7.36 ms of a 11.70 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.26 ms, limit 11 ms
- **FAIL** frame_ms: 32 stutters (> 1.5x moving median of 60) in 1.09 min = 29.24/min, limit 1/min
- **FAIL** worst frame 47.49 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.204 | 2.232 | 2.794 | 0.99 | 0.177 |
| main_opaque_pass_3d | 1.401 | 1.418 | 2.043 | 0.99 | 0.059 |
| early prepass | 0.541 | 0.548 | 0.987 | 0.99 | 0.105 |
| bloom | 0.337 | 0.342 | 0.488 | 0.99 | 0.064 |
| atmosphere_luts | 0.330 | 0.334 | 0.516 | 0.99 | 0.003 |
| water_velocity | 0.285 | 0.289 | 0.368 | 0.99 | 0.001 |
| water_sspr | 0.228 | 0.231 | 0.230 | 0.99 | 0.002 |
| uniform_allocation | 0.212 | 0.215 | 0.223 | 0.99 | 0.035 |
| ssao | 0.202 | 0.205 | 0.209 | 0.99 | 0.008 |
| auxiliary_views | 0.196 | 0.198 | 0.865 | 0.99 | 0.165 |
| bin_unpacking | 0.175 | 0.177 | 0.183 | 0.99 | 0.025 |
| water_fft | 0.140 | 0.142 | 0.138 | 0.99 | 0.003 |
| atmos_froxel_inject | 0.137 | 0.138 | 0.132 | 0.99 | 0.002 |
| upscaling | 0.132 | 0.133 | 0.144 | 0.99 | 0.004 |
| clouds_composite | 0.109 | 0.110 | 0.110 | 0.99 | 0.002 |
| tonemapping | 0.092 | 0.094 | 0.093 | 0.99 | 0.003 |
| main_transmissive_pass_3d | 0.083 | 0.084 | 0.147 | 0.99 | 0.008 |
| clustering | 0.067 | 0.068 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.99 | 0.002 |
| clouds_trace | 0.054 | 0.055 | 0.074 | 0.99 | 0.001 |
| lightprobe_irradiance_map | 0.050 | 0.442 | 0.755 | 0.11 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| surface_trail | 0.044 | 0.059 | 0.057 | 0.74 | 0.002 |
| water_snapshot | 0.039 | 0.039 | 0.046 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.035 | 0.035 | 0.038 | 0.99 | 0.004 |
| lightprobe_radiance_map | 0.027 | 0.242 | 0.237 | 0.11 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.028 | 0.030 | 0.99 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.017 | 0.017 | 0.033 | 0.99 | 0.112 |
| night_sky | 0.016 | 0.016 | 0.035 | 0.99 | 0.001 |
| per_view_shadow_pass | 0.015 | 0.015 | 0.016 | 0.99 | 0.005 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.99 | 0.003 |
| clouds_shadow | 0.004 | 0.004 | 0.004 | 0.99 | 0.003 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 30.073 | 0.000 | 17132.96 | 168770.8 |
| render/thread/ms | 10.516 | 13.169 | 46.22 | 59015.9 |
| render/busy/ms | 9.926 | 12.553 | 33.89 | 55702.1 |
| main/busy/ms | 8.769 | 11.031 | 29.33 | 49214.1 |
| render/graph/ms | 5.914 | 7.745 | 16.72 | 33191.9 |
| pipelines/compile_ms | 0.569 | 0.000 | 164.17 | 3194.7 |
| bigspace/total/ms | 0.563 | 1.546 | 3.70 | 3162.0 |
| pipelines/front_ms | 0.549 | 0.000 | 158.04 | 3083.5 |
| render/present/ms | 0.498 | 0.586 | 34.17 | 2793.6 |
| terrain/update/ms | 0.305 | 0.387 | 1.75 | 1710.9 |
| terrain/select/ms | 0.272 | 0.346 | 0.69 | 1527.1 |
| controller/feed/ms | 0.272 | 0.365 | 0.67 | 1527.1 |
| stream/drive/ms | 0.244 | 0.346 | 21.35 | 1369.6 |
| bigspace/low_precision/ms | 0.235 | 0.625 | 2.93 | 1321.0 |
| stream/tick/ms | 0.188 | 0.259 | 0.60 | 1055.0 |
| far/select/ms | 0.188 | 0.253 | 0.58 | 1054.4 |
| bigspace/high_precision/ms | 0.164 | 0.554 | 3.31 | 920.4 |
| anim/evaluate/ms | 0.160 | 0.275 | 2.81 | 900.5 |
| anim/evaluate_cpu/ms | 0.159 | 0.198 | 0.68 | 889.7 |
| controller/move/ms | 0.135 | 0.233 | 0.54 | 759.2 |
| render/acquire/ms | 0.093 | 0.124 | 0.43 | 520.2 |
| physics/insert/ms | 0.053 | 0.449 | 0.92 | 297.1 |
| physics/step/ms | 0.049 | 0.084 | 1.75 | 274.7 |
| pipelines/driver_ms | 0.020 | 0.000 | 6.51 | 111.2 |
| weather/map/ms | 0.015 | 0.020 | 0.18 | 86.3 |
| anim/apply/ms | 0.011 | 0.020 | 0.07 | 62.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.20 | 60.6 |
| weather/eval/ms | 0.010 | 0.013 | 0.27 | 54.3 |
| script/update/ms | 0.007 | 0.010 | 0.08 | 39.8 |
| script/ui/ms | 0.005 | 0.007 | 0.12 | 30.1 |
| stream/despawn/ms | 0.005 | 0.000 | 1.05 | 29.6 |
| script/input/ms | 0.005 | 0.007 | 0.27 | 29.0 |
| stream/spawn/ms | 0.004 | 0.000 | 1.10 | 25.0 |
| controller/write/ms | 0.003 | 0.004 | 0.14 | 19.4 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 14.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.50 | 5.7 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 1.5 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.01 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 23 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 89 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 51 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 535 |
| crowd/c0/meshes/count | count | 353 | 646 | 144 |
| crowd/c1/count | count | 146 | 157 | 149 |
| crowd/c1/joints/count | count | 5515 | 5992 | 5661 |
| crowd/c1/meshes/count | count | 1167 | 1308 | 1197 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 157 | 215 | 201 |
| crowd/joints_extracted/count | count | 3574 | 8676 | 5564 |
| crowd/joints_uploaded/count | count | 3526 | 6129 | 5564 |
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
| frame/work/ms | ms | 9.63 | 33.89 | 9.42 |
| gpu/timed/ms | ms | 8.14 | 11.50 | 8.79 |
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
| phases/alpha_mask3d/items | count | 804 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 804 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d/bins | count | 1278 | 1400 | 1349 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3392 | 5116 | 3184 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 28 | 30 | 29 |
| phases/opaque3d_prepass/bins | count | 1278 | 1400 | 1349 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3392 | 5116 | 3184 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 28 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 34 | 256 | 29 |
| phases/shadow/bins | count | 2262 | 4028 | 1136 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 3890 | 6943 | 2414 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 129 | 264 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2348 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 810 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 218 | 223 | 223 |
| pipelines/waiting | count | 0 | 40 | 0 |
| preprocess/bind_groups | count | 10 | 708 | 8 |
| preprocess/bind_groups_created | count | 10 | 708 | 8 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 219 | 329 | 216 |
| process/cpu_usage | % | 25.65 | 39.42 | 28.29 |
| process/mem_usage | GiB | 5.08 | 5.24 | 5.24 |
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
| started_load_count |  | 4762 | 4856 | 4856 |
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
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.12 | 42.71 | 30.21 |
| system/mem_usage | % | 63.66 | 64.30 | 64.08 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2820 | 2936 | 2936 |
| textures/images | count | 2587 | 2647 | 2647 |
| views/all/batch_sets | count | 104 | 322 | 101 |
| views/all/bins | count | 5008 | 6865 | 4628 |
| views/all/count | count | 21 | 51 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 11.8 k | 18.8 k | 10.0 k |
| views/all/pipelines | count | 196 | 331 | 97 |
| views/all/pipelines_distinct | count | 96 | 109 | 92 |
| views/all/work_items | count | 11.1 k | 16.8 k | 10.3 k |
| views/cascade/batch_sets | count | 33 | 40 | 29 |
| views/cascade/bins | count | 1176 | 1507 | 1136 |
| views/cascade/count | count | 2 | 2 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 2739 | 4291 | 2414 |
| views/cascade/pipelines | count | 33 | 40 | 29 |
| views/cascade/work_items | count | 2739 | 4291 | 2414 |
| views/lamp_face/batch_sets | count | 0 | 221 | 0 |
| views/lamp_face/bins | count | 1121 | 2865 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1295 | 3156 | 0 |
| views/lamp_face/pipelines | count | 94 | 227 | 0 |
| views/lamp_face/work_items | count | 0 | 2802 | 0 |
| views/main/batch_sets | count | 70 | 76 | 72 |
| views/main/bins | count | 2856 | 3574 | 3492 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8388 | 13.0 k | 7604 |
| views/main/pipelines | count | 66 | 72 | 68 |
| views/main/work_items | count | 8556 | 13.2 k | 7912 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.7 k | 37.8 k | 30.7 k |
| world/point_lights | count | 300 | 367 | 302 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 50.19 | 829 | 281.7 k |
| anim/events/count | count | 0.10 | 3 | 543 |
| assets/Image/added | count | 0.11 | 43 | 629 |
| assets/Image/modified | count | 0.00 | 3 | 17 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.75 | 152 | 4212 |
| assets/Mesh/removed | count | 0.87 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 498 |
| assets/StandardMaterial/modified | count | 3.86 | 18 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 394 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.79 | 3007 | 10.1 k |
| physics/inserts | count | 0.54 | 132 | 3057 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 161 | 2335 |
| physics/steps | count | 0.70 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 19 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 22 | 378 |
| stream/upload/bytes | bytes | 6032 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1323 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.1 ms; slowest 32, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.46 | 2460.3 | main/busy/ms 2434, surface/advance_cpu_ms 0.72, stream/drive/ms 0.38, surface/cell_upload_ms 0.32, stream/tick/ms 0.10, bigspace/total/ms 0.07, weather/map/ms 0.04, bigspace/high_precision/ms 0.04, script/update/ms 0.03, weather/eval/ms 0.02, script/input/ms 0.01, anim/evaluate/ms 0.01 |
| warm-up | 3.24 | 776.0 | main/busy/ms 2467, render/thread/ms 741, render/busy/ms 628, render/acquire/ms 110, render/graph/ms 5.82, render/present/ms 2.80, stream/drive/ms 1.37, surface/advance_cpu_ms 0.72, stream/tick/ms 0.70, terrain/update/ms 0.65, terrain/select/ms 0.44, surface/cell_upload_ms 0.32 |
| warm-up | 3.37 | 138.4 | pipelines/wait_ms 4805, render/thread/ms 752, render/busy/ms 638, pipelines/compile_ms 152, pipelines/front_ms 138, render/acquire/ms 110, main/busy/ms 50.41, terrain/update/ms 17.36, pipelines/driver_ms 13.84, render/graph/ms 9.63, render/present/ms 4.32, stream/drive/ms 1.55 |
| warm-up | 3.56 | 53.2 | main/busy/ms 55.07, render/thread/ms 55.04, render/busy/ms 53.47, render/graph/ms 48.91, render/present/ms 1.39, stream/drive/ms 1.28, stream/tick/ms 1.07, terrain/update/ms 0.76, terrain/select/ms 0.63, bigspace/total/ms 0.19, render/acquire/ms 0.18, bigspace/high_precision/ms 0.12 |
| warm-up | 3.59 | 31.6 | main/busy/ms 102, render/thread/ms 80.03, render/busy/ms 78.13, render/graph/ms 49.63, render/present/ms 1.68, stream/drive/ms 1.42, stream/tick/ms 1.07, terrain/update/ms 0.77, terrain/select/ms 0.64, render/acquire/ms 0.21, bigspace/total/ms 0.19, bigspace/high_precision/ms 0.10 |
| warm-up | 3.65 | 63.3 | pipelines/compile_ms 178, pipelines/front_ms 163, render/thread/ms 66.92, render/busy/ms 65.50, main/busy/ms 61.32, pipelines/wait_ms 45.46, pipelines/driver_ms 14.36, render/graph/ms 2.77, stream/drive/ms 1.33, render/present/ms 1.21, stream/tick/ms 0.95, terrain/update/ms 0.72 |
| warm-up | 3.75 | 45.1 | main/busy/ms 38.78, render/thread/ms 36.29, render/busy/ms 34.74, stream/drive/ms 24.66, render/graph/ms 4.09, render/present/ms 1.27, stream/tick/ms 1.21, terrain/update/ms 0.96, stream/spawn/ms 0.83, terrain/select/ms 0.76, bigspace/total/ms 0.30, render/acquire/ms 0.28 |
| warm-up | 3.78 | 33.2 | pipelines/wait_ms 606, pipelines/compile_ms 205, pipelines/front_ms 190, render/thread/ms 53.00, render/busy/ms 51.07, main/busy/ms 46.57, stream/drive/ms 30.28, pipelines/driver_ms 15.14, render/graph/ms 6.15, stream/tick/ms 1.57, render/present/ms 1.57, terrain/update/ms 0.85 |
| warm-up | 3.84 | 59.7 | pipelines/wait_ms 606, pipelines/compile_ms 205, pipelines/front_ms 190, render/thread/ms 68.48, render/busy/ms 66.51, main/busy/ms 49.29, stream/drive/ms 30.61, pipelines/driver_ms 15.14, render/graph/ms 6.84, render/present/ms 1.67, stream/tick/ms 1.61, stream/spawn/ms 0.89 |
| warm-up | 3.95 | 93.7 | pipelines/wait_ms 557, render/thread/ms 76.22, render/busy/ms 48.72, main/busy/ms 27.78, render/present/ms 27.29, stream/drive/ms 11.19, pipelines/compile_ms 9.34, pipelines/driver_ms 8.43, render/graph/ms 3.42, stream/spawn/ms 1.47, stream/tick/ms 1.03, pipelines/front_ms 0.92 |
| warm-up | 3.98 | 30.1 | pipelines/wait_ms 1482, pipelines/compile_ms 218, pipelines/front_ms 198, render/thread/ms 87.98, render/busy/ms 60.70, main/busy/ms 47.35, stream/drive/ms 28.64, render/present/ms 27.09, pipelines/driver_ms 19.97, render/graph/ms 3.35, stream/spawn/ms 1.93, stream/tick/ms 0.72 |
| warm-up | 4.03 | 47.2 | pipelines/wait_ms 926, pipelines/compile_ms 208, pipelines/front_ms 197, main/busy/ms 49.30, render/thread/ms 47.57, render/busy/ms 46.09, stream/drive/ms 30.12, pipelines/driver_ms 11.54, render/graph/ms 3.66, stream/spawn/ms 3.30, render/present/ms 1.27, terrain/update/ms 0.72 |
| warm-up | 4.13 | 37.7 | pipelines/wait_ms 3167, pipelines/compile_ms 161, pipelines/front_ms 155, render/thread/ms 50.64, render/busy/ms 47.72, main/busy/ms 18.19, render/graph/ms 15.68, pipelines/driver_ms 6.55, stream/drive/ms 4.05, render/present/ms 2.61, stream/spawn/ms 2.04, stream/tick/ms 0.74 |
| warm-up | 4.21 | 50.5 | main/busy/ms 66.82, render/thread/ms 54.05, render/busy/ms 52.55, stream/drive/ms 48.86, render/graph/ms 9.55, terrain/update/ms 3.13, stream/spawn/ms 2.14, stream/tick/ms 1.27, render/present/ms 1.21, terrain/select/ms 0.67, render/acquire/ms 0.29, bigspace/total/ms 0.24 |
| warm-up | 4.29 | 75.9 | pipelines/wait_ms 7203, pipelines/compile_ms 231, pipelines/front_ms 160, render/thread/ms 81.99, render/busy/ms 80.31, pipelines/driver_ms 71.38, main/busy/ms 64.42, stream/drive/ms 48.01, render/graph/ms 34.90, terrain/update/ms 3.16, stream/spawn/ms 2.03, render/present/ms 1.50 |
| warm-up | 4.38 | 71.1 | pipelines/wait_ms 8550, pipelines/compile_ms 102, pipelines/front_ms 91.80, main/busy/ms 73.93, stream/drive/ms 50.36, render/thread/ms 35.82, render/busy/ms 34.56, pipelines/driver_ms 9.81, render/graph/ms 7.88, stream/spawn/ms 2.26, render/present/ms 1.07, stream/activate/ms 1.06 |
| warm-up | 4.44 | 38.7 | pipelines/wait_ms 1961, main/busy/ms 43.62, render/thread/ms 33.59, stream/drive/ms 31.94, render/busy/ms 31.79, pipelines/compile_ms 9.09, pipelines/front_ms 8.41, render/graph/ms 3.87, stream/activate/ms 1.18, render/present/ms 0.97, render/acquire/ms 0.82, stream/spawn/ms 0.72 |
| warm-up | 4.48 | 29.9 | main/busy/ms 31.95, stream/drive/ms 21.64, render/thread/ms 13.77, render/busy/ms 12.62, render/graph/ms 3.76, render/present/ms 0.95, stream/tick/ms 0.71, stream/spawn/ms 0.64, terrain/update/ms 0.61, stream/activate/ms 0.54, terrain/select/ms 0.52, physics/insert/ms 0.37 |
| warm-up | 4.60 | 37.8 | pipelines/wait_ms 9590, pipelines/compile_ms 219, pipelines/front_ms 204, render/thread/ms 42.83, render/busy/ms 41.52, main/busy/ms 18.00, pipelines/driver_ms 14.95, render/graph/ms 5.14, stream/drive/ms 3.26, stream/spawn/ms 1.44, render/present/ms 1.08, stream/tick/ms 1.03 |
| warm-up | 4.70 | 40.0 | pipelines/wait_ms 2549, pipelines/compile_ms 62.22, pipelines/front_ms 60.37, render/thread/ms 43.06, render/busy/ms 41.39, main/busy/ms 29.50, render/graph/ms 9.06, stream/drive/ms 6.67, terrain/update/ms 2.97, pipelines/driver_ms 1.85, stream/tick/ms 1.65, render/present/ms 1.46 |
| warm-up | 4.78 | 48.7 | main/busy/ms 55.26, render/thread/ms 50.94, render/busy/ms 49.17, render/graph/ms 6.50, stream/drive/ms 2.57, render/present/ms 1.57, terrain/update/ms 0.92, stream/spawn/ms 0.78, stream/tick/ms 0.76, bigspace/total/ms 0.73, terrain/select/ms 0.62, anim/evaluate_cpu/ms 0.43 |
| warm-up | 5.46 | 28.4 | pipelines/wait_ms 8502, pipelines/compile_ms 78.48, pipelines/front_ms 75.21, main/busy/ms 44.99, render/thread/ms 42.58, render/busy/ms 41.11, stream/drive/ms 10.24, render/graph/ms 7.29, pipelines/driver_ms 3.28, stream/spawn/ms 2.17, physics/insert/ms 1.20, render/present/ms 1.20 |
| warm-up | 5.55 | 30.5 | main/busy/ms 33.69, render/thread/ms 33.33, render/busy/ms 29.66, render/graph/ms 9.36, stream/drive/ms 7.28, render/present/ms 3.45, stream/spawn/ms 3.30, physics/insert/ms 1.52, stream/tick/ms 1.09, bigspace/total/ms 0.85, physics/step/ms 0.79, terrain/update/ms 0.78 |
| warm-up | 5.66 | 38.6 | render/thread/ms 42.75, render/busy/ms 41.66, main/busy/ms 33.96, render/graph/ms 7.30, stream/drive/ms 3.79, stream/spawn/ms 2.12, physics/insert/ms 1.41, stream/tick/ms 0.99, bigspace/total/ms 0.93, render/present/ms 0.93, terrain/update/ms 0.72, terrain/select/ms 0.61 |
| warm-up | 6.66 | 30.0 | pipelines/wait_ms 7938, pipelines/compile_ms 49.69, main/busy/ms 48.99, pipelines/front_ms 46.70, render/thread/ms 34.81, render/busy/ms 32.83, render/graph/ms 9.08, pipelines/driver_ms 2.99, stream/drive/ms 1.32, physics/insert/ms 1.23, render/present/ms 1.21, bigspace/total/ms 0.92 |
| warm-up | 6.69 | 35.5 | pipelines/wait_ms 5367, main/busy/ms 40.95, render/thread/ms 38.08, pipelines/compile_ms 37.37, render/busy/ms 36.26, pipelines/front_ms 35.11, render/graph/ms 5.67, pipelines/driver_ms 2.27, stream/drive/ms 1.43, render/present/ms 1.07, bigspace/total/ms 1.04, physics/insert/ms 1.01 |
| measure | 0.40 | 32.3 | render/thread/ms 43.03, render/busy/ms 37.02, render/graph/ms 22.55, main/busy/ms 18.93, render/present/ms 5.82, physics/insert/ms 1.00, bigspace/total/ms 0.75, terrain/update/ms 0.57, terrain/select/ms 0.52, controller/feed/ms 0.51, stream/drive/ms 0.44, stream/tick/ms 0.37 |
| measure | 2.73 | 35.8 | render/thread/ms 48.04, render/busy/ms 46.72, render/graph/ms 24.76, main/busy/ms 20.52, render/present/ms 1.17, physics/insert/ms 1.07, bigspace/total/ms 0.79, terrain/update/ms 0.67, terrain/select/ms 0.60, stream/drive/ms 0.45, controller/feed/ms 0.44, stream/tick/ms 0.40 |
| measure | 7.19 | 47.5 | render/thread/ms 58.44, render/present/ms 34.66, render/busy/ms 23.61, main/busy/ms 20.86, render/graph/ms 13.17, stream/drive/ms 1.04, bigspace/total/ms 0.93, stream/spawn/ms 0.60, terrain/update/ms 0.56, terrain/select/ms 0.51, bigspace/low_precision/ms 0.48, controller/feed/ms 0.46 |
| measure | 18.18 | 31.5 | main/busy/ms 38.67, render/thread/ms 22.86, render/busy/ms 21.79, stream/drive/ms 21.56, render/graph/ms 11.91, bigspace/total/ms 0.92, render/present/ms 0.88, controller/feed/ms 0.61, terrain/update/ms 0.59, terrain/select/ms 0.54, far/select/ms 0.35, stream/tick/ms 0.35 |
| measure | 35.39 | 31.4 | render/thread/ms 45.63, render/busy/ms 29.85, main/busy/ms 25.51, render/graph/ms 18.39, render/present/ms 15.56, bigspace/total/ms 0.99, stream/drive/ms 0.84, terrain/update/ms 0.82, terrain/select/ms 0.74, controller/feed/ms 0.63, far/select/ms 0.61, anim/evaluate_cpu/ms 0.45 |
| measure | 59.03 | 29.4 | render/thread/ms 39.47, render/busy/ms 38.30, main/busy/ms 27.71, render/graph/ms 12.64, bigspace/total/ms 1.77, controller/feed/ms 1.08, terrain/update/ms 1.05, render/present/ms 0.94, terrain/select/ms 0.94, bigspace/low_precision/ms 0.81, stream/drive/ms 0.73, stream/tick/ms 0.64 |

## Run

- `D:\Gamedev\Bevyrrowind\target\profiling\lampsel_old.exe` view Balmora --hour 22 --weather clear --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lampsel_old\balmora_walk_night@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 13%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **78%** (min 42%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.6 W · 77 °C (max 81) · VRAM 5283 MiB · throttled 100% of samples
