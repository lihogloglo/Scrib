# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: flatnormal · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1222 frames over 15.0 s · **median 12.21 ms (82 fps)** · mean 12.28 · p95 13.97 · p99 15.29 (1% low 65 fps) · worst 17.31 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 384 frames over 9.9 s, worst 2753.8 ms
- hitches (> 30.5 ms): **0** measured, 34 in warm-up
- timed GPU passes: 10.17 ms of a 12.28 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 12.21 ms, limit 11 ms
- pass worst frame 17.31 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.448 | 3.468 | 4.387 | 0.99 | 0.064 |
| dlss_super_resolution | 2.425 | 2.439 | 3.119 | 0.99 | 0.200 |
| early prepass | 0.696 | 0.700 | 1.082 | 0.99 | 0.095 |
| atmosphere_luts | 0.396 | 0.399 | 0.725 | 0.99 | 0.004 |
| bloom | 0.375 | 0.378 | 0.709 | 0.99 | 0.074 |
| main_transmissive_pass_3d | 0.322 | 0.324 | 0.666 | 0.99 | 0.009 |
| water_velocity | 0.289 | 0.291 | 0.640 | 0.99 | 0.002 |
| water_sspr | 0.271 | 0.273 | 0.582 | 0.99 | 0.002 |
| ssao | 0.221 | 0.222 | 0.502 | 0.99 | 0.009 |
| uniform_allocation | 0.188 | 0.190 | 0.459 | 0.99 | 0.033 |
| atmos_froxel_inject | 0.170 | 0.171 | 0.344 | 0.99 | 0.002 |
| water_fft | 0.151 | 0.152 | 0.296 | 0.99 | 0.003 |
| clouds_trace | 0.150 | 0.151 | 0.187 | 0.99 | 0.001 |
| upscaling | 0.140 | 0.141 | 0.351 | 0.99 | 0.005 |
| bin_unpacking | 0.134 | 0.135 | 0.136 | 0.99 | 0.024 |
| clouds_composite | 0.126 | 0.126 | 0.121 | 0.99 | 0.002 |
| tonemapping | 0.101 | 0.101 | 0.116 | 0.99 | 0.004 |
| clustering | 0.076 | 0.077 | 0.089 | 0.99 | 0.025 |
| lightprobe_irradiance_map | 0.063 | 0.506 | 0.889 | 0.12 | 0.001 |
| render_sky | 0.059 | 0.059 | 0.056 | 0.99 | 0.002 |
| water_snapshot | 0.052 | 0.053 | 0.057 | 0.99 | 0.002 |
| clouds_resolve | 0.051 | 0.051 | 0.054 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.045 | 0.045 | 0.042 | 0.99 | 0.006 |
| water_mips | 0.032 | 0.032 | 0.026 | 0.99 | 0.003 |
| lightprobe_radiance_map | 0.032 | 0.257 | 0.515 | 0.12 | 0.005 |
| clouds_shadow | 0.032 | 0.032 | 0.036 | 0.99 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.032 | 0.99 | 0.001 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 0.99 | 0.079 |
| per_view_shadow_pass | 0.023 | 0.023 | 0.030 | 0.99 | 0.007 |
| atmosphere_environment | 0.012 | 0.012 | 0.014 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.009 | 0.99 | 0.002 |
| lightprobe_copy | 0.003 | 0.025 | 0.017 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.019 | 0.015 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.002 | 0.014 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.008 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 37.973 | 0.000 | 5818.89 | 46403.1 |
| render/thread/ms | 11.137 | 12.804 | 15.66 | 13609.4 |
| encode//sum | 8.719 | 11.874 | 18.55 | 10654.9 |
| render/busy/ms | 8.372 | 9.930 | 14.65 | 10230.2 |
| main/busy/ms | 7.655 | 9.619 | 15.32 | 9354.2 |
| render/graph/ms | 5.142 | 6.522 | 8.91 | 6284.0 |
| stage/main/post_update | 4.557 | 6.183 | 9.33 | 5568.5 |
| stage/render/graph_submit | 4.010 | 5.270 | 7.62 | 4900.0 |
| stage/render/render | 3.242 | 7.019 | 10.98 | 3961.8 |
| stage/render/cleanup | 2.661 | 6.882 | 10.53 | 3252.2 |
| stage/render/prepare_views | 2.317 | 3.791 | 5.47 | 2831.2 |
| render/acquire/ms | 2.260 | 3.727 | 5.43 | 2762.1 |
| encode/finish_wall | 2.025 | 2.611 | 4.30 | 2474.7 |
| encode/submit | 2.025 | 2.849 | 6.74 | 2474.6 |
| stage/main/update | 1.853 | 2.432 | 8.38 | 2264.2 |
| stage/render/camera_driver | 1.060 | 1.340 | 1.94 | 1295.1 |
| stage/render/prepare_write_phase_buffers | 0.794 | 1.026 | 3.13 | 970.4 |
| stage/render/prepare_assets | 0.514 | 0.701 | 3.85 | 627.8 |
| render/present/ms | 0.505 | 0.646 | 2.63 | 617.1 |
| pipelines/compile_ms | 0.504 | 0.000 | 140.95 | 616.1 |
| pipelines/front_ms | 0.488 | 0.000 | 136.36 | 595.7 |
| stage/render/prepare_resources | 0.426 | 0.546 | 0.98 | 520.1 |
| bigspace/total/ms | 0.375 | 0.692 | 2.63 | 458.3 |
| stage/main/pre_update | 0.346 | 0.500 | 1.02 | 423.2 |
| stage/main/first | 0.329 | 0.433 | 1.08 | 402.0 |
| terrain/update/ms | 0.244 | 0.311 | 0.69 | 297.6 |
| stage/main/fixed_main_loop | 0.239 | 0.415 | 1.28 | 292.3 |
| stage/render/prepare_meshes | 0.238 | 0.322 | 0.53 | 291.1 |
| stage/render/prepare_bind_groups | 0.212 | 0.309 | 1.36 | 258.6 |
| terrain/select/ms | 0.211 | 0.270 | 0.61 | 258.1 |
| stream/drive/ms | 0.191 | 0.279 | 0.63 | 233.4 |
| stage/render/prepare_batch_phases | 0.183 | 0.267 | 0.58 | 223.8 |
| stage/render/prepare_flush | 0.177 | 0.319 | 0.71 | 216.6 |
| stage/render/create_views | 0.174 | 0.233 | 0.55 | 212.1 |
| far/select/ms | 0.168 | 0.251 | 0.55 | 205.9 |
| stage/render/extract_commands | 0.166 | 0.216 | 0.59 | 202.4 |
| stream/tick/ms | 0.163 | 0.242 | 0.59 | 198.8 |
| controller/feed/ms | 0.154 | 0.231 | 0.53 | 188.2 |
| controller/move/ms | 0.153 | 0.269 | 0.57 | 186.7 |
| bigspace/high_precision/ms | 0.134 | 0.292 | 2.45 | 164.2 |
| stage/main/last | 0.130 | 0.230 | 0.68 | 158.5 |
| stage/render/queue | 0.126 | 0.179 | 0.55 | 154.2 |
| anim/evaluate/ms | 0.102 | 0.215 | 1.97 | 124.6 |
| bigspace/low_precision/ms | 0.092 | 0.212 | 2.11 | 112.5 |
| stage/render/specialize | 0.085 | 0.120 | 0.63 | 104.2 |
| stage/render/prepare_collect_phase_buffers | 0.066 | 0.138 | 0.32 | 81.2 |
| anim/evaluate_cpu/ms | 0.062 | 0.087 | 0.36 | 75.4 |
| encode/resolve_diagnostics/sum | 0.049 | 0.088 | 0.51 | 60.3 |
| physics/step/ms | 0.040 | 0.069 | 0.48 | 49.1 |
| weather/map/ms | 0.025 | 0.035 | 0.16 | 30.8 |
| encode/camera_driver_span_begin/sum | 0.023 | 0.042 | 0.24 | 28.6 |
| encode/camera_driver_span_end/sum | 0.019 | 0.039 | 0.34 | 23.3 |
| pipelines/driver_ms | 0.017 | 0.000 | 4.60 | 20.4 |
| stage/render/phase_sort | 0.015 | 0.031 | 0.16 | 18.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.36 | 14.0 |
| script/update/ms | 0.008 | 0.011 | 0.15 | 9.4 |
| controller/avoid/ms | 0.007 | 0.009 | 0.10 | 8.3 |
| physics/insert/ms | 0.006 | 0.000 | 0.80 | 7.0 |
| script/input/ms | 0.006 | 0.008 | 0.07 | 7.0 |
| script/ui/ms | 0.005 | 0.008 | 0.16 | 6.6 |
| anim/apply/ms | 0.005 | 0.007 | 0.10 | 5.8 |
| controller/write/ms | 0.002 | 0.003 | 0.14 | 2.9 |
| weather/fx/ms | 0.001 | 0.002 | 0.04 | 1.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.10 | 1.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.8 |
| weather/apply/ms | 0.000 | 0.000 | 0.00 | 0.3 |
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
| anim/a2/count | count | 0 | 4 | 0 |
| anim/a3/count | count | 414 | 414 | 414 |
| anim/characters/count | count | 414 | 414 | 414 |
| anim/evaluated/count | count | 7 | 31 | 7 |
| controller/movers/count | count | 7 | 7 | 7 |
| crowd/c0/count | count | 8 | 11 | 8 |
| crowd/c0/joints/count | count | 272 | 374 | 272 |
| crowd/c0/meshes/count | count | 48 | 66 | 48 |
| crowd/c1/count | count | 122 | 133 | 122 |
| crowd/c1/joints/count | count | 4848 | 4998 | 4848 |
| crowd/c1/meshes/count | count | 1080 | 1132 | 1080 |
| crowd/c2/count | count | 284 | 284 | 284 |
| crowd/c2/joints/count | count | 4948 | 4948 | 4948 |
| crowd/c2/meshes/count | count | 1381 | 1381 | 1381 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 5101 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.0 k |
| encode//max | ms | 1.72 | 3.71 | 1.82 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.24 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.34 | 0.01 |
| encode/finish_critical | ms | 1.72 | 3.71 | 1.82 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.51 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 10.89 | 15.32 | 10.49 |
| gpu/timed/ms | ms | 10.87 | 14.44 | 10.49 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 1853 | 1853 | 1853 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 193.7 k | 193.7 k | 193.7 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 12.0 k | 12.0 k | 12.0 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 474.10 M | 474.10 M | 474.10 M |
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
| phases/opaque3d/batch_sets | count | 22 | 23 | 22 |
| phases/opaque3d/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 22 | 21 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 18 | 19 | 18 |
| phases/opaque3d_prepass/bins | count | 1305 | 1306 | 1305 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 17 | 18 | 17 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 26 | 23 |
| phases/shadow/bins | count | 2028 | 2336 | 2028 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5345 | 5620 | 5345 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 23 | 26 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 83 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 165 | 165 | 165 |
| pipelines/waiting | count | 0 | 9 | 0 |
| preprocess/bind_groups | count | 0 | 106 | 0 |
| preprocess/bind_groups_created | count | 0 | 106 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 177 | 177 | 177 |
| process/cpu_usage | % | 22.96 | 38.57 | 31.94 |
| process/mem_usage | GiB | 4.31 | 4.36 | 4.36 |
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
| started_load_count |  | 3547 | 3549 | 3549 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 4 | 4 |
| statics/material_pipelines | count | 4 | 4 | 4 |
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
| system/cpu_usage | % | 40.47 | 65.63 | 42.09 |
| system/mem_usage | % | 68.48 | 68.64 | 68.64 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 77 | 80 | 77 |
| views/all/bins | count | 5198 | 5506 | 5198 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 14.8 k | 14.6 k |
| views/all/pipelines | count | 73 | 76 | 73 |
| views/all/pipelines_distinct | count | 65 | 67 | 65 |
| views/all/work_items | count | 14.8 k | 15.0 k | 14.8 k |
| views/cascade/batch_sets | count | 23 | 26 | 23 |
| views/cascade/bins | count | 2028 | 2336 | 2028 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5345 | 5620 | 5345 |
| views/cascade/pipelines | count | 23 | 26 | 23 |
| views/cascade/work_items | count | 5345 | 5620 | 5345 |
| views/lamp_face/batch_sets | count | 0 | 0 | 0 |
| views/lamp_face/bins | count | 0 | 0 | 0 |
| views/lamp_face/count | count | 0 | 0 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 0 | 0 | 0 |
| views/lamp_face/pipelines | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 0 | 0 |
| views/main/batch_sets | count | 54 | 56 | 54 |
| views/main/bins | count | 3170 | 3172 | 3170 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 9282 | 9282 | 9282 |
| views/main/pipelines | count | 50 | 52 | 50 |
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
| anim/bones_written/count | count | 0.24 | 132 | 297 |
| anim/events/count | count | 0.13 | 3 | 159 |
| assets/Image/added | count | 0.01 | 14 | 14 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0.02 | 22 | 22 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0.01 | 14 | 14 |
| assets/StandardMaterial/modified | count | 4.05 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.50 | 822 | 1827 |
| encode//count | count | 31.25 | 33 | 38.2 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1222 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1222 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1222 |
| physics/inserts | count | 0.07 | 25 | 83 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.74 | 1 | 900 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0 | 0 | 0 |
| pipelines/created | count | 0 | 0 | 0 |
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
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 30.5 ms; slowest 34, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.75 | 2753.8 | main/busy/ms 2723, stage/main/post_update 2573, stage/main/update 144, surface/advance_cpu_ms 112, stage/main/pre_update 2.82, stage/main/last 1.01, surface/cell_upload_ms 0.70, stage/main/first 0.51, stage/main/fixed_main_loop 0.45, stream/drive/ms 0.43, weather/map/ms 0.19, bigspace/total/ms 0.10 |
| warm-up | 3.40 | 641.8 | main/busy/ms 2764, stage/main/post_update 2603, render/thread/ms 596, render/busy/ms 458, stage/render/prepare_assets 384, stage/main/update 149, stage/render/prepare_views 137, render/acquire/ms 136, surface/advance_cpu_ms 112, stage/render/prepare_resources 52.95, stage/render/create_views 11.58, stage/render/render 8.57 |
| warm-up | 3.76 | 362.9 | pipelines/wait_ms 5456, render/thread/ms 609, render/busy/ms 468, pipelines/compile_ms 388, stage/render/prepare_assets 385, pipelines/front_ms 361, stage/render/prepare_views 137, render/acquire/ms 137, main/busy/ms 61.11, stage/render/prepare_resources 54.77, stage/main/post_update 42.35, pipelines/driver_ms 27.55 |
| warm-up | 4.03 | 99.6 | main/busy/ms 69.63, render/thread/ms 63.87, render/busy/ms 62.03, stage/main/update 58.14, stage/render/prepare_assets 49.11, stage/main/post_update 6.52, render/graph/ms 5.77, stage/render/render 4.67, stage/render/cleanup 3.49, stage/render/graph_submit 3.31, encode/submit 2.77, stage/render/camera_driver 2.29 |
| warm-up | 4.19 | 55.4 | render/thread/ms 52.26, render/busy/ms 50.63, main/busy/ms 44.69, stage/render/prepare_assets 39.30, stage/main/update 34.74, stream/drive/ms 30.90, stage/render/render 6.91, stage/main/post_update 6.52, render/graph/ms 4.88, stage/render/graph_submit 3.07, encode/submit 2.70, encode//sum 2.06 |
| warm-up | 4.25 | 54.1 | pipelines/wait_ms 1365, render/thread/ms 39.13, render/busy/ms 36.85, main/busy/ms 34.57, stage/render/prepare_assets 25.27, stage/main/update 21.96, stream/drive/ms 17.45, pipelines/compile_ms 13.22, pipelines/front_ms 11.60, stage/render/render 9.07, stage/main/post_update 7.62, render/graph/ms 6.10 |
| warm-up | 4.29 | 37.7 | pipelines/wait_ms 5572, pipelines/compile_ms 249, pipelines/front_ms 223, render/thread/ms 59.41, render/busy/ms 57.44, stage/render/prepare_assets 43.00, main/busy/ms 27.30, pipelines/driver_ms 26.65, stage/main/post_update 10.39, stage/main/update 9.69, stage/render/render 8.34, render/graph/ms 5.80 |
| warm-up | 4.34 | 43.4 | pipelines/wait_ms 6743, pipelines/compile_ms 257, pipelines/front_ms 230, render/thread/ms 58.63, render/busy/ms 56.72, main/busy/ms 50.39, stage/render/prepare_assets 41.35, stage/main/update 34.32, stream/drive/ms 28.98, pipelines/driver_ms 26.74, stage/main/post_update 9.09, stage/render/render 8.83 |
| warm-up | 4.37 | 39.1 | pipelines/wait_ms 5068, pipelines/compile_ms 80.67, pipelines/front_ms 74.89, render/thread/ms 53.87, render/busy/ms 52.25, main/busy/ms 45.44, stage/render/prepare_assets 38.89, stage/main/update 33.58, stream/drive/ms 29.62, stage/render/render 8.93, render/graph/ms 6.80, stage/main/post_update 6.67 |
| warm-up | 4.46 | 71.0 | pipelines/wait_ms 2140, main/busy/ms 60.94, pipelines/compile_ms 51.67, pipelines/front_ms 47.59, stage/main/update 44.05, render/thread/ms 37.95, stream/drive/ms 36.64, render/busy/ms 36.53, stage/render/prepare_assets 20.89, stage/main/post_update 8.67, stage/render/render 7.37, encode//sum 5.70 |
| warm-up | 4.50 | 33.2 | pipelines/wait_ms 1585, render/thread/ms 50.68, render/busy/ms 48.72, main/busy/ms 38.68, pipelines/compile_ms 35.28, pipelines/front_ms 32.12, stage/render/prepare_assets 28.20, stage/main/update 16.55, stage/main/post_update 13.63, stage/render/render 9.86, stream/drive/ms 9.48, render/graph/ms 7.42 |
| warm-up | 4.54 | 39.9 | render/thread/ms 50.99, render/busy/ms 49.06, main/busy/ms 35.04, stage/render/prepare_assets 33.51, stage/main/post_update 14.40, stage/main/update 13.67, stream/drive/ms 8.84, render/graph/ms 6.79, stage/render/render 5.84, encode//sum 5.07, stage/render/graph_submit 4.30, stage/render/cleanup 3.47 |
| warm-up | 4.62 | 59.8 | pipelines/wait_ms 603, pipelines/compile_ms 163, pipelines/front_ms 159, main/busy/ms 54.87, render/thread/ms 48.43, render/busy/ms 46.69, stage/main/update 34.74, stream/drive/ms 30.41, stage/render/prepare_assets 29.00, stage/main/post_update 14.89, render/graph/ms 9.64, stage/render/render 9.26 |
| warm-up | 4.68 | 34.0 | render/thread/ms 40.13, render/busy/ms 38.71, main/busy/ms 24.67, stage/render/prepare_assets 19.93, stage/main/post_update 10.11, stage/main/update 9.71, terrain/update/ms 8.92, render/graph/ms 6.08, encode//sum 5.51, stage/render/render 5.17, stream/drive/ms 4.50, stage/render/graph_submit 3.60 |
| warm-up | 4.82 | 123.6 | pipelines/wait_ms 546, pipelines/compile_ms 153, pipelines/front_ms 146, main/busy/ms 109, stage/main/update 96.02, stream/drive/ms 83.58, render/thread/ms 43.89, render/busy/ms 42.51, stage/render/prepare_assets 31.88, stage/main/post_update 8.35, pipelines/driver_ms 6.89, stage/render/render 5.66 |
| warm-up | 4.98 | 37.5 | main/busy/ms 39.51, render/thread/ms 34.15, render/busy/ms 31.90, stage/main/update 24.13, stream/drive/ms 14.35, stage/render/prepare_assets 11.99, stage/main/post_update 10.04, stage/render/render 7.87, encode//sum 5.74, render/graph/ms 5.49, stage/render/graph_submit 3.26, stage/render/prepare_write_phase_buffers 3.00 |
| warm-up | 5.05 | 37.5 | main/busy/ms 53.93, render/thread/ms 48.29, render/busy/ms 45.53, stage/main/update 36.87, stage/render/prepare_assets 13.42, stage/render/render 12.66, stage/main/post_update 11.28, render/graph/ms 8.87, stage/render/prepare_write_phase_buffers 6.51, encode//sum 6.08, stage/render/graph_submit 5.17, stage/render/prepare_bind_groups 4.89 |
| warm-up | 5.08 | 32.9 | render/thread/ms 49.50, render/busy/ms 46.64, main/busy/ms 43.74, stage/main/update 20.10, stage/render/prepare_assets 17.20, stage/main/post_update 15.40, stage/render/render 14.60, render/graph/ms 10.49, encode//sum 6.53, stage/render/graph_submit 6.12, stream/drive/ms 5.69, stage/render/prepare_bind_groups 5.07 |
| warm-up | 5.14 | 58.2 | render/thread/ms 65.07, render/busy/ms 63.02, main/busy/ms 51.26, stage/render/prepare_assets 40.52, stage/main/post_update 23.78, stage/main/update 18.01, render/graph/ms 8.28, stream/drive/ms 7.66, stage/render/render 7.24, encode//sum 5.32, stage/render/graph_submit 4.80, stream/spawn/ms 4.47 |
| warm-up | 5.17 | 32.2 | render/thread/ms 63.08, render/busy/ms 61.50, stage/render/prepare_assets 45.39, main/busy/ms 42.53, stage/main/post_update 19.32, stage/main/update 14.74, stage/render/cleanup 7.72, stream/drive/ms 6.10, render/graph/ms 5.66, encode//sum 4.51, stage/render/graph_submit 3.36, stream/spawn/ms 3.29 |
| warm-up | 5.31 | 32.2 | pipelines/wait_ms 1429, pipelines/compile_ms 73.27, pipelines/front_ms 70.80, render/thread/ms 42.68, render/busy/ms 40.95, stage/render/prepare_assets 21.52, main/busy/ms 21.03, stage/main/update 8.42, stage/render/cleanup 7.78, stage/main/post_update 7.77, render/graph/ms 5.90, encode//sum 5.86 |
| warm-up | 5.36 | 31.9 | pipelines/wait_ms 2769, render/thread/ms 40.65, render/busy/ms 38.80, main/busy/ms 26.69, pipelines/compile_ms 24.92, pipelines/front_ms 23.02, stage/render/prepare_assets 16.69, stage/main/post_update 10.82, stage/main/update 9.01, render/graph/ms 8.48, encode//sum 7.09, stage/render/cleanup 5.87 |
| warm-up | 5.42 | 44.0 | render/thread/ms 57.65, render/busy/ms 55.99, main/busy/ms 32.88, stage/render/prepare_assets 30.03, stage/main/update 14.43, stage/main/post_update 11.24, render/graph/ms 10.54, stage/render/render 8.67, encode//sum 7.10, stage/render/graph_submit 6.62, stream/drive/ms 5.81, stage/render/cleanup 4.72 |
| warm-up | 5.48 | 42.3 | render/thread/ms 50.00, render/busy/ms 48.04, main/busy/ms 32.32, stage/render/prepare_assets 20.25, stage/main/post_update 13.20, stage/render/render 12.99, stage/main/update 12.50, render/graph/ms 9.94, encode//sum 8.18, stage/render/graph_submit 6.52, stream/drive/ms 5.82, encode/submit 4.36 |
| warm-up | 5.52 | 36.1 | pipelines/wait_ms 1718, render/thread/ms 61.77, render/busy/ms 59.76, main/busy/ms 36.34, stage/render/prepare_assets 32.35, stage/main/update 15.07, stage/main/post_update 13.10, stage/render/render 11.35, render/graph/ms 8.40, encode//sum 7.19, stream/drive/ms 5.49, stage/render/graph_submit 5.47 |
| warm-up | 5.56 | 48.1 | pipelines/wait_ms 3847, pipelines/compile_ms 212, pipelines/front_ms 205, render/thread/ms 66.06, render/busy/ms 64.17, stage/render/prepare_assets 45.69, main/busy/ms 34.78, stage/main/update 15.11, stage/main/post_update 12.63, stage/render/render 8.73, pipelines/driver_ms 6.94, render/graph/ms 6.24 |
| warm-up | 5.62 | 33.0 | pipelines/wait_ms 1749, pipelines/compile_ms 64.74, pipelines/front_ms 63.32, render/thread/ms 49.25, render/busy/ms 47.76, main/busy/ms 33.72, stage/render/prepare_assets 31.57, stage/main/post_update 15.95, stage/main/update 13.18, render/graph/ms 6.01, encode//sum 6.01, stage/render/cleanup 4.17 |
| warm-up | 5.77 | 59.1 | pipelines/wait_ms 8150, pipelines/compile_ms 70.30, pipelines/front_ms 63.67, main/busy/ms 49.60, render/thread/ms 36.62, render/busy/ms 35.31, stage/main/update 25.40, stage/render/prepare_assets 21.27, stream/drive/ms 17.96, stage/main/post_update 17.34, stage/render/render 7.91, encode//sum 6.75 |
| warm-up | 5.93 | 32.8 | pipelines/wait_ms 3587, pipelines/compile_ms 174, pipelines/front_ms 168, main/busy/ms 39.49, render/thread/ms 30.57, render/busy/ms 28.73, stage/main/update 24.38, stream/drive/ms 20.02, stage/main/post_update 11.21, stage/render/prepare_assets 10.91, stage/render/render 9.98, encode//sum 7.60 |
| warm-up | 6.04 | 39.4 | pipelines/wait_ms 6102, main/busy/ms 53.20, render/thread/ms 43.65, render/busy/ms 40.17, stage/main/update 33.78, render/graph/ms 14.21, stage/render/prepare_assets 13.63, stage/main/post_update 12.20, encode//sum 12.13, pipelines/compile_ms 11.21, stream/drive/ms 10.92, stage/render/render 10.10 |
| warm-up | 6.08 | 38.6 | render/thread/ms 52.70, render/busy/ms 50.57, main/busy/ms 47.67, stage/main/update 26.17, stage/render/prepare_assets 23.19, stage/main/post_update 13.50, encode//sum 12.49, render/graph/ms 11.31, stream/drive/ms 11.21, stage/render/cleanup 8.51, stage/render/graph_submit 7.55, stage/render/render 5.70 |
| warm-up | 6.14 | 42.8 | main/busy/ms 43.75, render/thread/ms 31.65, stage/main/update 30.40, render/busy/ms 30.00, stage/render/render 11.50, encode//sum 9.96, stage/render/prepare_assets 9.37, render/graph/ms 9.31, stage/main/post_update 9.06, stage/render/graph_submit 6.66, encode/submit 4.92, stream/drive/ms 3.60 |
| warm-up | 6.41 | 49.1 | pipelines/wait_ms 1290, pipelines/compile_ms 179, pipelines/front_ms 172, render/thread/ms 47.49, render/busy/ms 45.58, main/busy/ms 42.05, stage/render/prepare_assets 28.29, stage/main/update 19.64, stage/main/post_update 15.37, stream/drive/ms 12.84, encode//sum 11.06, stage/render/render 10.43 |
| warm-up | 8.03 | 34.0 | render/thread/ms 46.72, render/busy/ms 45.06, main/busy/ms 38.62, encode//sum 21.29, stage/main/update 19.28, stage/render/render 18.14, stage/render/prepare_assets 17.69, render/graph/ms 15.73, stage/main/post_update 13.72, stage/render/graph_submit 11.86, encode/submit 6.10, encode/finish_wall 5.86 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\b2_off\balmora@2560x1440_off=flatnormal-3 --bench-baseline none --off flatnormal
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 20%
- during the run: other processes 18% CPU (system minus this process)
- nvidia-smi over the window (57 samples): GPU load **98%** (min 97%) · 2580 MHz (min 2580) · memory 8001 MHz · 88.0 W · 73 °C (max 75) · VRAM 6718 MiB · throttled 100% of samples
