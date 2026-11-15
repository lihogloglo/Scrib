# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: flatnormal · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 4753 frames over 65.6 s · **median 13.41 ms (75 fps)** · mean 13.81 · p95 17.54 · p99 21.44 (1% low 47 fps) · worst 35.98 ms · 66 spikes over 1.5× median · 53.0 stutters/min (work 53.9/min)
- warm-up: 336 frames over 9.2 s, worst 2943.9 ms
- hitches (> 33.5 ms): **3** measured, 19 in warm-up
- timed GPU passes: 8.10 ms of a 13.81 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- **FAIL** Median frame 13.41 ms, limit 11 ms
- **FAIL** frame_ms: 58 stutters (> 1.5x moving median of 60) in 1.09 min = 53.01/min, limit 1/min
- **FAIL** worst frame 35.98 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 2.275 | 2.295 | 2.872 | 0.99 | 0.190 |
| main_opaque_pass_3d | 1.821 | 1.836 | 2.653 | 0.99 | 0.076 |
| early prepass | 0.554 | 0.558 | 0.940 | 0.99 | 0.092 |
| bloom | 0.352 | 0.355 | 0.596 | 0.99 | 0.072 |
| atmosphere_luts | 0.341 | 0.344 | 0.606 | 0.99 | 0.004 |
| water_velocity | 0.320 | 0.322 | 0.617 | 0.99 | 0.002 |
| water_sspr | 0.235 | 0.236 | 0.463 | 0.99 | 0.003 |
| ssao | 0.210 | 0.211 | 0.435 | 0.99 | 0.009 |
| uniform_allocation | 0.197 | 0.199 | 0.201 | 0.99 | 0.041 |
| auxiliary_views | 0.192 | 0.193 | 0.763 | 0.99 | 0.188 |
| atmos_froxel_inject | 0.164 | 0.165 | 0.332 | 0.99 | 0.002 |
| bin_unpacking | 0.157 | 0.158 | 0.368 | 0.99 | 0.029 |
| water_fft | 0.140 | 0.141 | 0.140 | 0.99 | 0.003 |
| clouds_trace | 0.140 | 0.141 | 0.234 | 0.99 | 0.001 |
| upscaling | 0.135 | 0.136 | 0.174 | 0.99 | 0.005 |
| clouds_composite | 0.114 | 0.114 | 0.111 | 0.99 | 0.002 |
| main_transmissive_pass_3d | 0.096 | 0.097 | 0.184 | 0.99 | 0.010 |
| tonemapping | 0.095 | 0.096 | 0.094 | 0.99 | 0.004 |
| clustering | 0.071 | 0.071 | 0.070 | 0.99 | 0.022 |
| lightprobe_irradiance_map | 0.056 | 0.468 | 0.760 | 0.12 | 0.001 |
| render_sky | 0.055 | 0.055 | 0.054 | 0.99 | 0.002 |
| surface_trail | 0.053 | 0.063 | 0.059 | 0.85 | 0.002 |
| clouds_resolve | 0.049 | 0.050 | 0.051 | 0.99 | 0.001 |
| early_mesh_preprocessing | 0.048 | 0.048 | 0.049 | 0.99 | 0.007 |
| water_snapshot | 0.038 | 0.039 | 0.041 | 0.99 | 0.002 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 0.99 | 0.004 |
| atmos_froxel_integrate | 0.029 | 0.029 | 0.031 | 0.99 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.233 | 0.238 | 0.12 | 0.005 |
| water_mips | 0.026 | 0.026 | 0.024 | 0.99 | 0.003 |
| per_view_shadow_pass | 0.022 | 0.022 | 0.023 | 0.99 | 0.008 |
| main_transparent_pass_3d | 0.015 | 0.016 | 0.031 | 0.99 | 0.122 |
| atmosphere_environment | 0.011 | 0.011 | 0.011 | 0.99 | 0.003 |
| clouds_fog_light | 0.010 | 0.010 | 0.011 | 0.99 | 0.002 |
| sparse buffer updates | 0.008 | 0.008 | 0.008 | 0.99 | 0.003 |
| night_sky | 0.006 | 0.006 | 0.013 | 0.99 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.015 | 0.12 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.013 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.011 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |
| weather_map | — | — | — | - | — |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| pipelines/wait_ms | 32.488 | 0.000 | 12785.22 | 154416.0 |
| encode//sum | 12.513 | 18.312 | 57.08 | 59476.1 |
| render/thread/ms | 12.292 | 15.755 | 34.25 | 58424.3 |
| render/busy/ms | 11.669 | 15.026 | 33.55 | 55462.6 |
| main/busy/ms | 10.372 | 13.256 | 30.03 | 49296.2 |
| render/graph/ms | 7.119 | 9.415 | 17.86 | 33834.6 |
| stage/main/post_update | 6.492 | 8.799 | 14.74 | 30856.8 |
| stage/render/graph_submit | 5.745 | 7.628 | 13.40 | 27306.3 |
| stage/render/render | 4.819 | 9.818 | 18.86 | 22907.1 |
| encode/finish_wall | 3.243 | 4.514 | 6.98 | 15411.7 |
| stage/render/cleanup | 3.143 | 9.512 | 18.95 | 14941.0 |
| encode/submit | 2.560 | 3.634 | 7.05 | 12169.9 |
| stage/main/update | 2.431 | 3.175 | 23.32 | 11556.8 |
| stage/render/camera_driver | 1.307 | 1.873 | 5.22 | 6214.5 |
| stage/render/prepare_write_phase_buffers | 1.086 | 1.501 | 3.27 | 5160.9 |
| stage/render/prepare_assets | 0.650 | 0.931 | 11.19 | 3090.2 |
| bigspace/total/ms | 0.555 | 1.254 | 5.15 | 2636.9 |
| pipelines/compile_ms | 0.521 | 0.000 | 174.92 | 2474.1 |
| render/present/ms | 0.516 | 0.705 | 4.56 | 2451.9 |
| pipelines/front_ms | 0.503 | 0.000 | 168.33 | 2391.5 |
| stage/render/prepare_resources | 0.500 | 0.669 | 3.42 | 2377.5 |
| stage/main/first | 0.439 | 0.595 | 5.30 | 2084.9 |
| stage/render/prepare_meshes | 0.437 | 0.618 | 2.74 | 2075.3 |
| stage/main/pre_update | 0.371 | 0.575 | 3.64 | 1764.0 |
| stage/render/specialize | 0.338 | 0.505 | 3.20 | 1606.7 |
| stage/render/queue | 0.333 | 0.531 | 2.92 | 1584.1 |
| terrain/update/ms | 0.318 | 0.422 | 2.04 | 1512.2 |
| stage/render/prepare_bind_groups | 0.308 | 0.573 | 3.30 | 1462.5 |
| controller/feed/ms | 0.307 | 0.433 | 0.92 | 1460.6 |
| stage/main/fixed_main_loop | 0.303 | 0.493 | 3.57 | 1440.3 |
| terrain/select/ms | 0.282 | 0.374 | 1.69 | 1340.0 |
| stream/drive/ms | 0.276 | 0.406 | 21.33 | 1312.2 |
| bigspace/low_precision/ms | 0.224 | 0.532 | 4.31 | 1063.2 |
| far/select/ms | 0.221 | 0.335 | 1.00 | 1048.2 |
| stage/render/create_views | 0.212 | 0.294 | 8.02 | 1006.6 |
| stream/tick/ms | 0.208 | 0.306 | 0.76 | 989.3 |
| stage/render/prepare_batch_phases | 0.196 | 0.301 | 1.70 | 933.1 |
| anim/evaluate_cpu/ms | 0.181 | 0.241 | 0.73 | 861.1 |
| stage/render/prepare_flush | 0.179 | 0.355 | 0.97 | 849.3 |
| stage/render/extract_commands | 0.175 | 0.228 | 0.43 | 829.4 |
| stage/render/prepare_views | 0.157 | 0.228 | 2.93 | 744.2 |
| bigspace/high_precision/ms | 0.154 | 0.361 | 4.33 | 729.9 |
| anim/evaluate/ms | 0.146 | 0.277 | 3.53 | 694.6 |
| stage/main/last | 0.131 | 0.238 | 1.83 | 622.7 |
| controller/move/ms | 0.131 | 0.250 | 0.72 | 620.5 |
| render/acquire/ms | 0.107 | 0.159 | 2.20 | 509.7 |
| physics/insert/ms | 0.080 | 0.514 | 1.23 | 381.1 |
| stage/render/prepare_collect_phase_buffers | 0.068 | 0.150 | 0.48 | 324.1 |
| physics/step/ms | 0.064 | 0.105 | 1.56 | 305.6 |
| encode/resolve_diagnostics/sum | 0.038 | 0.053 | 0.95 | 181.5 |
| stage/render/phase_sort | 0.032 | 0.057 | 0.30 | 152.7 |
| weather/map/ms | 0.027 | 0.036 | 0.30 | 127.2 |
| pipelines/driver_ms | 0.017 | 0.000 | 6.60 | 82.6 |
| encode/camera_driver_span_begin/sum | 0.017 | 0.026 | 0.28 | 82.2 |
| encode/camera_driver_span_end/sum | 0.013 | 0.022 | 0.28 | 60.5 |
| anim/apply/ms | 0.013 | 0.022 | 0.19 | 59.7 |
| controller/avoid/ms | 0.012 | 0.015 | 0.26 | 55.1 |
| weather/eval/ms | 0.011 | 0.015 | 0.13 | 52.0 |
| script/update/ms | 0.008 | 0.012 | 0.17 | 40.3 |
| stream/despawn/ms | 0.007 | 0.000 | 1.19 | 31.7 |
| script/input/ms | 0.006 | 0.008 | 0.14 | 29.1 |
| stream/spawn/ms | 0.006 | 0.000 | 1.38 | 27.1 |
| script/ui/ms | 0.006 | 0.008 | 0.09 | 26.9 |
| controller/write/ms | 0.004 | 0.005 | 0.28 | 17.5 |
| stream/deactivate/ms | 0.003 | 0.000 | 0.68 | 15.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.55 | 5.7 |
| weather/fx/ms | 0.001 | 0.002 | 0.02 | 4.7 |
| shapes/activate/ms | 0.001 | 0.001 | 0.02 | 3.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.23 | 3.0 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 1.3 |
| stream/upload/ms | 0.000 | 0.000 | 0.04 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 1 | 0 |
| anim/a1/count | count | 0 | 18 | 0 |
| anim/a2/count | count | 1 | 11 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 19 | 63 | 19 |
| controller/movers/count | count | 20 | 22 | 19 |
| crowd/c0/count | count | 27 | 49 | 14 |
| crowd/c0/joints/count | count | 1306 | 2262 | 641 |
| crowd/c0/meshes/count | count | 353 | 646 | 174 |
| crowd/c1/count | count | 146 | 156 | 147 |
| crowd/c1/joints/count | count | 5507 | 5992 | 5555 |
| crowd/c1/meshes/count | count | 1163 | 1309 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 186 | 236 | 223 |
| crowd/joints_extracted/count | count | 4262 | 7274 | 6179 |
| crowd/joints_uploaded/count | count | 4242 | 6643 | 6179 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 2.64 | 6.46 | 2.59 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.28 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.28 | 0.03 |
| encode/finish_critical | ms | 2.64 | 6.46 | 2.59 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.95 | 0.04 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 205.19 M | 205.19 M | 205.19 M |
| far/triangles | count | 5.06 M | 5.06 M | 5.06 M |
| frame/work/ms | ms | 11.45 | 33.55 | 10.35 |
| gpu/timed/ms | ms | 8.83 | 15.04 | 9.18 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2525 | 2582 | 2582 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 276.1 k | 276.1 k | 276.1 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 14.0 k | 15.2 k | 14.2 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 545.28 M | 552.74 M | 552.74 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 191 | 404 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 838 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 191 | 404 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 838 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 22 | 24 | 23 |
| phases/opaque3d/bins | count | 1291 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3480 | 5139 | 3188 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 21 | 23 | 22 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 19 | 20 | 19 |
| phases/opaque3d_prepass/bins | count | 1291 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3480 | 5139 | 3188 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 18 | 19 | 18 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 39 | 231 | 28 |
| phases/shadow/bins | count | 4830 | 5835 | 2221 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 10.6 k | 12.9 k | 5672 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 207 | 253 | 28 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2348 |
| physics/items | count | 179 | 476 | 19 |
| physics/pending | count | 0 | 1854 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 188 | 190 | 190 |
| pipelines/waiting | count | 0 | 32 | 0 |
| preprocess/bind_groups | count | 12 | 633 | 6 |
| preprocess/bind_groups_created | count | 12 | 633 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 192 | 302 | 185 |
| process/cpu_usage | % | 24.88 | 47.34 | 22.12 |
| process/mem_usage | GiB | 5.13 | 5.30 | 5.30 |
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
| started_load_count |  | 4764 | 4858 | 4858 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 4 | 5 | 4 |
| statics/material_pipelines | count | 4 | 5 | 4 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 2 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 1 | 0 |
| stream/latency/s | s | 0.04 | 0.10 | 0.04 |
| stream/model_queue | count | 0 | 44 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.32 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/requests | count | 0 | 2 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 38.24 | 93.45 | 32.16 |
| system/mem_usage | % | 69.13 | 71.72 | 71.51 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2822 | 2934 | 2934 |
| textures/images | count | 2589 | 2644 | 2644 |
| views/all/batch_sets | count | 92 | 284 | 84 |
| views/all/bins | count | 7428 | 8662 | 5719 |
| views/all/count | count | 46 | 54 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 19.7 k | 25.6 k | 13.3 k |
| views/all/pipelines | count | 258 | 300 | 80 |
| views/all/pipelines_distinct | count | 75 | 79 | 67 |
| views/all/work_items | count | 17.6 k | 23.2 k | 13.6 k |
| views/cascade/batch_sets | count | 35 | 47 | 28 |
| views/cascade/bins | count | 2608 | 3156 | 2221 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8164 | 9930 | 5672 |
| views/cascade/pipelines | count | 35 | 47 | 28 |
| views/cascade/work_items | count | 8164 | 9930 | 5672 |
| views/lamp_face/batch_sets | count | 0 | 196 | 0 |
| views/lamp_face/bins | count | 2177 | 2881 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2431 | 3175 | 0 |
| views/lamp_face/pipelines | count | 172 | 209 | 0 |
| views/lamp_face/work_items | count | 0 | 2882 | 0 |
| views/main/batch_sets | count | 55 | 58 | 56 |
| views/main/bins | count | 2874 | 3580 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8622 | 13.0 k | 7610 |
| views/main/pipelines | count | 51 | 54 | 52 |
| views/main/work_items | count | 8774 | 13.2 k | 7918 |
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
| anim/bones_written/count | count | 54.32 | 704 | 258.2 k |
| anim/events/count | count | 0.11 | 3 | 529 |
| assets/Image/added | count | 0.13 | 43 | 639 |
| assets/Image/modified | count | 0.00 | 1 | 15 |
| assets/Image/removed | count | 0.01 | 6 | 46 |
| assets/Mesh/added | count | 0.89 | 168 | 4232 |
| assets/Mesh/removed | count | 1.03 | 1224 | 4878 |
| assets/StandardMaterial/added | count | 0.11 | 35 | 513 |
| assets/StandardMaterial/modified | count | 4.56 | 11 | 21.7 k |
| assets/StandardMaterial/removed | count | 0.09 | 32 | 408 |
| assets/WorldAsset/added | count | 0.01 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.02 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 2.72 | 3584 | 13.0 k |
| encode//count | count | 40.72 | 274 | 193.5 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 4753 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 4753 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 4753 |
| physics/inserts | count | 0.85 | 167 | 4038 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.49 | 129 | 2335 |
| physics/steps | count | 0.83 | 2 | 3939 |
| physics/steps_dropped | count | 0 | 0 | 0 |
| pipelines/after_reveal | count | 0.00 | 1 | 1 |
| pipelines/created | count | 0.00 | 2 | 8 |
| shapes/activations | count | 0 | 0 | 0 |
| shapes/impacts | count | 0 | 0 | 0 |
| shapes/removals | count | 0 | 0 | 0 |
| stream/activate/items | count | 0.05 | 21 | 219 |
| stream/deactivate/items | count | 0.05 | 21 | 223 |
| stream/despawn/items | count | 0.05 | 6 | 222 |
| stream/fetch/items | count | 0.02 | 1 | 83 |
| stream/spawn/items | count | 0.08 | 26 | 378 |
| stream/upload/bytes | bytes | 7122 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 1 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_redraws | count | 0.01 | 1 | 59 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1562 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 33.5 ms; slowest 22, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.94 | 2943.9 | main/busy/ms 2917, stage/main/post_update 2801, stage/main/update 110, surface/advance_cpu_ms 78.00, stage/main/pre_update 2.49, stage/main/last 1.38, surface/cell_upload_ms 0.81, stage/main/first 0.48, stage/main/fixed_main_loop 0.33, stream/drive/ms 0.32, weather/map/ms 0.14, stream/tick/ms 0.09 |
| warm-up | 3.64 | 699.1 | main/busy/ms 2954, stage/main/post_update 2828, render/thread/ms 657, render/busy/ms 533, stage/render/prepare_assets 455, stage/render/prepare_views 117, render/acquire/ms 117, stage/main/update 114, surface/advance_cpu_ms 78.00, stage/render/prepare_resources 49.44, stage/render/create_views 19.47, stage/render/render 13.36 |
| warm-up | 3.89 | 244.9 | pipelines/wait_ms 4215, render/thread/ms 668, render/busy/ms 542, stage/render/prepare_assets 455, pipelines/compile_ms 268, pipelines/front_ms 250, stage/render/prepare_views 118, render/acquire/ms 117, main/busy/ms 53.77, stage/render/prepare_resources 51.35, stage/main/post_update 34.54, terrain/update/ms 20.37 |
| warm-up | 4.14 | 92.6 | pipelines/compile_ms 254, pipelines/front_ms 239, main/busy/ms 69.49, render/thread/ms 58.40, stage/main/update 57.71, render/busy/ms 56.15, stage/render/prepare_assets 46.19, pipelines/driver_ms 14.21, stage/main/post_update 8.41, render/graph/ms 4.45, stage/render/cleanup 3.71, stage/render/render 3.43 |
| warm-up | 4.31 | 55.4 | main/busy/ms 51.93, render/thread/ms 43.88, render/busy/ms 42.49, stage/main/update 42.31, stream/drive/ms 38.53, stage/render/prepare_assets 33.63, stage/main/post_update 5.57, render/graph/ms 3.39, stage/render/cleanup 2.79, stage/render/render 2.47, stage/render/graph_submit 2.19, encode/submit 1.96 |
| warm-up | 4.42 | 66.3 | pipelines/wait_ms 3527, pipelines/compile_ms 257, pipelines/front_ms 232, main/busy/ms 55.15, render/thread/ms 53.21, render/busy/ms 50.80, stage/render/prepare_assets 37.10, stage/main/update 31.19, stream/drive/ms 26.57, pipelines/driver_ms 24.49, stage/main/post_update 14.73, render/graph/ms 5.41 |
| warm-up | 4.48 | 64.1 | pipelines/wait_ms 5075, pipelines/compile_ms 339, pipelines/front_ms 312, render/thread/ms 89.33, render/busy/ms 85.67, stage/render/prepare_assets 62.97, main/busy/ms 43.24, pipelines/driver_ms 27.28, stage/main/update 20.16, stream/drive/ms 13.87, stage/main/post_update 11.56, stage/render/render 11.41 |
| warm-up | 4.53 | 51.2 | pipelines/wait_ms 1547, pipelines/compile_ms 82.23, pipelines/front_ms 79.44, render/thread/ms 78.66, render/busy/ms 74.98, main/busy/ms 54.55, stage/render/prepare_assets 49.35, stage/main/update 38.35, stream/drive/ms 32.97, render/graph/ms 8.24, stage/main/post_update 7.72, stage/render/render 6.80 |
| warm-up | 4.63 | 102.1 | pipelines/wait_ms 4579, pipelines/compile_ms 215, pipelines/front_ms 148, render/thread/ms 106, render/busy/ms 104, pipelines/driver_ms 66.62, main/busy/ms 51.75, stage/render/prepare_assets 49.48, stage/main/update 37.57, stage/render/create_views 34.44, stream/drive/ms 32.55, encode//sum 14.26 |
| warm-up | 4.77 | 33.6 | main/busy/ms 73.69, stage/main/update 59.01, stream/drive/ms 54.64, render/thread/ms 36.11, render/busy/ms 33.95, stage/render/prepare_assets 16.46, stage/main/post_update 8.88, stage/render/render 8.66, render/graph/ms 5.33, encode//sum 3.93, stage/render/graph_submit 2.75, stage/main/pre_update 2.71 |
| warm-up | 4.85 | 88.5 | pipelines/wait_ms 936, main/busy/ms 72.78, stage/main/update 59.22, stream/drive/ms 54.72, render/thread/ms 50.00, render/busy/ms 47.98, stage/render/prepare_assets 33.03, stage/main/post_update 8.72, stage/render/render 8.26, render/graph/ms 5.31, encode//sum 3.40, stage/render/graph_submit 2.94 |
| warm-up | 4.91 | 52.7 | pipelines/wait_ms 2414, pipelines/compile_ms 212, pipelines/front_ms 196, render/thread/ms 74.91, main/busy/ms 65.70, stage/main/update 52.19, stream/drive/ms 47.95, render/busy/ms 41.28, stage/render/cleanup 34.71, render/present/ms 33.47, stage/render/prepare_assets 30.99, pipelines/driver_ms 15.16 |
| warm-up | 4.94 | 37.6 | pipelines/wait_ms 3444, pipelines/compile_ms 285, pipelines/front_ms 266, main/busy/ms 72.09, render/thread/ms 69.80, stage/main/update 57.80, stream/drive/ms 53.04, stage/render/cleanup 36.93, render/busy/ms 36.57, render/present/ms 33.06, stage/render/prepare_assets 27.32, pipelines/driver_ms 19.15 |
| warm-up | 5.01 | 47.4 | main/busy/ms 55.58, stage/main/update 37.95, render/thread/ms 37.44, render/busy/ms 35.96, stream/drive/ms 33.37, stage/main/post_update 12.99, stage/render/prepare_assets 12.91, stage/render/render 7.41, render/graph/ms 5.56, stage/render/specialize 4.55, encode//sum 4.37, stage/render/queue 3.99 |
| warm-up | 5.82 | 35.6 | pipelines/wait_ms 1499, pipelines/compile_ms 151, pipelines/front_ms 140, render/thread/ms 50.83, render/busy/ms 49.10, stage/render/prepare_assets 27.39, main/busy/ms 26.72, pipelines/driver_ms 11.08, stage/main/post_update 10.93, stage/main/update 9.30, stage/render/render 7.20, stage/render/specialize 5.65 |
| warm-up | 6.17 | 38.3 | pipelines/wait_ms 6362, main/busy/ms 59.22, stage/main/update 42.52, pipelines/compile_ms 38.77, pipelines/front_ms 34.96, render/thread/ms 29.69, render/busy/ms 27.94, stream/drive/ms 14.67, stage/render/render 12.94, encode//sum 12.36, stage/main/post_update 11.58, render/graph/ms 10.33 |
| warm-up | 6.22 | 50.9 | pipelines/wait_ms 13.3 k, pipelines/compile_ms 101, pipelines/front_ms 93.92, main/busy/ms 60.69, render/thread/ms 46.05, render/busy/ms 44.02, stage/main/update 42.06, stage/render/prepare_assets 17.72, encode//sum 13.41, stage/main/post_update 13.33, stage/render/render 13.19, render/graph/ms 10.24 |
| warm-up | 6.38 | 36.6 | pipelines/wait_ms 11.5 k, pipelines/compile_ms 112, pipelines/front_ms 105, main/busy/ms 53.55, render/thread/ms 43.91, render/busy/ms 42.33, stage/main/update 33.23, stage/render/prepare_assets 20.19, stage/main/post_update 13.76, encode//sum 10.42, stage/render/render 9.03, render/graph/ms 6.89 |
| warm-up | 6.77 | 40.2 | pipelines/wait_ms 5020, pipelines/compile_ms 55.86, pipelines/front_ms 51.22, render/thread/ms 45.53, main/busy/ms 44.09, render/busy/ms 43.39, stage/main/update 27.65, stage/render/prepare_assets 17.53, encode//sum 15.14, stage/render/render 14.76, render/graph/ms 11.78, stage/main/post_update 11.52 |
| measure | 2.73 | 36.0 | encode//sum 69.89, render/thread/ms 45.88, render/busy/ms 44.64, stage/render/cleanup 26.62, render/graph/ms 24.79, main/busy/ms 19.92, stage/render/graph_submit 18.87, stage/main/post_update 12.53, encode/finish_wall 9.91, encode/submit 9.10, stage/render/create_views 8.19, stage/render/camera_driver 5.81 |
| measure | 59.01 | 35.2 | render/thread/ms 45.14, render/busy/ms 43.05, encode//sum 37.73, main/busy/ms 29.52, stage/render/render 23.85, render/graph/ms 21.33, stage/main/post_update 17.69, stage/render/graph_submit 17.33, encode/finish_wall 9.78, encode/submit 7.71, stage/main/update 7.47, stage/render/prepare_assets 5.88 |
| measure | 59.08 | 34.7 | render/thread/ms 42.89, render/busy/ms 41.68, encode//sum 20.39, stage/render/prepare_assets 19.74, main/busy/ms 19.67, stage/render/render 14.92, render/graph/ms 13.42, stage/render/graph_submit 11.21, stage/main/post_update 10.03, encode/submit 5.83, encode/finish_wall 5.48, stage/main/update 5.04 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-b2.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --resolution 2560x1440 --bench-out target/perf\b2_off\balmora_walk@2560x1440_off=flatnormal-3 --bench-baseline none --off flatnormal
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 17%
- during the run: other processes 13% CPU (system minus this process)
- nvidia-smi over the window (251 samples): GPU load **76%** (min 55%) · 2580 MHz (min 2580) · memory 8001 MHz · 72.2 W · 73 °C (max 78) · VRAM 7073 MiB · throttled 100% of samples
