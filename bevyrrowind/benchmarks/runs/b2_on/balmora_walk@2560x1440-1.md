# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 5049 frames over 65.7 s · **median 12.67 ms (79 fps)** · mean 13.00 · p95 16.60 · p99 19.50 (1% low 51 fps) · worst 74.32 ms · 63 spikes over 1.5× median · 32.0 stutters/min (work 28.3/min)
- warm-up: 1939 frames over 28.9 s, worst 4175.6 ms
- hitches (> 31.7 ms): **4** measured, 38 in warm-up
- timed GPU passes: 7.54 ms of a 13.00 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.67 ms, limit 11 ms
- **FAIL** frame_ms: 35 stutters (> 1.5x moving median of 60) in 1.09 min = 31.99/min, limit 1/min
- **FAIL** worst frame 74.32 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.115 | 2.197 | 2.828 | 0.96 | 0.188 |
| main_opaque_pass_3d | 1.688 | 1.753 | 2.673 | 0.96 | 0.057 |
| early prepass | 0.523 | 0.543 | 0.953 | 0.96 | 0.082 |
| bloom | 0.325 | 0.338 | 0.492 | 0.96 | 0.070 |
| atmosphere_luts | 0.317 | 0.329 | 0.459 | 0.96 | 0.004 |
| water_velocity | 0.275 | 0.285 | 0.361 | 0.96 | 0.002 |
| water_sspr | 0.224 | 0.233 | 0.251 | 0.96 | 0.002 |
| auxiliary_views | 0.203 | 0.211 | 0.711 | 0.96 | 0.192 |
| ssao | 0.197 | 0.204 | 0.208 | 0.96 | 0.008 |
| uniform_allocation | 0.177 | 0.184 | 0.191 | 0.96 | 0.035 |
| atmos_froxel_inject | 0.153 | 0.159 | 0.154 | 0.96 | 0.001 |
| clouds_trace | 0.137 | 0.142 | 0.236 | 0.96 | 0.001 |
| bin_unpacking | 0.136 | 0.141 | 0.148 | 0.96 | 0.025 |
| water_fft | 0.131 | 0.136 | 0.139 | 0.96 | 0.003 |
| upscaling | 0.120 | 0.124 | 0.131 | 0.96 | 0.005 |
| clouds_composite | 0.107 | 0.111 | 0.111 | 0.96 | 0.002 |
| tonemapping | 0.088 | 0.091 | 0.091 | 0.96 | 0.004 |
| main_transmissive_pass_3d | 0.087 | 0.090 | 0.174 | 0.96 | 0.009 |
| clustering | 0.069 | 0.071 | 0.070 | 0.96 | 0.021 |
| surface_trail | 0.054 | 0.068 | 0.060 | 0.79 | 0.002 |
| render_sky | 0.053 | 0.055 | 0.055 | 0.96 | 0.002 |
| lightprobe_irradiance_map | 0.051 | 0.441 | 0.676 | 0.12 | 0.001 |
| clouds_resolve | 0.048 | 0.050 | 0.050 | 0.96 | 0.001 |
| early_mesh_preprocessing | 0.048 | 0.049 | 0.049 | 0.96 | 0.006 |
| water_snapshot | 0.036 | 0.038 | 0.042 | 0.96 | 0.002 |
| clouds_shadow | 0.030 | 0.031 | 0.034 | 0.96 | 0.003 |
| atmos_froxel_integrate | 0.027 | 0.028 | 0.030 | 0.96 | 0.001 |
| lightprobe_radiance_map | 0.026 | 0.226 | 0.235 | 0.12 | 0.004 |
| water_mips | 0.023 | 0.024 | 0.024 | 0.96 | 0.002 |
| per_view_shadow_pass | 0.021 | 0.022 | 0.023 | 0.96 | 0.007 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.030 | 0.96 | 0.116 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.96 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.96 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.96 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.96 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 57.600 | 0.000 | 44640.54 | 290823.6 |
| encode//sum | 12.070 | 18.529 | 96.04 | 60940.9 |
| render/thread/ms | 11.478 | 14.817 | 72.98 | 57952.0 |
| render/busy/ms | 10.864 | 14.151 | 72.38 | 54850.3 |
| main/busy/ms | 9.815 | 12.457 | 29.39 | 49556.0 |
| render/graph/ms | 6.701 | 8.883 | 68.41 | 33835.0 |
| stage/main/post_update | 6.159 | 8.356 | 12.69 | 31094.7 |
| stage/render/graph_submit | 5.411 | 7.170 | 66.80 | 27321.7 |
| stage/render/render | 4.255 | 9.078 | 25.46 | 21483.6 |
| pipelines/compile_ms | 3.736 | 0.000 | 3062.87 | 18864.3 |
| pipelines/driver_ms | 3.464 | 0.000 | 2921.47 | 17490.3 |
| stage/render/cleanup | 3.269 | 9.044 | 69.22 | 16507.3 |
| encode/finish_wall | 2.954 | 4.140 | 14.34 | 14915.6 |
| encode/submit | 2.530 | 3.489 | 63.24 | 12775.0 |
| stage/main/update | 2.279 | 2.833 | 23.03 | 11505.5 |
| stage/render/camera_driver | 1.227 | 1.710 | 4.21 | 6196.1 |
| stage/render/prepare_write_phase_buffers | 0.961 | 1.307 | 2.94 | 4852.6 |
| stage/render/prepare_assets | 0.621 | 0.867 | 8.09 | 3133.7 |
| bigspace/total/ms | 0.579 | 1.684 | 4.80 | 2922.2 |
| render/present/ms | 0.510 | 0.687 | 2.19 | 2574.3 |
| stage/render/prepare_resources | 0.459 | 0.588 | 0.90 | 2315.8 |
| stage/main/first | 0.430 | 0.569 | 5.71 | 2173.5 |
| stage/render/prepare_meshes | 0.422 | 0.584 | 1.95 | 2129.3 |
| stage/main/pre_update | 0.355 | 0.535 | 1.63 | 1793.4 |
| terrain/update/ms | 0.309 | 0.396 | 1.84 | 1561.8 |
| stage/render/specialize | 0.291 | 0.428 | 2.29 | 1471.6 |
| controller/feed/ms | 0.287 | 0.402 | 0.93 | 1450.6 |
| stage/render/queue | 0.286 | 0.465 | 2.78 | 1442.4 |
| stage/render/prepare_bind_groups | 0.276 | 0.498 | 3.09 | 1391.3 |
| terrain/select/ms | 0.274 | 0.351 | 0.66 | 1384.3 |
| stream/drive/ms | 0.273 | 0.399 | 20.78 | 1378.4 |
| pipelines/front_ms | 0.272 | 0.000 | 163.45 | 1374.0 |
| stage/main/fixed_main_loop | 0.272 | 0.449 | 1.16 | 1373.9 |
| bigspace/low_precision/ms | 0.250 | 0.672 | 4.43 | 1260.6 |
| stream/tick/ms | 0.206 | 0.299 | 0.68 | 1040.8 |
| far/select/ms | 0.204 | 0.294 | 0.73 | 1028.6 |
| stage/render/create_views | 0.199 | 0.267 | 8.11 | 1002.3 |
| anim/evaluate_cpu/ms | 0.185 | 0.237 | 0.59 | 933.6 |
| stage/render/prepare_batch_phases | 0.185 | 0.270 | 0.56 | 933.0 |
| stage/render/prepare_flush | 0.166 | 0.305 | 0.83 | 840.7 |
| stage/render/extract_commands | 0.166 | 0.215 | 0.50 | 839.0 |
| bigspace/high_precision/ms | 0.164 | 0.533 | 3.83 | 826.4 |
| stage/render/prepare_views | 0.150 | 0.202 | 6.27 | 755.3 |
| controller/move/ms | 0.135 | 0.234 | 0.72 | 680.8 |
| stage/main/last | 0.124 | 0.216 | 1.73 | 628.0 |
| anim/evaluate/ms | 0.108 | 0.206 | 3.12 | 546.0 |
| render/acquire/ms | 0.104 | 0.144 | 6.21 | 527.3 |
| stage/render/prepare_collect_phase_buffers | 0.064 | 0.140 | 0.52 | 325.4 |
| physics/step/ms | 0.060 | 0.104 | 0.70 | 304.6 |
| physics/insert/ms | 0.040 | 0.401 | 1.19 | 199.7 |
| encode/resolve_diagnostics/sum | 0.034 | 0.050 | 0.48 | 173.7 |
| stage/render/phase_sort | 0.032 | 0.058 | 0.21 | 162.8 |
| weather/map/ms | 0.026 | 0.036 | 0.26 | 132.0 |
| encode/camera_driver_span_begin/sum | 0.016 | 0.024 | 0.43 | 79.7 |
| encode/camera_driver_span_end/sum | 0.013 | 0.021 | 7.90 | 65.1 |
| anim/apply/ms | 0.012 | 0.024 | 0.26 | 61.6 |
| weather/eval/ms | 0.011 | 0.015 | 0.41 | 56.6 |
| controller/avoid/ms | 0.011 | 0.015 | 0.10 | 55.7 |
| script/update/ms | 0.008 | 0.012 | 0.25 | 39.5 |
| stream/despawn/ms | 0.006 | 0.000 | 0.97 | 30.6 |
| script/ui/ms | 0.006 | 0.008 | 0.04 | 28.9 |
| script/input/ms | 0.006 | 0.007 | 0.18 | 27.9 |
| stream/spawn/ms | 0.005 | 0.000 | 1.19 | 25.8 |
| controller/write/ms | 0.004 | 0.005 | 0.04 | 18.2 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.65 | 14.8 |
| stream/activate/ms | 0.001 | 0.000 | 0.48 | 5.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 5.2 |
| shapes/activate/ms | 0.001 | 0.001 | 0.04 | 4.1 |
| stage/render/prepare | 0.001 | 0.003 | 0.17 | 3.1 |
| weather/apply/ms | 0.000 | 0.000 | 0.00 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.03 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 5 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 566 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 29 | 18 |
| controller/movers/count | count | 20 | 23 | 18 |
| crowd/c0/count | count | 27 | 43 | 15 |
| crowd/c0/joints/count | count | 1301 | 2262 | 723 |
| crowd/c0/meshes/count | count | 353 | 646 | 193 |
| crowd/c1/count | count | 146 | 156 | 146 |
| crowd/c1/joints/count | count | 5518 | 5991 | 5473 |
| crowd/c1/meshes/count | count | 1163 | 1295 | 1148 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4276 | 10.0 k | 6179 |
| crowd/joints_uploaded/count | count | 4251 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.39 | 9.26 | 1.90 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.43 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 7.90 | 0.01 |
| encode/finish_critical | ms | 2.39 | 9.26 | 1.90 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.48 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.70 | 72.38 | 9.41 |
| gpu/timed/ms | ms | 8.40 | 11.93 | 9.26 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2526 | 2583 | 2583 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 545.28 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 823 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 194 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 823 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1282 | 1393 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3425 | 5128 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1282 | 1393 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3425 | 5128 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 38 | 225 | 29 |
| phases/shadow/bins | count | 4838 | 5887 | 2248 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 13.2 k | 5699 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 206 | 245 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2645 | 3268 | 2347 |
| physics/items | count | 221 | 476 | 19 |
| physics/pending | count | 0 | 200 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 180 | 182 | 182 |
| pipelines/waiting | count | 0 | 13 | 0 |
| preprocess/bind_groups | count | 11 | 609 | 6 |
| preprocess/bind_groups_created | count | 11 | 609 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 181 | 279 | 175 |
| process/cpu_usage | % | 25.96 | 49.79 | 21.88 |
| process/mem_usage | GiB | 5.27 | 5.50 | 5.50 |
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
| started_load_count |  | 4769 | 4864 | 4864 |
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
| stream/fetch/queue | count | 0 | 2 | 0 |
| stream/latency/s | s | 0.04 | 0.12 | 0.04 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.39 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 39.36 | 73.98 | 44.59 |
| system/mem_usage | % | 71.95 | 73.08 | 72.73 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2934 | 2934 |
| textures/images | count | 2592 | 2645 | 2645 |
| views/all/batch_sets | count | 87 | 272 | 79 |
| views/all/bins | count | 7435 | 8757 | 5746 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.6 k | 25.7 k | 13.3 k |
| views/all/pipelines | count | 250 | 292 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 17.6 k | 23.6 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 29 |
| views/cascade/bins | count | 2624 | 3170 | 2248 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8172 | 10.2 k | 5699 |
| views/cascade/pipelines | count | 35 | 47 | 29 |
| views/cascade/work_items | count | 8172 | 10.2 k | 5699 |
| views/lamp_face/batch_sets | count | 0 | 190 | 0 |
| views/lamp_face/bins | count | 2171 | 2808 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2423 | 3102 | 0 |
| views/lamp_face/pipelines | count | 170 | 202 | 0 |
| views/lamp_face/work_items | count | 0 | 2846 | 0 |
| views/main/batch_sets | count | 49 | 53 | 50 |
| views/main/bins | count | 2860 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8496 | 13.0 k | 7610 |
| views/main/pipelines | count | 45 | 49 | 46 |
| views/main/work_items | count | 8661 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 54.12 | 269 | 273.3 k |
| anim/events/count | count | 0.11 | 3 | 573 |
| assets/Image/added | count | 0.12 | 22 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 14 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.81 | 152 | 4075 |
| assets/Mesh/removed | count | 0.97 | 1238 | 4878 |
| assets/StandardMaterial/added | count | 0.09 | 35 | 466 |
| assets/StandardMaterial/modified | count | 4.29 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.08 | 38 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.93 | 1485 | 4674 |
| encode//count | count | 41.61 | 272 | 210.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 5049 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 5049 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 5049 |
| physics/inserts | count | 0.48 | 106 | 2420 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.46 | 145 | 2335 |
| physics/steps | count | 0.78 | 3 | 3939 |
| physics/steps_dropped | count | 0.00 | 1 | 1 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0.00 | 2 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.04 | 21 | 219 |
| stream/deactivate/items | count | 0.04 | 21 | 223 |
| stream/despawn/items | count | 0.04 | 7 | 222 |
| stream/fetch/items | count | 0.02 | 2 | 83 |
| stream/spawn/items | count | 0.07 | 17 | 378 |
| stream/upload/bytes | bytes | 6705 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 57 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1470 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 31.7 ms; slowest 42, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 4.18 | 4175.6 | main/busy/ms 4138, stage/main/post_update 4005, stage/main/update 125, surface/advance_cpu_ms 89.55, stage/main/pre_update 4.50, stage/main/last 1.39, surface/cell_upload_ms 1.22, stage/main/first 0.68, stage/main/fixed_main_loop 0.62, stream/drive/ms 0.40, weather/map/ms 0.16, bigspace/total/ms 0.14 |
| warm-up | 4.84 | 665.6 | main/busy/ms 4185, stage/main/post_update 4038, render/thread/ms 612, render/busy/ms 495, stage/render/prepare_assets 369, stage/main/update 130, stage/render/prepare_views 113, render/acquire/ms 113, stage/render/prepare_resources 104, surface/advance_cpu_ms 89.55, stage/render/render 15.50, encode//sum 10.30 |
| warm-up | 4.99 | 39.2 | pipelines/compile_ms 185, pipelines/driver_ms 177, render/thread/ms 46.85, pipelines/wait_ms 42.91, stage/render/cleanup 34.37, render/present/ms 32.53, render/busy/ms 13.80, main/busy/ms 12.99, pipelines/front_ms 8.55, stage/render/prepare_assets 6.09, stage/main/post_update 5.66, stage/main/update 5.05 |
| warm-up | 5.63 | 81.6 | main/busy/ms 55.98, stage/main/update 46.27, render/thread/ms 43.22, render/busy/ms 41.42, stage/render/prepare_assets 30.72, stage/render/render 7.79, stage/main/post_update 6.03, render/graph/ms 5.22, stage/render/graph_submit 3.04, encode/submit 2.50, encode//sum 2.04, stage/render/camera_driver 1.89 |
| warm-up | 5.73 | 52.7 | pipelines/wait_ms 4714, pipelines/compile_ms 117, pipelines/driver_ms 101, main/busy/ms 53.18, stage/main/update 44.11, stream/drive/ms 40.84, render/thread/ms 23.59, render/busy/ms 21.80, pipelines/front_ms 16.85, stage/render/prepare_assets 10.80, stage/render/render 6.20, stage/main/post_update 5.46 |
| warm-up | 6.48 | 42.3 | main/busy/ms 51.42, stage/main/update 42.03, stream/drive/ms 38.31, render/thread/ms 22.21, render/busy/ms 20.67, stage/render/prepare_assets 8.91, stage/render/render 6.80, stage/main/post_update 6.04, render/graph/ms 4.56, encode//sum 2.35, stage/render/graph_submit 2.20, stage/render/camera_driver 2.18 |
| warm-up | 6.75 | 31.8 | render/thread/ms 41.22, render/busy/ms 39.34, main/busy/ms 29.38, stage/render/prepare_assets 16.28, stage/main/update 15.60, stream/drive/ms 11.25, stage/render/cleanup 10.71, render/graph/ms 7.71, stage/main/post_update 7.58, encode//sum 6.32, stage/render/graph_submit 4.44, stage/render/prepare_write_phase_buffers 3.76 |
| warm-up | 7.16 | 109.3 | main/busy/ms 82.32, stage/main/update 69.06, stream/drive/ms 64.76, render/thread/ms 44.85, render/busy/ms 42.81, stage/render/prepare_assets 20.88, stage/main/post_update 8.47, render/graph/ms 5.37, stage/render/render 4.32, stage/render/cleanup 3.99, encode//sum 3.61, stage/render/specialize 3.52 |
| warm-up | 7.32 | 117.4 | main/busy/ms 112, stage/main/update 97.71, stream/drive/ms 92.50, render/thread/ms 38.43, render/busy/ms 36.26, stage/render/prepare_assets 19.39, stage/main/post_update 8.44, render/graph/ms 5.27, stage/render/render 4.08, stage/render/cleanup 3.80, stage/render/graph_submit 3.35, stage/render/prepare_write_phase_buffers 3.07 |
| warm-up | 7.44 | 65.6 | pipelines/wait_ms 1962, pipelines/compile_ms 756, pipelines/driver_ms 676, pipelines/front_ms 80.25, main/busy/ms 61.61, stage/main/update 43.72, stream/drive/ms 39.47, render/thread/ms 31.11, render/busy/ms 29.36, stage/main/post_update 12.31, stage/render/prepare_assets 11.94, render/graph/ms 4.82 |
| warm-up | 8.35 | 35.6 | main/busy/ms 42.10, stage/main/update 29.39, stream/drive/ms 25.15, render/thread/ms 22.60, render/busy/ms 20.65, stage/render/render 8.04, stage/main/post_update 7.46, render/graph/ms 5.35, stage/render/prepare_assets 4.72, stage/render/graph_submit 3.44, encode//sum 3.39, encode/submit 2.32 |
| warm-up | 8.62 | 42.1 | render/thread/ms 63.14, render/busy/ms 57.36, main/busy/ms 46.51, stage/render/prepare_assets 19.78, stage/main/post_update 19.36, stage/main/update 16.36, stage/render/render 12.93, stage/render/prepare_bind_groups 7.85, render/graph/ms 6.25, stage/render/prepare_write_phase_buffers 5.99, stream/drive/ms 5.67, render/present/ms 5.27 |
| warm-up | 8.66 | 36.0 | render/thread/ms 68.65, render/busy/ms 62.05, main/busy/ms 51.35, stage/main/post_update 22.13, stage/render/prepare_assets 19.81, stage/render/render 15.91, stage/main/update 15.65, render/graph/ms 8.78, stage/render/prepare_write_phase_buffers 7.64, stage/render/prepare_bind_groups 7.44, render/present/ms 6.10, stage/render/specialize 5.54 |
| warm-up | 8.71 | 45.6 | render/thread/ms 67.92, render/busy/ms 64.19, main/busy/ms 56.89, stage/render/prepare_assets 27.40, stage/main/post_update 19.61, stage/main/update 17.43, stage/render/render 12.40, render/graph/ms 8.44, stage/main/fixed_main_loop 7.12, stream/drive/ms 6.55, stage/render/prepare_bind_groups 6.03, stage/main/pre_update 5.62 |
| warm-up | 8.76 | 53.7 | main/busy/ms 83.96, render/thread/ms 82.12, render/busy/ms 77.89, stage/main/update 33.57, stage/render/prepare_assets 32.19, stage/main/post_update 18.61, stage/main/fixed_main_loop 12.49, stage/render/prepare_write_phase_buffers 12.13, stage/render/render 11.27, stage/main/first 7.52, stage/main/pre_update 6.94, stream/drive/ms 6.78 |
| warm-up | 8.84 | 79.1 | pipelines/compile_ms 6507, pipelines/driver_ms 6121, pipelines/wait_ms 1516, pipelines/front_ms 385, render/thread/ms 106, render/busy/ms 102, main/busy/ms 80.64, stage/render/prepare_assets 47.99, stage/main/update 32.58, stage/main/post_update 18.35, stage/render/render 17.54, stage/render/prepare_write_phase_buffers 12.44 |
| warm-up | 8.91 | 72.9 | pipelines/compile_ms 6507, pipelines/driver_ms 6121, pipelines/wait_ms 1516, pipelines/front_ms 385, render/thread/ms 120, render/busy/ms 117, main/busy/ms 73.40, stage/render/prepare_assets 44.59, stage/main/update 36.97, stage/render/prepare_write_phase_buffers 21.75, stage/render/render 18.23, stage/main/post_update 15.40 |
| warm-up | 8.96 | 46.3 | pipelines/wait_ms 4162, render/thread/ms 93.62, render/busy/ms 89.90, main/busy/ms 84.57, pipelines/compile_ms 67.49, pipelines/driver_ms 44.18, stage/main/update 34.68, stage/main/post_update 29.93, stage/render/prepare_assets 27.45, stage/render/prepare_write_phase_buffers 23.71, pipelines/front_ms 23.31, stage/render/render 12.06 |
| warm-up | 9.03 | 73.9 | pipelines/wait_ms 4162, main/busy/ms 106, render/thread/ms 100, render/busy/ms 96.44, pipelines/compile_ms 67.49, stage/main/post_update 65.64, pipelines/driver_ms 44.18, stage/render/prepare_assets 35.08, pipelines/front_ms 23.31, stage/main/update 23.13, stage/render/prepare_bind_groups 16.58, stage/render/render 12.31 |
| warm-up | 9.07 | 35.2 | render/thread/ms 98.34, main/busy/ms 96.06, render/busy/ms 95.08, stage/main/post_update 54.27, stage/render/prepare_assets 26.90, stage/main/update 23.40, stage/render/prepare_bind_groups 16.31, stage/render/render 13.30, stage/render/prepare_resources 10.40, render/graph/ms 8.76, stage/render/prepare_flush 8.53, stage/main/fixed_main_loop 8.12 |
| warm-up | 9.15 | 80.3 | pipelines/wait_ms 13.0 k, pipelines/compile_ms 178, pipelines/driver_ms 178, render/thread/ms 107, render/busy/ms 103, main/busy/ms 89.45, stage/render/prepare_assets 35.95, stage/main/update 28.84, stage/render/prepare_flush 27.41, stage/main/post_update 21.43, stream/drive/ms 16.92, stage/main/pre_update 16.76 |
| warm-up | 9.24 | 96.4 | pipelines/wait_ms 13.0 k, pipelines/compile_ms 178, pipelines/driver_ms 178, render/thread/ms 164, render/busy/ms 160, main/busy/ms 150, stage/main/update 74.85, stage/render/prepare_assets 58.15, stage/main/post_update 24.50, stage/render/prepare_flush 20.83, stage/render/prepare_batch_phases 18.39, stage/main/pre_update 17.86 |
| warm-up | 9.34 | 97.2 | pipelines/wait_ms 4407, render/thread/ms 175, render/busy/ms 163, main/busy/ms 141, pipelines/compile_ms 131, pipelines/driver_ms 103, stage/main/update 85.03, stage/render/prepare_assets 48.13, stage/render/prepare_resources 29.12, pipelines/front_ms 28.42, stage/render/prepare_batch_phases 27.42, stage/main/post_update 22.25 |
| warm-up | 9.40 | 59.1 | pipelines/wait_ms 8946, pipelines/compile_ms 261, pipelines/driver_ms 209, render/thread/ms 140, render/busy/ms 128, main/busy/ms 71.89, pipelines/front_ms 51.58, stage/main/update 36.25, stage/render/prepare_resources 34.28, stage/render/prepare_assets 31.64, stage/render/prepare_write_phase_buffers 16.77, stage/main/post_update 15.12 |
| warm-up | 9.46 | 64.5 | pipelines/wait_ms 18.6 k, pipelines/compile_ms 180, pipelines/driver_ms 148, render/thread/ms 114, render/busy/ms 111, main/busy/ms 78.89, stage/main/post_update 34.11, stage/render/prepare_assets 33.66, pipelines/front_ms 31.70, stage/main/update 26.52, stage/render/prepare_resources 22.07, stage/render/prepare_write_phase_buffers 15.51 |
| warm-up | 9.52 | 60.4 | pipelines/wait_ms 14.1 k, render/thread/ms 116, render/busy/ms 113, main/busy/ms 103, stage/main/post_update 56.68, pipelines/compile_ms 49.72, stage/render/prepare_assets 44.55, pipelines/driver_ms 41.18, stage/main/update 24.83, stage/render/prepare_resources 13.96, stage/render/render 13.47, stage/main/fixed_main_loop 12.15 |
| warm-up | 9.58 | 59.5 | render/thread/ms 111, render/busy/ms 108, main/busy/ms 86.72, stage/main/post_update 43.52, stage/render/prepare_assets 41.22, stage/main/update 21.62, stage/render/prepare_resources 13.80, stage/render/render 12.90, stage/render/prepare_write_phase_buffers 11.62, stage/render/prepare_flush 11.37, stage/main/fixed_main_loop 8.93, render/graph/ms 8.31 |
| warm-up | 9.65 | 64.5 | pipelines/wait_ms 9589, pipelines/compile_ms 182, pipelines/driver_ms 161, render/thread/ms 109, render/busy/ms 105, main/busy/ms 68.41, stage/main/post_update 30.84, stage/render/prepare_write_phase_buffers 29.34, stage/render/prepare_assets 28.10, stage/main/update 22.61, pipelines/front_ms 20.87, stage/render/render 15.60 |
| warm-up | 9.71 | 60.7 | pipelines/wait_ms 14.5 k, pipelines/compile_ms 263, pipelines/driver_ms 222, render/thread/ms 107, render/busy/ms 103, main/busy/ms 83.53, stage/main/post_update 45.26, pipelines/front_ms 40.60, stage/render/prepare_assets 29.52, stage/render/prepare_write_phase_buffers 26.81, stage/main/update 18.71, stage/render/render 14.44 |
| warm-up | 9.77 | 60.1 | pipelines/wait_ms 8838, pipelines/compile_ms 1019, pipelines/driver_ms 906, pipelines/front_ms 113, render/thread/ms 96.49, main/busy/ms 95.01, render/busy/ms 93.43, stage/main/update 39.13, stage/main/post_update 38.70, stage/render/prepare_assets 38.45, stream/drive/ms 21.38, stage/render/prepare_bind_groups 12.55 |
| warm-up | 9.81 | 40.3 | pipelines/wait_ms 8919, pipelines/compile_ms 1009, pipelines/driver_ms 900, pipelines/front_ms 109, render/thread/ms 82.39, render/busy/ms 78.22, main/busy/ms 77.71, stage/main/update 40.53, stream/drive/ms 26.89, stage/main/post_update 25.96, stage/render/prepare_assets 20.44, stage/render/render 19.34 |
| warm-up | 9.84 | 34.1 | pipelines/wait_ms 4981, pipelines/compile_ms 70.83, render/thread/ms 68.17, render/busy/ms 63.68, main/busy/ms 60.44, pipelines/driver_ms 55.50, stage/main/post_update 29.48, stage/main/update 20.80, stage/render/render 19.33, pipelines/front_ms 15.33, stage/render/prepare_assets 13.99, stream/drive/ms 10.43 |
| warm-up | 9.95 | 44.6 | render/thread/ms 57.48, main/busy/ms 56.38, render/busy/ms 55.02, stage/main/post_update 35.64, stage/render/prepare_assets 21.49, stage/render/render 12.00, stage/main/update 11.20, render/graph/ms 8.57, stage/render/prepare_write_phase_buffers 5.76, stage/render/graph_submit 4.98, stage/render/prepare_flush 4.82, encode//sum 4.82 |
| warm-up | 10.08 | 39.1 | main/busy/ms 52.25, stage/main/update 30.15, render/thread/ms 28.19, render/busy/ms 26.07, stream/drive/ms 24.03, stage/main/post_update 13.98, stage/render/prepare_assets 6.99, render/graph/ms 5.60, encode//sum 5.37, stage/render/cleanup 4.50, stage/render/render 4.20, stage/render/prepare_write_phase_buffers 3.52 |
| warm-up | 10.14 | 33.5 | main/busy/ms 47.01, render/thread/ms 42.11, render/busy/ms 39.84, stage/main/post_update 21.40, stage/main/update 14.51, stage/render/render 9.64, stage/render/prepare_flush 8.37, stage/render/prepare_assets 7.87, bigspace/total/ms 7.47, stream/drive/ms 6.96, render/graph/ms 6.51, stage/render/prepare_write_phase_buffers 5.56 |
| warm-up | 10.17 | 31.9 | main/busy/ms 41.77, render/thread/ms 41.67, render/busy/ms 39.13, stage/main/post_update 14.94, stage/main/update 14.72, stage/render/render 13.17, stage/render/prepare_assets 10.25, render/graph/ms 9.74, encode//sum 7.90, stage/render/graph_submit 7.08, stream/drive/ms 5.29, stage/render/prepare_resources 4.98 |
| warm-up | 10.20 | 33.0 | render/thread/ms 50.63, render/busy/ms 47.57, main/busy/ms 44.49, stage/main/post_update 20.50, stage/main/update 15.32, stage/render/prepare_assets 14.10, stage/render/render 14.08, render/graph/ms 9.61, encode//sum 7.95, stage/render/graph_submit 6.33, stage/render/prepare_write_phase_buffers 6.07, stream/drive/ms 5.18 |
| warm-up | 15.74 | 40.5 | main/busy/ms 43.48, stage/main/update 27.73, render/thread/ms 25.71, render/busy/ms 24.54, stage/main/post_update 11.91, stage/render/prepare_assets 9.85, stage/render/cleanup 5.91, encode//sum 5.79, render/graph/ms 4.09, stage/render/graph_submit 2.41, stage/render/prepare_write_phase_buffers 2.34, stage/render/prepare_bind_groups 2.14 |
| measure | 0.84 | 39.0 | main/busy/ms 45.97, render/thread/ms 38.04, render/busy/ms 36.12, encode//sum 27.45, stage/main/update 23.72, stage/render/render 21.42, render/graph/ms 18.85, stream/drive/ms 18.00, stage/main/post_update 17.33, stage/render/graph_submit 14.67, encode/finish_wall 7.61, encode/submit 7.23 |
| measure | 2.19 | 42.2 | pipelines/wait_ms 22.6 k, pipelines/compile_ms 1908, pipelines/driver_ms 1761, pipelines/front_ms 148, encode//sum 113, render/thread/ms 54.43, render/busy/ms 52.76, render/graph/ms 32.55, stage/render/graph_submit 27.15, stage/render/render 25.46, main/busy/ms 21.19, encode/finish_wall 17.52 |
| measure | 2.90 | 74.3 | render/thread/ms 85.65, render/busy/ms 84.40, render/graph/ms 76.16, stage/render/graph_submit 72.72, stage/render/cleanup 69.23, encode/submit 66.18, encode//sum 32.85, main/busy/ms 23.82, stage/main/post_update 15.59, stage/render/render 8.60, encode/finish_wall 6.66, stage/main/update 5.30 |
| measure | 18.19 | 32.6 | main/busy/ms 40.87, stage/main/update 25.44, encode//sum 23.31, render/thread/ms 22.93, render/busy/ms 21.84, stream/drive/ms 21.03, render/graph/ms 12.55, stage/main/post_update 12.26, stage/render/graph_submit 10.00, stage/render/cleanup 7.26, stage/render/render 6.79, encode/submit 5.31 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\b2_on\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 8%, CPU 17%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **77%** (min 50%) · 2580 MHz (min 2580) · memory 8001 MHz · 75.7 W · 73 °C (max 78) · VRAM 6846 MiB · throttled 100% of samples
