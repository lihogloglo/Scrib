# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: normalmaps,orm,parallax · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 958 frames over 65.7 s · **median 67.93 ms (15 fps)** · mean 68.56 · p95 81.63 · p99 93.76 (1% low 11 fps) · worst 140.98 ms · 8 spikes over 1.5× median · 8.2 stutters/min (work 4.6/min)
- warm-up: 332 frames over 18.0 s, worst 3254.4 ms
- hitches (> 169.8 ms): **0** measured, 3 in warm-up
- timed GPU passes: 48.80 ms of a 68.56 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 67.93 ms, limit 11 ms
- **FAIL** frame_ms: 9 stutters (> 1.5x moving median of 60) in 1.09 min = 8.22/min, limit 1/min
- **FAIL** worst frame 140.98 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 8.718 | 8.764 | 15.851 | 0.99 | 0.060 |
| early prepass | 7.246 | 7.284 | 13.714 | 0.99 | 0.095 |
| dlss_super_resolution | 5.529 | 5.564 | 8.167 | 0.99 | 0.210 |
| water_fft | 4.494 | 4.518 | 5.496 | 0.99 | 0.003 |
| bin_unpacking | 4.229 | 4.242 | 6.828 | 1.00 | 0.025 |
| auxiliary_views | 3.567 | 3.585 | 14.890 | 0.99 | 0.284 |
| early_mesh_preprocessing | 2.641 | 2.649 | 3.959 | 1.00 | 0.004 |
| surface_trail | 2.123 | 2.145 | 3.321 | 0.99 | 0.002 |
| uniform_allocation | 1.680 | 1.685 | 2.696 | 1.00 | 0.036 |
| water_snapshot | 1.220 | 1.226 | 1.573 | 0.99 | 0.001 |
| water_velocity | 0.929 | 0.935 | 1.300 | 0.99 | 0.001 |
| water_sspr | 0.860 | 0.864 | 1.235 | 0.99 | 0.002 |
| bloom | 0.828 | 0.834 | 1.361 | 0.99 | 0.068 |
| atmosphere_luts | 0.823 | 0.827 | 1.320 | 0.99 | 0.004 |
| clustering | 0.724 | 0.728 | 1.196 | 0.99 | 0.021 |
| main_transmissive_pass_3d | 0.671 | 0.675 | 1.245 | 0.99 | 0.007 |
| ssao | 0.480 | 0.482 | 1.090 | 0.99 | 0.008 |
| clouds_trace | 0.399 | 0.401 | 0.536 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.271 | 0.272 | 0.776 | 0.99 | 0.002 |
| tonemapping | 0.251 | 0.253 | 0.842 | 0.99 | 0.003 |
| upscaling | 0.156 | 0.157 | 0.187 | 0.99 | 0.004 |
| lightprobe_irradiance_map | 0.151 | 1.206 | 2.678 | 0.13 | 0.001 |
| clouds_shadow | 0.128 | 0.128 | 0.224 | 0.99 | 0.003 |
| clouds_composite | 0.111 | 0.111 | 0.112 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.102 | 0.102 | 0.655 | 0.99 | 0.006 |
| render_sky | 0.086 | 0.087 | 0.067 | 0.99 | 0.002 |
| water_mips | 0.084 | 0.085 | 0.088 | 0.99 | 0.002 |
| lightprobe_radiance_map | 0.075 | 0.597 | 1.037 | 0.13 | 0.004 |
| atmos_froxel_integrate | 0.063 | 0.063 | 0.577 | 0.99 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.056 | 0.99 | 0.001 |
| atmosphere_environment | 0.028 | 0.028 | 0.014 | 0.99 | 0.004 |
| sparse buffer updates | 0.024 | 0.024 | 0.015 | 1.00 | 0.003 |
| main_transparent_pass_3d | 0.023 | 0.023 | 0.042 | 0.99 | 0.117 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| lightprobe_copy | 0.010 | 0.079 | 0.624 | 0.13 | 0.004 |
| lightprobe_downsampling_first_pass | 0.009 | 0.070 | 0.036 | 0.13 | 0.001 |
| night_sky | 0.006 | 0.007 | 0.013 | 0.99 | 0.001 |
| lightprobe_downsampling_second_pass | 0.005 | 0.041 | 0.012 | 0.13 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.10 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 66.995 | 80.118 | 139.44 | 64181.7 |
| render/acquire/ms | 53.093 | 67.141 | 124.27 | 50863.3 |
| render/busy/ms | 13.246 | 18.996 | 39.29 | 12689.3 |
| main/busy/ms | 10.100 | 13.722 | 40.80 | 9675.4 |
| render/graph/ms | 7.255 | 10.889 | 28.38 | 6950.4 |
| pipelines/compile_ms | 0.707 | 0.000 | 148.93 | 677.2 |
| pipelines/front_ms | 0.677 | 0.000 | 143.30 | 648.2 |
| render/present/ms | 0.657 | 0.771 | 21.08 | 629.1 |
| bigspace/total/ms | 0.430 | 0.600 | 1.33 | 412.0 |
| stream/drive/ms | 0.420 | 0.998 | 27.10 | 401.9 |
| terrain/update/ms | 0.367 | 0.520 | 1.97 | 351.8 |
| controller/feed/ms | 0.331 | 0.469 | 0.89 | 317.6 |
| terrain/select/ms | 0.316 | 0.434 | 1.26 | 302.5 |
| physics/insert/ms | 0.239 | 0.666 | 1.48 | 228.9 |
| stream/tick/ms | 0.209 | 0.362 | 0.80 | 200.6 |
| far/select/ms | 0.204 | 0.313 | 1.95 | 195.0 |
| anim/evaluate_cpu/ms | 0.196 | 0.279 | 0.58 | 187.5 |
| pipelines/wait_ms | 0.182 | 0.000 | 115.53 | 174.3 |
| physics/step/ms | 0.167 | 0.248 | 0.93 | 160.0 |
| bigspace/low_precision/ms | 0.159 | 0.237 | 0.76 | 152.5 |
| controller/move/ms | 0.158 | 0.335 | 0.59 | 151.8 |
| anim/evaluate/ms | 0.129 | 0.205 | 1.12 | 123.3 |
| bigspace/high_precision/ms | 0.105 | 0.142 | 0.88 | 100.5 |
| stream/despawn/ms | 0.035 | 0.265 | 1.15 | 33.6 |
| pipelines/driver_ms | 0.030 | 0.000 | 6.83 | 29.0 |
| weather/map/ms | 0.029 | 0.045 | 0.36 | 28.2 |
| stream/spawn/ms | 0.028 | 0.030 | 1.63 | 27.1 |
| stream/deactivate/ms | 0.015 | 0.000 | 1.56 | 14.0 |
| anim/apply/ms | 0.014 | 0.026 | 0.05 | 12.9 |
| weather/eval/ms | 0.013 | 0.020 | 0.05 | 12.0 |
| controller/avoid/ms | 0.011 | 0.015 | 0.12 | 10.8 |
| script/update/ms | 0.009 | 0.014 | 0.08 | 8.5 |
| script/input/ms | 0.006 | 0.009 | 0.08 | 5.7 |
| stream/activate/ms | 0.006 | 0.000 | 0.50 | 5.7 |
| script/ui/ms | 0.006 | 0.009 | 0.04 | 5.4 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 3.6 |
| stream/upload/ms | 0.001 | 0.008 | 0.07 | 1.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.7 |
| weather/apply/ms | 0.001 | 0.001 | 0.01 | 0.5 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 560 | 579 | 578 |
| anim/characters/count | count | 562 | 579 | 578 |
| anim/evaluated/count | count | 19 | 38 | 17 |
| controller/movers/count | count | 20 | 23 | 17 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1280 | 2262 | 641 |
| crowd/c0/meshes/count | count | 353 | 646 | 174 |
| crowd/c1/count | count | 145 | 155 | 147 |
| crowd/c1/joints/count | count | 5515 | 5968 | 5555 |
| crowd/c1/meshes/count | count | 1167 | 1270 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 185 | 236 | 223 |
| crowd/joints_extracted/count | count | 4329 | 11.2 k | 6179 |
| crowd/joints_uploaded/count | count | 4215 | 6643 | 6179 |
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
| frame/work/ms | ms | 50.13 | 125 | 26.76 |
| gpu/timed/ms | ms | 50.13 | 125 | 26.76 |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 17 | 17 | 17 |
| mesh_allocator_slabs_size |  bytes | 602.13 M | 603.31 M | 603.31 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d/bins | count | 188 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 848 | 1414 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 9 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 188 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 848 | 1414 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 8 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1280 | 1392 | 1343 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3489 | 5119 | 3181 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d_prepass/bins | count | 1280 | 1392 | 1343 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3489 | 5119 | 3181 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 61 | 288 | 41 |
| phases/shadow/bins | count | 3842 | 5421 | 2258 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 9674 | 12.9 k | 5709 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 143 | 301 | 41 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2609 | 2989 | 2346 |
| physics/items | count | 108 | 233 | 19 |
| physics/pending | count | 63.50 | 603 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 202 | 206 | 206 |
| pipelines/waiting | count | 0 | 3 | 0 |
| preprocess/bind_groups | count | 44 | 745 | 9 |
| preprocess/bind_groups_created | count | 44 | 745 | 9 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 213 | 323 | 216 |
| process/cpu_usage | % | 11.61 | 25.58 | 13.21 |
| process/mem_usage | GiB | 6.52 | 6.74 | 6.74 |
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
| started_load_count |  | 4771 | 4864 | 4864 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 2 | 2 | 2 |
| statics/material_pipelines | count | 2 | 2 | 2 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 4 | 0 |
| stream/deactivate/queue | count | 0 | 2 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.19 | 0.67 | 0.19 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 337 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 1.37 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 28.94 | 54.17 | 34.66 |
| system/mem_usage | % | 87.85 | 89.99 | 80.34 |
| terrain/patches | count | 511 | 520 | 514 |
| textures/gpu_mib | MiB | 2822 | 2938 | 2938 |
| textures/images | count | 2589 | 2649 | 2649 |
| views/all/batch_sets | count | 119 | 340 | 101 |
| views/all/bins | count | 6505 | 8309 | 5738 |
| views/all/count | count | 22 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 18.5 k | 25.6 k | 13.3 k |
| views/all/pipelines | count | 199 | 355 | 97 |
| views/all/pipelines_distinct | count | 83 | 90 | 77 |
| views/all/work_items | count | 17.7 k | 23.9 k | 13.6 k |
| views/cascade/batch_sets | count | 49 | 62 | 41 |
| views/cascade/bins | count | 2619 | 3155 | 2258 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8209 | 10.3 k | 5709 |
| views/cascade/pipelines | count | 49 | 62 | 41 |
| views/cascade/work_items | count | 8209 | 10.3 k | 5709 |
| views/lamp_face/batch_sets | count | 11 | 235 | 0 |
| views/lamp_face/bins | count | 1128 | 2485 | 0 |
| views/lamp_face/count | count | 18 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 1306 | 2808 | 0 |
| views/lamp_face/pipelines | count | 93 | 244 | 0 |
| views/lamp_face/work_items | count | 148 | 2587 | 0 |
| views/main/batch_sets | count | 58 | 62 | 60 |
| views/main/bins | count | 2860 | 3584 | 3480 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8682 | 13.0 k | 7596 |
| views/main/pipelines | count | 54 | 58 | 56 |
| views/main/work_items | count | 8830 | 13.2 k | 7904 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.6 k | 37.7 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 62.72 | 272 | 60.1 k |
| anim/events/count | count | 0.59 | 4 | 563 |
| assets/Image/added | count | 0.62 | 43 | 594 |
| assets/Image/modified | count | 0.01 | 1 | 13 |
| assets/Image/removed | count | 0.05 | 6 | 46 |
| assets/Mesh/added | count | 4.28 | 191 | 4096 |
| assets/Mesh/removed | count | 5.14 | 1238 | 4920 |
| assets/StandardMaterial/added | count | 0.49 | 35 | 466 |
| assets/StandardMaterial/modified | count | 11.11 | 29 | 10.6 k |
| assets/StandardMaterial/removed | count | 0.43 | 40 | 408 |
| assets/WorldAsset/added | count | 0.03 | 5 | 28 |
| assets/WorldAsset/removed | count | 0.09 | 6 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 4.92 | 1563 | 4717 |
| physics/inserts | count | 2.15 | 111 | 2058 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 2.07 | 111 | 1980 |
| physics/steps | count | 2.98 | 3 | 2851 |
| physics/steps_dropped | count | 1.14 | 6 | 1092 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.01 | 2 | 13 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.23 | 21 | 219 |
| stream/deactivate/items | count | 0.23 | 21 | 223 |
| stream/despawn/items | count | 0.23 | 6 | 222 |
| stream/fetch/items | count | 0.09 | 2 | 83 |
| stream/spawn/items | count | 0.39 | 20 | 378 |
| stream/upload/bytes | bytes | 35.3 k | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.06 | 3 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.03 | 1 | 33 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 7747 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 169.8 ms; slowest 3, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.25 | 3254.4 | main/busy/ms 3227, surface/advance_cpu_ms 76.42, surface/cell_upload_ms 0.84, stream/drive/ms 0.33, weather/map/ms 0.16, bigspace/total/ms 0.10, stream/tick/ms 0.10, bigspace/high_precision/ms 0.07, weather/eval/ms 0.04, script/update/ms 0.02, far/select/ms 0.02, anim/evaluate/ms 0.01 |
| warm-up | 4.23 | 972.9 | main/busy/ms 3271, render/thread/ms 928, render/busy/ms 794, render/acquire/ms 131, surface/advance_cpu_ms 76.42, render/graph/ms 5.17, render/present/ms 2.41, stream/drive/ms 1.40, surface/cell_upload_ms 0.84, terrain/update/ms 0.79, stream/tick/ms 0.71, terrain/select/ms 0.58 |
| warm-up | 4.49 | 264.4 | pipelines/wait_ms 1956, render/thread/ms 948, render/busy/ms 812, pipelines/compile_ms 282, pipelines/front_ms 255, render/acquire/ms 131, main/busy/ms 72.76, pipelines/driver_ms 27.56, terrain/update/ms 21.68, render/graph/ms 11.53, render/present/ms 3.98, stream/drive/ms 2.26 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\h1gpucull.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\h1_old_nomaps\balmora_walk@2560x1440_off=normalmaps_orm_parallax-2 --bench-baseline none --off normalmaps,orm,parallax
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 1%, CPU 19%
- during the run: other processes 17% CPU (system minus this process)
- nvidia-smi over the window (253 samples): GPU load **98%** (min 50%) · 2580 MHz (min 2580) · memory 8001 MHz · 46.6 W · 67 °C (max 69) · VRAM 7850 MiB · throttled 100% of samples
