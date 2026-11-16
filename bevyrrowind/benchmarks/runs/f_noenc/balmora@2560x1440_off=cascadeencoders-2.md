# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadeencoders · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1456 frames over 15.0 s · **median 10.25 ms (98 fps)** · mean 10.30 · p95 11.70 · p99 12.34 (1% low 81 fps) · worst 20.00 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 281 frames over 8.5 s, worst 2514.8 ms
- hitches (> 25.6 ms): **0** measured, 56 in warm-up
- timed GPU passes: 8.88 ms of a 10.30 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.25 ms, limit 11 ms
- pass worst frame 20.00 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.943 | 2.973 | 3.432 | 0.99 | 0.064 |
| dlss_super_resolution | 2.062 | 2.084 | 2.357 | 0.99 | 0.187 |
| early prepass | 0.673 | 0.680 | 0.991 | 0.99 | 0.090 |
| atmosphere_luts | 0.440 | 0.444 | 0.728 | 0.99 | 0.003 |
| bloom | 0.320 | 0.324 | 0.337 | 0.99 | 0.066 |
| main_transmissive_pass_3d | 0.279 | 0.281 | 0.296 | 0.99 | 0.009 |
| ssao | 0.249 | 0.252 | 0.545 | 0.99 | 0.007 |
| water_velocity | 0.247 | 0.250 | 0.268 | 0.99 | 0.002 |
| water_sspr | 0.216 | 0.218 | 0.218 | 0.99 | 0.002 |
| atmos_froxel_inject | 0.153 | 0.154 | 0.167 | 0.99 | 0.001 |
| clouds_trace | 0.149 | 0.150 | 0.172 | 0.99 | 0.001 |
| water_fft | 0.146 | 0.147 | 0.144 | 0.99 | 0.003 |
| uniform_allocation | 0.123 | 0.124 | 0.153 | 0.99 | 0.023 |
| upscaling | 0.119 | 0.120 | 0.128 | 0.99 | 0.004 |
| clouds_composite | 0.114 | 0.115 | 0.116 | 0.99 | 0.002 |
| bin_unpacking | 0.091 | 0.092 | 0.112 | 0.99 | 0.017 |
| tonemapping | 0.086 | 0.087 | 0.093 | 0.99 | 0.003 |
| clustering | 0.067 | 0.067 | 0.079 | 0.99 | 0.022 |
| lightprobe_irradiance_map | 0.053 | 0.427 | 0.421 | 0.12 | 0.001 |
| render_sky | 0.053 | 0.053 | 0.054 | 0.99 | 0.002 |
| clouds_resolve | 0.050 | 0.050 | 0.052 | 0.99 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.99 | 0.003 |
| early_mesh_preprocessing | 0.030 | 0.030 | 0.038 | 0.99 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.030 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.228 | 0.236 | 0.12 | 0.004 |
| water_mips | 0.028 | 0.028 | 0.025 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.026 | 0.027 | 0.034 | 0.99 | 0.074 |
| water_snapshot | 0.026 | 0.026 | 0.031 | 0.99 | 0.002 |
| atmosphere_environment | 0.016 | 0.016 | 0.015 | 0.99 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.011 | 0.011 | 0.015 | 0.99 | 0.005 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.004 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.373 | 10.764 | 19.10 | 13647.3 |
| render/busy/ms | 7.591 | 8.885 | 17.34 | 11052.9 |
| encode//sum | 7.424 | 10.192 | 39.02 | 10809.0 |
| main/busy/ms | 7.266 | 8.512 | 12.04 | 10579.3 |
| stage/render/render | 5.386 | 6.587 | 15.48 | 7841.8 |
| render/graph/ms | 4.671 | 5.781 | 14.70 | 6800.7 |
| stage/main/post_update | 4.360 | 5.550 | 8.45 | 6348.4 |
| stage/render/graph_submit | 3.640 | 4.736 | 13.67 | 5300.5 |
| encode/submit | 1.907 | 2.658 | 4.85 | 2776.0 |
| stage/main/update | 1.805 | 2.182 | 3.36 | 2628.2 |
| encode/finish_wall | 1.782 | 2.287 | 10.51 | 2594.2 |
| stage/render/prepare_views | 1.344 | 2.992 | 4.16 | 1957.2 |
| render/acquire/ms | 1.297 | 2.935 | 4.09 | 1888.2 |
| stage/render/camera_driver | 0.968 | 1.183 | 1.59 | 1408.7 |
| stage/render/prepare_write_phase_buffers | 0.588 | 0.775 | 1.21 | 856.7 |
| render/present/ms | 0.485 | 0.601 | 2.62 | 706.2 |
| stage/render/prepare_assets | 0.439 | 0.579 | 1.70 | 638.6 |
| bigspace/total/ms | 0.405 | 0.966 | 4.51 | 589.5 |
| stage/render/prepare_resources | 0.399 | 0.483 | 0.72 | 580.8 |
| stage/main/pre_update | 0.321 | 0.432 | 0.84 | 466.8 |
| stage/main/first | 0.286 | 0.356 | 1.34 | 416.3 |
| terrain/update/ms | 0.235 | 0.300 | 0.72 | 341.5 |
| stage/render/prepare_bind_groups | 0.211 | 0.290 | 0.65 | 307.6 |
| terrain/select/ms | 0.204 | 0.259 | 0.69 | 296.7 |
| stream/drive/ms | 0.193 | 0.277 | 0.57 | 281.6 |
| stage/main/fixed_main_loop | 0.190 | 0.352 | 0.90 | 277.2 |
| stage/render/prepare_meshes | 0.189 | 0.251 | 0.62 | 275.7 |
| stage/render/extract_commands | 0.175 | 0.214 | 0.36 | 254.2 |
| stage/render/prepare_flush | 0.173 | 0.338 | 0.75 | 251.4 |
| stream/tick/ms | 0.165 | 0.242 | 0.53 | 240.5 |
| controller/feed/ms | 0.164 | 0.219 | 0.50 | 239.2 |
| stage/render/prepare_batch_phases | 0.162 | 0.227 | 0.44 | 236.3 |
| bigspace/high_precision/ms | 0.160 | 0.554 | 2.30 | 233.2 |
| far/select/ms | 0.159 | 0.229 | 0.55 | 231.9 |
| stage/render/create_views | 0.156 | 0.211 | 0.41 | 227.8 |
| stage/main/last | 0.119 | 0.174 | 0.60 | 172.7 |
| controller/move/ms | 0.108 | 0.168 | 0.42 | 157.6 |
| bigspace/low_precision/ms | 0.106 | 0.231 | 4.32 | 154.7 |
| anim/evaluate/ms | 0.104 | 0.227 | 1.70 | 151.6 |
| stage/render/specialize | 0.077 | 0.104 | 0.42 | 112.6 |
| stage/render/queue | 0.064 | 0.088 | 0.32 | 92.9 |
| stage/render/phase_sort | 0.061 | 0.084 | 0.28 | 88.7 |
| anim/evaluate_cpu/ms | 0.058 | 0.077 | 0.36 | 85.1 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.079 | 0.20 | 70.2 |
| encode/resolve_diagnostics/sum | 0.046 | 0.079 | 0.47 | 67.7 |
| physics/step/ms | 0.030 | 0.058 | 0.17 | 43.7 |
| weather/map/ms | 0.025 | 0.033 | 0.06 | 36.6 |
| stage/render/cleanup | 0.025 | 0.035 | 0.20 | 35.7 |
| encode/camera_driver_span_begin/sum | 0.022 | 0.038 | 0.16 | 32.6 |
| encode/camera_driver_span_end/sum | 0.018 | 0.035 | 0.22 | 26.7 |
| weather/eval/ms | 0.011 | 0.015 | 0.03 | 16.0 |
| script/update/ms | 0.007 | 0.010 | 0.03 | 9.6 |
| controller/avoid/ms | 0.006 | 0.008 | 0.18 | 9.1 |
| script/ui/ms | 0.005 | 0.007 | 0.10 | 7.0 |
| script/input/ms | 0.005 | 0.006 | 0.07 | 7.0 |
| anim/apply/ms | 0.004 | 0.006 | 0.10 | 6.3 |
| controller/write/ms | 0.002 | 0.002 | 0.19 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.5 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 0.9 |
| weather/apply/ms | 0.000 | 0.001 | 0.00 | 0.5 |
| physics/insert/ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/compile_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/driver_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/front_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.00 | 0.0 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.00 | 0.0 |
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
| crowd/joint_blocks/count | count | 181 | 181 | 181 |
| crowd/joints_extracted/count | count | 4512 | 4512 | 4512 |
| crowd/joints_uploaded/count | count | 4512 | 4512 | 4512 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.47 | 10.01 | 1.27 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.16 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.22 | 0.02 |
| encode/finish_critical | ms | 1.47 | 10.01 | 1.27 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.47 | 0.06 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.55 | 17.34 | 9.78 |
| gpu/timed/ms | ms | 9.55 | 11.18 | 9.78 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1943 | 1943 | 1943 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 207.2 k | 207.2 k | 207.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 13.3 k | 13.3 k | 13.3 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 485.99 M | 485.99 M | 485.99 M |
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
| phases/opaque3d/batch_sets | count | 21 | 21 | 21 |
| phases/opaque3d/bins | count | 1306 | 1306 | 1306 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1306 | 1306 | 1306 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 2 | 20 | 2 |
| phases/shadow/bins | count | 1887 | 2194 | 1887 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5746 | 6072 | 5746 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 17 | 20 | 17 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 0 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 295 | 295 | 295 |
| pipelines/waiting | count | 0 | 0 | 0 |
| precache/draw_waiting | count | 0 | 0 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 0 | 39 | 0 |
| preprocess/bind_groups_created | count | 0 | 39 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 129 | 129 | 129 |
| process/cpu_usage | % | 25 | 39.90 | 27.35 |
| process/mem_usage | GiB | 4.60 | 4.67 | 4.65 |
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
| started_load_count |  | 3703 | 3705 | 3705 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 386 | 386 | 386 |
| statics/materials_bound | count | 386 | 386 | 386 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 0 | 0 |
| stream/deactivate/queue | count | 0 | 0 | 0 |
| stream/despawn/queue | count | 0 | 0 | 0 |
| stream/fetch/queue | count | 0 | 0 | 0 |
| stream/latency/s | s | 0.05 | 0.05 | 0.05 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 33.56 | 46.78 | 33.69 |
| system/mem_usage | % | 66.00 | 66.24 | 66.23 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2544 | 2544 | 2544 |
| textures/images | count | 2009 | 2009 | 2009 |
| views/all/batch_sets | count | 54 | 72 | 54 |
| views/all/bins | count | 5059 | 5366 | 5059 |
| views/all/count | count | 3 | 5 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 15.0 k | 15.4 k | 15.0 k |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 9498 | 15.5 k | 9498 |
| views/cascade/batch_sets | count | 2 | 20 | 2 |
| views/cascade/bins | count | 1887 | 2194 | 1887 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5746 | 6072 | 5746 |
| views/cascade/pipelines | count | 17 | 20 | 17 |
| views/cascade/work_items | count | 36 | 6072 | 36 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 52 | 52 | 52 |
| views/main/bins | count | 3172 | 3172 | 3172 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 48 | 48 | 48 |
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
| world/meshes | count | 24.7 k | 24.7 k | 24.7 k |
| world/point_lights | count | 210 | 210 | 210 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.10 | 3 | 149 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.40 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 31.25 | 33 | 45.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1456 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1456 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1456 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.62 | 2 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
| precache/hit | count | 0 | 0 | 0 |
| precache/missed | count | 0 | 0 | 0 |
| precache/requested | count | 0 | 0 | 0 |
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
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 25.6 ms; slowest 56, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2514.8 | main/busy/ms 2489, stage/main/post_update 2386, stage/main/update 96.59, surface/advance_cpu_ms 67.43, stage/main/pre_update 2.50, stage/main/last 0.97, surface/cell_upload_ms 0.68, stream/drive/ms 0.46, stage/main/first 0.45, stage/main/fixed_main_loop 0.41, weather/map/ms 0.12, stream/tick/ms 0.11 |
| warm-up | 3.25 | 732.6 | main/busy/ms 2631, stage/main/post_update 2414, render/thread/ms 703, render/busy/ms 539, stage/render/prepare_assets 513, stage/main/update 204, stage/render/prepare_views 163, render/acquire/ms 162, surface/advance_cpu_ms 67.43, stage/render/render 10.49, encode//sum 10.17, stage/render/prepare_resources 8.70 |
| warm-up | 3.31 | 64.1 | render/thread/ms 750, render/busy/ms 583, stage/render/prepare_assets 539, stage/render/prepare_views 164, render/acquire/ms 162, main/busy/ms 161, stage/main/update 112, pipelines/compile_ms 62.55, pipelines/front_ms 58.99, stage/main/post_update 38.61, stage/render/render 22.88, terrain/update/ms 21.32 |
| warm-up | 3.34 | 31.9 | pipelines/compile_ms 158, pipelines/front_ms 148, main/busy/ms 102, stage/main/post_update 86.40, render/thread/ms 75.91, render/busy/ms 66.19, pipelines/wait_ms 61.29, stage/render/render 32.79, terrain/update/ms 27.67, stage/render/prepare_assets 27.05, render/graph/ms 22.52, pipelines/lock_ms 10.90 |
| warm-up | 3.48 | 132.7 | pipelines/wait_ms 2114, pipelines/compile_ms 882, pipelines/front_ms 838, render/thread/ms 108, main/busy/ms 105, render/busy/ms 99.44, stage/main/post_update 90.42, pipelines/driver_ms 44.22, stage/render/prepare_assets 28.83, stage/render/prepare_flush 26.00, stage/render/render 23.43, stage/render/prepare_resources 17.29 |
| warm-up | 3.59 | 118.6 | pipelines/wait_ms 8597, pipelines/compile_ms 2480, pipelines/front_ms 2377, render/thread/ms 196, render/busy/ms 194, pipelines/driver_ms 103, main/busy/ms 77.62, stage/render/prepare_resources 53.16, stage/render/prepare_flush 50.70, stage/render/prepare_assets 50.18, stage/main/post_update 37.36, stage/main/pre_update 28.61 |
| warm-up | 3.65 | 54.1 | pipelines/wait_ms 7271, pipelines/compile_ms 2517, pipelines/front_ms 2349, render/thread/ms 169, pipelines/driver_ms 168, render/busy/ms 167, main/busy/ms 130, stage/render/prepare_assets 64.16, stage/main/update 54.52, stage/render/prepare_resources 39.01, stage/main/post_update 28.27, stage/main/pre_update 28.23 |
| warm-up | 3.76 | 112.7 | pipelines/compile_ms 2239, pipelines/front_ms 2071, pipelines/wait_ms 1191, pipelines/driver_ms 168, render/thread/ms 138, render/busy/ms 113, main/busy/ms 94.96, stage/render/prepare_assets 93.19, stage/main/update 57.70, stage/render/render 35.17, render/present/ms 24.40, stage/main/post_update 18.47 |
| warm-up | 3.79 | 28.5 | pipelines/compile_ms 2113, pipelines/front_ms 1930, pipelines/wait_ms 623, pipelines/driver_ms 183, render/thread/ms 113, render/busy/ms 87.68, stage/render/prepare_assets 64.97, stage/render/render 39.84, main/busy/ms 31.87, render/present/ms 25.05, encode//sum 22.41, stage/main/post_update 17.26 |
| warm-up | 3.88 | 32.2 | pipelines/compile_ms 236, pipelines/front_ms 230, main/busy/ms 50.25, render/thread/ms 45.23, render/busy/ms 43.24, stage/main/update 40.65, stream/drive/ms 36.70, stage/render/prepare_assets 25.91, stage/render/render 10.53, render/graph/ms 8.15, encode//sum 7.67, pipelines/driver_ms 6.35 |
| warm-up | 3.94 | 56.9 | pipelines/compile_ms 799, pipelines/front_ms 763, render/thread/ms 65.87, render/busy/ms 64.14, main/busy/ms 50.94, stage/render/prepare_assets 50.00, stage/main/update 41.05, stream/drive/ms 36.66, pipelines/driver_ms 36.27, stage/render/render 9.65, render/graph/ms 7.64, encode//sum 6.96 |
| warm-up | 4.00 | 44.3 | pipelines/compile_ms 478, pipelines/front_ms 462, render/thread/ms 48.17, render/busy/ms 43.64, main/busy/ms 41.88, stage/render/prepare_assets 22.38, stage/render/render 18.21, stage/main/update 17.25, pipelines/driver_ms 16.31, render/graph/ms 13.19, stream/drive/ms 12.22, stage/main/post_update 12.08 |
| warm-up | 4.04 | 41.5 | pipelines/compile_ms 693, pipelines/front_ms 664, render/thread/ms 65.31, render/busy/ms 63.34, stage/render/prepare_assets 39.29, main/busy/ms 34.59, pipelines/driver_ms 28.56, stage/render/render 16.56, render/graph/ms 14.08, stage/main/post_update 10.64, encode//sum 9.44, stage/main/update 9.33 |
| warm-up | 4.10 | 57.7 | pipelines/compile_ms 539, pipelines/front_ms 517, main/busy/ms 106, render/thread/ms 79.95, render/busy/ms 77.89, stage/main/update 76.24, stream/drive/ms 50.01, stage/render/prepare_assets 48.00, pipelines/driver_ms 22.19, stage/main/post_update 21.43, stage/render/render 19.24, render/graph/ms 16.59 |
| warm-up | 4.22 | 128.6 | pipelines/compile_ms 1462, pipelines/front_ms 1417, main/busy/ms 108, render/thread/ms 103, render/busy/ms 100, stage/main/update 79.57, stage/render/prepare_assets 68.23, stream/drive/ms 51.71, pipelines/driver_ms 44.89, stage/main/post_update 21.35, stage/render/render 21.32, render/graph/ms 18.09 |
| warm-up | 4.30 | 77.9 | pipelines/compile_ms 2078, pipelines/front_ms 2020, render/thread/ms 111, main/busy/ms 92.75, render/busy/ms 91.75, stage/main/update 64.37, pipelines/driver_ms 58.71, stage/render/prepare_assets 57.85, stream/drive/ms 43.94, stage/render/render 42.23, render/graph/ms 22.63, render/present/ms 18.66 |
| warm-up | 4.45 | 145.7 | pipelines/compile_ms 2002, pipelines/front_ms 1943, render/thread/ms 146, render/busy/ms 128, main/busy/ms 116, stage/render/prepare_assets 99.38, stage/main/update 73.30, pipelines/driver_ms 58.54, stream/drive/ms 43.38, stage/render/render 38.94, stage/main/post_update 27.63, render/graph/ms 20.20 |
| warm-up | 4.52 | 72.4 | pipelines/compile_ms 2837, pipelines/front_ms 2751, render/thread/ms 143, render/busy/ms 141, stage/render/prepare_assets 111, pipelines/driver_ms 85.74, main/busy/ms 80.69, pipelines/wait_ms 46.80, stage/main/update 35.99, stage/main/post_update 24.06, stage/render/render 17.44, render/graph/ms 15.06 |
| warm-up | 4.58 | 64.4 | pipelines/compile_ms 1923, pipelines/front_ms 1863, pipelines/wait_ms 146, render/thread/ms 110, render/busy/ms 107, main/busy/ms 88.76, pipelines/driver_ms 59.70, stage/render/prepare_assets 46.11, stage/main/update 37.14, stage/main/post_update 22.66, stage/render/prepare_resources 20.59, stage/render/render 14.31 |
| warm-up | 4.66 | 78.8 | pipelines/compile_ms 873, pipelines/front_ms 843, pipelines/wait_ms 170, render/thread/ms 123, render/busy/ms 121, main/busy/ms 92.93, stage/main/post_update 45.83, stage/render/prepare_resources 40.73, stage/render/prepare_assets 34.55, pipelines/driver_ms 29.66, stage/main/update 25.63, stage/render/prepare_write_phase_buffers 14.32 |
| warm-up | 4.70 | 36.7 | pipelines/compile_ms 1436, pipelines/front_ms 1396, pipelines/wait_ms 369, render/thread/ms 96.66, render/busy/ms 95.01, main/busy/ms 66.88, pipelines/driver_ms 40.48, stage/main/post_update 38.98, stage/render/prepare_resources 32.29, stage/render/prepare_assets 25.52, stage/main/update 17.49, stage/render/prepare_write_phase_buffers 13.25 |
| warm-up | 4.75 | 50.9 | pipelines/compile_ms 2006, pipelines/front_ms 1948, pipelines/wait_ms 384, render/thread/ms 72.06, render/busy/ms 69.83, main/busy/ms 59.28, pipelines/driver_ms 57.97, stage/main/post_update 33.21, stage/render/prepare_resources 22.75, stage/main/update 16.47, stage/render/prepare_assets 14.34, stage/render/render 12.64 |
| warm-up | 4.79 | 34.7 | pipelines/compile_ms 1285, pipelines/front_ms 1246, pipelines/wait_ms 83.98, render/thread/ms 68.59, render/busy/ms 66.35, main/busy/ms 50.21, pipelines/driver_ms 39.11, stage/main/post_update 30.16, stage/render/prepare_assets 24.35, stage/render/render 14.02, stage/main/update 12.06, stage/render/prepare_resources 11.72 |
| warm-up | 4.81 | 26.8 | pipelines/compile_ms 265, pipelines/front_ms 257, pipelines/wait_ms 80.46, main/busy/ms 69.32, stage/main/update 54.64, render/thread/ms 47.89, stream/drive/ms 47.30, render/busy/ms 46.27, stage/render/prepare_assets 28.30, stage/render/render 11.67, encode//sum 10.84, render/graph/ms 9.61 |
| warm-up | 4.86 | 49.7 | pipelines/compile_ms 661, pipelines/front_ms 643, pipelines/wait_ms 113, main/busy/ms 101, stage/main/update 86.61, stream/drive/ms 81.22, render/thread/ms 33.30, render/busy/ms 31.65, pipelines/driver_ms 17.95, stage/render/prepare_assets 16.66, encode//sum 10.72, stage/render/render 9.85 |
| warm-up | 4.92 | 59.1 | pipelines/compile_ms 1199, pipelines/front_ms 1167, pipelines/wait_ms 352, main/busy/ms 65.50, stage/main/update 43.24, stream/drive/ms 37.01, render/thread/ms 32.65, pipelines/driver_ms 32.54, render/busy/ms 30.60, stage/render/render 15.11, stage/main/post_update 14.08, render/graph/ms 12.56 |
| warm-up | 4.96 | 35.7 | pipelines/compile_ms 1208, pipelines/front_ms 1166, pipelines/wait_ms 323, main/busy/ms 52.15, render/thread/ms 47.54, render/busy/ms 44.92, pipelines/driver_ms 42.04, stage/main/update 24.68, stage/render/render 19.80, stage/main/post_update 17.05, render/graph/ms 16.56, encode//sum 14.24 |
| warm-up | 5.01 | 57.2 | pipelines/compile_ms 714, pipelines/front_ms 684, render/thread/ms 63.92, main/busy/ms 62.57, render/busy/ms 61.65, stage/main/update 41.02, pipelines/driver_ms 30.12, stage/render/prepare_assets 27.98, stage/render/render 15.92, encode//sum 14.06, stage/main/post_update 13.25, render/graph/ms 13.00 |
| warm-up | 5.05 | 38.5 | pipelines/compile_ms 323, pipelines/front_ms 312, render/thread/ms 66.50, render/busy/ms 65.01, main/busy/ms 52.74, stage/main/update 37.84, stage/render/prepare_assets 37.64, stream/drive/ms 14.35, stage/render/render 11.50, encode//sum 11.38, pipelines/driver_ms 11.32, render/graph/ms 9.54 |
| warm-up | 5.13 | 27.1 | pipelines/compile_ms 991, pipelines/front_ms 967, render/thread/ms 44.66, render/busy/ms 41.77, main/busy/ms 37.17, pipelines/driver_ms 23.63, stage/main/update 21.95, stage/render/render 13.89, encode//sum 12.28, render/graph/ms 10.66, stage/main/post_update 9.18, stage/render/graph_submit 7.86 |
| warm-up | 5.16 | 30.4 | pipelines/compile_ms 1392, pipelines/front_ms 1355, render/thread/ms 47.76, main/busy/ms 45.27, render/busy/ms 45.03, pipelines/driver_ms 36.45, stage/main/update 31.47, stage/render/render 18.51, render/graph/ms 16.36, stream/drive/ms 16.24, stage/render/graph_submit 13.75, encode//sum 12.93 |
| warm-up | 5.21 | 29.5 | pipelines/compile_ms 380, pipelines/front_ms 357, render/thread/ms 43.52, render/busy/ms 41.95, main/busy/ms 34.09, pipelines/driver_ms 22.67, stage/main/post_update 15.39, encode//sum 13.53, stage/main/update 12.70, stage/render/render 11.95, render/graph/ms 10.07, stage/render/prepare_resources 9.06 |
| warm-up | 5.24 | 35.2 | pipelines/compile_ms 435, pipelines/front_ms 420, render/thread/ms 58.58, render/busy/ms 56.58, main/busy/ms 41.70, stage/main/post_update 17.79, stage/main/update 17.45, stage/render/render 15.71, stage/render/prepare_resources 15.41, pipelines/driver_ms 15.13, encode//sum 13.41, render/graph/ms 13.28 |
| warm-up | 5.28 | 33.1 | pipelines/compile_ms 480, pipelines/front_ms 467, render/thread/ms 54.59, render/busy/ms 52.41, main/busy/ms 41.61, stage/main/update 23.24, stage/render/render 15.51, stage/render/prepare_assets 14.97, encode//sum 13.30, render/graph/ms 12.94, pipelines/driver_ms 12.92, stream/drive/ms 10.98 |
| warm-up | 5.34 | 68.8 | pipelines/compile_ms 1277, pipelines/front_ms 1244, render/thread/ms 79.71, render/busy/ms 77.87, stage/render/prepare_assets 43.47, main/busy/ms 40.80, pipelines/driver_ms 32.68, encode//sum 22.61, stage/render/render 22.14, stage/main/update 20.70, render/graph/ms 19.69, stage/render/graph_submit 16.14 |
| warm-up | 5.38 | 35.6 | pipelines/compile_ms 1250, pipelines/front_ms 1218, render/thread/ms 88.45, render/busy/ms 86.34, main/busy/ms 40.63, stage/render/prepare_assets 38.70, pipelines/driver_ms 32.85, stage/render/render 28.05, encode//sum 28.02, render/graph/ms 25.21, stage/render/graph_submit 20.54, stage/main/post_update 18.62 |
| warm-up | 5.41 | 34.9 | pipelines/compile_ms 716, pipelines/front_ms 697, render/thread/ms 56.26, render/busy/ms 54.23, main/busy/ms 37.11, stage/render/prepare_assets 19.34, pipelines/driver_ms 18.60, encode//sum 17.72, stage/render/render 17.48, stage/main/update 15.38, render/graph/ms 14.90, stage/main/post_update 14.43 |
| warm-up | 5.46 | 42.3 | pipelines/compile_ms 699, pipelines/front_ms 679, render/thread/ms 57.74, render/busy/ms 56.16, main/busy/ms 39.94, stage/render/prepare_assets 35.89, stage/main/update 22.68, pipelines/driver_ms 19.19, encode//sum 12.65, stream/drive/ms 12.49, stage/render/render 11.68, stage/main/post_update 10.00 |
| warm-up | 5.49 | 29.5 | pipelines/compile_ms 797, pipelines/front_ms 775, render/thread/ms 61.50, render/busy/ms 60.02, main/busy/ms 40.91, stage/render/prepare_assets 37.74, pipelines/driver_ms 22.61, stage/main/update 20.87, encode//sum 14.67, stream/drive/ms 13.38, stage/main/post_update 13.20, stage/render/render 12.16 |
| warm-up | 5.53 | 25.7 | pipelines/compile_ms 194, pipelines/front_ms 189, render/thread/ms 40.24, render/busy/ms 38.80, main/busy/ms 29.80, stage/render/prepare_assets 16.09, encode//sum 14.16, stage/main/update 13.01, stage/main/post_update 11.26, stage/render/render 11.23, render/graph/ms 9.37, stage/render/graph_submit 6.69 |
| warm-up | 5.56 | 26.1 | pipelines/compile_ms 394, pipelines/front_ms 385, render/thread/ms 47.65, render/busy/ms 46.21, main/busy/ms 32.13, stage/render/prepare_assets 22.65, stage/main/update 15.30, encode//sum 14.72, stage/main/post_update 11.37, stage/render/render 10.52, pipelines/driver_ms 9.79, render/graph/ms 8.57 |
| warm-up | 5.61 | 31.0 | pipelines/compile_ms 739, pipelines/front_ms 720, render/thread/ms 48.61, render/busy/ms 47.16, main/busy/ms 37.54, stage/render/prepare_assets 23.24, pipelines/driver_ms 18.86, encode//sum 18.12, stage/main/update 17.78, stage/render/render 13.98, stage/main/post_update 12.81, render/graph/ms 11.90 |
| warm-up | 5.66 | 26.5 | render/thread/ms 42.81, render/busy/ms 41.58, main/busy/ms 26.13, stage/render/prepare_assets 19.82, encode//sum 12.17, stage/main/update 10.64, stage/main/post_update 10.00, stage/render/render 8.86, render/graph/ms 7.46, stage/render/graph_submit 5.47, stage/render/specialize 4.41, stream/drive/ms 3.78 |
| warm-up | 5.69 | 27.4 | pipelines/compile_ms 315, pipelines/front_ms 309, render/thread/ms 48.36, render/busy/ms 46.80, stage/render/prepare_assets 25.80, main/busy/ms 23.95, encode//sum 16.52, stage/render/render 12.18, stage/main/update 11.24, render/graph/ms 10.03, stage/main/post_update 7.58, stage/render/graph_submit 7.48 |
| warm-up | 5.75 | 42.5 | render/thread/ms 52.68, render/busy/ms 51.39, main/busy/ms 36.13, stage/render/prepare_assets 32.16, stage/main/post_update 14.81, stage/main/update 14.74, encode//sum 13.50, stage/render/render 9.08, render/graph/ms 7.44, stream/drive/ms 5.41, stage/render/graph_submit 5.23, encode/finish_wall 2.77 |
| warm-up | 5.78 | 29.4 | render/thread/ms 55.26, render/busy/ms 54.20, stage/render/prepare_assets 38.56, main/busy/ms 30.65, stage/main/update 13.81, encode//sum 11.98, stage/main/post_update 11.31, stage/render/render 8.55, render/graph/ms 7.16, stage/render/graph_submit 5.39, stream/drive/ms 4.24, encode/finish_wall 2.73 |
| warm-up | 5.86 | 32.0 | render/thread/ms 43.24, render/busy/ms 41.54, main/busy/ms 37.00, stage/main/update 19.42, encode//sum 18.39, stage/render/render 16.34, stage/render/prepare_assets 14.06, render/graph/ms 14.05, stage/main/post_update 12.10, stage/render/graph_submit 9.99, stream/drive/ms 6.02, encode/submit 5.85 |
| warm-up | 5.89 | 25.9 | render/thread/ms 45.81, render/busy/ms 43.86, main/busy/ms 37.70, encode//sum 20.30, stage/render/render 16.96, stage/main/post_update 15.71, stage/main/update 15.13, stage/render/prepare_assets 14.64, render/graph/ms 14.27, stage/render/graph_submit 9.93, encode/submit 5.48, stream/drive/ms 5.37 |
| warm-up | 5.93 | 45.1 | render/thread/ms 53.64, main/busy/ms 52.26, render/busy/ms 51.56, stage/main/update 29.57, encode//sum 23.33, stage/render/render 21.48, render/graph/ms 18.39, stage/render/prepare_assets 16.40, stage/main/post_update 15.59, stream/drive/ms 14.72, stage/render/graph_submit 14.18, encode/finish_wall 7.58 |
| warm-up | 5.96 | 28.8 | render/thread/ms 50.90, render/busy/ms 49.34, main/busy/ms 44.47, stage/main/update 26.24, stage/render/prepare_assets 22.70, encode//sum 17.71, pipelines/compile_ms 15.29, stage/render/render 15.28, pipelines/front_ms 14.36, stream/drive/ms 13.73, render/graph/ms 12.91, stage/main/post_update 11.33 |
| warm-up | 6.11 | 31.3 | main/busy/ms 34.08, render/thread/ms 30.19, render/busy/ms 28.98, stage/main/update 22.96, encode//sum 18.90, stream/drive/ms 14.60, stage/render/render 12.27, render/graph/ms 10.53, stage/render/prepare_assets 10.38, stage/main/post_update 8.05, stage/render/graph_submit 7.78, encode/finish_wall 4.52 |
| warm-up | 6.20 | 32.1 | render/thread/ms 47.44, render/busy/ms 46.02, main/busy/ms 34.26, stage/render/prepare_assets 26.11, stage/main/update 18.26, encode//sum 17.59, stage/render/render 12.91, render/graph/ms 11.01, stage/main/post_update 10.42, stage/render/graph_submit 8.36, encode/submit 4.51, encode/finish_wall 3.96 |
| warm-up | 6.35 | 28.9 | render/thread/ms 36.76, render/busy/ms 34.19, main/busy/ms 32.32, encode//sum 20.89, stage/render/render 17.86, stage/main/post_update 15.00, render/graph/ms 14.70, stage/main/update 12.06, stage/render/graph_submit 10.96, stage/render/prepare_assets 9.17, encode/finish_wall 5.74, encode/submit 5.40 |
| warm-up | 6.38 | 26.4 | render/thread/ms 42.30, render/busy/ms 40.15, main/busy/ms 31.98, encode//sum 18.44, stage/render/prepare_assets 18.08, stage/main/post_update 14.03, stage/render/render 13.47, stage/main/update 11.54, render/graph/ms 10.68, stage/render/graph_submit 7.57, encode/finish_wall 4.44, stream/drive/ms 4.05 |
| warm-up | 6.42 | 37.0 | render/thread/ms 42.33, render/busy/ms 41.08, main/busy/ms 26.33, stage/render/prepare_assets 26.31, encode//sum 14.68, stage/main/post_update 10.79, stage/main/update 9.89, stage/render/render 8.94, render/graph/ms 7.33, stage/render/graph_submit 5.46, stream/drive/ms 3.65, encode/finish_wall 3.48 |
| warm-up | 6.51 | 33.6 | main/busy/ms 33.77, render/thread/ms 29.99, render/busy/ms 28.57, stage/main/update 18.22, encode//sum 15.13, stage/main/post_update 11.71, stage/render/render 11.03, render/graph/ms 8.91, stream/drive/ms 8.85, stage/render/prepare_assets 8.76, stage/render/graph_submit 6.68, encode/finish_wall 3.97 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_noenc\balmora@2560x1440_off=cascadeencoders-2 --bench-baseline none --off cascadeencoders
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 20%
- during the run: other processes 9% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 97%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.7 W · 74 °C (max 75) · VRAM 5683 MiB · throttled 100% of samples
