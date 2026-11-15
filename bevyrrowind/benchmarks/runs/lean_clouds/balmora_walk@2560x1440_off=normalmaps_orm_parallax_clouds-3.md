# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: clouds,normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5844 frames over 65.6 s · **median 10.94 ms (91 fps)** · mean 11.23 · p95 13.88 · p99 15.92 (1% low 63 fps) · worst 30.47 ms · 43 spikes over 1.5× median · 27.4 stutters/min (work 27.4/min)
- warm-up: 500 frames over 9.7 s, worst 2511.6 ms
- hitches (> 27.3 ms): **3** measured, 31 in warm-up
- timed GPU passes: 7.09 ms of a 11.23 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.94 ms, limit 11 ms
- **FAIL** frame_ms: 30 stutters (> 1.5x moving median of 60) in 1.09 min = 27.42/min, limit 1/min
- pass worst frame 30.47 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.098 | 2.219 | 2.754 | 0.95 | 0.179 |
| main_opaque_pass_3d | 1.486 | 1.572 | 2.228 | 0.95 | 0.057 |
| early prepass | 0.479 | 0.506 | 0.893 | 0.95 | 0.095 |
| bloom | 0.322 | 0.340 | 0.489 | 0.95 | 0.066 |
| atmosphere_luts | 0.314 | 0.332 | 0.471 | 0.95 | 0.003 |
| water_velocity | 0.288 | 0.305 | 0.474 | 0.95 | 0.002 |
| uniform_allocation | 0.214 | 0.226 | 0.231 | 0.95 | 0.039 |
| water_sspr | 0.212 | 0.224 | 0.226 | 0.95 | 0.002 |
| ssao | 0.194 | 0.205 | 0.209 | 0.95 | 0.008 |
| bin_unpacking | 0.174 | 0.184 | 0.188 | 0.95 | 0.027 |
| auxiliary_views | 0.149 | 0.158 | 0.632 | 0.95 | 0.142 |
| atmos_froxel_inject | 0.136 | 0.144 | 0.138 | 0.95 | 0.002 |
| water_fft | 0.135 | 0.143 | 0.141 | 0.95 | 0.003 |
| upscaling | 0.123 | 0.130 | 0.156 | 0.95 | 0.004 |
| clouds_composite | 0.099 | 0.104 | 0.104 | 0.95 | 0.002 |
| tonemapping | 0.087 | 0.092 | 0.091 | 0.95 | 0.003 |
| main_transmissive_pass_3d | 0.085 | 0.090 | 0.168 | 0.95 | 0.008 |
| clustering | 0.065 | 0.068 | 0.070 | 0.95 | 0.021 |
| render_sky | 0.052 | 0.055 | 0.054 | 0.95 | 0.002 |
| clouds_resolve | 0.046 | 0.048 | 0.050 | 0.95 | 0.001 |
| lightprobe_irradiance_map | 0.045 | 0.443 | 0.756 | 0.10 | 0.001 |
| early_mesh_preprocessing | 0.043 | 0.045 | 0.049 | 0.95 | 0.005 |
| surface_trail | 0.041 | 0.060 | 0.057 | 0.69 | 0.002 |
| water_snapshot | 0.035 | 0.037 | 0.046 | 0.95 | 0.002 |
| atmos_froxel_integrate | 0.026 | 0.028 | 0.030 | 0.95 | 0.001 |
| clouds_trace | 0.024 | 0.026 | 0.027 | 0.95 | 0.001 |
| lightprobe_radiance_map | 0.024 | 0.239 | 0.238 | 0.10 | 0.004 |
| water_mips | 0.023 | 0.025 | 0.024 | 0.95 | 0.002 |
| per_view_shadow_pass | 0.020 | 0.021 | 0.022 | 0.95 | 0.007 |
| main_transparent_pass_3d | 0.014 | 0.015 | 0.031 | 0.95 | 0.110 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.95 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.95 | 0.003 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.95 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.95 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.10 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.014 | 0.10 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.10 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 10.049 | 12.495 | 27.98 | 58723.7 |
| render/busy/ms | 9.436 | 11.898 | 27.41 | 55142.6 |
| main/busy/ms | 8.580 | 10.586 | 28.29 | 50144.1 |
| render/graph/ms | 5.301 | 6.943 | 14.60 | 30981.9 |
| pipelines/wait_ms | 3.948 | 0.000 | 8490.35 | 23072.6 |
| render/present/ms | 0.517 | 1.267 | 14.17 | 3023.9 |
| bigspace/total/ms | 0.510 | 1.031 | 3.43 | 2980.2 |
| terrain/update/ms | 0.304 | 0.388 | 2.25 | 1778.2 |
| controller/feed/ms | 0.278 | 0.377 | 0.79 | 1622.8 |
| terrain/select/ms | 0.271 | 0.344 | 0.75 | 1583.4 |
| stream/drive/ms | 0.252 | 0.355 | 20.61 | 1470.4 |
| bigspace/low_precision/ms | 0.213 | 0.561 | 2.98 | 1244.7 |
| stream/tick/ms | 0.196 | 0.277 | 0.88 | 1145.4 |
| far/select/ms | 0.194 | 0.273 | 0.66 | 1134.2 |
| anim/evaluate_cpu/ms | 0.170 | 0.215 | 0.56 | 995.2 |
| pipelines/compile_ms | 0.169 | 0.000 | 152.07 | 989.3 |
| pipelines/front_ms | 0.164 | 0.000 | 147.05 | 957.6 |
| controller/move/ms | 0.136 | 0.251 | 0.67 | 795.2 |
| bigspace/high_precision/ms | 0.129 | 0.300 | 3.08 | 754.7 |
| anim/evaluate/ms | 0.126 | 0.246 | 2.11 | 738.6 |
| render/acquire/ms | 0.095 | 0.126 | 1.37 | 557.2 |
| physics/step/ms | 0.050 | 0.090 | 0.55 | 291.2 |
| physics/insert/ms | 0.036 | 0.377 | 0.98 | 208.6 |
| weather/map/ms | 0.025 | 0.034 | 0.29 | 145.1 |
| anim/apply/ms | 0.012 | 0.022 | 0.14 | 69.7 |
| controller/avoid/ms | 0.011 | 0.014 | 0.19 | 61.9 |
| weather/eval/ms | 0.010 | 0.015 | 0.13 | 60.4 |
| script/update/ms | 0.007 | 0.010 | 0.26 | 41.9 |
| pipelines/driver_ms | 0.005 | 0.000 | 5.02 | 31.7 |
| stream/despawn/ms | 0.005 | 0.000 | 0.98 | 29.6 |
| script/ui/ms | 0.005 | 0.007 | 0.29 | 29.5 |
| script/input/ms | 0.005 | 0.006 | 0.23 | 28.3 |
| stream/spawn/ms | 0.004 | 0.000 | 1.08 | 25.8 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 19.0 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.59 | 14.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.44 | 5.3 |
| shapes/activate/ms | 0.001 | 0.001 | 0.17 | 4.7 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 1.7 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 0.9 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 26 | 43 | 11 |
| crowd/c0/joints/count | count | 1301 | 2262 | 492 |
| crowd/c0/meshes/count | count | 353 | 646 | 113 |
| crowd/c1/count | count | 146 | 155 | 150 |
| crowd/c1/joints/count | count | 5570 | 5949 | 5704 |
| crowd/c1/meshes/count | count | 1181 | 1281 | 1228 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4259 | 9879 | 6179 |
| crowd/joints_uploaded/count | count | 4239 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.2 k | 68.2 k | 59.2 k |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.20 M | 205.20 M | 205.20 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.22 | 28.29 | 9.10 |
| gpu/timed/ms | ms | 8.18 | 10.21 | 8.40 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 551.12 M | 552.30 M | 552.30 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 818 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 818 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1297 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3428 | 5124 | 3171 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1297 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3428 | 5124 | 3171 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 50 | 290 | 39 |
| phases/shadow/bins | count | 3782 | 5417 | 2212 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9593 | 12.9 k | 5663 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 132 | 301 | 39 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 187 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 199 | 203 | 203 |
| pipelines/waiting | count | 0 | 6 | 0 |
| preprocess/bind_groups | count | 10 | 749 | 6 |
| preprocess/bind_groups_created | count | 10 | 749 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 224 | 335 | 215 |
| process/cpu_usage | % | 25.47 | 41.35 | 31.28 |
| process/mem_usage | GiB | 5.14 | 5.43 | 5.42 |
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
| started_load_count |  | 4765 | 4859 | 4859 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.03 | 0.08 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.28 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.87 | 52.13 | 33.83 |
| system/mem_usage | % | 58.76 | 59.80 | 59.68 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 108 | 342 | 99 |
| views/all/bins | count | 6497 | 8301 | 5692 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.3 k | 25.4 k | 13.2 k |
| views/all/pipelines | count | 187 | 355 | 95 |
| views/all/pipelines_distinct | count | 81 | 93 | 77 |
| views/all/work_items | count | 17.4 k | 23.8 k | 13.5 k |
| views/cascade/batch_sets | count | 48 | 60 | 39 |
| views/cascade/bins | count | 2600 | 3123 | 2212 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8151 | 10.2 k | 5663 |
| views/cascade/pipelines | count | 48 | 60 | 39 |
| views/cascade/work_items | count | 8151 | 10.2 k | 5663 |
| views/lamp_face/batch_sets | count | 0 | 238 | 0 |
| views/lamp_face/bins | count | 1106 | 2452 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1232 | 2817 | 0 |
| views/lamp_face/pipelines | count | 83 | 241 | 0 |
| views/lamp_face/work_items | count | 0 | 2590 | 0 |
| views/main/batch_sets | count | 60 | 62 | 60 |
| views/main/bins | count | 2888 | 3562 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8475 | 13.0 k | 7576 |
| views/main/pipelines | count | 56 | 58 | 56 |
| views/main/work_items | count | 8638 | 13.2 k | 7884 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 60.85 | 319 | 355.6 k |
| anim/events/count | count | 0.10 | 3 | 574 |
| assets/Image/added | count | 0.10 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.70 | 152 | 4075 |
| assets/Mesh/removed | count | 0.83 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 466 |
| assets/StandardMaterial/modified | count | 3.73 | 29 | 21.8 k |
| assets/StandardMaterial/removed | count | 0.07 | 32 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.81 | 1389 | 4719 |
| physics/inserts | count | 0.40 | 104 | 2318 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.40 | 145 | 2335 |
| physics/steps | count | 0.67 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.01 | 2 | 83 |
| stream/spawn/items | count | 0.06 | 24 | 378 |
| stream/upload/bytes | bytes | 5793 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1270 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 27.3 ms; slowest 34, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2511.6 | main/busy/ms 2486, surface/advance_cpu_ms 68.31, surface/cell_upload_ms 0.94, stream/drive/ms 0.30, weather/map/ms 0.15, bigspace/total/ms 0.11, stream/tick/ms 0.07, bigspace/high_precision/ms 0.06, weather/eval/ms 0.03, script/update/ms 0.02, anim/evaluate/ms 0.01, far/select/ms 0.01 |
| warm-up | 3.20 | 689.6 | main/busy/ms 2522, render/thread/ms 657, render/busy/ms 536, render/acquire/ms 119, surface/advance_cpu_ms 68.31, render/graph/ms 6.24, render/present/ms 2.00, stream/drive/ms 1.08, surface/cell_upload_ms 0.94, stream/tick/ms 0.53, terrain/update/ms 0.50, terrain/select/ms 0.37 |
| warm-up | 3.27 | 70.3 | render/thread/ms 669, render/busy/ms 546, pipelines/wait_ms 524, render/acquire/ms 119, pipelines/compile_ms 76.06, pipelines/front_ms 69.81, main/busy/ms 50.53, terrain/update/ms 18.65, render/graph/ms 10.19, pipelines/driver_ms 6.25, render/present/ms 3.23, stream/drive/ms 1.38 |
| warm-up | 3.50 | 95.8 | render/thread/ms 67.16, main/busy/ms 66.86, render/busy/ms 64.86, render/graph/ms 4.71, render/present/ms 2.02, stream/drive/ms 1.94, stream/tick/ms 1.45, terrain/update/ms 0.76, terrain/select/ms 0.64, bigspace/total/ms 0.36, render/acquire/ms 0.29, bigspace/high_precision/ms 0.15 |
| warm-up | 3.56 | 35.4 | pipelines/compile_ms 301, pipelines/front_ms 281, pipelines/wait_ms 227, render/thread/ms 46.52, render/busy/ms 44.55, main/busy/ms 27.06, pipelines/driver_ms 20.20, render/graph/ms 5.64, stream/drive/ms 4.24, stream/tick/ms 2.11, render/present/ms 1.47, stream/spawn/ms 1.21 |
| warm-up | 3.68 | 66.1 | render/thread/ms 47.38, render/busy/ms 46.13, main/busy/ms 42.94, stream/drive/ms 28.67, render/graph/ms 2.72, render/present/ms 1.05, stream/spawn/ms 0.95, terrain/update/ms 0.93, stream/tick/ms 0.90, terrain/select/ms 0.77, bigspace/total/ms 0.34, render/acquire/ms 0.20 |
| warm-up | 3.74 | 36.3 | pipelines/wait_ms 707, pipelines/compile_ms 180, pipelines/front_ms 163, main/busy/ms 44.45, stream/drive/ms 30.61, render/thread/ms 30.60, render/busy/ms 28.97, pipelines/driver_ms 17.35, render/graph/ms 4.22, render/present/ms 1.40, stream/spawn/ms 1.34, stream/tick/ms 0.86 |
| warm-up | 3.77 | 32.6 | pipelines/wait_ms 707, pipelines/compile_ms 180, pipelines/front_ms 163, render/thread/ms 43.70, render/busy/ms 41.65, main/busy/ms 25.71, pipelines/driver_ms 17.35, stream/drive/ms 9.89, render/graph/ms 5.36, render/present/ms 1.77, stream/tick/ms 1.43, stream/spawn/ms 1.43 |
| warm-up | 3.80 | 28.4 | pipelines/wait_ms 3957, pipelines/compile_ms 95.47, pipelines/front_ms 90.52, render/thread/ms 46.19, render/busy/ms 43.87, main/busy/ms 23.75, render/graph/ms 5.32, pipelines/driver_ms 4.94, stream/drive/ms 4.63, render/present/ms 2.01, stream/tick/ms 1.68, stream/spawn/ms 0.99 |
| warm-up | 3.87 | 65.3 | pipelines/wait_ms 8331, pipelines/compile_ms 160, pipelines/front_ms 150, render/thread/ms 59.76, main/busy/ms 57.99, render/busy/ms 57.78, stream/drive/ms 34.71, pipelines/driver_ms 9.98, render/graph/ms 6.02, render/present/ms 1.74, stream/tick/ms 1.66, terrain/update/ms 0.95 |
| warm-up | 3.95 | 83.4 | pipelines/wait_ms 4374, render/thread/ms 98.92, render/busy/ms 78.03, pipelines/compile_ms 64.18, pipelines/front_ms 59.14, main/busy/ms 54.00, stream/drive/ms 33.71, render/present/ms 20.68, render/graph/ms 5.05, pipelines/driver_ms 5.04, terrain/update/ms 4.90, stream/tick/ms 1.31 |
| warm-up | 3.99 | 28.5 | pipelines/wait_ms 1196, pipelines/compile_ms 135, pipelines/front_ms 130, render/thread/ms 35.21, render/busy/ms 33.64, main/busy/ms 27.17, render/graph/ms 6.58, pipelines/driver_ms 4.78, stream/drive/ms 3.53, stream/spawn/ms 1.36, render/present/ms 1.24, terrain/update/ms 0.90 |
| warm-up | 4.11 | 41.7 | pipelines/wait_ms 5196, main/busy/ms 78.17, pipelines/compile_ms 77.09, pipelines/front_ms 71.62, stream/drive/ms 58.29, render/thread/ms 55.62, render/busy/ms 52.84, render/graph/ms 18.69, pipelines/driver_ms 5.47, stream/spawn/ms 5.19, render/present/ms 2.49, stream/tick/ms 1.40 |
| warm-up | 4.19 | 83.9 | pipelines/wait_ms 11.9 k, pipelines/compile_ms 139, pipelines/front_ms 129, main/busy/ms 79.47, render/thread/ms 72.61, render/busy/ms 70.49, stream/drive/ms 45.57, render/graph/ms 12.68, pipelines/driver_ms 10.32, terrain/update/ms 3.66, stream/spawn/ms 2.19, stream/tick/ms 1.74 |
| warm-up | 4.31 | 97.5 | pipelines/wait_ms 7328, pipelines/compile_ms 128, pipelines/front_ms 116, main/busy/ms 115, stream/drive/ms 74.51, render/thread/ms 49.51, render/busy/ms 47.39, pipelines/driver_ms 11.93, render/graph/ms 8.17, stream/spawn/ms 2.97, terrain/update/ms 2.63, render/present/ms 1.77 |
| warm-up | 4.35 | 32.8 | pipelines/wait_ms 4087, pipelines/compile_ms 123, pipelines/front_ms 114, main/busy/ms 64.94, render/thread/ms 47.76, render/busy/ms 45.45, stream/drive/ms 36.79, pipelines/driver_ms 9.48, render/graph/ms 7.80, stream/spawn/ms 2.55, render/present/ms 2.04, physics/insert/ms 0.99 |
| warm-up | 4.38 | 29.5 | pipelines/wait_ms 1019, main/busy/ms 55.59, stream/drive/ms 35.49, render/thread/ms 32.88, render/busy/ms 31.12, pipelines/compile_ms 28.61, pipelines/front_ms 27.20, render/graph/ms 4.56, stream/spawn/ms 2.28, render/present/ms 1.53, pipelines/driver_ms 1.41, stream/activate/ms 1.06 |
| warm-up | 4.50 | 27.8 | pipelines/wait_ms 3843, pipelines/compile_ms 178, pipelines/front_ms 166, render/thread/ms 33.76, render/busy/ms 31.77, main/busy/ms 26.48, pipelines/driver_ms 12.43, render/graph/ms 6.27, stream/drive/ms 5.44, stream/spawn/ms 2.88, render/present/ms 1.62, stream/tick/ms 1.26 |
| warm-up | 4.53 | 27.7 | pipelines/wait_ms 3067, pipelines/compile_ms 42.58, pipelines/front_ms 40.50, render/thread/ms 39.54, render/busy/ms 37.96, main/busy/ms 26.73, stream/drive/ms 4.70, render/graph/ms 4.30, stream/spawn/ms 2.46, pipelines/driver_ms 2.07, physics/insert/ms 1.33, stream/tick/ms 1.28 |
| warm-up | 4.57 | 30.2 | render/thread/ms 37.38, render/busy/ms 36.26, main/busy/ms 17.87, render/graph/ms 3.34, stream/drive/ms 1.95, render/present/ms 0.93, terrain/update/ms 0.87, physics/insert/ms 0.79, terrain/select/ms 0.70, stream/tick/ms 0.69, stream/spawn/ms 0.68, bigspace/total/ms 0.35 |
| warm-up | 5.16 | 27.6 | pipelines/wait_ms 7663, pipelines/compile_ms 182, pipelines/front_ms 174, render/thread/ms 48.09, render/busy/ms 46.45, main/busy/ms 26.05, pipelines/driver_ms 7.44, render/graph/ms 5.73, stream/drive/ms 4.44, stream/spawn/ms 2.30, render/present/ms 1.34, stream/tick/ms 1.20 |
| warm-up | 5.19 | 37.3 | pipelines/wait_ms 5821, pipelines/compile_ms 180, pipelines/front_ms 173, render/thread/ms 50.88, render/busy/ms 49.54, main/busy/ms 26.35, pipelines/driver_ms 7.08, render/graph/ms 5.08, stream/drive/ms 4.63, stream/spawn/ms 2.16, stream/tick/ms 1.40, render/present/ms 1.10 |
| warm-up | 5.33 | 31.1 | pipelines/wait_ms 4458, pipelines/compile_ms 180, pipelines/front_ms 173, render/thread/ms 45.83, render/busy/ms 44.11, main/busy/ms 32.59, stream/drive/ms 9.73, render/graph/ms 8.15, pipelines/driver_ms 6.86, render/present/ms 1.43, stream/spawn/ms 1.14, stream/tick/ms 1.01 |
| warm-up | 5.53 | 30.5 | main/busy/ms 35.94, render/thread/ms 25.95, render/busy/ms 24.70, stream/drive/ms 14.37, render/graph/ms 7.39, stream/spawn/ms 2.91, render/present/ms 1.00, stream/tick/ms 0.89, bigspace/total/ms 0.78, terrain/update/ms 0.77, terrain/select/ms 0.66, bigspace/high_precision/ms 0.37 |
| warm-up | 5.73 | 29.0 | pipelines/wait_ms 2376, pipelines/compile_ms 64.05, pipelines/front_ms 62.37, render/thread/ms 31.88, render/busy/ms 30.58, main/busy/ms 26.05, render/graph/ms 5.23, stream/drive/ms 3.48, pipelines/driver_ms 1.68, stream/spawn/ms 1.30, stream/tick/ms 1.21, render/present/ms 1.09 |
| warm-up | 6.21 | 29.6 | main/busy/ms 43.93, render/thread/ms 30.76, render/busy/ms 29.22, render/graph/ms 8.87, render/present/ms 1.26, stream/drive/ms 1.22, terrain/update/ms 0.61, bigspace/total/ms 0.61, physics/insert/ms 0.60, stream/tick/ms 0.59, terrain/select/ms 0.54, anim/evaluate_cpu/ms 0.40 |
| warm-up | 6.24 | 29.3 | main/busy/ms 39.84, render/thread/ms 34.96, render/busy/ms 33.62, render/graph/ms 5.73, stream/drive/ms 1.35, render/present/ms 1.06, stream/tick/ms 0.60, terrain/update/ms 0.59, physics/insert/ms 0.59, bigspace/total/ms 0.59, terrain/select/ms 0.52, anim/evaluate_cpu/ms 0.44 |
| warm-up | 6.37 | 28.6 | pipelines/wait_ms 7256, pipelines/compile_ms 46.28, pipelines/front_ms 44.12, main/busy/ms 37.70, render/thread/ms 30.01, render/busy/ms 28.53, render/graph/ms 6.20, pipelines/driver_ms 2.16, stream/drive/ms 1.19, render/present/ms 1.18, anim/evaluate_cpu/ms 1.04, bigspace/total/ms 0.83 |
| warm-up | 7.54 | 66.7 | render/thread/ms 72.42, render/present/ms 53.00, main/busy/ms 22.42, render/busy/ms 19.21, render/graph/ms 8.58, physics/insert/ms 1.38, stream/drive/ms 1.38, bigspace/total/ms 1.01, stream/tick/ms 0.70, terrain/update/ms 0.56, terrain/select/ms 0.49, far/select/ms 0.43 |
| warm-up | 7.58 | 28.5 | main/busy/ms 32.40, render/thread/ms 29.58, render/busy/ms 28.10, render/graph/ms 7.74, physics/insert/ms 1.49, render/present/ms 1.30, stream/drive/ms 1.01, bigspace/total/ms 0.92, physics/step/ms 0.83, far/select/ms 0.81, terrain/update/ms 0.56, terrain/select/ms 0.48 |
| warm-up | 8.30 | 30.6 | pipelines/wait_ms 4103, pipelines/compile_ms 44.28, pipelines/front_ms 42.06, render/thread/ms 31.06, render/busy/ms 29.65, main/busy/ms 29.14, render/graph/ms 10.52, pipelines/driver_ms 2.23, render/present/ms 1.19, physics/insert/ms 0.96, bigspace/total/ms 0.90, terrain/update/ms 0.63 |
| measure | 2.72 | 29.0 | render/thread/ms 36.60, render/busy/ms 35.54, main/busy/ms 16.82, render/graph/ms 15.90, render/present/ms 0.88, bigspace/total/ms 0.84, terrain/update/ms 0.63, physics/insert/ms 0.58, terrain/select/ms 0.57, controller/feed/ms 0.47, stream/drive/ms 0.42, stream/tick/ms 0.37 |
| measure | 18.17 | 30.5 | main/busy/ms 37.44, stream/drive/ms 20.84, render/thread/ms 19.49, render/busy/ms 18.39, render/graph/ms 9.66, render/present/ms 0.94, bigspace/total/ms 0.70, terrain/update/ms 0.58, controller/feed/ms 0.56, terrain/select/ms 0.53, far/select/ms 0.44, stream/tick/ms 0.34 |
| measure | 59.03 | 29.8 | render/thread/ms 39.64, render/busy/ms 38.29, main/busy/ms 29.74, render/graph/ms 10.14, bigspace/total/ms 2.96, bigspace/high_precision/ms 1.84, render/present/ms 1.12, stream/drive/ms 1.04, controller/feed/ms 1.00, stream/tick/ms 0.93, terrain/update/ms 0.75, far/select/ms 0.69 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\bevyrrowind.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\lean_clouds\balmora_walk@2560x1440_off=normalmaps_orm_parallax_clouds-3 --bench-baseline none --off normalmaps,orm,parallax,clouds
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **86%** (min 52%) · 2580 MHz (min 2580) · memory 8001 MHz · 82.3 W · 78 °C (max 83) · VRAM 6287 MiB · throttled 100% of samples
