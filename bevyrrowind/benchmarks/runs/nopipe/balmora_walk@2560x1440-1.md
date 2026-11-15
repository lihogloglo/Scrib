# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 3172 frames over 65.7 s · **median 20.48 ms (49 fps)** · mean 20.70 · p95 25.39 · p99 30.91 (1% low 32 fps) · worst 51.60 ms · 34 spikes over 1.5× median · 24.7 stutters/min (work 32.9/min)
- warm-up: 326 frames over 10.9 s, worst 3151.7 ms
- hitches (> 51.2 ms): **1** measured, 15 in warm-up
- timed GPU passes: 8.00 ms of a 20.70 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 20.48 ms, limit 11 ms
- **FAIL** frame_ms: 27 stutters (> 1.5x moving median of 60) in 1.09 min = 24.68/min, limit 1/min
- **FAIL** worst frame 51.60 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.234 | 2.236 | 2.864 | 1.00 | 0.176 |
| main_opaque_pass_3d | 1.781 | 1.782 | 2.533 | 1.00 | 0.053 |
| early prepass | 0.563 | 0.563 | 0.934 | 1.00 | 0.073 |
| bloom | 0.346 | 0.347 | 0.545 | 1.00 | 0.061 |
| atmosphere_luts | 0.337 | 0.337 | 0.530 | 1.00 | 0.003 |
| water_velocity | 0.298 | 0.298 | 0.518 | 1.00 | 0.001 |
| auxiliary_views | 0.227 | 0.227 | 0.791 | 1.00 | 0.181 |
| water_sspr | 0.225 | 0.225 | 0.375 | 1.00 | 0.002 |
| ssao | 0.207 | 0.207 | 0.214 | 1.00 | 0.007 |
| uniform_allocation | 0.188 | 0.188 | 0.191 | 1.00 | 0.029 |
| atmos_froxel_inject | 0.160 | 0.160 | 0.156 | 1.00 | 0.001 |
| clouds_trace | 0.141 | 0.141 | 0.230 | 1.00 | 0.001 |
| water_fft | 0.140 | 0.140 | 0.141 | 1.00 | 0.002 |
| bin_unpacking | 0.139 | 0.139 | 0.145 | 1.00 | 0.020 |
| upscaling | 0.133 | 0.133 | 0.139 | 1.00 | 0.004 |
| clouds_composite | 0.112 | 0.113 | 0.111 | 1.00 | 0.002 |
| main_transmissive_pass_3d | 0.093 | 0.093 | 0.172 | 1.00 | 0.007 |
| tonemapping | 0.093 | 0.093 | 0.093 | 1.00 | 0.003 |
| clustering | 0.073 | 0.073 | 0.069 | 1.00 | 0.019 |
| surface_trail | 0.073 | 0.073 | 0.213 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.056 | 0.450 | 0.720 | 0.12 | 0.001 |
| render_sky | 0.055 | 0.055 | 0.054 | 1.00 | 0.001 |
| clouds_resolve | 0.049 | 0.049 | 0.050 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.049 | 0.049 | 0.049 | 1.00 | 0.004 |
| water_snapshot | 0.038 | 0.038 | 0.043 | 1.00 | 0.001 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.029 | 0.230 | 0.237 | 0.12 | 0.003 |
| atmos_froxel_integrate | 0.028 | 0.028 | 0.030 | 1.00 | 0.001 |
| water_mips | 0.025 | 0.025 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 1.00 | 0.006 |
| main_transparent_pass_3d | 0.016 | 0.016 | 0.030 | 1.00 | 0.107 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 1.00 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 1.00 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 1.00 | 0.001 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.013 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.001 | 0.03 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| encode//sum | 11.995 | 18.431 | 72.27 | 38047.0 |
| render/thread/ms | 10.882 | 13.898 | 41.02 | 34518.4 |
| render/busy/ms | 10.290 | 13.258 | 40.47 | 32640.3 |
| main/busy/ms | 8.279 | 10.112 | 31.43 | 26261.6 |
| render/graph/ms | 6.093 | 8.052 | 36.95 | 19326.2 |
| stage/render/graph_submit | 4.911 | 6.511 | 35.52 | 15577.0 |
| stage/main/post_update | 4.687 | 5.889 | 10.44 | 14866.8 |
| stage/render/render | 4.073 | 8.413 | 37.66 | 12919.5 |
| stage/render/cleanup | 2.828 | 8.341 | 16.65 | 8970.2 |
| encode/finish_wall | 2.762 | 3.601 | 32.26 | 8760.1 |
| encode/submit | 2.187 | 3.158 | 23.13 | 6936.5 |
| stage/main/update | 2.100 | 2.739 | 23.84 | 6660.3 |
| stage/render/camera_driver | 1.112 | 1.562 | 4.07 | 3528.3 |
| stage/render/prepare_write_phase_buffers | 0.946 | 1.265 | 2.60 | 2999.4 |
| stage/render/prepare_assets | 0.661 | 0.897 | 11.75 | 2095.8 |
| render/present/ms | 0.494 | 0.632 | 1.17 | 1566.9 |
| stage/render/prepare_resources | 0.448 | 0.564 | 2.93 | 1421.8 |
| stage/main/first | 0.413 | 0.542 | 3.54 | 1310.4 |
| bigspace/total/ms | 0.398 | 0.500 | 1.37 | 1263.0 |
| stage/render/prepare_meshes | 0.391 | 0.517 | 2.23 | 1240.3 |
| stage/main/fixed_main_loop | 0.390 | 0.627 | 1.65 | 1236.4 |
| stage/main/pre_update | 0.365 | 0.494 | 1.38 | 1158.3 |
| stage/render/queue | 0.338 | 0.540 | 2.96 | 1073.0 |
| stage/render/specialize | 0.335 | 0.493 | 2.88 | 1063.0 |
| terrain/update/ms | 0.312 | 0.388 | 1.54 | 989.0 |
| terrain/select/ms | 0.278 | 0.341 | 0.60 | 880.9 |
| stage/render/prepare_bind_groups | 0.275 | 0.497 | 2.40 | 871.4 |
| controller/feed/ms | 0.269 | 0.366 | 0.60 | 852.5 |
| stream/drive/ms | 0.248 | 0.398 | 21.86 | 785.3 |
| pipelines/compile_ms | 0.198 | 0.000 | 156.95 | 626.8 |
| pipelines/front_ms | 0.191 | 0.000 | 151.13 | 604.9 |
| stage/render/create_views | 0.190 | 0.258 | 1.34 | 601.6 |
| anim/evaluate_cpu/ms | 0.181 | 0.232 | 0.49 | 574.8 |
| stage/render/prepare_batch_phases | 0.178 | 0.264 | 1.22 | 565.8 |
| stream/tick/ms | 0.176 | 0.223 | 0.77 | 556.8 |
| far/select/ms | 0.174 | 0.224 | 0.57 | 552.2 |
| stage/render/extract_commands | 0.162 | 0.216 | 0.63 | 514.4 |
| stage/render/prepare_flush | 0.158 | 0.243 | 1.00 | 502.6 |
| bigspace/low_precision/ms | 0.151 | 0.208 | 0.54 | 479.1 |
| controller/move/ms | 0.139 | 0.332 | 0.80 | 442.1 |
| stage/render/prepare_views | 0.138 | 0.196 | 0.48 | 436.6 |
| stage/main/last | 0.123 | 0.236 | 1.56 | 391.1 |
| anim/evaluate/ms | 0.099 | 0.146 | 0.42 | 314.3 |
| render/acquire/ms | 0.098 | 0.138 | 0.43 | 311.2 |
| bigspace/high_precision/ms | 0.095 | 0.123 | 0.39 | 300.2 |
| physics/step/ms | 0.082 | 0.122 | 0.48 | 259.7 |
| physics/insert/ms | 0.063 | 0.429 | 0.94 | 200.8 |
| stage/render/prepare_collect_phase_buffers | 0.051 | 0.110 | 0.39 | 162.7 |
| encode/resolve_diagnostics/sum | 0.035 | 0.047 | 0.78 | 109.8 |
| stage/render/phase_sort | 0.032 | 0.057 | 0.23 | 101.4 |
| weather/map/ms | 0.025 | 0.034 | 0.29 | 79.7 |
| encode/camera_driver_span_begin/sum | 0.014 | 0.023 | 0.07 | 45.9 |
| anim/apply/ms | 0.012 | 0.022 | 0.12 | 38.4 |
| controller/avoid/ms | 0.011 | 0.014 | 0.16 | 35.0 |
| encode/camera_driver_span_end/sum | 0.011 | 0.020 | 0.64 | 34.0 |
| weather/eval/ms | 0.011 | 0.014 | 1.56 | 33.8 |
| stream/despawn/ms | 0.008 | 0.000 | 0.97 | 25.3 |
| pipelines/wait_ms | 0.008 | 0.000 | 16.12 | 24.6 |
| stream/spawn/ms | 0.007 | 0.000 | 1.08 | 23.1 |
| script/update/ms | 0.007 | 0.011 | 0.11 | 22.5 |
| pipelines/driver_ms | 0.007 | 0.000 | 5.83 | 21.9 |
| script/input/ms | 0.005 | 0.006 | 0.08 | 15.4 |
| script/ui/ms | 0.004 | 0.007 | 0.03 | 13.5 |
| stream/deactivate/ms | 0.004 | 0.000 | 0.57 | 12.7 |
| controller/write/ms | 0.003 | 0.004 | 0.11 | 10.8 |
| stream/activate/ms | 0.002 | 0.000 | 0.39 | 4.9 |
| weather/fx/ms | 0.001 | 0.001 | 0.04 | 3.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.05 | 2.0 |
| stream/upload/ms | 0.000 | 0.000 | 0.05 | 1.0 |
| stage/render/prepare | 0.000 | 0.002 | 0.01 | 1.0 |
| weather/apply/ms | 0.000 | 0.000 | 0.00 | 0.8 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.3 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 1 | 4 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 567 | 579 | 578 |
| anim/characters/count | count | 569 | 579 | 578 |
| anim/evaluated/count | count | 19 | 29 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 43 | 14 |
| crowd/c0/joints/count | count | 1252 | 2262 | 675 |
| crowd/c0/meshes/count | count | 349 | 646 | 175 |
| crowd/c1/count | count | 146 | 158 | 147 |
| crowd/c1/joints/count | count | 5566 | 6092 | 5521 |
| crowd/c1/meshes/count | count | 1178 | 1338 | 1166 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1822 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 187 | 236 | 223 |
| crowd/joints_extracted/count | count | 4288 | 9879 | 6179 |
| crowd/joints_uploaded/count | count | 4245 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.2 k | 59.3 k |
| encode//max | ms | 2.27 | 31.89 | 1.76 |
| encode/camera_driver_span_begin/max | ms | 0.01 | 0.07 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.64 | 0.01 |
| encode/finish_critical | ms | 2.27 | 31.89 | 1.76 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.78 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 10.00 | 40.47 | 8.57 |
| gpu/timed/ms | ms | 8.52 | 13.86 | 8.30 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2526 | 2583 | 2583 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 545.28 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 805 | 1417 | 618 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 196 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 805 | 1417 | 618 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 19 | 21 | 20 |
| phases/opaque3d/bins | count | 1294 | 1420 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3396 | 5140 | 3191 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 18 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1294 | 1420 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3396 | 5140 | 3191 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 40 | 226 | 29 |
| phases/shadow/bins | count | 4813 | 5909 | 2275 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 12.9 k | 5730 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 204 | 256 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2643 | 3268 | 2349 |
| physics/items | count | 207 | 476 | 19 |
| physics/pending | count | 0 | 209 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 179 | 181 | 181 |
| pipelines/waiting | count | 0 | 2 | 0 |
| preprocess/bind_groups | count | 15 | 611 | 11 |
| preprocess/bind_groups_created | count | 15 | 611 | 11 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 178 | 292 | 170 |
| process/cpu_usage | % | 15.62 | 31.25 | 13.46 |
| process/mem_usage | GiB | 5.00 | 5.17 | 5.17 |
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
| stream/latency/s | s | 0.06 | 0.16 | 0.05 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.45 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 3 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 26.99 | 73.88 | 20.98 |
| system/mem_usage | % | 62.46 | 63.52 | 63.26 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2823 | 2938 | 2938 |
| textures/images | count | 2592 | 2650 | 2650 |
| views/all/batch_sets | count | 88 | 273 | 79 |
| views/all/bins | count | 7391 | 8863 | 5773 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.6 k | 25.5 k | 13.3 k |
| views/all/pipelines | count | 249 | 303 | 75 |
| views/all/pipelines_distinct | count | 69 | 74 | 61 |
| views/all/work_items | count | 17.4 k | 23.5 k | 13.7 k |
| views/cascade/batch_sets | count | 35 | 47 | 29 |
| views/cascade/bins | count | 2607 | 3151 | 2275 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8146 | 10.0 k | 5730 |
| views/cascade/pipelines | count | 35 | 47 | 29 |
| views/cascade/work_items | count | 8146 | 10.0 k | 5730 |
| views/lamp_face/batch_sets | count | 4 | 190 | 0 |
| views/lamp_face/bins | count | 2152 | 2827 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2405 | 3148 | 0 |
| views/lamp_face/pipelines | count | 168 | 211 | 0 |
| views/lamp_face/work_items | count | 73 | 2846 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2888 | 3634 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8398 | 13.0 k | 7618 |
| views/main/pipelines | count | 46 | 49 | 46 |
| views/main/work_items | count | 8567 | 13.2 k | 7926 |
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
| anim/bones_written/count | count | 61.12 | 288 | 193.9 k |
| anim/events/count | count | 0.17 | 3 | 548 |
| assets/Image/added | count | 0.19 | 43 | 594 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 1.28 | 152 | 4075 |
| assets/Mesh/removed | count | 1.54 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.15 | 35 | 466 |
| assets/StandardMaterial/modified | count | 6.81 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.13 | 52 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.03 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 1.47 | 1566 | 4675 |
| encode//count | count | 42.78 | 272 | 135.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 3172 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 3172 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 3172 |
| physics/inserts | count | 0.74 | 111 | 2332 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.74 | 145 | 2335 |
| physics/steps | count | 1.24 | 3 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 10 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.07 | 21 | 219 |
| stream/deactivate/items | count | 0.07 | 21 | 223 |
| stream/despawn/items | count | 0.07 | 6 | 222 |
| stream/fetch/items | count | 0.03 | 1 | 83 |
| stream/spawn/items | count | 0.12 | 17 | 378 |
| stream/upload/bytes | bytes | 10.7 k | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.02 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.02 | 1 | 53 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 2340 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 51.2 ms; slowest 16, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 3.15 | 3151.7 | main/busy/ms 2538, stage/main/post_update 2435, render/thread/ms 552, render/busy/ms 502, stage/render/prepare_assets 321, stage/render/prepare_resources 135, stage/main/update 97.41, surface/advance_cpu_ms 70.33, stage/render/prepare_views 47.38, render/acquire/ms 47.19, stage/render/render 36.91, render/graph/ms 33.26 |
| warm-up | 3.28 | 126.1 | main/busy/ms 2570, stage/main/post_update 2457, render/thread/ms 559, render/busy/ms 509, stage/render/prepare_assets 322, pipelines/wait_ms 154, pipelines/compile_ms 142, stage/render/prepare_resources 136, pipelines/front_ms 135, stage/main/update 102, surface/advance_cpu_ms 70.33, stage/render/prepare_views 47.61 |
| warm-up | 3.48 | 89.1 | pipelines/compile_ms 197, pipelines/front_ms 186, main/busy/ms 53.35, render/thread/ms 48.05, render/busy/ms 46.20, stage/main/update 44.47, stage/render/prepare_assets 38.26, pipelines/driver_ms 11.16, stage/render/render 6.19, stage/main/post_update 5.36, render/graph/ms 3.85, stage/render/graph_submit 2.18 |
| warm-up | 3.72 | 62.4 | pipelines/wait_ms 2503, pipelines/compile_ms 217, pipelines/front_ms 202, render/thread/ms 37.73, render/busy/ms 36.67, main/busy/ms 32.71, stage/render/prepare_assets 28.28, stage/main/update 23.08, stream/drive/ms 19.50, pipelines/driver_ms 15.76, stage/main/post_update 6.71, render/graph/ms 3.61 |
| warm-up | 3.84 | 68.0 | pipelines/wait_ms 1243, pipelines/compile_ms 135, pipelines/front_ms 124, main/busy/ms 38.50, render/thread/ms 37.33, render/busy/ms 36.11, stage/render/prepare_assets 29.97, stage/main/update 28.86, stream/drive/ms 25.74, pipelines/driver_ms 10.51, stage/main/post_update 6.69, render/graph/ms 2.36 |
| warm-up | 4.07 | 81.8 | pipelines/wait_ms 7773, pipelines/compile_ms 165, pipelines/front_ms 152, main/busy/ms 39.51, render/thread/ms 37.23, render/busy/ms 35.56, stage/main/update 25.81, stage/render/prepare_assets 22.58, stream/drive/ms 21.63, pipelines/driver_ms 13.07, stage/main/post_update 8.52, render/graph/ms 4.22 |
| warm-up | 4.13 | 57.8 | pipelines/wait_ms 8399, pipelines/compile_ms 109, pipelines/front_ms 99.55, render/thread/ms 53.47, render/busy/ms 52.01, main/busy/ms 44.17, stage/render/prepare_assets 36.39, stage/main/update 26.06, stream/drive/ms 21.69, stage/main/post_update 11.55, pipelines/driver_ms 9.35, render/graph/ms 5.55 |
| warm-up | 4.22 | 91.7 | pipelines/wait_ms 4544, pipelines/compile_ms 171, pipelines/front_ms 165, main/busy/ms 71.86, render/thread/ms 48.88, render/busy/ms 47.44, stage/main/update 47.44, stream/drive/ms 43.58, stage/render/prepare_assets 27.02, stage/main/post_update 17.54, encode//sum 13.03, stage/render/render 8.02 |
| warm-up | 4.28 | 54.1 | pipelines/wait_ms 14.2 k, pipelines/compile_ms 199, pipelines/front_ms 189, main/busy/ms 70.05, render/thread/ms 51.33, render/busy/ms 49.40, stage/main/update 46.96, stream/drive/ms 43.38, stage/render/prepare_assets 21.34, encode//sum 20.32, stage/main/post_update 15.42, stage/render/render 10.94 |
| warm-up | 4.43 | 155.0 | pipelines/wait_ms 19.1 k, pipelines/compile_ms 180, pipelines/front_ms 165, main/busy/ms 88.83, render/thread/ms 75.57, render/busy/ms 73.61, stage/main/update 71.02, stream/drive/ms 46.64, stage/render/prepare_assets 37.13, encode//sum 26.29, pipelines/driver_ms 14.68, stage/render/render 13.70 |
| warm-up | 4.51 | 79.3 | pipelines/wait_ms 7166, pipelines/compile_ms 117, pipelines/front_ms 108, main/busy/ms 95.69, render/thread/ms 90.55, stage/main/update 80.13, render/busy/ms 75.93, stream/drive/ms 47.16, stage/render/prepare_assets 45.73, stage/render/render 24.05, encode//sum 18.36, render/present/ms 14.38 |
| warm-up | 4.61 | 95.7 | pipelines/wait_ms 4100, pipelines/compile_ms 229, pipelines/front_ms 216, main/busy/ms 77.75, render/thread/ms 68.95, stage/main/update 59.65, render/busy/ms 54.45, stream/drive/ms 46.66, stage/render/prepare_assets 37.08, stage/render/render 20.49, render/present/ms 14.26, pipelines/driver_ms 13.31 |
| warm-up | 6.30 | 75.0 | main/busy/ms 49.41, render/thread/ms 40.32, render/busy/ms 39.02, stage/main/update 30.77, stage/render/prepare_assets 17.37, stage/main/post_update 13.66, encode//sum 13.59, stage/render/render 11.21, render/graph/ms 9.47, stage/render/graph_submit 7.34, encode/submit 3.84, stream/drive/ms 3.71 |
| warm-up | 6.90 | 64.3 | main/busy/ms 42.70, render/thread/ms 38.34, render/busy/ms 37.10, stage/main/update 28.69, stage/render/prepare_assets 18.79, encode//sum 14.07, stage/render/render 10.37, stage/main/post_update 9.09, render/graph/ms 8.48, stage/render/graph_submit 6.59, encode/finish_wall 3.71, encode/submit 2.90 |
| warm-up | 7.02 | 52.2 | main/busy/ms 45.99, render/thread/ms 38.36, render/busy/ms 37.21, stage/main/update 29.62, stage/render/prepare_assets 18.15, encode//sum 15.02, stage/main/post_update 11.71, stage/render/render 10.17, render/graph/ms 8.47, stage/render/graph_submit 6.62, encode/finish_wall 3.74, encode/submit 2.94 |
| measure | 0.68 | 51.6 | encode//sum 87.14, render/thread/ms 51.52, render/busy/ms 50.41, stage/render/render 44.59, render/graph/ms 43.13, stage/render/graph_submit 40.20, encode/finish_wall 34.94, main/busy/ms 18.06, stage/main/post_update 9.85, encode/submit 5.30, stage/main/update 5.15, stage/render/camera_driver 2.78 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\nopipe\balmora_walk@2560x1440-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_NO_PIPELINED=1
- before the run: GPU 0%, CPU 18%
- during the run: other processes 11% CPU (system minus this process)
- nvidia-smi over the window (252 samples): GPU load **52%** (min 36%) · 2580 MHz (min 2580) · memory 8001 MHz · 56.7 W · 68 °C (max 73) · VRAM 7478 MiB · throttled 100% of samples
