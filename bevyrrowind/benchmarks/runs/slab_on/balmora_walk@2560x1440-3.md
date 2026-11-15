# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4995 frames over 65.6 s · **median 12.70 ms (79 fps)** · mean 13.14 · p95 15.98 · p99 18.09 (1% low 55 fps) · worst 33.73 ms · 31 spikes over 1.5× median · 17.4 stutters/min (work 17.4/min)
- warm-up: 474 frames over 9.8 s, worst 2485.8 ms
- hitches (> 31.8 ms): **2** measured, 17 in warm-up
- timed GPU passes: 7.95 ms of a 13.14 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.70 ms, limit 11 ms
- **FAIL** frame_ms: 19 stutters (> 1.5x moving median of 60) in 1.09 min = 17.36/min, limit 1/min
- **FAIL** worst frame 33.73 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.206 | 2.230 | 2.785 | 0.99 | 0.179 |
| main_opaque_pass_3d | 1.744 | 1.763 | 2.546 | 0.99 | 0.070 |
| early prepass | 0.541 | 0.547 | 1.012 | 0.99 | 0.105 |
| atmosphere_luts | 0.337 | 0.341 | 0.628 | 0.99 | 0.004 |
| bloom | 0.336 | 0.340 | 0.341 | 0.99 | 0.064 |
| water_velocity | 0.305 | 0.308 | 0.559 | 0.99 | 0.002 |
| uniform_allocation | 0.269 | 0.271 | 0.269 | 0.99 | 0.050 |
| water_sspr | 0.222 | 0.225 | 0.221 | 0.99 | 0.002 |
| bin_unpacking | 0.205 | 0.207 | 0.213 | 0.99 | 0.033 |
| ssao | 0.205 | 0.207 | 0.209 | 0.99 | 0.008 |
| auxiliary_views | 0.179 | 0.181 | 0.682 | 0.99 | 0.163 |
| atmos_froxel_inject | 0.158 | 0.159 | 0.153 | 0.99 | 0.002 |
| clouds_trace | 0.140 | 0.141 | 0.230 | 0.99 | 0.001 |
| water_fft | 0.139 | 0.140 | 0.138 | 0.99 | 0.003 |
| upscaling | 0.131 | 0.132 | 0.160 | 0.99 | 0.004 |
| clouds_composite | 0.111 | 0.112 | 0.111 | 0.99 | 0.002 |
| tonemapping | 0.094 | 0.095 | 0.092 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.091 | 0.091 | 0.168 | 0.99 | 0.008 |
| clustering | 0.066 | 0.066 | 0.069 | 0.99 | 0.020 |
| render_sky | 0.054 | 0.055 | 0.054 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.053 | 0.453 | 0.757 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| surface_trail | 0.048 | 0.059 | 0.057 | 0.82 | 0.002 |
| early_mesh_preprocessing | 0.044 | 0.044 | 0.048 | 0.99 | 0.006 |
| water_snapshot | 0.039 | 0.039 | 0.045 | 0.99 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.243 | 0.242 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.030 | 0.99 | 0.001 |
| water_mips | 0.024 | 0.024 | 0.024 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 0.99 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.015 | 0.032 | 0.99 | 0.110 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.010 | 0.99 | 0.002 |
| sparse buffer updates | 0.006 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.99 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 11.955 | 14.652 | 32.50 | 59713.9 |
| render/busy/ms | 11.363 | 14.031 | 31.74 | 56757.4 |
| pipelines/wait_ms | 10.051 | 0.000 | 10274.81 | 50202.4 |
| main/busy/ms | 9.233 | 11.874 | 26.67 | 46119.6 |
| render/graph/ms | 6.938 | 8.697 | 16.15 | 34655.8 |
| render/present/ms | 0.501 | 0.600 | 12.91 | 2501.8 |
| bigspace/total/ms | 0.460 | 0.708 | 4.41 | 2296.4 |
| pipelines/compile_ms | 0.411 | 0.000 | 178.59 | 2053.9 |
| pipelines/front_ms | 0.397 | 0.000 | 172.69 | 1981.9 |
| terrain/update/ms | 0.295 | 0.378 | 2.11 | 1474.6 |
| controller/feed/ms | 0.272 | 0.362 | 0.80 | 1357.2 |
| terrain/select/ms | 0.262 | 0.335 | 0.74 | 1309.4 |
| stream/drive/ms | 0.256 | 0.362 | 17.39 | 1279.2 |
| stream/tick/ms | 0.196 | 0.270 | 0.62 | 979.1 |
| far/select/ms | 0.195 | 0.273 | 0.63 | 972.1 |
| bigspace/low_precision/ms | 0.182 | 0.295 | 3.48 | 909.1 |
| anim/evaluate_cpu/ms | 0.171 | 0.217 | 0.50 | 852.6 |
| controller/move/ms | 0.139 | 0.256 | 0.63 | 696.1 |
| anim/evaluate/ms | 0.133 | 0.269 | 2.87 | 664.0 |
| bigspace/high_precision/ms | 0.111 | 0.220 | 3.96 | 552.0 |
| render/acquire/ms | 0.091 | 0.120 | 0.41 | 454.8 |
| physics/step/ms | 0.059 | 0.091 | 0.58 | 292.5 |
| physics/insert/ms | 0.057 | 0.454 | 0.95 | 287.2 |
| weather/map/ms | 0.024 | 0.032 | 0.41 | 118.7 |
| pipelines/driver_ms | 0.014 | 0.000 | 5.90 | 72.0 |
| anim/apply/ms | 0.012 | 0.021 | 0.21 | 59.2 |
| controller/avoid/ms | 0.011 | 0.014 | 0.25 | 54.0 |
| weather/eval/ms | 0.010 | 0.014 | 0.19 | 51.6 |
| script/update/ms | 0.007 | 0.009 | 0.17 | 36.3 |
| stream/despawn/ms | 0.006 | 0.000 | 1.08 | 29.4 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 27.1 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 26.6 |
| script/ui/ms | 0.005 | 0.006 | 0.04 | 22.7 |
| controller/write/ms | 0.004 | 0.004 | 0.16 | 17.7 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.62 | 14.4 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.4 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 4.8 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 3.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.4 |
| stream/upload/ms | 0.000 | 0.000 | 0.22 | 1.1 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 10 | 0 |
| anim/a2/count | count | 1 | 13 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 18 | 40 | 18 |
| controller/movers/count | count | 19 | 22 | 18 |
| crowd/c0/count | count | 26 | 45 | 14 |
| crowd/c0/joints/count | count | 1285 | 2262 | 641 |
| crowd/c0/meshes/count | count | 353 | 646 | 174 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5534 | 5991 | 5555 |
| crowd/c1/meshes/count | count | 1169 | 1295 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4249 | 6643 | 6179 |
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
| frame/work/ms | ms | 10.98 | 31.74 | 9.78 |
| gpu/timed/ms | ms | 8.64 | 12.39 | 9.68 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
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
| phases/alpha_mask3d/items | count | 811 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 811 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d/bins | count | 1286 | 1393 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3399 | 5121 | 3179 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 27 | 30 | 28 |
| phases/opaque3d_prepass/bins | count | 1286 | 1393 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3399 | 5121 | 3179 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 27 | 29 | 27 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 62 | 293 | 47 |
| phases/shadow/bins | count | 3780 | 5363 | 2236 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9592 | 12.4 k | 5687 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 142 | 308 | 47 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2642 | 3268 | 2347 |
| physics/items | count | 185 | 476 | 19 |
| physics/pending | count | 0 | 571 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 225 | 231 | 231 |
| pipelines/waiting | count | 0 | 16 | 0 |
| preprocess/bind_groups | count | 12 | 779 | 6 |
| preprocess/bind_groups_created | count | 12 | 779 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 261 | 375 | 251 |
| process/cpu_usage | % | 23.08 | 42.65 | 22.62 |
| process/mem_usage | GiB | 5.19 | 5.36 | 5.36 |
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
| started_load_count |  | 4766 | 4860 | 4860 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.34 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 29.31 | 46.92 | 28.75 |
| system/mem_usage | % | 63.71 | 64.46 | 64.46 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2591 | 2644 | 2644 |
| views/all/batch_sets | count | 129 | 357 | 117 |
| views/all/bins | count | 6480 | 8202 | 5716 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.1 k | 25.1 k | 13.3 k |
| views/all/pipelines | count | 206 | 374 | 113 |
| views/all/pipelines_distinct | count | 97 | 108 | 91 |
| views/all/work_items | count | 17.4 k | 23.8 k | 13.6 k |
| views/cascade/batch_sets | count | 57 | 71 | 47 |
| views/cascade/bins | count | 2608 | 3157 | 2236 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8161 | 10.0 k | 5687 |
| views/cascade/pipelines | count | 57 | 71 | 47 |
| views/cascade/work_items | count | 8161 | 10.0 k | 5687 |
| views/lamp_face/batch_sets | count | 0 | 233 | 0 |
| views/lamp_face/bins | count | 1079 | 2431 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1218 | 2799 | 0 |
| views/lamp_face/pipelines | count | 84 | 245 | 0 |
| views/lamp_face/work_items | count | 0 | 2536 | 0 |
| views/main/batch_sets | count | 70 | 74 | 70 |
| views/main/bins | count | 2878 | 3580 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8416 | 13.0 k | 7592 |
| views/main/pipelines | count | 66 | 70 | 66 |
| views/main/work_items | count | 8579 | 13.2 k | 7900 |
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
| anim/bones_written/count | count | 60.82 | 559 | 303.8 k |
| anim/events/count | count | 0.11 | 3 | 529 |
| assets/Image/added | count | 0.12 | 43 | 600 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.82 | 152 | 4102 |
| assets/Mesh/removed | count | 0.98 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 472 |
| assets/StandardMaterial/modified | count | 4.34 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.31 | 1562 | 6557 |
| physics/inserts | count | 0.57 | 106 | 2823 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.47 | 145 | 2335 |
| physics/steps | count | 0.79 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 2 |
| pipelines/created | count | 0.00 | 2 | 18 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.08 | 17 | 378 |
| stream/upload/bytes | bytes | 6777 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1486 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.8 ms; slowest 19, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2485.8 | main/busy/ms 2460, surface/advance_cpu_ms 66.78, surface/cell_upload_ms 0.86, stream/drive/ms 0.36, stream/tick/ms 0.10, weather/map/ms 0.09, bigspace/total/ms 0.08, bigspace/high_precision/ms 0.05, script/update/ms 0.03, weather/eval/ms 0.02, anim/evaluate/ms 0.01, script/input/ms 0.01 |
| warm-up | 3.03 | 541.1 | main/busy/ms 2494, render/thread/ms 506, render/busy/ms 376, render/acquire/ms 128, surface/advance_cpu_ms 66.78, render/graph/ms 5.91, render/present/ms 2.02, stream/drive/ms 1.15, surface/cell_upload_ms 0.86, stream/tick/ms 0.61, terrain/update/ms 0.44, terrain/select/ms 0.28 |
| warm-up | 3.25 | 223.5 | pipelines/wait_ms 985, render/thread/ms 518, render/busy/ms 386, pipelines/compile_ms 238, pipelines/front_ms 223, render/acquire/ms 129, main/busy/ms 49.88, terrain/update/ms 18.72, pipelines/driver_ms 15.02, render/graph/ms 9.57, render/present/ms 2.90, stream/drive/ms 1.46 |
| warm-up | 3.49 | 94.8 | pipelines/compile_ms 232, pipelines/front_ms 218, render/thread/ms 64.93, render/busy/ms 63.13, main/busy/ms 60.48, pipelines/driver_ms 14.36, render/graph/ms 3.93, stream/drive/ms 3.89, stream/tick/ms 2.15, render/present/ms 1.48, terrain/update/ms 0.70, terrain/select/ms 0.58 |
| warm-up | 3.64 | 51.4 | render/thread/ms 38.05, main/busy/ms 37.56, render/busy/ms 34.49, stream/drive/ms 26.20, render/graph/ms 3.37, render/present/ms 3.37, stream/tick/ms 0.74, terrain/update/ms 0.69, terrain/select/ms 0.54, stream/spawn/ms 0.40, bigspace/total/ms 0.26, render/acquire/ms 0.19 |
| warm-up | 3.81 | 110.3 | render/thread/ms 77.88, render/busy/ms 73.74, main/busy/ms 55.16, stream/drive/ms 32.16, render/present/ms 3.85, render/graph/ms 3.79, stream/tick/ms 3.02, stream/spawn/ms 1.47, terrain/update/ms 0.85, terrain/select/ms 0.69, stream/activate/ms 0.39, bigspace/total/ms 0.36 |
| warm-up | 3.85 | 38.5 | render/thread/ms 81.51, render/busy/ms 77.55, main/busy/ms 51.86, stream/drive/ms 28.76, render/present/ms 3.76, render/graph/ms 3.19, stream/spawn/ms 2.48, stream/tick/ms 2.38, terrain/update/ms 0.88, terrain/select/ms 0.73, bigspace/total/ms 0.42, stream/activate/ms 0.35 |
| warm-up | 3.92 | 69.3 | pipelines/wait_ms 4146, pipelines/compile_ms 235, pipelines/front_ms 228, render/thread/ms 58.72, render/busy/ms 57.47, main/busy/ms 49.78, stream/drive/ms 26.53, pipelines/driver_ms 6.78, render/graph/ms 3.84, stream/spawn/ms 2.21, stream/tick/ms 1.35, render/present/ms 1.07 |
| warm-up | 4.08 | 38.8 | pipelines/wait_ms 8011, pipelines/compile_ms 191, pipelines/front_ms 171, main/busy/ms 63.93, stream/drive/ms 48.27, render/thread/ms 38.85, render/busy/ms 37.27, pipelines/driver_ms 19.32, render/graph/ms 4.40, stream/tick/ms 1.50, stream/spawn/ms 1.39, render/present/ms 1.25 |
| warm-up | 4.15 | 65.4 | pipelines/wait_ms 11.0 k, pipelines/compile_ms 253, pipelines/front_ms 230, main/busy/ms 67.83, stream/drive/ms 48.89, render/thread/ms 44.46, render/busy/ms 43.36, pipelines/driver_ms 22.77, render/graph/ms 3.26, stream/spawn/ms 2.31, terrain/update/ms 0.93, render/present/ms 0.92 |
| warm-up | 4.23 | 71.4 | pipelines/wait_ms 14.5 k, main/busy/ms 64.02, pipelines/compile_ms 60.90, pipelines/front_ms 51.21, stream/drive/ms 45.61, render/thread/ms 32.65, render/busy/ms 31.46, pipelines/driver_ms 9.69, render/graph/ms 4.31, stream/spawn/ms 2.13, render/present/ms 0.97, stream/tick/ms 0.90 |
| warm-up | 4.32 | 66.7 | main/busy/ms 56.95, render/thread/ms 45.38, render/busy/ms 43.35, stream/drive/ms 38.02, render/graph/ms 9.80, terrain/update/ms 5.02, stream/spawn/ms 2.19, render/present/ms 1.80, stream/tick/ms 1.31, terrain/select/ms 0.81, stream/activate/ms 0.60, physics/insert/ms 0.50 |
| warm-up | 5.28 | 36.3 | render/thread/ms 47.03, render/busy/ms 45.54, main/busy/ms 38.04, stream/drive/ms 16.79, render/graph/ms 6.11, stream/spawn/ms 2.47, stream/tick/ms 1.28, render/present/ms 1.12, bigspace/total/ms 0.84, terrain/update/ms 0.80, terrain/select/ms 0.65, render/acquire/ms 0.37 |
| warm-up | 5.43 | 51.5 | render/thread/ms 51.30, render/busy/ms 49.84, main/busy/ms 46.79, render/graph/ms 9.03, stream/drive/ms 4.19, terrain/update/ms 3.33, stream/spawn/ms 2.27, render/present/ms 1.28, physics/insert/ms 1.15, bigspace/total/ms 0.91, stream/tick/ms 0.78, terrain/select/ms 0.76 |
| warm-up | 5.51 | 34.2 | pipelines/wait_ms 1422, pipelines/compile_ms 138, pipelines/front_ms 134, main/busy/ms 40.43, render/thread/ms 29.59, render/busy/ms 27.89, stream/drive/ms 15.00, render/graph/ms 7.94, pipelines/driver_ms 4.25, stream/spawn/ms 2.99, render/present/ms 1.43, stream/tick/ms 1.29 |
| warm-up | 7.79 | 36.7 | pipelines/wait_ms 7621, main/busy/ms 57.44, render/thread/ms 40.07, pipelines/compile_ms 39.61, render/busy/ms 38.38, pipelines/front_ms 36.79, render/graph/ms 14.91, pipelines/driver_ms 2.81, stream/drive/ms 1.49, render/present/ms 1.21, anim/evaluate_cpu/ms 0.94, bigspace/total/ms 0.91 |
| warm-up | 7.84 | 47.7 | pipelines/wait_ms 6436, main/busy/ms 61.47, render/thread/ms 49.89, render/busy/ms 48.44, pipelines/compile_ms 48.35, pipelines/front_ms 45.86, render/graph/ms 11.86, pipelines/driver_ms 2.49, stream/drive/ms 1.36, physics/insert/ms 1.09, render/present/ms 1.01, anim/evaluate_cpu/ms 0.99 |
| measure | 0.79 | 32.2 | main/busy/ms 38.09, render/thread/ms 31.70, render/busy/ms 30.62, stream/drive/ms 15.53, render/graph/ms 14.66, physics/insert/ms 1.12, render/present/ms 0.92, bigspace/total/ms 0.83, physics/step/ms 0.69, terrain/update/ms 0.57, controller/feed/ms 0.54, terrain/select/ms 0.51 |
| measure | 2.73 | 33.7 | render/thread/ms 44.71, render/busy/ms 43.43, render/graph/ms 22.39, main/busy/ms 19.37, render/present/ms 1.13, bigspace/total/ms 0.78, terrain/update/ms 0.56, controller/feed/ms 0.52, stream/drive/ms 0.51, terrain/select/ms 0.50, stream/tick/ms 0.43, physics/insert/ms 0.39 |

## Run

- `C:\Users\LEOGLO~1\AppData\Local\Temp\tex-slab-on\tex-slab-on.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\slab_on\balmora_walk@2560x1440-3 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 9%
- during the run: other processes 6% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **79%** (min 52%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.8 W · 76 °C (max 81) · VRAM 5289 MiB · throttled 100% of samples
