# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs,cascadeencoders,cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5044 frames over 65.6 s · **median 12.63 ms (79 fps)** · mean 13.01 · p95 15.80 · p99 18.07 (1% low 55 fps) · worst 35.57 ms · 37 spikes over 1.5× median · 17.4 stutters/min (work 17.4/min)
- warm-up: 324 frames over 9.5 s, worst 2548.8 ms
- hitches (> 31.6 ms): **1** measured, 36 in warm-up
- timed GPU passes: 7.40 ms of a 13.01 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.63 ms, limit 11 ms
- **FAIL** frame_ms: 19 stutters (> 1.5x moving median of 60) in 1.09 min = 17.37/min, limit 1/min
- **FAIL** worst frame 35.57 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.070 | 2.072 | 2.111 | 1.00 | 0.175 |
| main_opaque_pass_3d | 1.679 | 1.680 | 2.076 | 1.00 | 0.061 |
| early prepass | 0.524 | 0.525 | 0.752 | 1.00 | 0.079 |
| bloom | 0.320 | 0.320 | 0.326 | 1.00 | 0.062 |
| atmosphere_luts | 0.312 | 0.313 | 0.372 | 1.00 | 0.003 |
| water_velocity | 0.282 | 0.282 | 0.353 | 1.00 | 0.002 |
| water_sspr | 0.205 | 0.205 | 0.205 | 1.00 | 0.002 |
| ssao | 0.192 | 0.192 | 0.195 | 1.00 | 0.007 |
| uniform_allocation | 0.183 | 0.183 | 0.186 | 1.00 | 0.031 |
| auxiliary_views | 0.165 | 0.166 | 0.637 | 1.00 | 0.154 |
| atmos_froxel_inject | 0.149 | 0.149 | 0.151 | 1.00 | 0.001 |
| clouds_trace | 0.141 | 0.141 | 0.231 | 1.00 | 0.001 |
| bin_unpacking | 0.133 | 0.133 | 0.143 | 1.00 | 0.023 |
| water_fft | 0.133 | 0.133 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.120 | 0.120 | 0.128 | 1.00 | 0.004 |
| clouds_composite | 0.108 | 0.108 | 0.110 | 1.00 | 0.002 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.003 |
| main_transmissive_pass_3d | 0.086 | 0.086 | 0.166 | 1.00 | 0.008 |
| clustering | 0.065 | 0.065 | 0.068 | 1.00 | 0.018 |
| lightprobe_irradiance_map | 0.053 | 0.424 | 0.421 | 0.12 | 0.001 |
| render_sky | 0.052 | 0.052 | 0.053 | 1.00 | 0.002 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| surface_trail | 0.046 | 0.057 | 0.057 | 0.81 | 0.002 |
| early_mesh_preprocessing | 0.042 | 0.042 | 0.048 | 1.00 | 0.005 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.223 | 0.232 | 0.12 | 0.004 |
| water_snapshot | 0.024 | 0.024 | 0.027 | 1.00 | 0.002 |
| water_mips | 0.023 | 0.023 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.021 | 0.024 | 1.00 | 0.007 |
| main_transparent_pass_3d | 0.013 | 0.013 | 0.024 | 1.00 | 0.113 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.011 | 1.00 | 0.003 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.013 | 0.014 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 12.804 | 18.570 | 56.16 | 64581.4 |
| render/thread/ms | 11.756 | 14.411 | 27.10 | 59294.9 |
| render/busy/ms | 11.185 | 13.824 | 26.25 | 56418.8 |
| main/busy/ms | 9.665 | 12.375 | 28.40 | 48748.9 |
| stage/render/render | 7.794 | 9.820 | 17.96 | 39312.0 |
| render/graph/ms | 7.026 | 8.983 | 16.65 | 35440.4 |
| stage/main/post_update | 6.198 | 8.560 | 12.14 | 31262.0 |
| stage/render/graph_submit | 5.828 | 7.524 | 14.72 | 29395.5 |
| encode/finish_wall | 3.475 | 4.694 | 11.09 | 17528.3 |
| encode/submit | 2.401 | 3.220 | 6.25 | 12111.9 |
| stage/main/update | 2.167 | 2.647 | 22.28 | 10930.4 |
| stage/render/camera_driver | 1.139 | 1.533 | 4.22 | 5746.5 |
| stage/render/prepare_write_phase_buffers | 0.950 | 1.226 | 3.81 | 4793.4 |
| bigspace/total/ms | 0.590 | 1.940 | 4.54 | 2974.8 |
| stage/render/prepare_assets | 0.584 | 0.744 | 11.94 | 2948.1 |
| render/present/ms | 0.480 | 0.576 | 1.55 | 2420.0 |
| stage/main/first | 0.412 | 0.535 | 5.09 | 2079.2 |
| stage/render/prepare_resources | 0.397 | 0.513 | 1.09 | 2000.1 |
| stage/render/prepare_meshes | 0.394 | 0.514 | 1.91 | 1988.6 |
| stage/main/pre_update | 0.330 | 0.476 | 2.52 | 1664.0 |
| stage/render/specialize | 0.311 | 0.447 | 2.42 | 1569.2 |
| terrain/update/ms | 0.310 | 0.411 | 2.27 | 1564.5 |
| terrain/select/ms | 0.277 | 0.369 | 0.84 | 1395.4 |
| stage/render/queue | 0.276 | 0.435 | 2.53 | 1394.3 |
| controller/feed/ms | 0.271 | 0.352 | 0.68 | 1365.9 |
| stage/render/prepare_bind_groups | 0.257 | 0.433 | 2.75 | 1295.7 |
| stream/drive/ms | 0.255 | 0.349 | 20.55 | 1287.8 |
| stage/main/fixed_main_loop | 0.254 | 0.385 | 1.11 | 1282.5 |
| bigspace/low_precision/ms | 0.253 | 0.697 | 4.11 | 1275.7 |
| stream/tick/ms | 0.194 | 0.266 | 0.49 | 978.3 |
| far/select/ms | 0.190 | 0.264 | 0.54 | 958.5 |
| anim/evaluate/ms | 0.188 | 0.282 | 3.41 | 947.8 |
| bigspace/high_precision/ms | 0.184 | 0.649 | 3.97 | 929.0 |
| stage/render/prepare_flush | 0.174 | 0.339 | 0.89 | 880.2 |
| stage/render/extract_commands | 0.171 | 0.205 | 0.43 | 864.8 |
| stage/render/create_views | 0.170 | 0.225 | 0.87 | 857.9 |
| stage/render/prepare_batch_phases | 0.166 | 0.232 | 0.61 | 835.6 |
| anim/evaluate_cpu/ms | 0.144 | 0.185 | 0.47 | 728.7 |
| stage/render/prepare_views | 0.132 | 0.168 | 0.36 | 665.6 |
| stage/main/last | 0.124 | 0.198 | 1.78 | 623.6 |
| controller/move/ms | 0.122 | 0.260 | 0.42 | 614.6 |
| stage/render/phase_sort | 0.093 | 0.141 | 0.37 | 469.1 |
| render/acquire/ms | 0.090 | 0.117 | 0.30 | 456.1 |
| pipelines/compile_ms | 0.065 | 0.000 | 166.94 | 327.0 |
| pipelines/front_ms | 0.063 | 0.000 | 162.94 | 315.8 |
| physics/step/ms | 0.056 | 0.088 | 0.63 | 283.0 |
| stage/render/prepare_collect_phase_buffers | 0.048 | 0.076 | 0.25 | 240.6 |
| physics/insert/ms | 0.038 | 0.393 | 0.95 | 190.9 |
| stage/render/cleanup | 0.034 | 0.108 | 0.74 | 172.0 |
| encode/resolve_diagnostics/sum | 0.031 | 0.043 | 0.47 | 157.3 |
| weather/map/ms | 0.026 | 0.034 | 0.28 | 131.0 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.21 | 75.5 |
| anim/apply/ms | 0.012 | 0.021 | 0.23 | 58.4 |
| encode/camera_driver_span_end/sum | 0.011 | 0.019 | 0.14 | 54.0 |
| weather/eval/ms | 0.011 | 0.014 | 0.21 | 54.0 |
| controller/avoid/ms | 0.010 | 0.014 | 0.17 | 52.0 |
| script/update/ms | 0.007 | 0.009 | 0.04 | 34.5 |
| stream/despawn/ms | 0.006 | 0.000 | 1.17 | 29.0 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 25.7 |
| script/input/ms | 0.005 | 0.006 | 0.03 | 24.9 |
| script/ui/ms | 0.005 | 0.006 | 0.05 | 24.3 |
| controller/write/ms | 0.003 | 0.004 | 0.03 | 17.6 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.84 | 14.7 |
| pipelines/driver_ms | 0.002 | 0.000 | 4.00 | 11.2 |
| stream/activate/ms | 0.001 | 0.000 | 0.46 | 5.0 |
| weather/fx/ms | 0.001 | 0.001 | 0.01 | 4.6 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.8 |
| stage/render/prepare | 0.001 | 0.002 | 0.03 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.8 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.06 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.10 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.09 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 4 | 0 |
| anim/a2/count | count | 1 | 8 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 21 |
| controller/movers/count | count | 20 | 22 | 21 |
| crowd/c0/count | count | 26 | 43 | 12 |
| crowd/c0/joints/count | count | 1301 | 2262 | 535 |
| crowd/c0/meshes/count | count | 353 | 646 | 144 |
| crowd/c1/count | count | 145 | 155 | 148 |
| crowd/c1/joints/count | count | 5531 | 5949 | 5629 |
| crowd/c1/meshes/count | count | 1190 | 1281 | 1189 |
| crowd/c2/count | count | 386 | 418 | 418 |
| crowd/c2/joints/count | count | 6398 | 6936 | 6936 |
| crowd/c2/meshes/count | count | 1820 | 1988 | 1988 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 188 | 236 | 223 |
| crowd/joints_extracted/count | count | 4295 | 9951 | 6179 |
| crowd/joints_uploaded/count | count | 4258 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.89 | 10.22 | 2.63 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.21 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.14 | 0.02 |
| encode/finish_critical | ms | 2.89 | 10.22 | 2.63 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.47 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.93 | 28.40 | 9.49 |
| gpu/timed/ms | ms | 8.08 | 11.93 | 8.30 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 813 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 813 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1284 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3406 | 5116 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1284 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3406 | 5116 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 37 | 228 | 27 |
| phases/shadow/bins | count | 4659 | 5683 | 2045 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.2 k | 12.8 k | 5334 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 202 | 242 | 27 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2350 |
| physics/items | count | 220 | 476 | 19 |
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 303 | 304 | 304 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 11 | 615 | 6 |
| preprocess/bind_groups_created | count | 11 | 615 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 176 | 288 | 171 |
| process/cpu_usage | % | 25.24 | 52.73 | 22.27 |
| process/mem_usage | GiB | 5.36 | 5.53 | 5.53 |
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
| started_load_count |  | 4915 | 5008 | 5008 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 0 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.09 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.58 | 66.54 | 30.36 |
| system/mem_usage | % | 71.54 | 72.27 | 72.20 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2886 | 3001 | 3001 |
| textures/images | count | 2677 | 2737 | 2737 |
| views/all/batch_sets | count | 84 | 275 | 77 |
| views/all/bins | count | 7196 | 8574 | 5543 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.2 k | 25.2 k | 12.9 k |
| views/all/pipelines | count | 247 | 289 | 73 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 17.1 k | 22.7 k | 13.3 k |
| views/cascade/batch_sets | count | 32 | 44 | 27 |
| views/cascade/bins | count | 2437 | 3004 | 2045 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 7788 | 9784 | 5334 |
| views/cascade/pipelines | count | 32 | 44 | 27 |
| views/cascade/work_items | count | 7788 | 9784 | 5334 |
| views/lamp_face/batch_sets | count | 0 | 194 | 0 |
| views/lamp_face/bins | count | 2179 | 2789 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2428 | 3083 | 0 |
| views/lamp_face/pipelines | count | 170 | 204 | 0 |
| views/lamp_face/work_items | count | 0 | 2903 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2862 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8430 | 13.0 k | 7612 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8593 | 13.2 k | 7920 |
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
| world/point_lights | count | 286 | 353 | 289 |

## Churn (per frame)

| counter | unit | mean | max | total |
|---|---|---:|---:|---:|
| anim/bones_written/count | count | 57.72 | 246 | 291.2 k |
| anim/events/count | count | 0.11 | 3 | 570 |
| assets/Image/added | count | 0.12 | 33 | 592 |
| assets/Image/modified | count | 0.00 | 1 | 16 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.81 | 152 | 4061 |
| assets/Mesh/removed | count | 0.96 | 1226 | 4842 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 464 |
| assets/StandardMaterial/modified | count | 4.29 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.93 | 1562 | 4675 |
| encode//count | count | 40.04 | 272 | 202.0 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5044 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5044 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5044 |
| physics/inserts | count | 0.44 | 111 | 2241 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.46 | 145 | 2335 |
| physics/steps | count | 0.78 | 2 | 3939 |
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
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6711 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1471 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.6 ms; slowest 37, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.55 | 2548.8 | main/busy/ms 2518, stage/main/post_update 2415, stage/main/update 96.79, surface/advance_cpu_ms 67.18, stage/main/pre_update 2.33, stage/main/last 1.10, surface/cell_upload_ms 0.84, stage/main/first 0.55, stage/main/fixed_main_loop 0.45, stream/drive/ms 0.39, weather/map/ms 0.14, bigspace/total/ms 0.09 |
| warm-up | 3.29 | 743.1 | main/busy/ms 2652, stage/main/post_update 2439, render/thread/ms 712, render/busy/ms 570, stage/render/prepare_assets 514, stage/main/update 201, stage/render/prepare_views 139, render/acquire/ms 139, surface/advance_cpu_ms 67.18, stage/render/create_views 34.64, stage/render/render 11.98, encode//sum 9.63 |
| warm-up | 3.38 | 86.4 | render/thread/ms 782, render/busy/ms 638, stage/render/prepare_assets 560, main/busy/ms 159, stage/render/prepare_views 140, render/acquire/ms 139, stage/main/update 110, pipelines/compile_ms 92.25, pipelines/front_ms 88.20, stage/main/post_update 36.24, stage/render/create_views 34.93, stage/render/render 26.81 |
| warm-up | 3.42 | 43.4 | pipelines/compile_ms 245, pipelines/front_ms 228, pipelines/wait_ms 191, render/thread/ms 112, render/busy/ms 98.33, main/busy/ms 64.75, stage/render/prepare_assets 47.97, stage/render/render 47.55, stage/main/post_update 43.35, render/graph/ms 33.68, terrain/update/ms 24.91, pipelines/driver_ms 17.58 |
| warm-up | 3.56 | 139.9 | pipelines/wait_ms 3521, pipelines/compile_ms 1522, pipelines/front_ms 1404, render/thread/ms 180, render/busy/ms 168, pipelines/driver_ms 118, main/busy/ms 108, stage/render/prepare_resources 70.76, stage/main/post_update 54.14, stage/render/render 38.98, stage/main/fixed_main_loop 36.63, render/graph/ms 26.25 |
| warm-up | 3.61 | 51.5 | pipelines/wait_ms 4747, pipelines/compile_ms 2326, pipelines/front_ms 2173, render/thread/ms 188, render/busy/ms 186, pipelines/driver_ms 153, main/busy/ms 147, stage/render/prepare_resources 79.15, stage/main/fixed_main_loop 53.89, stage/main/update 51.97, stage/main/post_update 32.55, stage/render/prepare_assets 24.90 |
| warm-up | 3.75 | 140.1 | pipelines/wait_ms 1636, pipelines/compile_ms 1602, pipelines/front_ms 1493, render/thread/ms 154, render/busy/ms 113, pipelines/driver_ms 110, main/busy/ms 99.97, stage/render/prepare_assets 74.13, stage/render/render 57.88, stage/main/update 55.86, render/present/ms 41.34, stage/main/post_update 21.01 |
| warm-up | 3.79 | 35.5 | pipelines/compile_ms 1879, pipelines/front_ms 1644, pipelines/wait_ms 487, pipelines/driver_ms 235, render/thread/ms 139, render/busy/ms 96.78, stage/render/prepare_assets 67.27, stage/render/render 62.66, main/busy/ms 46.25, render/present/ms 41.50, encode//sum 29.94, stage/main/post_update 29.07 |
| warm-up | 3.95 | 63.9 | pipelines/compile_ms 1057, pipelines/front_ms 1019, render/thread/ms 57.96, render/busy/ms 55.74, main/busy/ms 46.73, stage/render/prepare_assets 38.90, pipelines/driver_ms 37.53, stage/main/update 34.53, stream/drive/ms 30.09, stage/render/render 12.21, render/graph/ms 9.79, stage/main/post_update 8.23 |
| warm-up | 3.99 | 38.0 | pipelines/compile_ms 638, pipelines/front_ms 616, render/thread/ms 67.95, main/busy/ms 62.00, render/busy/ms 61.71, stage/main/update 46.87, stage/render/prepare_assets 42.65, stream/drive/ms 42.43, pipelines/driver_ms 21.66, stage/render/render 14.43, render/graph/ms 11.22, stage/render/graph_submit 9.23 |
| warm-up | 4.07 | 81.0 | pipelines/compile_ms 733, pipelines/front_ms 706, render/thread/ms 83.81, render/busy/ms 77.06, main/busy/ms 64.05, stage/render/prepare_assets 53.19, stage/main/update 47.80, stream/drive/ms 43.70, pipelines/driver_ms 26.62, stage/render/render 17.46, render/graph/ms 13.69, stage/render/graph_submit 9.76 |
| warm-up | 4.11 | 41.3 | pipelines/compile_ms 718, pipelines/front_ms 694, render/thread/ms 89.97, render/busy/ms 86.31, stage/render/prepare_assets 48.75, main/busy/ms 41.87, pipelines/driver_ms 23.75, stage/render/render 18.65, stage/main/update 18.30, stage/main/post_update 17.26, render/graph/ms 14.79, stream/drive/ms 13.21 |
| warm-up | 4.19 | 81.2 | pipelines/compile_ms 1437, pipelines/front_ms 1396, render/thread/ms 78.45, render/busy/ms 73.70, main/busy/ms 61.50, pipelines/driver_ms 41.31, stage/render/prepare_assets 38.15, stage/main/update 36.69, stage/render/render 17.93, stage/main/post_update 16.91, render/graph/ms 12.93, stream/drive/ms 11.56 |
| warm-up | 4.31 | 116.9 | pipelines/compile_ms 1659, pipelines/front_ms 1612, render/thread/ms 132, render/busy/ms 129, stage/render/prepare_assets 71.70, main/busy/ms 50.55, pipelines/driver_ms 46.67, stage/render/prepare_bind_groups 35.84, stage/main/update 32.06, stage/render/render 16.68, render/graph/ms 12.81, stage/render/graph_submit 10.71 |
| warm-up | 4.37 | 62.7 | pipelines/compile_ms 1235, pipelines/front_ms 1176, render/thread/ms 146, render/busy/ms 144, main/busy/ms 68.28, pipelines/driver_ms 58.29, stage/render/prepare_assets 49.06, stage/render/prepare_bind_groups 39.90, pipelines/wait_ms 35.72, stage/render/prepare_flush 21.65, stage/main/update 20.99, stage/render/render 19.13 |
| warm-up | 4.45 | 76.9 | pipelines/compile_ms 1327, pipelines/front_ms 1263, render/thread/ms 122, render/busy/ms 120, main/busy/ms 71.40, pipelines/driver_ms 64.15, stage/render/prepare_assets 42.10, pipelines/wait_ms 33.77, stage/main/update 25.33, stage/render/prepare_flush 23.31, stage/main/post_update 18.60, stage/render/render 16.08 |
| warm-up | 4.52 | 65.9 | pipelines/compile_ms 1487, pipelines/front_ms 1445, render/thread/ms 122, render/busy/ms 120, pipelines/wait_ms 55.80, stage/render/prepare_assets 46.73, pipelines/driver_ms 42.85, main/busy/ms 42.46, stage/render/render 38.43, render/graph/ms 35.37, stage/render/graph_submit 31.62, encode/finish_wall 26.62 |
| warm-up | 4.60 | 78.7 | pipelines/compile_ms 2432, pipelines/front_ms 2369, pipelines/wait_ms 738, render/thread/ms 126, render/busy/ms 124, pipelines/driver_ms 63.40, main/busy/ms 59.47, stage/render/render 44.62, stage/main/update 44.40, render/graph/ms 41.78, stage/render/graph_submit 33.76, encode/finish_wall 28.16 |
| warm-up | 4.67 | 79.2 | pipelines/compile_ms 2008, pipelines/front_ms 1953, pipelines/wait_ms 936, render/thread/ms 145, render/busy/ms 143, main/busy/ms 92.98, pipelines/driver_ms 55.78, stage/main/update 51.57, encode//sum 43.84, stage/render/prepare_resources 36.47, stage/main/post_update 33.56, stage/render/prepare_assets 31.53 |
| warm-up | 4.72 | 49.4 | pipelines/compile_ms 1469, pipelines/front_ms 1425, pipelines/wait_ms 474, main/busy/ms 148, render/thread/ms 120, render/busy/ms 118, encode//sum 117, stage/main/update 97.08, stream/drive/ms 44.98, pipelines/driver_ms 43.52, stage/render/render 41.46, render/graph/ms 38.68 |
| warm-up | 4.85 | 123.7 | pipelines/compile_ms 1829, pipelines/front_ms 1781, pipelines/wait_ms 529, main/busy/ms 112, render/thread/ms 101, render/busy/ms 98.63, encode//sum 98.10, stage/main/update 85.53, pipelines/driver_ms 47.63, stream/drive/ms 45.84, stage/render/prepare_assets 34.08, stage/render/render 33.44 |
| warm-up | 4.89 | 40.1 | pipelines/compile_ms 1513, pipelines/front_ms 1476, pipelines/wait_ms 313, render/thread/ms 90.14, render/busy/ms 88.18, main/busy/ms 81.65, stage/main/update 63.53, stream/drive/ms 49.66, stage/render/prepare_assets 38.90, pipelines/driver_ms 37.48, stage/render/render 23.40, render/graph/ms 20.61 |
| warm-up | 5.00 | 110.1 | pipelines/compile_ms 845, pipelines/front_ms 810, main/busy/ms 94.07, render/thread/ms 80.74, render/busy/ms 79.05, stage/main/update 68.69, stream/drive/ms 49.96, stage/render/prepare_assets 46.08, pipelines/driver_ms 35.32, stage/render/render 20.15, render/graph/ms 17.81, stage/main/post_update 15.87 |
| warm-up | 5.08 | 86.6 | pipelines/compile_ms 1445, pipelines/front_ms 1315, pipelines/driver_ms 130, render/thread/ms 127, render/busy/ms 126, stage/render/prepare_assets 102, main/busy/ms 88.16, stage/main/update 61.81, stream/drive/ms 41.81, encode//sum 18.17, stage/main/post_update 17.42, stage/render/render 13.63 |
| warm-up | 5.13 | 45.6 | pipelines/compile_ms 1334, pipelines/front_ms 1216, pipelines/driver_ms 118, render/thread/ms 112, render/busy/ms 111, stage/render/prepare_assets 86.37, main/busy/ms 72.73, stage/main/update 53.79, stream/drive/ms 41.30, encode//sum 19.18, stage/render/render 14.93, pipelines/wait_ms 13.02 |
| warm-up | 5.22 | 65.5 | pipelines/compile_ms 632, pipelines/front_ms 611, main/busy/ms 66.48, stage/main/update 51.10, render/thread/ms 49.56, render/busy/ms 47.67, stream/drive/ms 32.82, pipelines/driver_ms 21.43, encode//sum 18.26, stage/render/render 16.60, stage/render/prepare_assets 15.26, render/graph/ms 14.13 |
| warm-up | 5.41 | 63.7 | pipelines/compile_ms 543, pipelines/front_ms 527, render/thread/ms 56.78, render/busy/ms 54.66, main/busy/ms 47.81, stage/render/prepare_assets 28.54, stage/main/update 27.42, stage/render/render 17.64, encode//sum 16.71, pipelines/driver_ms 16.06, render/graph/ms 14.95, stage/main/post_update 13.61 |
| warm-up | 5.45 | 40.8 | pipelines/compile_ms 773, pipelines/front_ms 750, render/thread/ms 70.62, render/busy/ms 68.83, stage/render/prepare_assets 41.17, main/busy/ms 38.76, pipelines/driver_ms 22.94, stage/main/update 20.48, encode//sum 17.68, stage/render/render 15.74, render/graph/ms 13.26, stream/drive/ms 12.22 |
| warm-up | 5.49 | 39.2 | pipelines/compile_ms 495, pipelines/front_ms 478, render/thread/ms 65.49, render/busy/ms 63.73, main/busy/ms 35.93, stage/render/prepare_assets 34.42, stage/main/update 18.74, encode//sum 18.01, pipelines/driver_ms 17.01, stage/render/render 16.31, render/graph/ms 13.94, stream/drive/ms 11.11 |
| warm-up | 5.52 | 31.9 | pipelines/compile_ms 710, pipelines/front_ms 688, render/thread/ms 61.05, render/busy/ms 59.40, main/busy/ms 36.74, stage/render/prepare_assets 27.99, pipelines/driver_ms 22.58, encode//sum 19.21, stage/render/render 16.46, render/graph/ms 14.32, stage/main/post_update 12.69, stage/main/update 11.92 |
| warm-up | 5.56 | 34.8 | pipelines/compile_ms 873, pipelines/front_ms 847, render/thread/ms 59.50, render/busy/ms 57.92, main/busy/ms 36.13, stage/render/prepare_assets 30.33, pipelines/driver_ms 25.57, encode//sum 19.84, stage/render/render 15.12, stage/main/post_update 13.54, render/graph/ms 12.97, stage/main/update 10.23 |
| warm-up | 5.66 | 32.4 | pipelines/compile_ms 779, pipelines/front_ms 762, render/thread/ms 50.29, render/busy/ms 48.59, main/busy/ms 29.67, stage/render/prepare_assets 23.35, encode//sum 19.65, pipelines/driver_ms 17.53, stage/main/post_update 15.93, stage/render/render 15.05, render/graph/ms 12.76, stage/render/graph_submit 10.00 |
| warm-up | 6.27 | 33.8 | render/thread/ms 41.93, render/busy/ms 40.68, main/busy/ms 27.41, encode//sum 17.12, stage/render/prepare_assets 15.66, stage/render/render 11.93, stage/main/update 11.32, render/graph/ms 10.32, stage/main/post_update 9.35, stage/render/graph_submit 8.16, encode/finish_wall 4.66, stage/render/specialize 4.28 |
| warm-up | 6.48 | 33.5 | render/thread/ms 46.04, render/busy/ms 43.95, main/busy/ms 43.28, encode//sum 26.87, stage/render/render 22.75, stage/main/update 21.44, render/graph/ms 19.72, stage/render/graph_submit 16.20, stage/main/post_update 14.56, encode/submit 8.33, encode/finish_wall 8.03, stream/drive/ms 5.77 |
| warm-up | 6.55 | 35.5 | pipelines/compile_ms 82.71, pipelines/front_ms 78.45, render/thread/ms 48.24, render/busy/ms 47.10, main/busy/ms 32.62, stage/render/prepare_assets 22.49, encode//sum 20.31, stage/main/update 14.05, stage/render/render 12.24, render/graph/ms 10.52, stage/main/post_update 10.48, stage/render/graph_submit 8.12 |
| warm-up | 6.85 | 41.2 | render/thread/ms 64.43, render/busy/ms 63.02, main/busy/ms 41.78, stage/render/prepare_assets 38.26, stage/main/update 26.44, encode//sum 26.19, stream/drive/ms 17.60, stage/render/render 16.20, render/graph/ms 14.15, stage/render/graph_submit 11.34, stage/main/post_update 10.15, encode/finish_wall 7.27 |
| measure | 59.07 | 35.6 | render/thread/ms 44.73, render/busy/ms 43.33, encode//sum 25.63, main/busy/ms 22.64, stage/render/prepare_assets 16.35, stage/render/render 15.90, render/graph/ms 14.19, stage/main/post_update 12.42, stage/render/graph_submit 11.43, encode/finish_wall 6.98, stage/main/update 5.21, encode/submit 4.55 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\f_off\balmora_walk@2560x1440_off=cascadeencoders_cascadehold_meshslabs-1 --bench-baseline none --off cascadeencoders,cascadehold,meshslabs
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_SUN_CASCADES=3:205
- before the run: GPU 0%, CPU 1%
- during the run: other processes 6% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (251 samples): GPU load **72%** (min 49%) · 2580 MHz (min 2580) · memory 8001 MHz · 73.0 W · 70 °C (max 75) · VRAM 5944 MiB · throttled 100% of samples
