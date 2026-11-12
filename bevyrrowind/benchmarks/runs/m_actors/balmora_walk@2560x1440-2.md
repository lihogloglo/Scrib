# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: actors · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 7675 frames over 65.6 s · **median 8.51 ms (118 fps)** · mean 8.55 · p95 10.04 · p99 11.35 (1% low 88 fps) · worst 30.35 ms · 27 spikes over 1.5× median · 31.1 stutters/min (work 17.4/min)
- warm-up: 335 frames over 4.1 s, worst 580.8 ms
- hitches (> 25.0 ms): **1** measured, 6 in warm-up
- timed GPU passes: 6.14 ms of a 8.55 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 8.51 ms, limit 11 ms
- **FAIL** frame_ms: 34 stutters (> 1.5x moving median of 60) in 1.09 min = 31.07/min, limit 1/min
- pass worst frame 30.35 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.082 | 2.238 | 2.751 | 0.93 | 0.188 |
| main_opaque_pass_3d | 1.356 | 1.458 | 2.112 | 0.93 | 0.036 |
| early prepass | 0.444 | 0.477 | 0.840 | 0.93 | 0.066 |
| bloom | 0.319 | 0.343 | 0.498 | 0.93 | 0.067 |
| atmosphere_luts | 0.306 | 0.329 | 0.531 | 0.93 | 0.003 |
| ssao | 0.205 | 0.220 | 0.216 | 0.93 | 0.007 |
| water_fft | 0.123 | 0.132 | 0.128 | 0.93 | 0.002 |
| uniform_allocation | 0.120 | 0.129 | 0.133 | 0.93 | 0.025 |
| clouds_trace | 0.119 | 0.128 | 0.215 | 0.93 | 0.001 |
| upscaling | 0.113 | 0.121 | 0.140 | 0.93 | 0.004 |
| atmos_froxel_inject | 0.093 | 0.099 | 0.108 | 0.93 | 0.001 |
| tonemapping | 0.087 | 0.094 | 0.094 | 0.93 | 0.003 |
| bin_unpacking | 0.087 | 0.093 | 0.098 | 0.93 | 0.018 |
| water_velocity | 0.081 | 0.087 | 0.085 | 0.93 | 0.002 |
| clustering | 0.065 | 0.070 | 0.071 | 0.93 | 0.020 |
| clouds_composite | 0.061 | 0.065 | 0.065 | 0.93 | 0.002 |
| main_transparent_pass_3d | 0.057 | 0.062 | 0.108 | 0.93 | 0.106 |
| render_sky | 0.051 | 0.054 | 0.054 | 0.93 | 0.002 |
| lightprobe_irradiance_map | 0.048 | 0.441 | 0.758 | 0.11 | 0.001 |
| early_mesh_preprocessing | 0.038 | 0.041 | 0.046 | 0.93 | 0.005 |
| auxiliary_views | 0.035 | 0.037 | 0.288 | 0.93 | 0.033 |
| clouds_resolve | 0.033 | 0.035 | 0.039 | 0.93 | 0.001 |
| surface_trail | 0.033 | 0.061 | 0.058 | 0.53 | 0.002 |
| clouds_shadow | 0.030 | 0.032 | 0.035 | 0.93 | 0.003 |
| lightprobe_radiance_map | 0.026 | 0.234 | 0.239 | 0.11 | 0.004 |
| atmos_froxel_integrate | 0.024 | 0.025 | 0.032 | 0.93 | 0.001 |
| water_mips | 0.022 | 0.024 | 0.024 | 0.93 | 0.002 |
| water_snapshot | 0.022 | 0.024 | 0.024 | 0.93 | 0.001 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.026 | 0.93 | 0.006 |
| clouds_fog_light | 0.012 | 0.013 | 0.013 | 0.93 | 0.002 |
| atmosphere_environment | 0.010 | 0.011 | 0.012 | 0.93 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.93 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.93 | 0.002 |
| lightprobe_copy | 0.002 | 0.016 | 0.016 | 0.11 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.11 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.012 | 0.11 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.003 | 0.01 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 9.449 | 0.000 | 7243.15 | 72522.2 |
| render/thread/ms | 7.528 | 8.881 | 31.29 | 57781.1 |
| main/busy/ms | 6.607 | 7.656 | 11.41 | 50709.1 |
| render/busy/ms | 6.492 | 7.704 | 21.03 | 49823.7 |
| render/graph/ms | 3.496 | 4.355 | 14.22 | 26831.7 |
| render/acquire/ms | 0.531 | 1.640 | 3.15 | 4073.7 |
| render/present/ms | 0.506 | 1.092 | 19.41 | 3883.7 |
| bigspace/total/ms | 0.331 | 0.605 | 4.04 | 2543.9 |
| terrain/update/ms | 0.291 | 0.366 | 1.72 | 2235.7 |
| terrain/select/ms | 0.259 | 0.326 | 0.65 | 1988.6 |
| stream/drive/ms | 0.237 | 0.351 | 3.48 | 1818.2 |
| stream/tick/ms | 0.195 | 0.284 | 0.65 | 1499.8 |
| far/select/ms | 0.188 | 0.266 | 0.60 | 1445.4 |
| pipelines/compile_ms | 0.169 | 0.000 | 164.81 | 1297.1 |
| pipelines/front_ms | 0.163 | 0.000 | 158.33 | 1252.4 |
| bigspace/high_precision/ms | 0.116 | 0.262 | 1.74 | 890.7 |
| bigspace/low_precision/ms | 0.104 | 0.236 | 2.11 | 798.7 |
| physics/insert/ms | 0.028 | 0.337 | 1.04 | 211.2 |
| weather/map/ms | 0.027 | 0.035 | 0.39 | 204.6 |
| physics/step/ms | 0.015 | 0.029 | 0.78 | 111.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.21 | 85.4 |
| script/update/ms | 0.007 | 0.009 | 0.17 | 50.2 |
| pipelines/driver_ms | 0.006 | 0.000 | 6.48 | 44.7 |
| script/input/ms | 0.005 | 0.006 | 0.15 | 35.1 |
| script/ui/ms | 0.004 | 0.006 | 0.16 | 30.2 |
| stream/despawn/ms | 0.003 | 0.000 | 0.80 | 26.1 |
| stream/spawn/ms | 0.003 | 0.000 | 1.09 | 25.1 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.59 | 13.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.13 | 7.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.13 | 5.3 |
| stream/activate/ms | 0.001 | 0.000 | 0.41 | 4.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.03 | 2.9 |
| stream/upload/ms | 0.000 | 0.000 | 0.03 | 0.6 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| ecs/entities | count | 35.1 k | 41.7 k | 33.6 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.41 M | 203.41 M | 203.41 M |
| far/triangles | count | 5.04 M | 5.04 M | 5.04 M |
| frame/work/ms | ms | 7.44 | 21.03 | 7.69 |
| gpu/timed/ms | ms | 7.25 | 10.56 | 7.69 |
| mesh_allocator_allocations |  meshes | 13.0 k | 14.2 k | 13.1 k |
| mesh_allocator_slabs |  slabs | 12 | 12 | 12 |
| mesh_allocator_slabs_size |  bytes | 538.27 M | 538.27 M | 538.27 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d/bins | count | 186 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 891 | 1419 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 7 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 186 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 891 | 1419 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 6 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d/bins | count | 878 | 1208 | 1152 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 2838 | 4289 | 2178 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 9 | 9 | 9 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 10 | 10 | 10 |
| phases/opaque3d_prepass/bins | count | 878 | 1208 | 1152 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 2838 | 4289 | 2178 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 9 | 9 | 9 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 198 | 21 |
| phases/shadow/bins | count | 3157 | 4466 | 1745 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 8505 | 11.3 k | 4434 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 107 | 213 | 21 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2711 | 3248 | 2329 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 1337 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 148 | 150 | 150 |
| pipelines/waiting | count | 0 | 22 | 0 |
| preprocess/bind_groups | count | 8 | 515 | 9 |
| preprocess/bind_groups_created | count | 8 | 515 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 125 | 157 | 121 |
| process/cpu_usage | % | 30.29 | 45.02 | 25 |
| process/mem_usage | GiB | 2.58 | 2.78 | 2.78 |
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
| started_load_count |  | 2864 | 2903 | 2903 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 437 | 530 | 398 |
| statics/materials_bound | count | 437 | 530 | 398 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 1 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.02 | 0.06 | 0.02 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 345 | 437 | 261 |
| stream/oldest_request/s | s | 0 | 0.19 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 37.00 | 49.08 | 36.42 |
| system/mem_usage | % | 53.93 | 54.60 | 54.45 |
| terrain/patches | count | 511 | 523 | 514 |
| views/all/batch_sets | count | 56 | 226 | 55 |
| views/all/bins | count | 5130 | 6515 | 4843 |
| views/all/count | count | 28 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 16.3 k | 21.8 k | 10.0 k |
| views/all/pipelines | count | 137 | 237 | 51 |
| views/all/pipelines_distinct | count | 43 | 47 | 40 |
| views/all/work_items | count | 14.9 k | 20.6 k | 10.3 k |
| views/cascade/batch_sets | count | 23 | 35 | 21 |
| views/cascade/bins | count | 1954 | 2442 | 1745 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7120 | 8898 | 4434 |
| views/cascade/pipelines | count | 23 | 35 | 21 |
| views/cascade/work_items | count | 7120 | 8898 | 4434 |
| views/lamp_face/batch_sets | count | 0 | 173 | 0 |
| views/lamp_face/bins | count | 1027 | 2241 | 0 |
| views/lamp_face/count | count | 24 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1216 | 2609 | 0 |
| views/lamp_face/pipelines | count | 84 | 178 | 0 |
| views/lamp_face/work_items | count | 0 | 2327 | 0 |
| views/main/batch_sets | count | 34 | 34 | 34 |
| views/main/bins | count | 2076 | 3210 | 3098 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 7492 | 11.3 k | 5592 |
| views/main/pipelines | count | 30 | 30 | 30 |
| views/main/work_items | count | 7644 | 11.5 k | 5900 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 28.0 k | 33.7 k | 26.9 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| assets/Image/added | count | 0.02 | 18 | 136 |
| assets/Image/modified | count | 0.00 | 1 | 8 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.45 | 160 | 3454 |
| assets/Mesh/removed | count | 0.64 | 1228 | 4892 |
| assets/StandardMaterial/added | count | 0.03 | 26 | 263 |
| assets/StandardMaterial/modified | count | 2.82 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.05 | 60 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 12 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.11 | 1556 | 867 |
| physics/inserts | count | 0.46 | 146 | 3556 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.30 | 145 | 2335 |
| physics/steps | count | 0.51 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 1 | 4 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.03 | 21 | 219 |
| stream/deactivate/items | count | 0.03 | 21 | 223 |
| stream/despawn/items | count | 0.03 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.05 | 24 | 378 |
| stream/upload/bytes | bytes | 4411 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 56 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 967 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.0 ms; slowest 7, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 0.58 | 580.8 | main/busy/ms 560, surface/advance_cpu_ms 68.89, surface/cell_upload_ms 1.03, stream/drive/ms 0.34, weather/map/ms 0.14, stream/tick/ms 0.09, bigspace/total/ms 0.09, bigspace/high_precision/ms 0.06, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.02, script/ui/ms 0.01 |
| warm-up | 0.93 | 354.2 | main/busy/ms 594, render/thread/ms 315, render/busy/ms 240, render/acquire/ms 72.49, surface/advance_cpu_ms 68.89, render/graph/ms 5.56, render/present/ms 2.06, stream/drive/ms 1.24, surface/cell_upload_ms 1.03, stream/tick/ms 0.67, terrain/update/ms 0.42, bigspace/total/ms 0.34 |
| warm-up | 1.00 | 62.3 | render/thread/ms 326, render/busy/ms 250, pipelines/wait_ms 124, pipelines/compile_ms 76.18, render/acquire/ms 72.67, pipelines/front_ms 69.54, main/busy/ms 50.53, terrain/update/ms 18.90, render/graph/ms 9.59, pipelines/driver_ms 6.63, render/present/ms 3.15, bigspace/total/ms 1.71 |
| warm-up | 1.23 | 89.5 | main/busy/ms 59.22, render/thread/ms 46.40, render/busy/ms 44.86, render/graph/ms 4.14, stream/drive/ms 1.41, render/present/ms 1.34, stream/tick/ms 1.17, terrain/update/ms 0.76, terrain/select/ms 0.65, bigspace/total/ms 0.31, render/acquire/ms 0.21, bigspace/high_precision/ms 0.15 |
| warm-up | 1.37 | 27.7 | render/thread/ms 28.55, render/busy/ms 27.19, main/busy/ms 12.53, render/graph/ms 3.65, stream/drive/ms 2.36, render/present/ms 1.17, stream/tick/ms 0.79, terrain/update/ms 0.69, terrain/select/ms 0.57, stream/spawn/ms 0.40, bigspace/total/ms 0.20, render/acquire/ms 0.19 |
| warm-up | 1.41 | 37.5 | render/thread/ms 34.71, render/busy/ms 33.45, main/busy/ms 17.81, stream/drive/ms 3.99, render/graph/ms 3.90, stream/spawn/ms 1.41, render/present/ms 1.09, terrain/update/ms 0.78, stream/tick/ms 0.75, terrain/select/ms 0.62, bigspace/total/ms 0.26, render/acquire/ms 0.17 |
| measure | 59.27 | 30.3 | render/thread/ms 37.19, render/present/ms 19.90, render/busy/ms 17.07, main/busy/ms 14.61, render/graph/ms 9.79, terrain/update/ms 0.56, bigspace/total/ms 0.54, terrain/select/ms 0.50, stream/drive/ms 0.49, stream/tick/ms 0.43, far/select/ms 0.40, bigspace/high_precision/ms 0.22 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\m_actors\balmora_walk@2560x1440-2 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_OFF=actors
- before the run: GPU 2%, CPU 9%
- during the run: other processes 7% CPU (system minus this process)
- nvidia-smi over the window (255 samples): GPU load **96%** (min 89%) · 2571 MHz (min 2565) · memory 8001 MHz · 93.7 W · 79 °C (max 82) · VRAM 4636 MiB · throttled 100% of samples
