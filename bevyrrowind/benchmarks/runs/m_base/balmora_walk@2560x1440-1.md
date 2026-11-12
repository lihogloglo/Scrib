# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 6512 frames over 65.6 s · **median 9.94 ms (101 fps)** · mean 10.08 · p95 12.10 · p99 13.57 (1% low 74 fps) · worst 31.61 ms · 26 spikes over 1.5× median · 18.3 stutters/min (work 15.5/min)
- warm-up: 523 frames over 6.2 s, worst 594.8 ms
- hitches (> 25.0 ms): **3** measured, 5 in warm-up
- timed GPU passes: 6.25 ms of a 10.08 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 9.94 ms, limit 11 ms
- **FAIL** frame_ms: 20 stutters (> 1.5x moving median of 60) in 1.09 min = 18.28/min, limit 1/min
- pass worst frame 31.61 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.053 | 2.251 | 2.876 | 0.91 | 0.176 |
| main_opaque_pass_3d | 1.337 | 1.467 | 2.148 | 0.91 | 0.048 |
| early prepass | 0.469 | 0.514 | 0.903 | 0.91 | 0.077 |
| bloom | 0.312 | 0.343 | 0.493 | 0.91 | 0.064 |
| atmosphere_luts | 0.302 | 0.331 | 0.503 | 0.91 | 0.003 |
| ssao | 0.199 | 0.218 | 0.211 | 0.91 | 0.007 |
| uniform_allocation | 0.169 | 0.185 | 0.186 | 0.91 | 0.032 |
| auxiliary_views | 0.132 | 0.144 | 0.521 | 0.91 | 0.130 |
| water_fft | 0.122 | 0.134 | 0.129 | 0.91 | 0.003 |
| bin_unpacking | 0.121 | 0.132 | 0.142 | 0.91 | 0.023 |
| upscaling | 0.117 | 0.128 | 0.147 | 0.91 | 0.004 |
| clouds_trace | 0.116 | 0.127 | 0.216 | 0.91 | 0.001 |
| atmos_froxel_inject | 0.090 | 0.099 | 0.106 | 0.91 | 0.001 |
| tonemapping | 0.084 | 0.092 | 0.092 | 0.91 | 0.003 |
| water_velocity | 0.079 | 0.087 | 0.085 | 0.91 | 0.002 |
| clustering | 0.062 | 0.068 | 0.069 | 0.91 | 0.019 |
| main_transparent_pass_3d | 0.059 | 0.065 | 0.109 | 0.91 | 0.106 |
| clouds_composite | 0.059 | 0.064 | 0.063 | 0.91 | 0.002 |
| render_sky | 0.049 | 0.054 | 0.054 | 0.91 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.045 | 0.049 | 0.91 | 0.005 |
| lightprobe_irradiance_map | 0.040 | 0.443 | 0.749 | 0.09 | 0.001 |
| surface_trail | 0.035 | 0.058 | 0.057 | 0.61 | 0.002 |
| clouds_resolve | 0.032 | 0.035 | 0.037 | 0.91 | 0.001 |
| clouds_shadow | 0.029 | 0.032 | 0.034 | 0.91 | 0.003 |
| atmos_froxel_integrate | 0.023 | 0.026 | 0.030 | 0.91 | 0.001 |
| water_snapshot | 0.022 | 0.024 | 0.024 | 0.91 | 0.001 |
| water_mips | 0.021 | 0.023 | 0.024 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.023 | 0.024 | 0.91 | 0.006 |
| lightprobe_radiance_map | 0.020 | 0.224 | 0.234 | 0.09 | 0.004 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.008 | 0.91 | 0.003 |
| night_sky | 0.005 | 0.006 | 0.013 | 0.91 | 0.002 |
| lightprobe_copy | 0.001 | 0.014 | 0.015 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.01 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 8.765 | 10.555 | 30.34 | 57080.2 |
| main/busy/ms | 8.190 | 9.694 | 16.03 | 53335.9 |
| render/busy/ms | 8.148 | 9.963 | 26.56 | 53062.6 |
| render/graph/ms | 4.329 | 5.520 | 22.84 | 28188.1 |
| pipelines/wait_ms | 1.019 | 0.000 | 6638.09 | 6638.2 |
| bigspace/total/ms | 0.529 | 1.201 | 2.63 | 3442.3 |
| render/present/ms | 0.523 | 1.203 | 20.42 | 3407.9 |
| terrain/update/ms | 0.295 | 0.377 | 1.97 | 1921.9 |
| terrain/select/ms | 0.262 | 0.335 | 0.71 | 1705.7 |
| controller/feed/ms | 0.259 | 0.354 | 0.81 | 1689.1 |
| stream/drive/ms | 0.232 | 0.322 | 1.99 | 1510.0 |
| bigspace/low_precision/ms | 0.207 | 0.627 | 2.41 | 1348.1 |
| far/select/ms | 0.196 | 0.279 | 0.65 | 1277.4 |
| stream/tick/ms | 0.189 | 0.263 | 0.67 | 1233.9 |
| anim/evaluate_cpu/ms | 0.177 | 0.217 | 0.53 | 1150.9 |
| bigspace/high_precision/ms | 0.161 | 0.606 | 1.69 | 1047.2 |
| anim/evaluate/ms | 0.143 | 0.264 | 2.04 | 931.5 |
| controller/move/ms | 0.121 | 0.200 | 0.51 | 789.7 |
| render/acquire/ms | 0.094 | 0.124 | 0.45 | 609.7 |
| pipelines/compile_ms | 0.079 | 0.000 | 164.59 | 513.1 |
| pipelines/front_ms | 0.076 | 0.000 | 157.20 | 492.3 |
| physics/step/ms | 0.043 | 0.087 | 0.62 | 283.2 |
| physics/insert/ms | 0.027 | 0.301 | 0.87 | 177.9 |
| weather/map/ms | 0.027 | 0.036 | 0.28 | 176.5 |
| anim/apply/ms | 0.012 | 0.022 | 0.35 | 78.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.23 | 72.0 |
| controller/avoid/ms | 0.011 | 0.014 | 0.35 | 70.1 |
| script/update/ms | 0.007 | 0.010 | 0.23 | 46.2 |
| script/input/ms | 0.005 | 0.006 | 0.33 | 31.7 |
| stream/despawn/ms | 0.004 | 0.000 | 1.01 | 28.2 |
| stream/spawn/ms | 0.004 | 0.000 | 1.09 | 25.4 |
| script/ui/ms | 0.004 | 0.005 | 0.22 | 24.0 |
| controller/write/ms | 0.003 | 0.004 | 0.28 | 22.6 |
| pipelines/driver_ms | 0.003 | 0.000 | 7.39 | 20.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.65 | 14.1 |
| weather/fx/ms | 0.001 | 0.001 | 0.18 | 6.6 |
| shapes/activate/ms | 0.001 | 0.002 | 0.02 | 5.5 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.1 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 2.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.7 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 28 | 21 |
| controller/movers/count | count | 20 | 22 | 21 |
| crowd/c0/count | count | 27 | 43 | 11 |
| crowd/c0/joints/count | count | 1306 | 2262 | 480 |
| crowd/c0/meshes/count | count | 355 | 646 | 117 |
| crowd/c1/count | count | 146 | 155 | 150 |
| crowd/c1/joints/count | count | 5531 | 5936 | 5716 |
| crowd/c1/meshes/count | count | 1165 | 1268 | 1224 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4254 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4236 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.4 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 8.42 | 26.56 | 8.00 |
| gpu/timed/ms | ms | 7.57 | 9.61 | 8.00 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.3 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 14 | 14 | 14 |
| mesh_allocator_slabs_size |  bytes | 493.16 M | 493.16 M | 493.16 M |
| phases/alpha_mask3d/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 836 | 1420 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 8 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 836 | 1420 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 7 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3472 | 5122 | 3178 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 15 | 16 | 15 |
| phases/opaque3d_prepass/bins | count | 1290 | 1394 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3472 | 5122 | 3178 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 14 | 15 | 14 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 40 | 230 | 28 |
| phases/shadow/bins | count | 3843 | 5405 | 2222 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9682 | 12.8 k | 5673 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 124 | 254 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2679 | 3268 | 2350 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 180 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 173 | 175 | 175 |
| pipelines/waiting | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 9 | 603 | 6 |
| preprocess/bind_groups_created | count | 9 | 603 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 172 | 272 | 158 |
| process/cpu_usage | % | 26.38 | 40.87 | 29.33 |
| process/mem_usage | GiB | 2.70 | 2.86 | 2.85 |
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
| started_load_count |  | 3481 | 3547 | 3547 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 435 | 530 | 398 |
| statics/materials_bound | count | 435 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.06 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.21 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.55 | 48.91 | 35.18 |
| system/mem_usage | % | 54.64 | 55.79 | 55.76 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 84 | 270 | 72 |
| views/all/bins | count | 6539 | 8189 | 5702 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.5 k | 25.3 k | 13.3 k |
| views/all/pipelines | count | 164 | 293 | 68 |
| views/all/pipelines_distinct | count | 62 | 69 | 55 |
| views/all/work_items | count | 17.5 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 36 | 50 | 28 |
| views/cascade/bins | count | 2624 | 3187 | 2222 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8202 | 10.2 k | 5673 |
| views/cascade/pipelines | count | 36 | 50 | 28 |
| views/cascade/work_items | count | 8202 | 10.2 k | 5673 |
| views/lamp_face/batch_sets | count | 0 | 191 | 0 |
| views/lamp_face/bins | count | 1115 | 2467 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1238 | 2839 | 0 |
| views/lamp_face/pipelines | count | 82 | 209 | 0 |
| views/lamp_face/work_items | count | 0 | 2552 | 0 |
| views/main/batch_sets | count | 44 | 48 | 44 |
| views/main/bins | count | 2884 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8610 | 13.0 k | 7590 |
| views/main/pipelines | count | 42 | 44 | 40 |
| views/main/work_items | count | 8762 | 13.2 k | 7898 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.8 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 49.43 | 269 | 321.9 k |
| anim/events/count | count | 0.23 | 4 | 1491 |
| assets/Image/added | count | 0.05 | 35 | 309 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.63 | 160 | 4086 |
| assets/Mesh/removed | count | 0.75 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.07 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.33 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.06 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.73 | 1328 | 4739 |
| physics/inserts | count | 0.34 | 106 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.36 | 129 | 2335 |
| physics/steps | count | 0.60 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 7 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.06 | 23 | 378 |
| stream/upload/bytes | bytes | 5198 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 61 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1140 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 8, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.59 | 594.8 | main/busy/ms 573, surface/advance_cpu_ms 71.21, surface/cell_upload_ms 0.69, stream/drive/ms 0.35, bigspace/total/ms 0.13, weather/map/ms 0.12, stream/tick/ms 0.10, bigspace/high_precision/ms 0.06, script/update/ms 0.03, weather/eval/ms 0.03, far/select/ms 0.01, script/input/ms 0.01 |
| warm-up | 0.93 | 333.8 | main/busy/ms 609, render/thread/ms 298, render/busy/ms 223, render/acquire/ms 73.43, surface/advance_cpu_ms 71.21, render/graph/ms 5.87, render/present/ms 1.99, stream/drive/ms 1.08, surface/cell_upload_ms 0.69, stream/tick/ms 0.53, terrain/update/ms 0.49, terrain/select/ms 0.36 |
| warm-up | 1.00 | 73.4 | pipelines/wait_ms 326, render/thread/ms 308, render/busy/ms 232, pipelines/compile_ms 74.56, render/acquire/ms 73.69, pipelines/front_ms 67.61, main/busy/ms 54.66, terrain/update/ms 21.01, render/graph/ms 9.96, pipelines/driver_ms 6.95, render/present/ms 2.96, stream/drive/ms 1.24 |
| warm-up | 1.24 | 80.7 | pipelines/compile_ms 192, pipelines/front_ms 177, main/busy/ms 58.88, pipelines/wait_ms 42.91, render/thread/ms 42.86, render/busy/ms 41.15, pipelines/driver_ms 15.55, render/graph/ms 3.64, stream/drive/ms 1.80, stream/tick/ms 1.58, render/present/ms 1.49, terrain/update/ms 0.67 |
| warm-up | 2.45 | 25.6 | pipelines/wait_ms 6473, pipelines/compile_ms 39.14, main/busy/ms 37.29, pipelines/front_ms 35.45, render/thread/ms 28.26, render/busy/ms 26.83, render/graph/ms 13.72, pipelines/driver_ms 3.69, stream/drive/ms 3.62, stream/spawn/ms 2.19, render/present/ms 1.12, stream/tick/ms 0.68 |
| measure | 0.51 | 28.8 | render/thread/ms 35.08, render/busy/ms 33.87, render/graph/ms 26.79, main/busy/ms 17.05, bigspace/total/ms 1.21, render/present/ms 1.02, bigspace/low_precision/ms 0.80, stream/drive/ms 0.59, terrain/update/ms 0.54, stream/tick/ms 0.51, terrain/select/ms 0.48, controller/feed/ms 0.38 |
| measure | 44.89 | 31.6 | render/thread/ms 38.89, render/present/ms 21.04, render/busy/ms 17.65, main/busy/ms 17.63, render/graph/ms 10.21, bigspace/total/ms 1.82, bigspace/high_precision/ms 0.74, bigspace/low_precision/ms 0.72, controller/move/ms 0.63, terrain/update/ms 0.58, terrain/select/ms 0.52, stream/drive/ms 0.51 |
| measure | 60.02 | 27.6 | render/thread/ms 34.69, render/busy/ms 17.99, main/busy/ms 16.45, render/present/ms 16.43, render/graph/ms 9.97, bigspace/total/ms 1.05, controller/feed/ms 0.67, terrain/update/ms 0.60, terrain/select/ms 0.54, far/select/ms 0.45, stream/drive/ms 0.43, anim/evaluate_cpu/ms 0.42 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_base\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 12%, CPU 20%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **87%** (min 72%) · 2580 MHz (min 2580) · memory 8001 MHz · 83.7 W · 74 °C (max 78) · VRAM 4391 MiB · throttled 100% of samples
