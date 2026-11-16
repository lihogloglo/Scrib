# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1479 frames over 15.0 s · **median 10.09 ms (99 fps)** · mean 10.15 · p95 11.52 · p99 11.99 (1% low 83 fps) · worst 14.78 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 280 frames over 8.3 s, worst 2494.3 ms
- hitches (> 25.2 ms): **0** measured, 56 in warm-up
- timed GPU passes: 8.92 ms of a 10.15 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.09 ms, limit 11 ms
- pass worst frame 14.78 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.790 | 2.809 | 2.928 | 0.99 | 0.060 |
| dlss_super_resolution | 2.039 | 2.053 | 2.076 | 0.99 | 0.186 |
| early prepass | 0.779 | 0.784 | 1.028 | 0.99 | 0.088 |
| atmosphere_luts | 0.445 | 0.448 | 0.714 | 0.99 | 0.003 |
| bloom | 0.318 | 0.320 | 0.335 | 0.99 | 0.064 |
| main_transmissive_pass_3d | 0.264 | 0.265 | 0.294 | 0.99 | 0.008 |
| water_velocity | 0.242 | 0.244 | 0.270 | 0.99 | 0.001 |
| ssao | 0.211 | 0.213 | 0.354 | 0.99 | 0.007 |
| water_sspr | 0.205 | 0.206 | 0.213 | 0.99 | 0.002 |
| per_view_shadow_pass_cascade1 | 0.174 | 0.701 | 0.987 | 0.25 | 0.046 |
| clouds_trace | 0.150 | 0.151 | 0.173 | 0.99 | 0.001 |
| atmos_froxel_inject | 0.149 | 0.150 | 0.158 | 0.99 | 0.001 |
| water_fft | 0.137 | 0.138 | 0.140 | 0.99 | 0.003 |
| uniform_allocation | 0.126 | 0.127 | 0.154 | 0.99 | 0.022 |
| upscaling | 0.115 | 0.116 | 0.121 | 0.99 | 0.004 |
| clouds_composite | 0.114 | 0.115 | 0.117 | 0.99 | 0.002 |
| bin_unpacking | 0.102 | 0.102 | 0.115 | 0.99 | 0.015 |
| tonemapping | 0.086 | 0.086 | 0.093 | 0.99 | 0.003 |
| clustering | 0.071 | 0.071 | 0.082 | 0.99 | 0.022 |
| render_sky | 0.053 | 0.053 | 0.055 | 0.99 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.420 | 0.417 | 0.12 | 0.001 |
| clouds_resolve | 0.051 | 0.051 | 0.052 | 0.99 | 0.001 |
| clouds_shadow | 0.032 | 0.032 | 0.036 | 0.99 | 0.003 |
| early_mesh_preprocessing | 0.031 | 0.032 | 0.038 | 0.99 | 0.005 |
| lightprobe_radiance_map | 0.028 | 0.227 | 0.240 | 0.12 | 0.004 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.029 | 0.99 | 0.001 |
| main_transparent_pass_3d | 0.027 | 0.027 | 0.035 | 0.99 | 0.074 |
| water_snapshot | 0.025 | 0.025 | 0.027 | 0.99 | 0.002 |
| water_mips | 0.025 | 0.025 | 0.024 | 0.99 | 0.002 |
| atmosphere_environment | 0.013 | 0.013 | 0.015 | 0.99 | 0.003 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 0.99 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.010 | 0.016 | 0.99 | 0.005 |
| sparse buffer updates | 0.007 | 0.007 | 0.007 | 0.99 | 0.003 |
| night_sky | 0.003 | 0.004 | 0.008 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.99 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.015 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.12 | 0.001 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.042 | 0.042 | 0.00 | 0.017 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.016 | 0.016 | 0.00 | 0.006 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.00 | 0.006 |
| surface_trail | — | — | — | - | — |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.236 | 10.597 | 13.81 | 13660.7 |
| encode//sum | 7.742 | 10.080 | 12.42 | 11450.9 |
| render/busy/ms | 7.607 | 8.625 | 13.21 | 11250.5 |
| main/busy/ms | 7.140 | 8.303 | 9.55 | 10560.1 |
| stage/render/render | 5.436 | 6.346 | 10.88 | 8039.3 |
| render/graph/ms | 4.733 | 5.614 | 10.16 | 7000.2 |
| stage/main/post_update | 4.298 | 5.368 | 6.42 | 6356.6 |
| stage/render/graph_submit | 3.737 | 4.594 | 9.20 | 5526.7 |
| encode/submit | 2.039 | 2.602 | 7.19 | 3016.3 |
| stage/main/update | 1.773 | 2.091 | 3.04 | 2621.7 |
| encode/finish_wall | 1.741 | 2.177 | 2.77 | 2574.9 |
| stage/render/prepare_views | 1.195 | 2.735 | 3.68 | 1766.9 |
| render/acquire/ms | 1.150 | 2.679 | 3.63 | 1700.6 |
| stage/render/camera_driver | 0.937 | 1.110 | 1.39 | 1386.2 |
| stage/render/prepare_write_phase_buffers | 0.580 | 0.759 | 1.08 | 857.7 |
| render/present/ms | 0.480 | 0.562 | 2.55 | 709.6 |
| stage/render/prepare_assets | 0.441 | 0.558 | 1.64 | 651.5 |
| bigspace/total/ms | 0.379 | 0.927 | 2.22 | 560.1 |
| stage/render/prepare_resources | 0.343 | 0.432 | 0.69 | 507.9 |
| stage/main/pre_update | 0.310 | 0.418 | 0.84 | 457.8 |
| stage/main/first | 0.290 | 0.355 | 1.44 | 429.6 |
| terrain/update/ms | 0.235 | 0.299 | 0.51 | 347.2 |
| stage/render/prepare_bind_groups | 0.207 | 0.296 | 0.61 | 305.5 |
| terrain/select/ms | 0.205 | 0.262 | 0.48 | 302.8 |
| stream/drive/ms | 0.186 | 0.262 | 0.49 | 275.5 |
| stage/render/prepare_meshes | 0.186 | 0.236 | 0.44 | 275.0 |
| stage/main/fixed_main_loop | 0.179 | 0.332 | 0.96 | 264.8 |
| stage/render/prepare_flush | 0.175 | 0.352 | 0.60 | 258.9 |
| controller/feed/ms | 0.165 | 0.222 | 0.53 | 244.2 |
| stage/render/prepare_batch_phases | 0.162 | 0.219 | 0.36 | 239.8 |
| stage/render/extract_commands | 0.161 | 0.195 | 0.33 | 238.0 |
| far/select/ms | 0.159 | 0.223 | 0.42 | 235.1 |
| stream/tick/ms | 0.158 | 0.227 | 0.45 | 234.0 |
| bigspace/high_precision/ms | 0.152 | 0.525 | 1.93 | 225.2 |
| stage/render/create_views | 0.143 | 0.181 | 0.45 | 211.2 |
| stage/main/last | 0.113 | 0.153 | 0.72 | 167.8 |
| controller/move/ms | 0.109 | 0.170 | 0.39 | 160.7 |
| anim/evaluate/ms | 0.104 | 0.238 | 1.86 | 153.9 |
| stage/render/queue | 0.103 | 0.135 | 0.47 | 151.9 |
| bigspace/low_precision/ms | 0.096 | 0.230 | 2.02 | 141.9 |
| stage/render/specialize | 0.086 | 0.113 | 0.49 | 127.0 |
| stage/render/phase_sort | 0.067 | 0.114 | 0.36 | 99.3 |
| anim/evaluate_cpu/ms | 0.057 | 0.072 | 0.26 | 83.6 |
| encode/resolve_diagnostics/sum | 0.056 | 0.093 | 0.33 | 82.9 |
| stage/render/prepare_collect_phase_buffers | 0.051 | 0.085 | 0.39 | 75.5 |
| encode/camera_driver_span_begin/sum | 0.028 | 0.047 | 0.23 | 40.7 |
| physics/step/ms | 0.027 | 0.052 | 0.17 | 39.9 |
| weather/map/ms | 0.025 | 0.032 | 0.12 | 36.8 |
| stage/render/cleanup | 0.023 | 0.034 | 0.21 | 34.3 |
| encode/camera_driver_span_end/sum | 0.023 | 0.043 | 0.11 | 34.2 |
| weather/eval/ms | 0.011 | 0.014 | 0.06 | 15.7 |
| script/update/ms | 0.006 | 0.009 | 0.11 | 9.6 |
| controller/avoid/ms | 0.006 | 0.007 | 0.02 | 8.3 |
| script/ui/ms | 0.005 | 0.007 | 0.04 | 7.3 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.0 |
| anim/apply/ms | 0.004 | 0.005 | 0.19 | 6.5 |
| controller/write/ms | 0.002 | 0.002 | 0.01 | 2.8 |
| weather/fx/ms | 0.001 | 0.002 | 0.00 | 1.5 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 1.0 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 0.9 |
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
| encode//max | ms | 1.43 | 2.26 | 1.38 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.23 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.11 | 0.03 |
| encode/finish_critical | ms | 1.43 | 2.26 | 1.38 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.33 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 10.13 | 13.21 | 10.22 |
| gpu/timed/ms | ms | 10.13 | 11.71 | 10.22 |
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
| process/cpu_usage | % | 24.34 | 41.15 | 22.12 |
| process/mem_usage | GiB | 4.62 | 4.69 | 4.69 |
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
| system/cpu_usage | % | 30.72 | 43.21 | 28.37 |
| system/mem_usage | % | 48.70 | 48.96 | 48.96 |
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
| anim/events/count | count | 0.09 | 2 | 126 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.35 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 31.50 | 34 | 46.6 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1479 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1479 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1479 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.61 | 1 | 900 |
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

## Hitches (> 25.2 ms; slowest 56, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.49 | 2494.3 | main/busy/ms 2466, stage/main/post_update 2365, stage/main/update 95.05, surface/advance_cpu_ms 67.45, stage/main/pre_update 2.34, stage/main/last 1.19, surface/cell_upload_ms 0.68, stage/main/first 0.46, stage/main/fixed_main_loop 0.40, stream/drive/ms 0.30, weather/map/ms 0.13, bigspace/total/ms 0.08 |
| warm-up | 3.23 | 731.5 | main/busy/ms 2595, stage/main/post_update 2389, render/thread/ms 703, render/busy/ms 531, stage/render/prepare_assets 494, stage/main/update 194, stage/render/prepare_views 169, render/acquire/ms 168, surface/advance_cpu_ms 67.45, stage/render/prepare_resources 21.67, stage/render/render 12.16, encode//sum 9.38 |
| warm-up | 3.31 | 83.8 | render/thread/ms 770, render/busy/ms 596, stage/render/prepare_assets 541, stage/render/prepare_views 170, render/acquire/ms 168, main/busy/ms 149, pipelines/compile_ms 138, pipelines/front_ms 124, stage/main/update 104, stage/main/post_update 35.07, stage/render/prepare_resources 26.61, stage/render/render 23.96 |
| warm-up | 3.35 | 38.6 | pipelines/compile_ms 243, pipelines/front_ms 216, pipelines/wait_ms 109, render/thread/ms 104, render/busy/ms 87.50, main/busy/ms 56.52, stage/render/prepare_assets 47.82, stage/render/render 41.29, stage/main/post_update 33.89, pipelines/driver_ms 26.53, terrain/update/ms 24.64, render/graph/ms 24.41 |
| warm-up | 3.40 | 52.5 | pipelines/wait_ms 1128, pipelines/compile_ms 437, pipelines/front_ms 399, render/thread/ms 88.09, main/busy/ms 76.53, render/busy/ms 65.97, stage/main/post_update 56.18, stage/render/render 40.64, pipelines/driver_ms 38.52, render/graph/ms 21.59, stage/render/prepare_resources 18.72, render/present/ms 14.84 |
| warm-up | 3.51 | 107.3 | pipelines/wait_ms 4110, pipelines/compile_ms 1291, pipelines/front_ms 1226, main/busy/ms 161, render/thread/ms 137, render/busy/ms 128, stage/main/post_update 69.90, pipelines/driver_ms 64.66, stage/main/update 56.38, stage/render/prepare_resources 55.04, stage/render/prepare_assets 26.37, stage/render/prepare_bind_groups 18.99 |
| warm-up | 3.62 | 113.2 | pipelines/wait_ms 6372, pipelines/compile_ms 2928, pipelines/front_ms 2799, render/thread/ms 162, render/busy/ms 159, main/busy/ms 142, pipelines/driver_ms 129, stage/render/prepare_assets 79.51, stage/main/update 59.85, stage/main/post_update 42.71, stage/render/prepare_resources 40.04, stage/render/prepare_bind_groups 19.12 |
| warm-up | 3.66 | 34.3 | pipelines/wait_ms 3321, pipelines/compile_ms 2121, pipelines/front_ms 2020, render/thread/ms 107, render/busy/ms 104, pipelines/driver_ms 101, stage/render/prepare_assets 82.75, main/busy/ms 53.18, stage/main/update 15.49, stage/main/post_update 14.62, stage/render/render 13.34, encode//sum 11.18 |
| warm-up | 3.70 | 40.9 | pipelines/compile_ms 411, pipelines/front_ms 393, pipelines/wait_ms 119, render/thread/ms 68.02, render/busy/ms 66.07, stage/render/prepare_assets 49.92, main/busy/ms 42.81, pipelines/driver_ms 18.83, stage/main/post_update 13.56, stage/main/update 12.88, stage/render/render 9.67, terrain/update/ms 9.03 |
| warm-up | 3.73 | 32.2 | pipelines/compile_ms 854, pipelines/front_ms 825, pipelines/wait_ms 166, render/thread/ms 67.62, render/busy/ms 65.63, pipelines/driver_ms 29.49, stage/render/prepare_assets 29.48, main/busy/ms 29.20, encode//sum 22.96, stage/render/prepare_resources 17.51, stage/main/post_update 13.90, stage/render/render 11.95 |
| warm-up | 3.78 | 27.7 | pipelines/compile_ms 328, pipelines/front_ms 315, render/thread/ms 43.06, render/busy/ms 40.92, main/busy/ms 26.04, pipelines/wait_ms 21.90, stage/render/render 18.05, stage/render/prepare_assets 15.85, render/graph/ms 15.67, stage/render/graph_submit 13.49, encode//sum 13.45, pipelines/driver_ms 12.86 |
| warm-up | 3.81 | 31.6 | pipelines/compile_ms 959, pipelines/front_ms 911, pipelines/wait_ms 213, main/busy/ms 56.34, render/thread/ms 54.41, render/busy/ms 52.20, pipelines/driver_ms 47.81, stage/main/update 42.42, stream/drive/ms 38.89, stage/render/prepare_assets 35.22, stage/render/render 11.91, render/graph/ms 9.37 |
| warm-up | 3.86 | 55.7 | pipelines/compile_ms 1258, pipelines/front_ms 1200, pipelines/wait_ms 203, render/thread/ms 62.78, render/busy/ms 61.11, pipelines/driver_ms 57.28, main/busy/ms 52.30, stage/render/prepare_assets 48.09, stage/main/update 43.03, stream/drive/ms 38.93, stage/render/render 9.19, render/graph/ms 7.23 |
| warm-up | 3.90 | 31.2 | pipelines/compile_ms 856, pipelines/front_ms 827, render/thread/ms 64.30, render/busy/ms 60.60, main/busy/ms 39.10, pipelines/driver_ms 29.13, stage/render/prepare_assets 28.62, stage/main/update 26.90, stage/render/render 23.79, stream/drive/ms 17.98, render/graph/ms 15.47, pipelines/wait_ms 12.79 |
| warm-up | 3.93 | 33.9 | pipelines/compile_ms 479, pipelines/front_ms 464, render/thread/ms 56.49, render/busy/ms 52.32, main/busy/ms 41.41, stage/main/update 27.98, stage/render/render 24.64, stream/drive/ms 19.40, stage/render/prepare_assets 18.76, render/graph/ms 15.91, pipelines/driver_ms 14.93, stage/render/graph_submit 8.83 |
| warm-up | 3.98 | 45.4 | pipelines/compile_ms 474, pipelines/front_ms 457, render/thread/ms 63.80, render/busy/ms 61.41, main/busy/ms 56.24, stage/render/prepare_assets 42.07, stage/main/update 29.23, stream/drive/ms 23.88, pipelines/driver_ms 17.24, stage/main/post_update 15.22, stage/render/render 13.48, render/graph/ms 10.57 |
| warm-up | 4.09 | 111.0 | pipelines/compile_ms 524, pipelines/front_ms 504, render/thread/ms 115, render/busy/ms 111, main/busy/ms 106, stage/render/prepare_assets 76.68, stage/main/update 69.79, stream/drive/ms 40.52, stage/render/render 24.22, stage/main/post_update 20.90, pipelines/driver_ms 20.18, render/graph/ms 18.79 |
| warm-up | 4.23 | 142.4 | pipelines/compile_ms 1955, pipelines/front_ms 1892, render/thread/ms 175, render/busy/ms 170, stage/render/prepare_assets 139, main/busy/ms 131, stage/main/update 104, pipelines/driver_ms 63.31, stream/drive/ms 62.23, stage/render/render 22.32, pipelines/wait_ms 17.98, render/graph/ms 16.47 |
| warm-up | 4.31 | 84.4 | pipelines/compile_ms 2505, pipelines/front_ms 2423, render/thread/ms 145, render/busy/ms 142, stage/render/prepare_assets 122, main/busy/ms 100, pipelines/wait_ms 88.55, pipelines/driver_ms 81.71, stage/main/update 75.69, stream/drive/ms 45.77, pipelines/lock_ms 13.21, stage/render/render 12.37 |
| warm-up | 4.38 | 69.1 | pipelines/compile_ms 822, pipelines/front_ms 794, render/thread/ms 110, render/busy/ms 108, pipelines/wait_ms 76.61, main/busy/ms 73.82, stage/main/update 52.32, stage/render/prepare_assets 43.98, pipelines/driver_ms 28.31, stage/render/prepare_resources 25.14, stage/render/render 17.07, render/graph/ms 14.44 |
| warm-up | 4.44 | 54.6 | pipelines/compile_ms 1913, pipelines/front_ms 1852, pipelines/wait_ms 270, render/thread/ms 97.80, render/busy/ms 95.99, main/busy/ms 71.32, pipelines/driver_ms 61.14, stage/main/update 55.41, stage/render/prepare_assets 28.86, stage/render/prepare_resources 25.26, stage/render/render 16.73, render/graph/ms 14.39 |
| warm-up | 4.49 | 32.4 | pipelines/compile_ms 472, pipelines/front_ms 449, pipelines/wait_ms 392, render/thread/ms 51.81, render/busy/ms 50.31, main/busy/ms 43.53, pipelines/driver_ms 22.81, stage/main/post_update 21.30, stage/render/prepare_resources 18.69, stage/main/update 14.32, stage/render/render 10.82, encode//sum 9.03 |
| warm-up | 4.53 | 37.5 | pipelines/compile_ms 634, pipelines/front_ms 611, pipelines/wait_ms 178, render/thread/ms 61.88, render/busy/ms 60.35, main/busy/ms 49.09, stage/render/prepare_assets 25.39, pipelines/driver_ms 23.81, stage/main/post_update 23.74, stage/main/update 15.26, stage/render/prepare_resources 14.27, stage/render/render 10.63 |
| warm-up | 4.58 | 52.3 | pipelines/compile_ms 526, pipelines/front_ms 506, pipelines/wait_ms 189, render/thread/ms 79.97, render/busy/ms 78.05, main/busy/ms 67.58, stage/main/post_update 44.71, stage/render/render 33.57, stage/render/prepare_assets 32.75, render/graph/ms 31.58, stage/render/camera_driver 23.34, pipelines/driver_ms 19.86 |
| warm-up | 4.61 | 29.1 | pipelines/compile_ms 494, pipelines/front_ms 477, pipelines/wait_ms 267, main/busy/ms 94.19, render/thread/ms 70.87, render/busy/ms 68.71, stage/main/update 54.10, stream/drive/ms 42.72, stage/render/render 35.40, stage/main/post_update 35.13, render/graph/ms 32.98, stage/render/camera_driver 23.62 |
| warm-up | 4.66 | 47.4 | pipelines/compile_ms 2146, pipelines/front_ms 2098, pipelines/wait_ms 841, main/busy/ms 92.97, stage/main/update 81.06, stream/drive/ms 75.24, pipelines/driver_ms 47.29, render/thread/ms 38.84, render/busy/ms 37.08, stage/render/prepare_assets 15.47, stage/render/render 12.47, encode//sum 11.09 |
| warm-up | 4.71 | 53.6 | pipelines/compile_ms 1997, pipelines/front_ms 1951, pipelines/wait_ms 731, main/busy/ms 69.99, pipelines/driver_ms 46.83, stage/main/update 43.23, stream/drive/ms 36.27, render/thread/ms 35.27, render/busy/ms 33.59, stage/main/post_update 18.46, stage/render/prepare_assets 12.14, stage/render/render 11.68 |
| warm-up | 4.76 | 43.8 | pipelines/compile_ms 353, pipelines/front_ms 336, main/busy/ms 62.93, render/thread/ms 50.06, render/busy/ms 47.39, stage/main/update 32.58, stage/render/render 19.70, stage/main/post_update 18.05, pipelines/driver_ms 16.89, render/graph/ms 16.41, stage/render/prepare_assets 13.48, encode//sum 11.69 |
| warm-up | 4.82 | 65.4 | pipelines/compile_ms 328, pipelines/front_ms 316, main/busy/ms 84.87, render/thread/ms 73.37, render/busy/ms 70.85, stage/main/update 63.83, stage/render/prepare_assets 35.82, stage/render/render 21.87, render/graph/ms 18.84, pipelines/driver_ms 12.91, stage/render/graph_submit 12.17, encode//sum 11.45 |
| warm-up | 4.87 | 45.2 | pipelines/compile_ms 834, pipelines/front_ms 807, render/thread/ms 73.32, render/busy/ms 69.88, main/busy/ms 61.89, stage/main/update 43.42, stage/render/prepare_assets 37.92, pipelines/wait_ms 36.66, pipelines/driver_ms 26.45, stage/render/render 15.08, render/graph/ms 11.20, stream/drive/ms 10.37 |
| warm-up | 4.90 | 29.8 | pipelines/compile_ms 842, pipelines/front_ms 816, render/thread/ms 58.65, render/busy/ms 55.04, main/busy/ms 38.40, pipelines/wait_ms 33.70, pipelines/driver_ms 26.43, stage/main/post_update 16.32, stage/main/update 15.95, stage/render/render 15.76, encode//sum 11.95, stage/render/prepare_assets 11.88 |
| warm-up | 4.92 | 25.5 | pipelines/compile_ms 1035, pipelines/front_ms 1007, render/thread/ms 48.48, render/busy/ms 46.69, main/busy/ms 39.89, pipelines/driver_ms 27.90, stage/main/update 18.11, stage/main/post_update 15.92, pipelines/wait_ms 15.64, pipelines/lock_ms 15.25, stage/render/render 14.83, render/graph/ms 12.65 |
| warm-up | 5.00 | 27.9 | pipelines/compile_ms 800, pipelines/front_ms 773, render/thread/ms 40.22, render/busy/ms 38.71, main/busy/ms 35.90, pipelines/driver_ms 26.73, stage/main/update 20.51, stage/render/prepare_resources 13.09, stage/render/render 10.76, encode//sum 10.55, stage/main/post_update 10.04, render/graph/ms 8.94 |
| warm-up | 5.03 | 30.3 | pipelines/compile_ms 845, pipelines/front_ms 819, render/thread/ms 51.35, render/busy/ms 49.57, main/busy/ms 44.84, pipelines/driver_ms 26.09, stage/main/update 25.33, encode//sum 17.64, stage/render/render 13.91, stage/main/post_update 13.75, stage/render/prepare_resources 13.32, render/graph/ms 11.78 |
| warm-up | 5.05 | 29.3 | pipelines/compile_ms 361, pipelines/front_ms 352, render/thread/ms 47.86, render/busy/ms 45.86, main/busy/ms 41.47, stage/main/update 21.07, encode//sum 17.91, stage/render/prepare_assets 16.87, stage/render/render 14.69, stage/main/post_update 13.43, render/graph/ms 12.19, stream/drive/ms 9.25 |
| warm-up | 5.10 | 42.6 | pipelines/compile_ms 239, pipelines/front_ms 230, render/thread/ms 54.74, render/busy/ms 52.96, main/busy/ms 36.25, stage/render/prepare_assets 28.06, stage/main/update 16.98, stage/main/post_update 12.03, encode//sum 11.66, stage/render/render 10.81, stream/drive/ms 9.53, pipelines/driver_ms 8.66 |
| warm-up | 5.13 | 34.5 | pipelines/compile_ms 464, pipelines/front_ms 448, render/thread/ms 62.65, render/busy/ms 60.94, main/busy/ms 43.15, stage/render/prepare_assets 30.14, stage/main/update 19.47, pipelines/driver_ms 16.32, stage/main/post_update 16.30, encode//sum 12.13, stage/render/render 11.30, render/graph/ms 9.33 |
| warm-up | 5.18 | 45.9 | pipelines/compile_ms 1190, pipelines/front_ms 1154, render/thread/ms 61.33, render/busy/ms 59.75, main/busy/ms 43.93, pipelines/wait_ms 39.53, pipelines/driver_ms 36.43, stage/render/prepare_assets 31.18, stage/main/update 21.68, stage/main/post_update 14.85, encode//sum 13.51, stage/render/render 12.45 |
| warm-up | 5.21 | 33.1 | pipelines/compile_ms 1158, pipelines/front_ms 1123, render/thread/ms 59.71, render/busy/ms 58.11, pipelines/wait_ms 40.28, pipelines/driver_ms 34.91, main/busy/ms 32.00, stage/render/prepare_assets 29.66, stage/render/render 14.76, stage/main/post_update 13.05, stage/main/update 12.74, render/graph/ms 12.65 |
| warm-up | 5.26 | 26.6 | pipelines/compile_ms 661, pipelines/front_ms 638, render/thread/ms 44.64, render/busy/ms 43.16, pipelines/wait_ms 40.87, main/busy/ms 33.55, pipelines/driver_ms 22.13, stage/main/post_update 21.58, encode//sum 13.86, stage/render/render 12.73, render/graph/ms 10.94, stage/render/prepare_resources 10.73 |
| warm-up | 5.28 | 25.3 | pipelines/compile_ms 475, pipelines/front_ms 459, render/thread/ms 49.42, render/busy/ms 47.88, pipelines/wait_ms 47.69, main/busy/ms 41.79, stage/main/post_update 19.48, stage/main/update 18.04, encode//sum 15.59, pipelines/driver_ms 15.54, stage/render/render 12.87, stage/render/prepare_resources 11.30 |
| warm-up | 5.35 | 27.9 | pipelines/compile_ms 200, pipelines/front_ms 194, render/thread/ms 40.78, render/busy/ms 39.33, main/busy/ms 29.22, stage/render/prepare_assets 17.61, pipelines/wait_ms 16.76, stage/main/post_update 13.10, encode//sum 12.88, stage/render/render 11.79, stage/main/update 10.69, render/graph/ms 10.00 |
| warm-up | 5.38 | 26.6 | pipelines/compile_ms 612, pipelines/front_ms 598, render/thread/ms 50.57, render/busy/ms 48.99, main/busy/ms 31.89, stage/render/prepare_assets 21.56, stage/main/post_update 15.24, encode//sum 14.38, pipelines/driver_ms 14.09, stage/render/render 12.55, stage/main/update 10.41, render/graph/ms 10.31 |
| warm-up | 5.43 | 28.4 | pipelines/compile_ms 768, pipelines/front_ms 750, render/thread/ms 45.81, render/busy/ms 44.61, main/busy/ms 33.70, stage/render/prepare_assets 25.55, pipelines/driver_ms 17.93, stage/main/update 17.86, encode//sum 13.79, stage/render/render 12.37, pipelines/wait_ms 11.77, render/graph/ms 10.76 |
| warm-up | 5.46 | 27.2 | render/thread/ms 49.01, render/busy/ms 47.70, stage/render/prepare_assets 30.59, main/busy/ms 25.94, stage/main/update 12.41, encode//sum 11.08, stage/main/post_update 8.52, stage/render/render 8.49, render/graph/ms 6.82, stage/render/graph_submit 5.05, stream/drive/ms 4.59, encode/submit 2.81 |
| warm-up | 5.51 | 36.2 | pipelines/compile_ms 311, pipelines/front_ms 305, render/thread/ms 46.81, render/busy/ms 45.19, main/busy/ms 30.52, stage/render/prepare_assets 19.79, encode//sum 19.10, stage/render/render 15.18, render/graph/ms 13.01, stage/main/post_update 12.04, stage/main/update 11.59, stage/render/graph_submit 9.57 |
| warm-up | 5.55 | 32.6 | pipelines/compile_ms 152, pipelines/front_ms 149, render/thread/ms 52.67, render/busy/ms 51.34, stage/render/prepare_assets 31.09, main/busy/ms 31.04, stage/main/update 12.52, encode//sum 12.46, stage/main/post_update 10.39, stage/render/render 9.08, render/graph/ms 7.50, stage/render/graph_submit 5.49 |
| warm-up | 5.58 | 30.2 | render/thread/ms 50.28, render/busy/ms 49.09, stage/render/prepare_assets 31.53, main/busy/ms 28.52, stage/main/update 12.75, encode//sum 11.16, stage/main/post_update 10.38, stage/render/render 8.25, render/graph/ms 6.87, stage/render/graph_submit 5.11, stream/drive/ms 4.22, encode/submit 2.78 |
| warm-up | 5.63 | 31.0 | render/thread/ms 46.06, render/busy/ms 44.87, main/busy/ms 30.51, stage/render/prepare_assets 25.28, encode//sum 14.49, stage/main/update 13.52, stage/main/post_update 11.13, stage/render/render 10.56, render/graph/ms 8.89, stage/render/graph_submit 6.41, stream/drive/ms 3.79, encode/submit 3.34 |
| warm-up | 5.69 | 40.4 | render/thread/ms 48.97, render/busy/ms 47.05, main/busy/ms 33.68, stage/render/prepare_assets 22.60, stage/main/post_update 15.39, encode//sum 14.71, stage/render/render 14.18, stage/main/update 12.70, render/graph/ms 11.80, stage/render/graph_submit 8.83, encode/submit 5.80, stream/drive/ms 5.17 |
| warm-up | 5.73 | 35.2 | render/thread/ms 51.32, render/busy/ms 50.12, stage/render/prepare_assets 32.29, main/busy/ms 29.61, encode//sum 13.90, stage/main/post_update 12.46, stage/main/update 11.46, stage/render/render 9.67, render/graph/ms 8.14, stage/render/graph_submit 6.02, encode/submit 3.41, stream/drive/ms 3.39 |
| warm-up | 5.79 | 35.8 | main/busy/ms 44.02, render/thread/ms 31.37, render/busy/ms 30.01, stage/main/update 27.54, stream/drive/ms 14.49, stage/render/prepare_assets 13.37, encode//sum 11.23, stage/main/post_update 10.43, stage/render/render 9.75, pipelines/compile_ms 8.46, render/graph/ms 8.02, pipelines/front_ms 7.71 |
| warm-up | 5.92 | 29.5 | main/busy/ms 32.56, render/thread/ms 25.65, render/busy/ms 24.54, stage/main/update 21.00, stream/drive/ms 13.64, encode//sum 12.28, stage/render/render 10.09, stage/render/prepare_assets 8.81, render/graph/ms 8.62, stage/main/post_update 7.85, stage/render/graph_submit 6.22, encode/submit 3.26 |
| warm-up | 6.15 | 50.1 | encode//sum 85.85, render/thread/ms 61.60, render/busy/ms 60.48, stage/render/render 46.45, render/graph/ms 45.03, stage/render/graph_submit 42.81, encode/finish_wall 39.09, main/busy/ms 21.12, stage/main/post_update 8.99, stage/main/update 8.40, stage/render/prepare_assets 7.81, encode/submit 3.77 |
| warm-up | 6.24 | 28.3 | main/busy/ms 42.71, render/thread/ms 41.32, render/busy/ms 39.25, encode//sum 23.92, stage/main/update 21.80, stage/render/render 18.90, render/graph/ms 16.00, stage/main/post_update 14.30, stage/render/prepare_assets 12.00, stage/render/graph_submit 11.63, stream/drive/ms 11.22, encode/finish_wall 6.42 |
| warm-up | 6.28 | 44.1 | render/thread/ms 49.92, render/busy/ms 48.42, main/busy/ms 40.09, stage/render/prepare_assets 28.61, stage/main/update 19.88, encode//sum 18.31, stage/main/post_update 14.30, stage/render/render 12.58, stream/drive/ms 11.25, render/graph/ms 10.36, stage/render/graph_submit 7.24, encode/finish_wall 4.63 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_noslab\balmora@2560x1440_off=meshslabs-3 --bench-baseline none --off meshslabs
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 6% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.7 W · 74 °C (max 76) · VRAM 4928 MiB · throttled 100% of samples
