# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5604 frames over 65.6 s · **median 11.24 ms (89 fps)** · mean 11.71 · p95 14.33 · p99 15.50 (1% low 65 fps) · worst 23.38 ms · 18 spikes over 1.5× median · 5.5 stutters/min (work 8.2/min)
- warm-up: 365 frames over 4.9 s, worst 581.9 ms
- hitches (> 28.1 ms): **0** measured, 5 in warm-up
- timed GPU passes: 7.41 ms of a 11.71 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.24 ms, limit 11 ms
- **FAIL** frame_ms: 6 stutters (> 1.5x moving median of 60) in 1.09 min = 5.48/min, limit 1/min
- pass worst frame 23.38 ms, limit 33 ms

## GPU passes (Bevy-timed, mean per frame)

| pass | GPU ms | GPU p95 | CPU ms |
|---|---:|---:|---:|
| dlss_super_resolution | 2.073 | 2.101 | 0.172 |
| main_opaque_pass_3d | 1.409 | 1.732 | 0.046 |
| early prepass | 0.534 | 0.766 | 0.076 |
| lightprobe_irradiance_map | 0.434 | 0.571 | 0.001 |
| bloom | 0.319 | 0.327 | 0.064 |
| atmosphere_luts | 0.310 | 0.371 | 0.003 |
| auxiliary_views | 0.238 | 0.807 | 0.203 |
| lightprobe_radiance_map | 0.223 | 0.233 | 0.004 |
| ssao | 0.206 | 0.207 | 0.007 |
| uniform_allocation | 0.154 | 0.162 | 0.015 |
| clouds_trace | 0.127 | 0.216 | 0.001 |
| water_fft | 0.127 | 0.127 | 0.002 |
| upscaling | 0.119 | 0.132 | 0.004 |
| bin_unpacking | 0.118 | 0.127 | 0.012 |
| early_mesh_preprocessing | 0.095 | 0.100 | 0.003 |
| gpu_instance_materialization | 0.093 | 0.097 | 0.004 |
| atmos_froxel_inject | 0.092 | 0.100 | 0.001 |
| tonemapping | 0.086 | 0.087 | 0.003 |
| water_velocity | 0.081 | 0.085 | 0.001 |
| clustering | 0.065 | 0.069 | 0.018 |
| clouds_composite | 0.062 | 0.063 | 0.002 |
| surface_trail | 0.058 | 0.057 | 0.002 |
| main_transparent_pass_3d | 0.058 | 0.103 | 0.082 |
| render_sky | 0.052 | 0.053 | 0.002 |
| per_view_shadow_pass | 0.036 | 0.038 | 0.008 |
| clouds_resolve | 0.035 | 0.038 | 0.001 |
| clouds_shadow | 0.032 | 0.035 | 0.003 |
| range_unpacking | 0.027 | 0.031 | 0.003 |
| atmos_froxel_integrate | 0.025 | 0.028 | 0.001 |
| water_snapshot | 0.023 | 0.024 | 0.001 |
| water_mips | 0.023 | 0.024 | 0.002 |
| lightprobe_downsampling_first_pass | 0.013 | 0.014 | 0.001 |
| lightprobe_copy | 0.013 | 0.014 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.002 |
| lightprobe_downsampling_second_pass | 0.011 | 0.011 | 0.001 |
| atmosphere_environment | 0.010 | 0.010 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.003 |
| night_sky | 0.006 | 0.013 | 0.002 |
| blas_build | 0.001 | 0.001 | 0.005 |
| weather_map | — | — | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.124 | 12.623 | 21.61 | 56732.6 |
| render/busy/ms | 9.601 | 12.058 | 20.96 | 53803.1 |
| main/busy/ms | 8.144 | 9.935 | 16.08 | 45639.3 |
| pipelines/wait_ms | 5.490 | 0.000 | 5264.87 | 30764.1 |
| render/graph/ms | 5.017 | 6.577 | 12.81 | 28117.1 |
| render/present/ms | 0.434 | 0.531 | 2.24 | 2432.4 |
| bigspace/total/ms | 0.404 | 0.614 | 2.40 | 2261.8 |
| terrain/update/ms | 0.284 | 0.362 | 2.08 | 1592.9 |
| stream/drive/ms | 0.270 | 0.444 | 1.95 | 1513.8 |
| terrain/select/ms | 0.249 | 0.317 | 0.89 | 1397.2 |
| controller/feed/ms | 0.239 | 0.310 | 0.54 | 1338.2 |
| stream/tick/ms | 0.216 | 0.372 | 0.82 | 1209.6 |
| far/select/ms | 0.212 | 0.362 | 0.71 | 1185.7 |
| pipelines/compile_ms | 0.191 | 0.000 | 178.15 | 1072.6 |
| pipelines/front_ms | 0.185 | 0.000 | 170.93 | 1038.1 |
| anim/evaluate_cpu/ms | 0.163 | 0.217 | 0.45 | 915.6 |
| bigspace/low_precision/ms | 0.163 | 0.244 | 1.76 | 911.5 |
| anim/evaluate/ms | 0.144 | 0.295 | 1.97 | 804.5 |
| controller/move/ms | 0.137 | 0.262 | 0.63 | 768.2 |
| bigspace/high_precision/ms | 0.096 | 0.141 | 1.89 | 538.2 |
| render/acquire/ms | 0.089 | 0.115 | 0.37 | 497.0 |
| anim/apply/ms | 0.083 | 0.112 | 2.35 | 466.6 |
| physics/insert/ms | 0.052 | 0.440 | 0.96 | 292.1 |
| physics/step/ms | 0.050 | 0.084 | 0.61 | 278.3 |
| weather/map/ms | 0.026 | 0.034 | 0.16 | 146.2 |
| controller/avoid/ms | 0.011 | 0.017 | 0.27 | 63.9 |
| weather/eval/ms | 0.011 | 0.014 | 0.26 | 59.5 |
| script/update/ms | 0.007 | 0.015 | 0.11 | 40.1 |
| pipelines/driver_ms | 0.006 | 0.000 | 7.22 | 34.4 |
| stream/despawn/ms | 0.006 | 0.000 | 1.06 | 34.3 |
| script/ui/ms | 0.005 | 0.008 | 0.18 | 27.3 |
| script/input/ms | 0.005 | 0.006 | 0.09 | 26.2 |
| stream/spawn/ms | 0.003 | 0.000 | 1.09 | 19.4 |
| controller/write/ms | 0.003 | 0.005 | 0.04 | 19.3 |
| stream/upload/ms | 0.003 | 0.000 | 1.12 | 18.1 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.70 | 16.0 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.6 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.09 | 5.5 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 5 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 17 |
| controller/movers/count | count | 20 | 22 | 17 |
| ecs/entities | count | 48.4 k | 56.7 k | 47.7 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 9.18 | 20.96 | 8.25 |
| gpu/timed/ms | ms | 7.47 | 9.48 | 8.25 |
| mesh_allocator_allocations |  meshes | 10.8 k | 11.8 k | 10.8 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 311.90 M | 311.90 M | 311.90 M |
| phases/alpha_mask3d/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d/bins | count | 158 | 388 | 380 |
| phases/alpha_mask3d/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d/instance_slots | count | 16.4 k | 16.4 k | 16.4 k |
| phases/alpha_mask3d/items | count | 345 | 588 | 516 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 5 |
| phases/alpha_mask3d_prepass/batch_sets | count | 9 | 9 | 8 |
| phases/alpha_mask3d_prepass/bins | count | 158 | 388 | 380 |
| phases/alpha_mask3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/alpha_mask3d_prepass/instance_draws | count | 666 | 716 | 716 |
| phases/alpha_mask3d_prepass/instance_slots | count | 16.4 k | 16.4 k | 16.4 k |
| phases/alpha_mask3d_prepass/items | count | 345 | 588 | 516 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 5 |
| phases/opaque3d/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d/bins | count | 760 | 973 | 757 |
| phases/opaque3d/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d/instance_draws | count | 5760 | 5881 | 5881 |
| phases/opaque3d/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| phases/opaque3d/items | count | 2518 | 3567 | 2451 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/bins | count | 760 | 973 | 757 |
| phases/opaque3d_prepass/instance_batches | count | 2 | 2 | 2 |
| phases/opaque3d_prepass/instance_draws | count | 5760 | 5881 | 5881 |
| phases/opaque3d_prepass/instance_slots | count | 32.8 k | 32.8 k | 32.8 k |
| phases/opaque3d_prepass/items | count | 2518 | 3567 | 2451 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/shadow/batch_sets | count | 51 | 229 | 41 |
| phases/shadow/bins | count | 1630 | 2582 | 989 |
| phases/shadow/instance_batches | count | 12 | 12 | 12 |
| phases/shadow/instance_draws | count | 19.3 k | 19.8 k | 19.8 k |
| phases/shadow/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| phases/shadow/items | count | 5131 | 7198 | 3886 |
| phases/shadow/pipelines | count | 92 | 238 | 31 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2346 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 1202 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 165 | 166 | 166 |
| pipelines/waiting | count | 0 | 14 | 0 |
| preprocess/bind_groups | count | 190 | 585 | 169 |
| preprocess/bind_groups_created | count | 183 | 578 | 162 |
| preprocess/bind_groups_materialize | count | 0 | 4 | 0 |
| preprocess/bind_groups_ranges | count | 7 | 7 | 7 |
| process/cpu_usage | % | 22.60 | 37.33 | 18.27 |
| process/mem_usage | GiB | 2.94 | 3.00 | 3.00 |
| scene/batch/chain/capacity | count | 0 | 0 | 0 |
| scene/batch/chain/count | count | 0 | 0 | 0 |
| scene/batch/chain/high_water | count | 0 | 0 | 0 |
| scene/batch/chain/live | count | 0 | 0 | 0 |
| scene/batch/chain/overflow | count | 0 | 0 | 0 |
| scene/batch/masked/capacity | count | 16.4 k | 16.4 k | 16.4 k |
| scene/batch/masked/count | count | 2 | 2 | 2 |
| scene/batch/masked/high_water | count | 2979 | 3169 | 3169 |
| scene/batch/masked/live | count | 2979 | 3169 | 3169 |
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
| scene/batch/opaque/capacity | count | 32.8 k | 32.8 k | 32.8 k |
| scene/batch/opaque/count | count | 2 | 2 | 2 |
| scene/batch/opaque/high_water | count | 18.8 k | 19.4 k | 19.4 k |
| scene/batch/opaque/live | count | 18.8 k | 19.4 k | 19.4 k |
| scene/batch/opaque/overflow | count | 0 | 461 | 461 |
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
| scene/batches/count | count | 4 | 4 | 4 |
| scene/cells/resident | count | 117 | 125 | 125 |
| scene/chains/high_water | count | 1594 | 1624 | 1624 |
| scene/chains/live | count | 1594 | 1624 | 1624 |
| scene/chains/pending | count | 0 | 0 | 0 |
| scene/overflow/instances | count | 0 | 461 | 461 |
| scene/promoted | count | 0 | 0 | 0 |
| scene/slots/batch_bump | count | 21.7 k | 22.6 k | 22.6 k |
| scene/slots/batch_capacity | count | 49.2 k | 49.2 k | 49.2 k |
| scene/slots/batch_capacity_bytes | bytes | 3.15 M | 3.15 M | 3.15 M |
| scene/slots/batch_live | count | 21.7 k | 22.6 k | 22.6 k |
| scene/slots/scene_bump | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/scene_live | count | 21.7 k | 23.1 k | 23.1 k |
| scene/slots/waste | count | 0 | 0 | 0 |
| script/ctx/mw.legacy/mib | count | 0.42 | 0.42 | 0.42 |
| shapes/proxies | count | 0 | 0 | 0 |
| shapes/sets | count | 0 | 0 | 0 |
| shapes/sync/ms | ms | 0 | 0 | 0 |
| started_load_count |  | 5034 | 5189 | 5189 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 237 | 333 | 225 |
| statics/materials_bound | count | 237 | 333 | 225 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.05 | 0.08 | 0.06 |
| stream/model_queue | count | 0 | 47 | 0 |
| stream/models | count | 290 | 380 | 265 |
| stream/oldest_request/s | s | 0 | 0.26 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 27.89 | 43.21 | 29.69 |
| system/mem_usage | % | 43.60 | 43.71 | 43.69 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 101 | 275 | 91 |
| views/all/bins | count | 3492 | 4579 | 3263 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 20 | 20 | 20 |
| views/all/instance_draws | count | 32.1 k | 33.0 k | 33.0 k |
| views/all/instance_slots | count | 245.8 k | 245.8 k | 245.8 k |
| views/all/items | count | 10.8 k | 14.1 k | 9820 |
| views/all/pipelines | count | 132 | 277 | 69 |
| views/all/pipelines_distinct | count | 60 | 66 | 53 |
| views/all/work_items | count | 256.4 k | 259.2 k | 255.9 k |
| views/cascade/batch_sets | count | 46 | 56 | 41 |
| views/cascade/bins | count | 1434 | 1746 | 989 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 12 | 12 | 12 |
| views/cascade/instance_draws | count | 19.3 k | 19.8 k | 19.8 k |
| views/cascade/instance_slots | count | 147.5 k | 147.5 k | 147.5 k |
| views/cascade/items | count | 4741 | 6091 | 3886 |
| views/cascade/pipelines | count | 38 | 47 | 31 |
| views/cascade/work_items | count | 152.2 k | 153.5 k | 151.3 k |
| views/lamp_face/batch_sets | count | 4 | 180 | 0 |
| views/lamp_face/bins | count | 195 | 876 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 331 | 1224 | 0 |
| views/lamp_face/pipelines | count | 54 | 195 | 0 |
| views/lamp_face/work_items | count | 38 | 1017 | 0 |
| views/main/batch_sets | count | 50 | 54 | 50 |
| views/main/bins | count | 2024 | 2318 | 2274 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 8 | 8 | 8 |
| views/main/instance_draws | count | 12.9 k | 13.2 k | 13.2 k |
| views/main/instance_slots | count | 98.3 k | 98.3 k | 98.3 k |
| views/main/items | count | 5952 | 7554 | 5934 |
| views/main/pipelines | count | 40 | 42 | 38 |
| views/main/work_items | count | 104.5 k | 106.0 k | 104.5 k |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 20.5 k | 26.4 k | 19.3 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 56.49 | 277 | 316.6 k |
| anim/events/count | count | 0.28 | 4 | 1581 |
| assets/Image/added | count | 0.05 | 35 | 308 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.59 | 244 | 3286 |
| assets/Mesh/removed | count | 0.88 | 1228 | 4910 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 512 |
| assets/StandardMaterial/modified | count | 3.86 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 48 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.61 | 1388 | 3409 |
| physics/inserts | count | 0.56 | 121 | 3114 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 129 | 2335 |
| physics/steps | count | 0.70 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 5 |
| scene/overflow/cells | count | 0.00 | 1 | 6 |
| scene/slots/overflow | count | 0.08 | 190 | 461 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 20 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 27 | 351 |
| stream/upload/bytes | bytes | 6041 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1324 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.1 ms; slowest 5, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 581.9 | main/busy/ms 558, surface/advance_cpu_ms 64.85, surface/cell_upload_ms 0.72, stream/drive/ms 0.34, weather/map/ms 0.12, bigspace/total/ms 0.10, stream/tick/ms 0.07, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.90 | 318.1 | main/busy/ms 593, render/thread/ms 284, render/busy/ms 207, render/acquire/ms 74.63, surface/advance_cpu_ms 64.85, render/graph/ms 4.74, render/present/ms 1.95, stream/drive/ms 1.34, surface/cell_upload_ms 0.72, stream/tick/ms 0.63, terrain/update/ms 0.40, bigspace/total/ms 0.26 |
| warm-up | 1.01 | 109.0 | pipelines/wait_ms 1810, render/thread/ms 301, render/busy/ms 222, pipelines/compile_ms 118, pipelines/front_ms 109, render/acquire/ms 74.94, main/busy/ms 52.69, terrain/update/ms 18.98, render/graph/ms 9.87, pipelines/driver_ms 8.30, render/present/ms 4.07, stream/drive/ms 1.44 |
| warm-up | 1.25 | 90.3 | pipelines/wait_ms 624, pipelines/compile_ms 101, pipelines/front_ms 95.03, main/busy/ms 71.60, render/thread/ms 44.09, render/busy/ms 42.51, terrain/update/ms 8.05, pipelines/driver_ms 6.29, render/graph/ms 4.05, stream/drive/ms 1.55, render/present/ms 1.38, stream/tick/ms 1.29 |
| warm-up | 4.21 | 38.9 | pipelines/wait_ms 12.0 k, render/thread/ms 61.30, pipelines/compile_ms 42.24, pipelines/front_ms 40.35, main/busy/ms 37.52, render/acquire/ms 35.97, render/busy/ms 22.34, render/graph/ms 8.22, terrain/update/ms 6.02, render/present/ms 2.99, pipelines/driver_ms 1.89, physics/insert/ms 1.13 |

## Run

- `../target/profiling/bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --scene --resolution 2560x1440 --bench-out target/perf\fix_on\balmora_walk@2560x1440_--scene-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 5%
- during the run: other processes 5% CPU (system minus this process)
- nvidia-smi over the window (256 samples): GPU load **75%** (min 60%) · 2580 MHz (min 2580) · memory 8001 MHz · 73.9 W · 69 °C (max 73) · VRAM 3715 MiB · throttled 100% of samples
