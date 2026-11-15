# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: flatnormal · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1314 frames over 15.0 s · **median 11.44 ms (87 fps)** · mean 11.42 · p95 12.75 · p99 13.30 (1% low 75 fps) · worst 23.93 ms · 1 spikes over 1.5× median · 4.0 stutters/min (work 4.0/min)
- warm-up: 389 frames over 8.9 s, worst 2724.4 ms
- hitches (> 28.6 ms): **0** measured, 26 in warm-up
- timed GPU passes: 9.54 ms of a 11.42 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 11.44 ms, limit 11 ms
- pass worst frame 23.93 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 3.184 | 3.192 | 3.778 | 1.00 | 0.062 |
| dlss_super_resolution | 2.355 | 2.361 | 2.804 | 1.00 | 0.193 |
| early prepass | 0.612 | 0.613 | 0.928 | 1.00 | 0.089 |
| atmosphere_luts | 0.354 | 0.355 | 0.494 | 1.00 | 0.004 |
| bloom | 0.350 | 0.351 | 0.660 | 1.00 | 0.069 |
| main_transmissive_pass_3d | 0.307 | 0.307 | 0.655 | 1.00 | 0.008 |
| water_velocity | 0.278 | 0.279 | 0.639 | 1.00 | 0.002 |
| water_sspr | 0.247 | 0.248 | 0.565 | 1.00 | 0.002 |
| ssao | 0.209 | 0.209 | 0.215 | 1.00 | 0.008 |
| uniform_allocation | 0.179 | 0.179 | 0.315 | 1.00 | 0.029 |
| atmos_froxel_inject | 0.161 | 0.162 | 0.154 | 1.00 | 0.002 |
| clouds_trace | 0.152 | 0.152 | 0.189 | 1.00 | 0.001 |
| water_fft | 0.147 | 0.147 | 0.141 | 1.00 | 0.003 |
| upscaling | 0.128 | 0.129 | 0.157 | 1.00 | 0.004 |
| bin_unpacking | 0.126 | 0.126 | 0.129 | 1.00 | 0.021 |
| clouds_composite | 0.120 | 0.120 | 0.119 | 1.00 | 0.002 |
| tonemapping | 0.097 | 0.098 | 0.110 | 1.00 | 0.004 |
| clustering | 0.073 | 0.074 | 0.084 | 1.00 | 0.023 |
| lightprobe_irradiance_map | 0.058 | 0.464 | 0.746 | 0.12 | 0.001 |
| render_sky | 0.057 | 0.057 | 0.056 | 1.00 | 0.002 |
| clouds_resolve | 0.052 | 0.052 | 0.054 | 1.00 | 0.001 |
| water_snapshot | 0.045 | 0.045 | 0.059 | 1.00 | 0.002 |
| early_mesh_preprocessing | 0.041 | 0.041 | 0.042 | 1.00 | 0.006 |
| clouds_shadow | 0.032 | 0.032 | 0.036 | 1.00 | 0.003 |
| atmos_froxel_integrate | 0.031 | 0.031 | 0.031 | 1.00 | 0.001 |
| lightprobe_radiance_map | 0.030 | 0.238 | 0.241 | 0.12 | 0.004 |
| water_mips | 0.027 | 0.027 | 0.024 | 1.00 | 0.002 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.029 | 1.00 | 0.074 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.029 | 1.00 | 0.007 |
| atmosphere_environment | 0.012 | 0.012 | 0.013 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.007 | 0.007 | 0.008 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.009 | 1.00 | 0.002 |
| lightprobe_copy | 0.003 | 0.020 | 0.018 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.015 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.002 | 0.014 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.00 | 0.007 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 76.086 | 0.000 | 11263.01 | 99976.4 |
| render/thread/ms | 10.314 | 11.609 | 22.85 | 13552.4 |
| encode//sum | 8.331 | 10.876 | 17.84 | 10947.5 |
| render/busy/ms | 8.005 | 9.234 | 20.37 | 10519.1 |
| main/busy/ms | 7.338 | 8.888 | 11.79 | 9641.7 |
| render/graph/ms | 4.988 | 6.079 | 17.24 | 6554.5 |
| stage/main/post_update | 4.440 | 5.862 | 8.71 | 5833.5 |
| stage/render/graph_submit | 3.921 | 5.012 | 16.20 | 5152.4 |
| stage/render/render | 3.259 | 6.557 | 8.72 | 4282.7 |
| stage/render/cleanup | 2.444 | 6.536 | 18.02 | 3212.0 |
| encode/finish_wall | 2.021 | 2.587 | 4.21 | 2655.9 |
| encode/submit | 1.933 | 2.582 | 13.78 | 2540.6 |
| stage/render/prepare_views | 1.884 | 3.193 | 4.02 | 2475.0 |
| render/acquire/ms | 1.834 | 3.136 | 3.95 | 2410.0 |
| stage/main/update | 1.727 | 2.143 | 3.07 | 2268.9 |
| stage/render/camera_driver | 1.000 | 1.208 | 1.64 | 1314.3 |
| stage/render/prepare_write_phase_buffers | 0.745 | 0.911 | 1.23 | 979.5 |
| pipelines/compile_ms | 0.614 | 0.000 | 123.87 | 806.4 |
| pipelines/front_ms | 0.581 | 0.000 | 119.23 | 762.9 |
| stage/render/prepare_assets | 0.485 | 0.662 | 1.58 | 637.3 |
| render/present/ms | 0.474 | 0.588 | 1.86 | 623.3 |
| stage/render/prepare_resources | 0.404 | 0.497 | 0.87 | 530.3 |
| bigspace/total/ms | 0.368 | 0.649 | 2.88 | 483.1 |
| stage/main/pre_update | 0.335 | 0.463 | 0.72 | 439.7 |
| stage/main/first | 0.307 | 0.387 | 1.40 | 403.5 |
| terrain/update/ms | 0.234 | 0.302 | 0.65 | 307.7 |
| stage/render/prepare_meshes | 0.224 | 0.300 | 0.55 | 293.8 |
| stage/main/fixed_main_loop | 0.218 | 0.381 | 1.11 | 285.8 |
| terrain/select/ms | 0.204 | 0.262 | 0.60 | 267.4 |
| stage/render/prepare_bind_groups | 0.199 | 0.274 | 1.00 | 260.9 |
| stream/drive/ms | 0.186 | 0.254 | 0.69 | 244.6 |
| stage/render/prepare_batch_phases | 0.169 | 0.233 | 0.41 | 221.5 |
| stage/render/create_views | 0.165 | 0.220 | 0.44 | 216.9 |
| far/select/ms | 0.162 | 0.228 | 0.55 | 213.3 |
| stage/render/extract_commands | 0.159 | 0.196 | 0.34 | 208.8 |
| stream/tick/ms | 0.158 | 0.217 | 0.66 | 207.3 |
| stage/render/prepare_flush | 0.157 | 0.334 | 0.72 | 205.9 |
| controller/feed/ms | 0.156 | 0.225 | 0.48 | 204.6 |
| bigspace/high_precision/ms | 0.134 | 0.293 | 2.66 | 176.5 |
| stage/main/last | 0.123 | 0.206 | 0.68 | 162.3 |
| controller/move/ms | 0.120 | 0.192 | 0.56 | 157.8 |
| stage/render/queue | 0.113 | 0.152 | 0.47 | 148.2 |
| bigspace/low_precision/ms | 0.091 | 0.206 | 1.84 | 119.2 |
| stage/render/specialize | 0.080 | 0.112 | 0.44 | 105.6 |
| anim/evaluate/ms | 0.074 | 0.191 | 1.62 | 97.5 |
| anim/evaluate_cpu/ms | 0.069 | 0.090 | 0.27 | 90.1 |
| stage/render/prepare_collect_phase_buffers | 0.063 | 0.124 | 0.23 | 82.7 |
| encode/resolve_diagnostics/sum | 0.045 | 0.081 | 0.31 | 59.4 |
| physics/step/ms | 0.037 | 0.062 | 0.79 | 48.1 |
| pipelines/driver_ms | 0.033 | 0.000 | 9.00 | 43.5 |
| weather/map/ms | 0.025 | 0.033 | 0.37 | 32.3 |
| encode/camera_driver_span_begin/sum | 0.022 | 0.039 | 0.34 | 28.4 |
| encode/camera_driver_span_end/sum | 0.017 | 0.034 | 0.38 | 22.7 |
| stage/render/phase_sort | 0.015 | 0.029 | 0.16 | 19.2 |
| weather/eval/ms | 0.011 | 0.015 | 0.08 | 14.8 |
| script/update/ms | 0.007 | 0.009 | 0.03 | 9.4 |
| controller/avoid/ms | 0.007 | 0.008 | 0.14 | 8.5 |
| script/input/ms | 0.005 | 0.007 | 0.03 | 7.2 |
| script/ui/ms | 0.005 | 0.007 | 0.03 | 6.4 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 5.9 |
| physics/insert/ms | 0.003 | 0.010 | 0.64 | 4.2 |
| controller/write/ms | 0.002 | 0.002 | 0.04 | 2.4 |
| weather/fx/ms | 0.001 | 0.002 | 0.01 | 1.3 |
| stage/render/prepare | 0.001 | 0.003 | 0.04 | 0.9 |
| shapes/activate/ms | 0.001 | 0.001 | 0.00 | 0.9 |
| weather/apply/ms | 0.000 | 0.000 | 0.28 | 0.6 |
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
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 6504 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.70 | 3.98 | 1.75 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.34 | 0.01 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.38 | 0.01 |
| encode/finish_critical | ms | 1.70 | 3.98 | 1.75 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.31 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 10.26 | 20.37 | 10.39 |
| gpu/timed/ms | ms | 10.26 | 11.60 | 10.39 |
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
| phases/shadow/items | count | 5345 | 5672 | 5345 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 23 | 26 | 23 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2084 | 2084 | 2084 |
| physics/items | count | 19 | 19 | 19 |
| physics/pending | count | 0 | 203 | 0 |
| physics/terrain/chunks | count | 52 | 52 | 52 |
| pipelines/total | count | 170 | 170 | 170 |
| pipelines/waiting | count | 0 | 18 | 0 |
| preprocess/bind_groups | count | 0 | 167 | 0 |
| preprocess/bind_groups_created | count | 0 | 167 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 177 | 177 | 177 |
| process/cpu_usage | % | 26.56 | 39.90 | 22.12 |
| process/mem_usage | GiB | 4.32 | 4.39 | 4.36 |
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
| system/cpu_usage | % | 35.79 | 49.81 | 29.97 |
| system/mem_usage | % | 67.51 | 67.72 | 67.70 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2480 | 2480 | 2480 |
| textures/images | count | 1919 | 1919 | 1919 |
| views/all/batch_sets | count | 77 | 80 | 77 |
| views/all/bins | count | 5198 | 5506 | 5198 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.6 k | 15.0 k | 14.6 k |
| views/all/pipelines | count | 73 | 76 | 73 |
| views/all/pipelines_distinct | count | 65 | 67 | 65 |
| views/all/work_items | count | 14.8 k | 15.1 k | 14.8 k |
| views/cascade/batch_sets | count | 23 | 26 | 23 |
| views/cascade/bins | count | 2028 | 2336 | 2028 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5345 | 5672 | 5345 |
| views/cascade/pipelines | count | 23 | 26 | 23 |
| views/cascade/work_items | count | 5345 | 5672 | 5345 |
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
| anim/bones_written/count | count | 0 | 0 | 0 |
| anim/events/count | count | 0.09 | 3 | 118 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.77 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 2 | 4 |
| encode//count | count | 31.25 | 33 | 41.1 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1314 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1314 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1314 |
| physics/inserts | count | 0.23 | 89 | 308 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.68 | 1 | 900 |
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

## Hitches (> 28.6 ms; slowest 26, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.72 | 2724.4 | main/busy/ms 2695, stage/main/post_update 2589, stage/main/update 99.53, surface/advance_cpu_ms 70.25, stage/main/pre_update 3.21, stage/main/last 1.18, surface/cell_upload_ms 0.92, stage/main/first 0.79, stage/main/fixed_main_loop 0.46, stream/drive/ms 0.32, weather/map/ms 0.11, bigspace/total/ms 0.08 |
| warm-up | 3.51 | 789.9 | main/busy/ms 2735, stage/main/post_update 2618, render/thread/ms 745, render/busy/ms 626, stage/render/prepare_assets 464, stage/render/prepare_resources 130, stage/render/prepare_views 117, render/acquire/ms 117, stage/main/update 104, surface/advance_cpu_ms 70.25, stage/render/create_views 20.39, stage/render/render 10.63 |
| warm-up | 3.66 | 147.7 | pipelines/wait_ms 1382, render/thread/ms 756, render/busy/ms 635, stage/render/prepare_assets 464, pipelines/compile_ms 153, pipelines/front_ms 144, stage/render/prepare_resources 132, stage/render/prepare_views 118, render/acquire/ms 118, main/busy/ms 57.38, stage/main/post_update 38.73, terrain/update/ms 22.56 |
| warm-up | 3.96 | 82.3 | pipelines/wait_ms 1326, pipelines/compile_ms 316, pipelines/front_ms 299, main/busy/ms 56.89, stage/main/update 48.25, render/thread/ms 42.35, render/busy/ms 40.77, stage/render/prepare_assets 32.70, pipelines/driver_ms 17.50, stage/main/post_update 5.53, render/graph/ms 3.89, stage/render/cleanup 3.02 |
| warm-up | 4.09 | 65.0 | main/busy/ms 44.85, render/thread/ms 43.74, render/busy/ms 42.21, stage/main/update 36.56, stream/drive/ms 33.15, stage/render/prepare_assets 32.89, stage/main/post_update 5.74, render/graph/ms 4.87, stage/render/render 3.75, stage/render/graph_submit 3.19, stage/render/cleanup 3.10, encode/submit 2.37 |
| warm-up | 4.16 | 50.2 | pipelines/wait_ms 1197, pipelines/compile_ms 208, pipelines/front_ms 183, render/thread/ms 51.41, render/busy/ms 49.34, main/busy/ms 36.83, stage/render/prepare_assets 34.88, pipelines/driver_ms 24.42, stage/main/update 23.36, stream/drive/ms 19.06, stage/render/render 9.03, stage/main/post_update 8.73 |
| warm-up | 4.25 | 70.6 | pipelines/wait_ms 3526, pipelines/compile_ms 98.30, pipelines/front_ms 92.47, render/thread/ms 57.21, render/busy/ms 55.89, stage/render/prepare_assets 44.49, main/busy/ms 39.76, stage/main/update 22.11, stream/drive/ms 17.47, stage/main/post_update 7.44, pipelines/driver_ms 5.83, stage/render/render 5.74 |
| warm-up | 4.29 | 38.9 | pipelines/wait_ms 2592, pipelines/compile_ms 81.69, pipelines/front_ms 77.00, render/thread/ms 74.06, render/busy/ms 72.64, main/busy/ms 63.24, stage/render/prepare_assets 60.72, stage/main/update 46.16, stream/drive/ms 40.22, stage/main/post_update 8.72, stage/render/render 6.78, render/graph/ms 4.98 |
| warm-up | 4.41 | 124.3 | pipelines/wait_ms 1425, render/thread/ms 94.39, main/busy/ms 72.92, render/busy/ms 70.85, stage/render/prepare_assets 58.74, stage/main/update 58.14, stream/drive/ms 52.63, pipelines/compile_ms 34.80, pipelines/driver_ms 32.48, stage/render/render 28.48, render/present/ms 23.31, stage/main/post_update 6.73 |
| warm-up | 4.51 | 95.3 | pipelines/wait_ms 11.8 k, pipelines/compile_ms 311, pipelines/front_ms 230, render/thread/ms 135, render/busy/ms 84.16, pipelines/driver_ms 81.03, stage/render/prepare_assets 71.89, stage/render/render 55.02, render/present/ms 50.99, main/busy/ms 48.99, stage/main/update 29.76, stream/drive/ms 24.24 |
| warm-up | 4.62 | 74.3 | pipelines/wait_ms 5167, pipelines/compile_ms 104, pipelines/front_ms 97.82, main/busy/ms 73.72, stage/main/update 59.06, stream/drive/ms 33.23, render/thread/ms 32.75, render/busy/ms 30.94, stage/render/prepare_assets 12.41, stage/main/post_update 10.30, render/graph/ms 8.68, stage/render/cleanup 8.33 |
| warm-up | 4.73 | 61.6 | main/busy/ms 95.25, stage/main/update 87.31, stream/drive/ms 83.71, render/thread/ms 26.09, render/busy/ms 24.51, stage/render/prepare_assets 13.64, stage/render/render 6.17, stage/main/post_update 4.44, render/graph/ms 4.18, encode//sum 3.61, stage/render/graph_submit 2.68, stage/render/prepare_write_phase_buffers 1.99 |
| warm-up | 4.78 | 48.1 | pipelines/wait_ms 1238, pipelines/compile_ms 147, pipelines/front_ms 141, main/busy/ms 51.93, stage/main/update 39.83, stream/drive/ms 35.29, render/thread/ms 29.91, render/busy/ms 28.69, stage/render/prepare_assets 18.70, stage/main/post_update 7.46, pipelines/driver_ms 5.84, stage/render/render 4.81 |
| warm-up | 4.96 | 31.9 | render/thread/ms 38.73, render/busy/ms 37.35, main/busy/ms 26.50, stage/render/prepare_assets 20.23, stage/main/post_update 11.41, stage/main/update 9.86, stage/render/render 6.68, render/graph/ms 4.72, encode//sum 4.69, stream/drive/ms 4.54, stage/render/prepare_write_phase_buffers 3.18, stage/render/graph_submit 2.63 |
| warm-up | 5.23 | 42.6 | render/thread/ms 51.01, render/busy/ms 49.43, stage/render/prepare_assets 30.66, main/busy/ms 26.70, stage/main/update 10.98, stage/main/post_update 9.56, stage/render/render 6.68, stream/drive/ms 6.26, render/graph/ms 4.64, encode//sum 4.24, stage/render/graph_submit 3.07, stage/render/specialize 2.95 |
| warm-up | 5.26 | 32.4 | pipelines/wait_ms 1384, pipelines/compile_ms 186, pipelines/front_ms 174, render/thread/ms 58.14, render/busy/ms 56.83, stage/render/prepare_assets 42.65, main/busy/ms 29.64, stage/main/update 11.76, pipelines/driver_ms 11.26, stage/main/post_update 10.00, stream/drive/ms 6.14, stage/render/render 5.80 |
| warm-up | 5.37 | 29.0 | render/thread/ms 48.79, render/busy/ms 46.94, main/busy/ms 27.31, stage/render/prepare_assets 26.85, stage/main/post_update 10.72, stage/main/update 10.18, render/graph/ms 8.29, stage/render/render 6.27, encode//sum 6.08, stream/drive/ms 5.64, stage/render/graph_submit 5.45, stage/render/cleanup 4.89 |
| warm-up | 5.40 | 33.1 | pipelines/wait_ms 846, pipelines/compile_ms 159, pipelines/front_ms 153, render/thread/ms 50.26, render/busy/ms 48.13, main/busy/ms 28.17, stage/render/prepare_assets 26.66, stage/main/post_update 11.63, stage/render/render 11.02, stage/main/update 10.50, render/graph/ms 8.37, encode//sum 6.68 |
| warm-up | 5.43 | 32.8 | pipelines/wait_ms 9734, pipelines/compile_ms 198, pipelines/front_ms 189, render/thread/ms 44.39, render/busy/ms 42.71, stage/render/prepare_assets 28.00, main/busy/ms 23.85, stage/main/post_update 9.94, pipelines/driver_ms 8.71, stage/main/update 8.22, stage/render/render 7.17, encode//sum 5.38 |
| warm-up | 5.84 | 31.6 | main/busy/ms 50.02, render/thread/ms 32.85, render/busy/ms 30.95, stage/main/update 30.64, stream/drive/ms 15.06, stage/main/post_update 13.36, stage/render/render 12.21, stage/render/prepare_assets 10.94, encode//sum 10.32, render/graph/ms 9.10, stage/render/graph_submit 5.89, encode/submit 3.50 |
| warm-up | 5.91 | 70.0 | render/thread/ms 54.27, render/busy/ms 52.64, main/busy/ms 51.17, stage/render/prepare_assets 34.45, stage/main/update 30.85, stream/drive/ms 15.66, stage/main/post_update 14.01, stage/render/render 9.06, render/graph/ms 6.66, encode//sum 6.53, stage/render/graph_submit 4.50, encode/submit 3.14 |
| warm-up | 5.94 | 37.0 | pipelines/wait_ms 3603, pipelines/compile_ms 169, pipelines/front_ms 162, render/thread/ms 58.36, render/busy/ms 57.01, main/busy/ms 47.60, stage/render/prepare_assets 43.10, stage/main/update 26.12, stage/main/post_update 14.70, stage/render/render 6.59, pipelines/driver_ms 6.32, encode//sum 4.86 |
| warm-up | 5.97 | 28.9 | pipelines/wait_ms 5842, pipelines/compile_ms 220, pipelines/front_ms 212, render/thread/ms 41.13, main/busy/ms 40.15, render/busy/ms 39.91, stage/render/prepare_assets 26.66, stage/main/update 23.10, stage/main/post_update 11.17, pipelines/driver_ms 7.79, stage/render/render 5.71, encode//sum 5.00 |
| warm-up | 6.61 | 44.6 | main/busy/ms 54.47, stage/main/update 42.51, render/thread/ms 40.24, render/busy/ms 38.68, stage/render/prepare_assets 18.29, encode//sum 10.30, stage/render/render 9.59, stage/main/post_update 8.16, render/graph/ms 7.39, stage/render/graph_submit 4.77, stage/render/prepare_write_phase_buffers 2.99, encode/submit 2.84 |
| warm-up | 6.65 | 38.5 | main/busy/ms 52.17, render/thread/ms 49.65, render/busy/ms 48.34, stage/main/update 39.43, stage/render/prepare_assets 28.94, stage/main/post_update 8.50, stage/render/render 8.36, encode//sum 8.28, render/graph/ms 6.74, stage/render/graph_submit 4.78, encode/submit 2.83, stage/render/specialize 2.01 |
| warm-up | 6.94 | 59.1 | pipelines/wait_ms 3192, pipelines/compile_ms 70.09, render/thread/ms 65.68, render/busy/ms 64.63, stage/render/prepare_assets 53.48, pipelines/front_ms 35.54, pipelines/driver_ms 34.55, main/busy/ms 12.42, encode//sum 8.74, stage/render/render 6.86, stage/main/post_update 6.43, render/graph/ms 5.52 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\b2_off\balmora@2560x1440_off=flatnormal-2 --bench-baseline none --off flatnormal
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 16%
- during the run: other processes 9% CPU (system minus this process)
- nvidia-smi over the window (58 samples): GPU load **98%** (min 97%) · 2580 MHz (min 2580) · memory 8001 MHz · 90.2 W · 73 °C (max 76) · VRAM 6098 MiB · throttled 100% of samples
