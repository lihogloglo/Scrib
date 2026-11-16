# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5612 frames over 65.6 s · **median 11.43 ms (87 fps)** · mean 11.70 · p95 13.96 · p99 16.24 (1% low 62 fps) · worst 30.10 ms · 41 spikes over 1.5× median · 28.3 stutters/min (work 21.9/min)
- warm-up: 324 frames over 9.1 s, worst 2562.0 ms
- hitches (> 28.6 ms): **2** measured, 39 in warm-up
- timed GPU passes: 7.38 ms of a 11.70 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.43 ms, limit 11 ms
- **FAIL** frame_ms: 31 stutters (> 1.5x moving median of 60) in 1.09 min = 28.34/min, limit 1/min
- pass worst frame 30.10 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.055 | 2.076 | 2.118 | 0.99 | 0.173 |
| main_opaque_pass_3d | 1.629 | 1.646 | 2.044 | 0.99 | 0.067 |
| early prepass | 0.539 | 0.545 | 0.783 | 0.99 | 0.080 |
| bloom | 0.317 | 0.320 | 0.326 | 0.99 | 0.063 |
| atmosphere_luts | 0.313 | 0.317 | 0.374 | 0.99 | 0.003 |
| water_velocity | 0.281 | 0.284 | 0.353 | 0.99 | 0.002 |
| water_sspr | 0.203 | 0.205 | 0.206 | 0.99 | 0.002 |
| ssao | 0.191 | 0.193 | 0.196 | 0.99 | 0.007 |
| auxiliary_views | 0.163 | 0.164 | 0.639 | 0.99 | 0.151 |
| per_view_shadow_pass_cascade1 | 0.147 | 0.583 | 0.655 | 0.25 | 0.041 |
| uniform_allocation | 0.146 | 0.148 | 0.432 | 0.99 | 0.026 |
| atmos_froxel_inject | 0.145 | 0.147 | 0.147 | 0.99 | 0.001 |
| clouds_trace | 0.141 | 0.142 | 0.232 | 0.99 | 0.001 |
| water_fft | 0.132 | 0.133 | 0.140 | 0.99 | 0.003 |
| upscaling | 0.114 | 0.115 | 0.125 | 0.99 | 0.004 |
| clouds_composite | 0.107 | 0.108 | 0.110 | 0.99 | 0.002 |
| bin_unpacking | 0.101 | 0.102 | 0.124 | 0.99 | 0.019 |
| tonemapping | 0.085 | 0.086 | 0.090 | 0.99 | 0.004 |
| main_transmissive_pass_3d | 0.083 | 0.084 | 0.160 | 0.99 | 0.009 |
| clustering | 0.064 | 0.065 | 0.068 | 0.99 | 0.019 |
| render_sky | 0.052 | 0.052 | 0.053 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.050 | 0.421 | 0.421 | 0.12 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 0.99 | 0.001 |
| surface_trail | 0.044 | 0.058 | 0.058 | 0.75 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.003 |
| early_mesh_preprocessing | 0.030 | 0.030 | 0.043 | 0.99 | 0.005 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.229 | 0.235 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.025 | 0.027 | 0.99 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 0.99 | 0.002 |
| main_transparent_pass_3d | 0.012 | 0.012 | 0.024 | 0.99 | 0.114 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.015 | 0.99 | 0.005 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.012 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.014 | 0.014 | 0.12 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.012 | 0.012 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.023 | 0.047 | 0.01 | 0.012 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.017 | 0.018 | 0.00 | 0.016 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.005 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 10.957 | 16.491 | 57.49 | 61493.3 |
| render/thread/ms | 10.402 | 12.562 | 24.82 | 58373.7 |
| render/busy/ms | 9.841 | 11.995 | 24.00 | 55225.0 |
| main/busy/ms | 9.345 | 11.449 | 25.62 | 52446.2 |
| stage/render/render | 6.604 | 8.203 | 16.79 | 37059.7 |
| stage/main/post_update | 5.891 | 7.650 | 12.28 | 33062.3 |
| render/graph/ms | 5.852 | 7.394 | 15.56 | 32843.8 |
| stage/render/graph_submit | 4.673 | 5.878 | 11.56 | 26227.0 |
| encode/finish_wall | 2.368 | 3.066 | 6.14 | 13290.2 |
| encode/submit | 2.356 | 3.196 | 6.81 | 13219.5 |
| stage/main/update | 2.197 | 2.638 | 19.09 | 12326.9 |
| stage/render/camera_driver | 1.124 | 1.536 | 4.06 | 6310.7 |
| stage/render/prepare_write_phase_buffers | 0.785 | 1.049 | 2.43 | 4404.1 |
| bigspace/total/ms | 0.633 | 2.083 | 3.49 | 3553.0 |
| stage/render/prepare_assets | 0.587 | 0.750 | 9.89 | 3296.9 |
| render/present/ms | 0.472 | 0.577 | 3.42 | 2647.5 |
| stage/render/prepare_meshes | 0.407 | 0.529 | 1.71 | 2281.4 |
| stage/render/prepare_resources | 0.400 | 0.504 | 1.26 | 2243.2 |
| stage/main/first | 0.397 | 0.516 | 5.02 | 2228.8 |
| stage/main/pre_update | 0.332 | 0.470 | 1.66 | 1860.5 |
| terrain/update/ms | 0.305 | 0.401 | 2.22 | 1712.1 |
| stage/render/specialize | 0.297 | 0.425 | 2.52 | 1667.1 |
| bigspace/low_precision/ms | 0.279 | 1.174 | 2.79 | 1566.3 |
| controller/feed/ms | 0.278 | 0.356 | 0.67 | 1558.8 |
| terrain/select/ms | 0.271 | 0.357 | 0.82 | 1522.5 |
| stage/render/prepare_bind_groups | 0.268 | 0.449 | 2.80 | 1505.4 |
| stage/render/queue | 0.261 | 0.411 | 2.82 | 1462.6 |
| stream/drive/ms | 0.251 | 0.343 | 17.09 | 1410.0 |
| stage/main/fixed_main_loop | 0.231 | 0.379 | 1.26 | 1296.8 |
| anim/evaluate/ms | 0.202 | 0.315 | 2.73 | 1133.5 |
| bigspace/high_precision/ms | 0.200 | 0.773 | 3.21 | 1119.8 |
| stream/tick/ms | 0.194 | 0.264 | 0.75 | 1087.1 |
| far/select/ms | 0.190 | 0.261 | 0.55 | 1066.5 |
| stage/render/create_views | 0.173 | 0.228 | 0.90 | 973.1 |
| stage/render/prepare_flush | 0.173 | 0.358 | 0.80 | 971.0 |
| stage/render/extract_commands | 0.172 | 0.204 | 0.43 | 967.8 |
| stage/render/prepare_batch_phases | 0.162 | 0.227 | 0.75 | 911.2 |
| anim/evaluate_cpu/ms | 0.148 | 0.191 | 0.82 | 830.1 |
| stage/render/prepare_views | 0.131 | 0.166 | 0.39 | 735.2 |
| controller/move/ms | 0.124 | 0.263 | 0.50 | 697.1 |
| stage/main/last | 0.120 | 0.193 | 1.70 | 673.9 |
| render/acquire/ms | 0.089 | 0.114 | 0.36 | 501.2 |
| stage/render/phase_sort | 0.089 | 0.135 | 0.41 | 497.9 |
| pipelines/compile_ms | 0.054 | 0.000 | 120.21 | 304.1 |
| pipelines/front_ms | 0.052 | 0.000 | 116.42 | 292.2 |
| physics/step/ms | 0.051 | 0.089 | 0.56 | 287.8 |
| stage/render/prepare_collect_phase_buffers | 0.047 | 0.074 | 0.30 | 265.7 |
| physics/insert/ms | 0.035 | 0.379 | 0.97 | 195.8 |
| stage/render/cleanup | 0.032 | 0.102 | 0.61 | 181.3 |
| encode/resolve_diagnostics/sum | 0.032 | 0.043 | 0.53 | 178.0 |
| weather/map/ms | 0.026 | 0.034 | 0.20 | 148.4 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.023 | 0.23 | 87.7 |
| anim/apply/ms | 0.012 | 0.022 | 0.20 | 66.8 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.26 | 64.9 |
| weather/eval/ms | 0.011 | 0.015 | 0.06 | 59.6 |
| controller/avoid/ms | 0.011 | 0.014 | 0.19 | 59.4 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 40.3 |
| stream/despawn/ms | 0.005 | 0.000 | 1.08 | 29.9 |
| script/input/ms | 0.005 | 0.007 | 0.02 | 28.7 |
| stream/spawn/ms | 0.005 | 0.000 | 1.09 | 26.2 |
| script/ui/ms | 0.005 | 0.006 | 0.19 | 25.8 |
| controller/write/ms | 0.004 | 0.005 | 0.23 | 20.0 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.71 | 14.7 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.84 | 11.9 |
| stream/activate/ms | 0.001 | 0.000 | 0.52 | 5.3 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 4.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.07 | 3.4 |
| weather/apply/ms | 0.000 | 0.001 | 0.10 | 2.1 |
| stream/upload/ms | 0.000 | 0.000 | 0.12 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.09 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.08 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 30 | 21 |
| controller/movers/count | count | 20 | 21 | 21 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 535 |
| crowd/c0/meshes/count | count | 353 | 646 | 144 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5949 | 5629 |
| crowd/c1/meshes/count | count | 1181 | 1281 | 1189 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 201 | 250 | 238 |
| crowd/joints_extracted/count | count | 4621 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4603 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 1.95 | 4.33 | 2.21 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.23 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.26 | 0.01 |
| encode/finish_critical | ms | 1.95 | 4.33 | 2.21 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.53 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 9.89 | 25.62 | 9.04 |
| gpu/timed/ms | ms | 8.59 | 10.61 | 8.72 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 822 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 822 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3427 | 5114 | 3190 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1283 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3427 | 5114 | 3190 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 201 | 4 |
| phases/shadow/bins | count | 4404 | 5427 | 1920 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.2 k | 12.6 k | 5438 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 194 | 233 | 19 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2661 | 3268 | 2350 |
| physics/items | count | 224 | 476 | 19 |
| physics/pending | count | 0 | 192 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 13 | 559 | 4 |
| preprocess/bind_groups_created | count | 13 | 559 | 4 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 226 | 125 |
| process/cpu_usage | % | 26.68 | 40.18 | 17.31 |
| process/mem_usage | GiB | 5.36 | 5.52 | 5.52 |
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
| started_load_count |  | 4914 | 5008 | 5008 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.29 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 32.86 | 67.85 | 30.57 |
| system/mem_usage | % | 54.59 | 55.24 | 54.19 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2679 | 2737 | 2737 |
| views/all/batch_sets | count | 67 | 248 | 54 |
| views/all/bins | count | 7004 | 8370 | 5418 |
| views/all/count | count | 45 | 53 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.2 k | 25.1 k | 13.1 k |
| views/all/pipelines | count | 240 | 280 | 65 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 10.9 k | 22.3 k | 8161 |
| views/cascade/batch_sets | count | 10 | 33 | 4 |
| views/cascade/bins | count | 2221 | 2877 | 1920 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7649 | 9641 | 5438 |
| views/cascade/pipelines | count | 24 | 33 | 19 |
| views/cascade/work_items | count | 522 | 9641 | 239 |
| views/lamp_face/batch_sets | count | 0 | 192 | 0 |
| views/lamp_face/bins | count | 2186 | 2802 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2431 | 3099 | 0 |
| views/lamp_face/pipelines | count | 171 | 204 | 0 |
| views/lamp_face/work_items | count | 0 | 2888 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2860 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8496 | 13.0 k | 7614 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8659 | 13.2 k | 7922 |
| views/other/batch_sets | count | 0 | 0 | 0 |
| views/other/bins | count | 0 | 0 | 0 |
| views/other/count | count | 0 | 0 | 0 |
| views/other/instance_batches | count | 0 | 0 | 0 |
| views/other/instance_draws | count | 0 | 0 | 0 |
| views/other/instance_slots | count | 0 | 0 | 0 |
| views/other/items | count | 0 | 0 | 0 |
| views/other/pipelines | count | 0 | 0 | 0 |
| views/other/work_items | count | 0 | 0 | 0 |
| world/meshes | count | 31.9 k | 37.8 k | 30.7 k |
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 58.33 | 268 | 327.3 k |
| anim/events/count | count | 0.10 | 3 | 563 |
| assets/Image/added | count | 0.11 | 35 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.72 | 152 | 4061 |
| assets/Mesh/removed | count | 0.86 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.86 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.07 | 48 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.83 | 1495 | 4675 |
| encode//count | count | 39.87 | 272 | 223.8 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5612 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5612 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5612 |
| physics/inserts | count | 0.40 | 87 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.42 | 145 | 2335 |
| physics/steps | count | 0.70 | 2 | 3938 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 5 |
| precache/hit | count | 0.00 | 2 | 5 |
| precache/missed | count | 0.00 | 2 | 5 |
| precache/requested | count | 0 | 0 | 0 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 19 | 378 |
| stream/upload/bytes | bytes | 6032 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1323 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 28.6 ms; slowest 41, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.56 | 2562.0 | main/busy/ms 2536, stage/main/post_update 2425, stage/main/update 104, surface/advance_cpu_ms 75.29, stage/main/pre_update 2.60, stage/main/last 0.99, surface/cell_upload_ms 0.70, stream/drive/ms 0.63, stage/main/first 0.52, stage/main/fixed_main_loop 0.48, stream/tick/ms 0.13, weather/map/ms 0.12 |
| warm-up | 3.27 | 703.7 | main/busy/ms 2671, stage/main/post_update 2448, render/thread/ms 674, render/busy/ms 511, stage/render/prepare_assets 475, stage/main/update 211, stage/render/prepare_views 161, render/acquire/ms 160, surface/advance_cpu_ms 75.29, stage/render/prepare_resources 22.02, stage/render/render 11.05, encode//sum 9.00 |
| warm-up | 3.34 | 75.9 | render/thread/ms 733, render/busy/ms 568, stage/render/prepare_assets 520, stage/render/prepare_views 162, render/acquire/ms 161, main/busy/ms 157, stage/main/update 112, stage/main/post_update 34.48, stage/render/prepare_resources 25.19, stage/render/render 17.77, terrain/update/ms 16.99, encode//sum 15.14 |
| warm-up | 3.37 | 28.8 | pipelines/compile_ms 238, pipelines/front_ms 217, render/thread/ms 86.01, pipelines/wait_ms 83.43, render/busy/ms 67.78, main/busy/ms 57.58, stage/render/prepare_assets 45.10, stage/render/render 27.24, stage/main/update 24.49, terrain/update/ms 23.03, stage/main/post_update 22.71, pipelines/driver_ms 20.78 |
| warm-up | 3.49 | 122.9 | pipelines/wait_ms 2902, pipelines/compile_ms 1109, pipelines/front_ms 1050, render/thread/ms 141, render/busy/ms 123, main/busy/ms 60.19, pipelines/driver_ms 59.47, stage/render/prepare_resources 46.67, stage/render/render 38.88, stage/render/prepare_assets 27.79, stage/main/update 22.02, encode//sum 21.08 |
| warm-up | 3.57 | 76.9 | pipelines/wait_ms 6277, pipelines/compile_ms 1825, pipelines/front_ms 1748, render/thread/ms 183, render/busy/ms 181, main/busy/ms 126, pipelines/driver_ms 76.99, stage/render/prepare_resources 64.18, stage/main/update 55.17, encode//sum 46.05, stage/render/render 45.87, render/graph/ms 43.41 |
| warm-up | 3.72 | 154.3 | pipelines/wait_ms 3763, pipelines/compile_ms 2244, pipelines/front_ms 2084, render/thread/ms 183, pipelines/driver_ms 159, render/busy/ms 144, main/busy/ms 115, stage/render/prepare_assets 74.17, stage/render/render 69.59, stage/main/update 56.82, render/present/ms 38.54, stage/main/pre_update 33.24 |
| warm-up | 3.76 | 37.1 | pipelines/compile_ms 2501, pipelines/front_ms 2218, pipelines/wait_ms 491, pipelines/driver_ms 283, render/thread/ms 149, render/busy/ms 110, stage/render/prepare_assets 81.81, stage/render/render 58.28, main/busy/ms 41.17, render/present/ms 38.81, render/graph/ms 18.76, stage/main/post_update 17.49 |
| warm-up | 3.85 | 33.7 | pipelines/compile_ms 227, pipelines/front_ms 221, render/thread/ms 42.85, render/busy/ms 40.36, stage/render/prepare_assets 23.65, main/busy/ms 18.14, stage/render/render 10.55, stage/main/post_update 9.37, terrain/update/ms 9.06, encode//sum 8.07, render/graph/ms 7.71, pipelines/driver_ms 6.27 |
| warm-up | 3.94 | 80.9 | pipelines/compile_ms 924, pipelines/front_ms 880, main/busy/ms 51.86, render/thread/ms 44.99, pipelines/driver_ms 43.91, render/busy/ms 43.50, stage/main/update 40.99, stream/drive/ms 36.47, stage/render/prepare_assets 29.87, stage/render/render 9.58, render/graph/ms 7.75, encode//sum 7.31 |
| warm-up | 4.04 | 68.6 | pipelines/compile_ms 624, pipelines/front_ms 601, render/thread/ms 55.40, main/busy/ms 55.32, render/busy/ms 53.19, stage/main/update 38.23, stream/drive/ms 33.63, stage/render/prepare_assets 30.36, pipelines/driver_ms 22.92, stage/render/render 13.14, render/graph/ms 10.34, stage/main/post_update 8.04 |
| warm-up | 4.07 | 34.6 | pipelines/compile_ms 261, pipelines/front_ms 252, render/thread/ms 65.40, render/busy/ms 62.77, main/busy/ms 42.86, stage/render/prepare_assets 34.77, stage/main/update 21.84, stage/render/render 15.87, stream/drive/ms 14.61, stage/main/post_update 13.36, render/graph/ms 12.89, pipelines/driver_ms 9.88 |
| warm-up | 4.19 | 120.2 | pipelines/compile_ms 1192, pipelines/front_ms 1133, render/thread/ms 111, render/busy/ms 78.73, main/busy/ms 68.20, pipelines/driver_ms 58.15, stage/render/prepare_assets 53.73, stage/main/update 43.05, stage/render/render 42.17, render/present/ms 31.71, stage/main/post_update 14.73, stream/drive/ms 14.06 |
| warm-up | 4.29 | 95.7 | pipelines/compile_ms 1605, pipelines/front_ms 1451, render/thread/ms 154, pipelines/driver_ms 154, render/busy/ms 123, stage/render/prepare_assets 106, main/busy/ms 47.60, stage/render/render 39.71, render/present/ms 31.38, stage/main/update 30.50, stage/main/post_update 8.88, render/graph/ms 7.58 |
| warm-up | 4.32 | 35.8 | pipelines/compile_ms 871, pipelines/front_ms 752, pipelines/driver_ms 119, render/thread/ms 103, render/busy/ms 102, stage/render/prepare_assets 67.22, main/busy/ms 46.21, stage/main/update 24.94, stage/render/render 22.57, render/graph/ms 20.57, stage/render/graph_submit 17.65, encode/submit 11.86 |
| warm-up | 4.41 | 84.0 | pipelines/compile_ms 573, pipelines/front_ms 543, render/thread/ms 112, render/busy/ms 110, main/busy/ms 64.65, stage/render/prepare_resources 38.46, stage/render/prepare_assets 38.29, stage/main/update 32.26, pipelines/driver_ms 29.50, pipelines/wait_ms 27.54, stage/render/render 21.21, stage/main/post_update 20.30 |
| warm-up | 4.45 | 41.9 | pipelines/compile_ms 190, pipelines/front_ms 179, render/thread/ms 114, render/busy/ms 112, main/busy/ms 60.10, stage/render/prepare_assets 44.34, stage/render/prepare_resources 41.77, stage/main/post_update 24.94, pipelines/wait_ms 24.90, stage/main/update 19.54, pipelines/driver_ms 10.94, stage/render/render 10.29 |
| warm-up | 4.51 | 64.4 | pipelines/compile_ms 2195, pipelines/front_ms 2140, render/thread/ms 82.79, render/busy/ms 81.20, pipelines/wait_ms 75.57, pipelines/driver_ms 55.62, main/busy/ms 53.14, stage/render/prepare_assets 30.16, stage/main/post_update 26.42, stage/main/update 13.55, stage/render/render 13.17, encode//sum 11.72 |
| warm-up | 4.59 | 75.5 | pipelines/compile_ms 2878, pipelines/front_ms 2797, pipelines/wait_ms 889, render/thread/ms 119, render/busy/ms 97.91, pipelines/driver_ms 80.95, main/busy/ms 78.49, stage/main/post_update 58.53, stage/render/prepare_assets 37.45, encode//sum 24.60, stage/render/prepare_views 20.10, render/acquire/ms 19.97 |
| warm-up | 4.63 | 43.6 | pipelines/wait_ms 1143, pipelines/compile_ms 950, pipelines/front_ms 916, main/busy/ms 160, render/thread/ms 110, stage/main/update 91.89, render/busy/ms 88.21, stage/main/post_update 58.82, stream/drive/ms 53.49, encode//sum 43.99, pipelines/driver_ms 33.49, stage/render/prepare_assets 31.65 |
| warm-up | 4.79 | 158.0 | pipelines/compile_ms 2584, pipelines/front_ms 2514, pipelines/wait_ms 1189, main/busy/ms 125, render/thread/ms 115, render/busy/ms 111, stage/main/update 93.42, pipelines/driver_ms 70.31, stream/drive/ms 54.66, encode//sum 52.83, stage/render/prepare_assets 43.64, stage/render/render 39.97 |
| warm-up | 4.83 | 37.8 | pipelines/compile_ms 2619, pipelines/front_ms 2548, pipelines/wait_ms 870, render/thread/ms 104, render/busy/ms 99.14, main/busy/ms 82.88, pipelines/driver_ms 70.84, stage/main/update 61.16, stream/drive/ms 50.68, stage/render/prepare_assets 41.20, encode//sum 32.44, stage/render/render 32.19 |
| warm-up | 4.93 | 99.5 | pipelines/compile_ms 985, pipelines/front_ms 958, main/busy/ms 88.23, render/thread/ms 72.44, render/busy/ms 70.50, pipelines/wait_ms 70.27, stage/main/update 65.50, stream/drive/ms 49.70, stage/render/prepare_assets 45.60, pipelines/driver_ms 27.24, stage/render/render 13.18, encode//sum 12.19 |
| warm-up | 4.99 | 60.1 | pipelines/compile_ms 1131, pipelines/front_ms 1064, render/thread/ms 97.73, main/busy/ms 91.18, stage/main/update 67.62, pipelines/driver_ms 67.35, render/busy/ms 65.14, pipelines/wait_ms 62.54, stage/render/render 44.90, stage/render/prepare_assets 43.68, stream/drive/ms 37.27, render/present/ms 32.40 |
| warm-up | 5.05 | 67.8 | pipelines/compile_ms 791, pipelines/front_ms 731, render/thread/ms 94.54, main/busy/ms 86.77, stage/main/update 68.64, render/busy/ms 61.98, pipelines/driver_ms 60.03, stage/render/render 45.81, stage/render/prepare_assets 37.92, stream/drive/ms 36.84, render/present/ms 32.36, encode//sum 16.83 |
| warm-up | 5.12 | 35.5 | pipelines/compile_ms 687, pipelines/front_ms 667, main/busy/ms 52.51, render/thread/ms 39.05, render/busy/ms 37.33, stage/main/update 36.77, stream/drive/ms 30.01, pipelines/driver_ms 20.08, encode//sum 15.35, stage/render/render 12.97, render/graph/ms 10.82, stage/render/prepare_assets 9.85 |
| warm-up | 5.22 | 28.8 | render/thread/ms 47.85, render/busy/ms 45.86, main/busy/ms 32.08, pipelines/compile_ms 20.47, pipelines/front_ms 18.93, stage/render/render 17.29, encode//sum 17.21, render/graph/ms 14.92, stage/main/update 13.58, stage/main/post_update 13.13, stage/render/prepare_assets 11.69, stage/render/graph_submit 11.03 |
| warm-up | 5.25 | 30.6 | pipelines/compile_ms 801, pipelines/front_ms 772, render/thread/ms 52.13, render/busy/ms 50.19, main/busy/ms 39.29, pipelines/driver_ms 28.75, stage/main/update 16.68, encode//sum 16.17, stage/render/render 15.13, stage/render/prepare_assets 14.69, stage/main/post_update 14.30, render/graph/ms 12.66 |
| warm-up | 5.29 | 45.4 | pipelines/compile_ms 1287, pipelines/front_ms 1249, render/thread/ms 61.99, render/busy/ms 60.32, main/busy/ms 39.35, pipelines/driver_ms 38.07, stage/render/prepare_assets 32.35, stage/main/update 20.42, stage/render/render 15.17, encode//sum 15.15, render/graph/ms 12.86, stage/main/post_update 11.17 |
| warm-up | 5.36 | 38.4 | pipelines/compile_ms 132, pipelines/front_ms 126, render/thread/ms 55.79, render/busy/ms 53.95, main/busy/ms 44.17, stage/main/post_update 20.44, stage/render/render 17.80, encode//sum 16.01, render/graph/ms 15.63, stage/main/update 14.38, stage/render/prepare_assets 13.55, stage/render/graph_submit 10.95 |
| warm-up | 5.39 | 35.0 | pipelines/compile_ms 306, pipelines/front_ms 296, render/thread/ms 63.78, render/busy/ms 62.05, main/busy/ms 41.98, stage/render/prepare_assets 28.49, stage/main/post_update 18.13, stage/main/update 16.07, encode//sum 16.02, stage/render/render 12.70, render/graph/ms 10.48, pipelines/driver_ms 9.08 |
| warm-up | 5.44 | 42.8 | pipelines/compile_ms 462, pipelines/front_ms 445, render/thread/ms 67.20, render/busy/ms 65.68, stage/render/prepare_assets 38.03, main/busy/ms 31.11, encode//sum 18.55, pipelines/driver_ms 17.47, stage/render/render 13.49, stage/main/post_update 12.66, stage/main/update 12.12, render/graph/ms 11.21 |
| warm-up | 5.48 | 47.4 | pipelines/compile_ms 864, pipelines/front_ms 828, render/thread/ms 83.36, render/busy/ms 81.81, pipelines/wait_ms 37.40, main/busy/ms 36.24, pipelines/driver_ms 35.86, stage/render/prepare_assets 34.39, encode//sum 18.10, stage/main/update 17.03, stage/render/prepare_write_phase_buffers 16.00, stage/render/render 13.98 |
| warm-up | 5.52 | 35.4 | pipelines/compile_ms 1673, pipelines/front_ms 1625, render/thread/ms 76.17, render/busy/ms 74.43, pipelines/driver_ms 47.80, main/busy/ms 35.92, stage/render/prepare_assets 35.27, pipelines/wait_ms 32.31, stage/main/update 16.62, stage/render/prepare_write_phase_buffers 14.85, encode//sum 14.64, stage/main/post_update 12.84 |
| warm-up | 5.58 | 40.6 | pipelines/compile_ms 601, pipelines/front_ms 586, render/thread/ms 46.83, render/busy/ms 44.80, main/busy/ms 40.48, stage/main/update 22.97, encode//sum 22.58, stage/render/render 18.25, stage/render/prepare_assets 17.38, render/graph/ms 15.45, pipelines/driver_ms 14.85, stream/drive/ms 13.21 |
| warm-up | 5.64 | 34.5 | pipelines/compile_ms 381, pipelines/front_ms 367, render/thread/ms 52.42, render/busy/ms 50.94, main/busy/ms 33.18, stage/render/prepare_assets 24.93, pipelines/wait_ms 21.15, stage/main/post_update 19.36, encode//sum 16.28, pipelines/driver_ms 14.05, stage/render/render 12.13, render/graph/ms 10.24 |
| warm-up | 6.17 | 31.8 | render/thread/ms 39.24, render/busy/ms 38.14, main/busy/ms 25.80, stage/render/prepare_assets 16.12, encode//sum 13.86, stage/main/update 10.59, stage/main/post_update 9.16, stage/render/render 8.65, render/graph/ms 7.24, stage/render/graph_submit 5.38, stage/render/specialize 4.15, stream/drive/ms 3.23 |
| warm-up | 6.42 | 30.4 | pipelines/compile_ms 82.14, pipelines/front_ms 78.48, render/thread/ms 41.07, render/busy/ms 39.99, main/busy/ms 33.28, stage/render/prepare_assets 20.30, encode//sum 16.33, stage/main/update 14.82, stage/main/post_update 10.76, stage/render/render 9.73, render/graph/ms 8.24, stage/render/graph_submit 6.09 |
| warm-up | 6.67 | 44.6 | render/thread/ms 46.03, render/busy/ms 44.78, main/busy/ms 34.98, stage/render/prepare_assets 25.55, stage/main/update 22.25, stream/drive/ms 16.48, encode//sum 15.79, stage/render/render 11.67, render/graph/ms 9.98, stage/main/post_update 9.35, stage/render/graph_submit 7.60, encode/submit 4.30 |
| measure | 59.02 | 29.5 | render/thread/ms 41.87, render/busy/ms 39.98, main/busy/ms 32.63, encode//sum 32.60, stage/render/render 20.19, stage/main/post_update 18.44, render/graph/ms 17.85, stage/render/graph_submit 13.95, stage/main/update 9.43, encode/finish_wall 7.26, encode/submit 6.87, stage/render/prepare_assets 5.83 |
| measure | 59.05 | 30.1 | render/thread/ms 44.75, render/busy/ms 43.35, main/busy/ms 26.61, encode//sum 24.08, stage/render/prepare_assets 15.08, stage/render/render 14.92, stage/main/post_update 14.65, render/graph/ms 13.21, stage/render/graph_submit 10.30, stage/main/update 7.33, encode/finish_wall 5.45, encode/submit 4.94 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_noslab\balmora_walk@2560x1440_off=meshslabs-1 --bench-baseline none --off meshslabs
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 4%
- during the run: other processes 6% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **77%** (min 61%) · 2580 MHz (min 2580) · memory 8001 MHz · 76.3 W · 72 °C (max 77) · VRAM 5271 MiB · throttled 100% of samples
