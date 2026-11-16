# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1407 frames over 15.0 s · **median 10.61 ms (94 fps)** · mean 10.66 · p95 11.65 · p99 12.22 (1% low 82 fps) · worst 15.00 ms · 0 spikes over 1.5× median · 0.0 stutters/min (work 0.0/min)
- warm-up: 281 frames over 8.4 s, worst 2514.8 ms
- hitches (> 26.5 ms): **0** measured, 49 in warm-up
- timed GPU passes: 9.48 ms of a 10.66 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.61 ms, limit 11 ms
- pass worst frame 15.00 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.852 | 2.858 | 2.903 | 1.00 | 0.064 |
| dlss_super_resolution | 2.047 | 2.051 | 2.076 | 1.00 | 0.186 |
| early prepass | 0.820 | 0.822 | 1.016 | 1.00 | 0.084 |
| per_view_shadow_pass_cascade1 | 0.679 | 0.680 | 0.944 | 1.00 | 0.049 |
| atmosphere_luts | 0.350 | 0.351 | 0.429 | 1.00 | 0.003 |
| bloom | 0.318 | 0.319 | 0.335 | 1.00 | 0.065 |
| main_transmissive_pass_3d | 0.278 | 0.279 | 0.296 | 1.00 | 0.008 |
| water_velocity | 0.243 | 0.243 | 0.268 | 1.00 | 0.002 |
| water_sspr | 0.214 | 0.215 | 0.216 | 1.00 | 0.002 |
| ssao | 0.195 | 0.196 | 0.203 | 1.00 | 0.007 |
| atmos_froxel_inject | 0.152 | 0.153 | 0.158 | 1.00 | 0.001 |
| clouds_trace | 0.150 | 0.150 | 0.174 | 1.00 | 0.001 |
| uniform_allocation | 0.148 | 0.148 | 0.153 | 1.00 | 0.027 |
| water_fft | 0.134 | 0.135 | 0.138 | 1.00 | 0.003 |
| upscaling | 0.115 | 0.115 | 0.123 | 1.00 | 0.004 |
| clouds_composite | 0.113 | 0.114 | 0.116 | 1.00 | 0.002 |
| bin_unpacking | 0.109 | 0.109 | 0.113 | 1.00 | 0.020 |
| tonemapping | 0.086 | 0.086 | 0.090 | 1.00 | 0.004 |
| clustering | 0.070 | 0.071 | 0.078 | 1.00 | 0.022 |
| render_sky | 0.053 | 0.053 | 0.053 | 1.00 | 0.002 |
| lightprobe_irradiance_map | 0.052 | 0.415 | 0.417 | 0.13 | 0.001 |
| clouds_resolve | 0.050 | 0.050 | 0.052 | 1.00 | 0.001 |
| early_mesh_preprocessing | 0.038 | 0.038 | 0.037 | 1.00 | 0.006 |
| clouds_shadow | 0.031 | 0.031 | 0.034 | 1.00 | 0.003 |
| lightprobe_radiance_map | 0.028 | 0.226 | 0.233 | 0.13 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.027 | 0.029 | 1.00 | 0.001 |
| main_transparent_pass_3d | 0.027 | 0.027 | 0.035 | 1.00 | 0.074 |
| water_snapshot | 0.025 | 0.025 | 0.026 | 1.00 | 0.002 |
| water_mips | 0.022 | 0.022 | 0.024 | 1.00 | 0.002 |
| per_view_shadow_pass | 0.016 | 0.016 | 0.024 | 1.00 | 0.005 |
| clouds_fog_light | 0.012 | 0.012 | 0.012 | 1.00 | 0.002 |
| atmosphere_environment | 0.010 | 0.010 | 0.013 | 1.00 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 1.00 | 0.003 |
| night_sky | 0.004 | 0.004 | 0.008 | 1.00 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.13 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 1.00 | 0.003 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.13 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.13 | 0.001 |
| per_view_shadow_pass_cascade3 | 0.000 | 0.040 | 0.040 | 0.00 | 0.010 |
| per_view_shadow_pass_cascade2 | 0.000 | 0.019 | 0.019 | 0.00 | 0.006 |
| blas_build | 0.000 | 0.002 | 0.003 | 0.00 | 0.005 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.743 | 10.768 | 14.12 | 13708.8 |
| encode//sum | 8.894 | 11.199 | 18.88 | 12513.2 |
| render/busy/ms | 7.747 | 8.674 | 11.62 | 10900.2 |
| main/busy/ms | 7.407 | 8.630 | 11.83 | 10421.6 |
| stage/render/render | 5.412 | 6.338 | 10.96 | 7614.0 |
| render/graph/ms | 4.708 | 5.564 | 8.65 | 6623.8 |
| stage/main/post_update | 4.587 | 5.605 | 8.64 | 6453.5 |
| stage/render/graph_submit | 3.691 | 4.532 | 7.72 | 5193.6 |
| encode/submit | 1.967 | 2.481 | 4.93 | 2767.3 |
| encode/finish_wall | 1.771 | 2.217 | 5.27 | 2491.8 |
| stage/main/update | 1.718 | 2.069 | 3.57 | 2416.7 |
| stage/render/prepare_views | 1.561 | 2.461 | 3.39 | 2195.9 |
| render/acquire/ms | 1.517 | 2.416 | 3.33 | 2134.2 |
| stage/render/camera_driver | 0.957 | 1.108 | 1.74 | 1346.8 |
| stage/render/prepare_write_phase_buffers | 0.704 | 0.851 | 1.19 | 990.6 |
| render/present/ms | 0.479 | 0.556 | 3.97 | 674.4 |
| stage/render/prepare_assets | 0.443 | 0.568 | 1.64 | 623.0 |
| stage/render/prepare_resources | 0.397 | 0.466 | 0.63 | 558.5 |
| bigspace/total/ms | 0.368 | 0.638 | 2.39 | 517.5 |
| stage/main/pre_update | 0.319 | 0.417 | 0.66 | 449.1 |
| stage/main/first | 0.295 | 0.358 | 1.48 | 415.6 |
| terrain/update/ms | 0.236 | 0.304 | 0.64 | 331.9 |
| terrain/select/ms | 0.206 | 0.264 | 0.60 | 289.3 |
| stage/render/prepare_bind_groups | 0.200 | 0.288 | 0.73 | 281.9 |
| stage/render/prepare_flush | 0.199 | 0.376 | 0.73 | 280.7 |
| stage/main/fixed_main_loop | 0.189 | 0.322 | 1.02 | 265.8 |
| stage/render/prepare_meshes | 0.188 | 0.238 | 0.38 | 264.8 |
| stream/drive/ms | 0.178 | 0.232 | 0.42 | 250.9 |
| stage/render/extract_commands | 0.176 | 0.210 | 0.41 | 247.4 |
| controller/feed/ms | 0.173 | 0.235 | 0.45 | 243.5 |
| stage/render/prepare_batch_phases | 0.164 | 0.223 | 0.43 | 230.8 |
| stream/tick/ms | 0.152 | 0.198 | 0.34 | 213.3 |
| far/select/ms | 0.151 | 0.198 | 0.38 | 212.0 |
| stage/render/create_views | 0.148 | 0.192 | 0.34 | 208.3 |
| bigspace/high_precision/ms | 0.132 | 0.321 | 2.15 | 185.6 |
| stage/main/last | 0.119 | 0.168 | 0.78 | 167.3 |
| controller/move/ms | 0.111 | 0.185 | 0.45 | 156.0 |
| anim/evaluate/ms | 0.108 | 0.244 | 1.61 | 151.4 |
| bigspace/low_precision/ms | 0.091 | 0.217 | 1.81 | 127.8 |
| stage/render/specialize | 0.075 | 0.098 | 0.47 | 105.5 |
| stage/render/queue | 0.063 | 0.086 | 0.36 | 89.0 |
| stage/render/phase_sort | 0.063 | 0.083 | 0.19 | 88.8 |
| anim/evaluate_cpu/ms | 0.060 | 0.080 | 0.36 | 84.9 |
| encode/resolve_diagnostics/sum | 0.055 | 0.088 | 0.26 | 77.2 |
| stage/render/prepare_collect_phase_buffers | 0.049 | 0.082 | 0.31 | 69.4 |
| physics/step/ms | 0.030 | 0.054 | 0.25 | 41.9 |
| encode/camera_driver_span_begin/sum | 0.027 | 0.042 | 0.22 | 38.3 |
| stage/render/cleanup | 0.025 | 0.036 | 0.25 | 35.0 |
| weather/map/ms | 0.025 | 0.032 | 0.06 | 34.5 |
| encode/camera_driver_span_end/sum | 0.022 | 0.038 | 0.13 | 31.3 |
| weather/eval/ms | 0.011 | 0.015 | 0.12 | 15.9 |
| script/update/ms | 0.007 | 0.008 | 0.03 | 9.3 |
| controller/avoid/ms | 0.005 | 0.007 | 0.02 | 7.7 |
| script/input/ms | 0.005 | 0.006 | 0.07 | 7.3 |
| anim/apply/ms | 0.005 | 0.006 | 0.09 | 6.5 |
| script/ui/ms | 0.005 | 0.006 | 0.11 | 6.4 |
| controller/write/ms | 0.002 | 0.002 | 0.10 | 2.6 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.06 | 0.9 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 0.8 |
| weather/apply/ms | 0.000 | 0.001 | 0.01 | 0.5 |
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
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 284 | 283 |
| crowd/c2/joints/count | count | 4916 | 4948 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1381 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 4212 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.47 | 5.13 | 1.44 |
| encode/camera_driver_span_begin/max | ms | 0.03 | 0.22 | 0.03 |
| encode/camera_driver_span_end/max | ms | 0.02 | 0.13 | 0.02 |
| encode/finish_critical | ms | 1.47 | 5.13 | 1.44 |
| encode/resolve_diagnostics/max | ms | 0.05 | 0.26 | 0.04 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 10.17 | 11.83 | 10.16 |
| gpu/timed/ms | ms | 10.17 | 11.51 | 10.16 |
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
| phases/shadow/batch_sets | count | 17 | 20 | 17 |
| phases/shadow/bins | count | 1798 | 2105 | 1798 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5089 | 5415 | 5089 |
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
| preprocess/bind_groups | count | 0 | 8 | 0 |
| preprocess/bind_groups_created | count | 0 | 8 | 0 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 160 | 160 | 160 |
| process/cpu_usage | % | 25 | 37.50 | 29.81 |
| process/mem_usage | GiB | 4.59 | 4.64 | 4.64 |
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
| stream/latency/s | s | 0.04 | 0.04 | 0.04 |
| stream/model_queue | count | 0 | 0 | 0 |
| stream/models | count | 233 | 233 | 233 |
| stream/oldest_request/s | s | 0 | 0 | 0 |
| stream/page_cache/bytes | bytes | 19.53 M | 19.53 M | 19.53 M |
| stream/requests | count | 0 | 0 | 0 |
| stream/resident_cells | count | 961 | 961 | 961 |
| stream/spawn/queue | count | 0 | 0 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 30.05 | 42.90 | 27.49 |
| system/mem_usage | % | 56.27 | 56.45 | 56.45 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2544 | 2544 | 2544 |
| textures/images | count | 2009 | 2009 | 2009 |
| views/all/batch_sets | count | 69 | 72 | 69 |
| views/all/bins | count | 4970 | 5277 | 4970 |
| views/all/count | count | 3 | 5 | 3 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.4 k | 14.7 k | 14.4 k |
| views/all/pipelines | count | 65 | 68 | 65 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 14.6 k | 14.9 k | 14.6 k |
| views/cascade/batch_sets | count | 17 | 20 | 17 |
| views/cascade/bins | count | 1798 | 2105 | 1798 |
| views/cascade/count | count | 2 | 4 | 2 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5089 | 5415 | 5089 |
| views/cascade/pipelines | count | 17 | 20 | 17 |
| views/cascade/work_items | count | 5089 | 5415 | 5089 |
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
| anim/events/count | count | 0.10 | 2 | 137 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.52 | 11 | 4950 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 32.25 | 34 | 45.4 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1407 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1407 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1407 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 1 | 900 |
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

## Hitches (> 26.5 ms; slowest 49, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.51 | 2514.8 | main/busy/ms 2488, stage/main/post_update 2385, stage/main/update 97.13, surface/advance_cpu_ms 67.48, stage/main/pre_update 2.58, stage/main/last 1.00, surface/cell_upload_ms 0.70, stage/main/fixed_main_loop 0.50, stage/main/first 0.45, stream/drive/ms 0.30, weather/map/ms 0.20, bigspace/total/ms 0.10 |
| warm-up | 3.22 | 706.0 | main/busy/ms 2620, stage/main/post_update 2406, render/thread/ms 675, render/busy/ms 511, stage/render/prepare_assets 475, stage/main/update 201, stage/render/prepare_views 162, render/acquire/ms 161, surface/advance_cpu_ms 67.48, stage/render/prepare_resources 21.68, stage/render/render 11.46, encode//sum 9.74 |
| warm-up | 3.30 | 84.0 | render/thread/ms 743, render/busy/ms 577, stage/render/prepare_assets 520, stage/render/prepare_views 162, main/busy/ms 162, render/acquire/ms 161, stage/main/update 112, stage/main/post_update 35.10, stage/render/prepare_resources 27.49, stage/render/render 23.17, pipelines/compile_ms 19.24, pipelines/front_ms 18.12 |
| warm-up | 3.33 | 28.3 | pipelines/compile_ms 273, pipelines/front_ms 249, render/thread/ms 93.37, render/busy/ms 88.09, pipelines/wait_ms 77.60, main/busy/ms 53.01, stage/render/prepare_assets 47.34, stage/render/render 29.08, stage/main/post_update 25.48, pipelines/driver_ms 24.28, render/graph/ms 23.67, terrain/update/ms 23.44 |
| warm-up | 3.49 | 153.7 | pipelines/wait_ms 3120, pipelines/compile_ms 1608, pipelines/front_ms 1531, render/thread/ms 176, render/busy/ms 172, pipelines/driver_ms 76.46, stage/render/prepare_write_phase_buffers 53.39, main/busy/ms 52.83, stage/render/prepare_resources 35.16, stage/render/render 22.58, stage/render/prepare_batch_phases 22.44, stage/main/post_update 21.48 |
| warm-up | 3.55 | 64.2 | pipelines/wait_ms 6728, pipelines/compile_ms 2402, pipelines/front_ms 2290, render/thread/ms 197, render/busy/ms 194, pipelines/driver_ms 112, main/busy/ms 95.31, stage/main/update 56.18, stage/render/prepare_write_phase_buffers 53.96, stage/render/prepare_resources 39.82, stage/render/prepare_assets 25.77, stage/render/prepare_batch_phases 22.29 |
| warm-up | 3.69 | 143.6 | pipelines/wait_ms 3742, pipelines/compile_ms 1941, pipelines/front_ms 1809, render/thread/ms 163, pipelines/driver_ms 132, render/busy/ms 126, stage/render/prepare_assets 83.16, main/busy/ms 80.15, stage/main/update 57.03, stage/render/render 49.01, render/present/ms 37.12, stage/main/post_update 17.69 |
| warm-up | 3.76 | 36.3 | pipelines/compile_ms 1185, pipelines/front_ms 1028, pipelines/wait_ms 240, pipelines/driver_ms 157, render/thread/ms 54.19, render/busy/ms 52.02, main/busy/ms 29.27, stage/render/render 27.82, render/graph/ms 25.45, encode//sum 23.50, stage/render/prepare_assets 18.89, stage/render/graph_submit 18.49 |
| warm-up | 3.82 | 26.9 | pipelines/compile_ms 532, pipelines/front_ms 500, render/thread/ms 35.25, render/busy/ms 33.40, pipelines/driver_ms 31.80, main/busy/ms 20.35, stage/render/prepare_assets 14.71, stage/main/post_update 11.62, stage/render/render 10.73, terrain/update/ms 9.83, encode//sum 8.68, render/graph/ms 8.61 |
| warm-up | 3.91 | 74.5 | pipelines/compile_ms 784, pipelines/front_ms 762, render/thread/ms 67.42, render/busy/ms 65.62, main/busy/ms 51.71, stage/render/prepare_assets 46.34, stage/main/update 41.36, stream/drive/ms 37.52, pipelines/driver_ms 22.34, stage/render/render 14.79, render/graph/ms 12.62, stage/render/graph_submit 10.34 |
| warm-up | 3.97 | 46.5 | pipelines/compile_ms 266, pipelines/front_ms 251, render/thread/ms 43.28, render/busy/ms 40.96, main/busy/ms 35.10, stage/render/prepare_assets 22.05, stage/main/update 21.84, stream/drive/ms 17.91, pipelines/driver_ms 15.79, stage/render/render 14.06, render/graph/ms 10.83, stage/render/graph_submit 8.36 |
| warm-up | 4.02 | 46.0 | pipelines/compile_ms 833, pipelines/front_ms 796, render/thread/ms 68.68, render/busy/ms 66.45, stage/render/prepare_assets 42.16, pipelines/driver_ms 37.70, main/busy/ms 32.06, stage/render/render 15.13, render/graph/ms 12.68, pipelines/wait_ms 10.78, stage/main/post_update 9.83, stage/main/update 9.30 |
| warm-up | 4.08 | 61.2 | pipelines/compile_ms 836, pipelines/front_ms 806, render/thread/ms 85.58, main/busy/ms 83.54, render/busy/ms 83.20, stage/render/prepare_assets 50.92, stage/main/update 45.83, pipelines/driver_ms 29.98, stage/main/post_update 23.90, stream/drive/ms 20.23, stage/render/render 18.52, pipelines/wait_ms 17.09 |
| warm-up | 4.19 | 109.5 | pipelines/compile_ms 1419, pipelines/front_ms 1381, render/thread/ms 110, render/busy/ms 108, main/busy/ms 108, stage/render/prepare_assets 80.59, pipelines/wait_ms 71.90, stage/main/update 48.94, stage/main/post_update 38.80, pipelines/driver_ms 37.72, stream/drive/ms 21.28, stage/render/render 16.00 |
| warm-up | 4.27 | 76.5 | pipelines/compile_ms 2019, pipelines/front_ms 1955, main/busy/ms 158, render/thread/ms 125, render/busy/ms 123, stage/render/prepare_assets 82.35, stage/main/update 81.82, pipelines/wait_ms 67.05, stream/drive/ms 64.99, pipelines/driver_ms 63.74, stage/main/post_update 45.89, stage/render/render 26.29 |
| warm-up | 4.39 | 123.1 | pipelines/compile_ms 2724, pipelines/front_ms 2632, pipelines/wait_ms 203, main/busy/ms 134, render/thread/ms 112, render/busy/ms 110, pipelines/driver_ms 91.99, stage/main/update 82.78, stage/render/prepare_assets 66.09, stream/drive/ms 65.86, stage/main/post_update 31.38, stage/render/render 25.82 |
| warm-up | 4.45 | 64.9 | pipelines/compile_ms 2940, pipelines/front_ms 2848, pipelines/wait_ms 384, render/thread/ms 101, render/busy/ms 99.17, pipelines/driver_ms 92.00, main/busy/ms 71.07, stage/render/prepare_assets 51.37, stage/main/update 41.28, stage/render/prepare_resources 19.61, stream/drive/ms 15.27, stage/main/post_update 14.00 |
| warm-up | 4.53 | 80.4 | pipelines/compile_ms 1861, pipelines/front_ms 1803, pipelines/wait_ms 356, render/thread/ms 106, render/busy/ms 104, main/busy/ms 83.11, stage/render/prepare_assets 59.14, pipelines/driver_ms 57.66, stage/main/update 53.35, stage/render/prepare_resources 19.24, stage/main/post_update 16.02, stream/drive/ms 14.48 |
| warm-up | 4.58 | 43.0 | pipelines/compile_ms 821, pipelines/front_ms 795, pipelines/wait_ms 172, render/thread/ms 89.35, render/busy/ms 87.64, stage/render/prepare_assets 64.28, main/busy/ms 53.33, stage/main/update 31.84, pipelines/driver_ms 25.93, stage/main/post_update 14.45, stage/render/render 10.34, encode//sum 9.30 |
| warm-up | 4.61 | 32.8 | pipelines/compile_ms 694, pipelines/front_ms 672, render/thread/ms 60.33, render/busy/ms 58.39, main/busy/ms 35.65, stage/render/prepare_assets 35.41, pipelines/driver_ms 21.20, stage/main/update 15.75, pipelines/wait_ms 11.67, stage/render/render 10.63, stage/main/post_update 10.39, encode//sum 10.06 |
| warm-up | 4.64 | 28.4 | pipelines/compile_ms 1132, pipelines/front_ms 1094, render/thread/ms 49.69, render/busy/ms 47.39, pipelines/driver_ms 38.17, pipelines/wait_ms 35.50, main/busy/ms 27.02, stage/render/prepare_assets 23.15, stage/render/render 13.28, encode//sum 11.27, render/graph/ms 10.34, stage/main/update 9.62 |
| warm-up | 4.69 | 29.9 | pipelines/compile_ms 1075, pipelines/front_ms 1042, pipelines/wait_ms 83.11, main/busy/ms 66.34, stage/main/update 53.52, stream/drive/ms 44.16, render/thread/ms 39.30, render/busy/ms 37.62, pipelines/driver_ms 32.41, stage/render/prepare_assets 15.28, stage/render/render 12.33, encode//sum 10.71 |
| warm-up | 4.75 | 62.5 | pipelines/compile_ms 947, pipelines/front_ms 919, main/busy/ms 113, stage/main/update 103, stream/drive/ms 92.60, render/thread/ms 47.39, render/busy/ms 45.53, pipelines/wait_ms 36.29, pipelines/driver_ms 28.68, stage/render/prepare_assets 26.42, stage/render/render 11.23, encode//sum 10.76 |
| warm-up | 4.81 | 60.9 | pipelines/compile_ms 725, pipelines/front_ms 703, main/busy/ms 75.97, stage/main/update 63.58, stream/drive/ms 52.26, render/thread/ms 44.98, render/busy/ms 43.11, pipelines/driver_ms 22.07, pipelines/wait_ms 20.45, stage/render/prepare_assets 17.90, encode//sum 10.23, stage/render/render 9.80 |
| warm-up | 4.86 | 52.0 | pipelines/compile_ms 735, pipelines/front_ms 711, render/thread/ms 57.71, render/busy/ms 56.08, main/busy/ms 33.56, pipelines/driver_ms 24.50, pipelines/wait_ms 18.75, stage/main/update 14.08, stage/render/prepare_assets 13.76, stage/render/render 13.00, stage/render/prepare_write_phase_buffers 12.53, stage/render/prepare_resources 11.38 |
| warm-up | 4.91 | 43.6 | pipelines/compile_ms 1725, pipelines/front_ms 1669, render/thread/ms 76.63, render/busy/ms 74.78, main/busy/ms 59.07, pipelines/driver_ms 55.98, stage/main/update 28.35, stage/render/render 22.20, render/graph/ms 19.81, stage/main/post_update 18.63, encode//sum 17.53, stage/render/prepare_assets 16.21 |
| warm-up | 4.98 | 78.3 | pipelines/compile_ms 1718, pipelines/front_ms 1662, render/thread/ms 95.77, render/busy/ms 93.82, main/busy/ms 81.27, pipelines/driver_ms 55.43, stage/main/update 54.86, stage/render/prepare_assets 39.93, stage/render/render 21.10, render/graph/ms 18.53, encode//sum 18.39, stage/main/post_update 16.86 |
| warm-up | 5.02 | 35.4 | pipelines/compile_ms 739, pipelines/front_ms 711, render/thread/ms 83.95, render/busy/ms 82.27, main/busy/ms 69.38, stage/main/update 48.92, stage/render/prepare_assets 42.77, pipelines/driver_ms 28.15, encode//sum 14.28, stage/render/render 13.94, stage/main/post_update 13.25, render/graph/ms 11.55 |
| warm-up | 5.07 | 49.9 | pipelines/compile_ms 725, pipelines/front_ms 700, render/thread/ms 73.81, render/busy/ms 72.13, main/busy/ms 63.52, stage/main/post_update 31.56, stage/render/prepare_assets 28.40, stage/main/update 25.71, pipelines/driver_ms 25.34, stage/render/prepare_resources 15.91, bigspace/total/ms 14.20, encode//sum 14.14 |
| warm-up | 5.10 | 33.4 | pipelines/compile_ms 1063, pipelines/front_ms 1027, render/thread/ms 76.49, render/busy/ms 74.71, main/busy/ms 54.27, pipelines/driver_ms 35.51, stage/render/prepare_assets 31.62, stage/main/post_update 26.03, stage/main/update 20.88, stage/render/prepare_resources 15.84, encode//sum 13.67, stage/render/render 12.88 |
| warm-up | 5.17 | 37.2 | pipelines/compile_ms 351, pipelines/front_ms 338, main/busy/ms 54.32, render/thread/ms 52.44, render/busy/ms 50.46, stage/main/update 31.05, stage/render/render 23.47, encode//sum 22.02, render/graph/ms 20.68, stage/main/post_update 16.76, stage/render/graph_submit 16.21, pipelines/driver_ms 13.35 |
| warm-up | 5.20 | 36.4 | pipelines/compile_ms 649, pipelines/front_ms 632, render/thread/ms 59.80, render/busy/ms 57.67, main/busy/ms 48.35, stage/render/render 22.68, stage/main/update 22.23, encode//sum 20.77, stage/main/post_update 20.22, render/graph/ms 19.65, pipelines/driver_ms 17.61, stage/render/graph_submit 16.40 |
| warm-up | 5.24 | 41.5 | pipelines/compile_ms 1516, pipelines/front_ms 1473, render/thread/ms 61.08, render/busy/ms 59.04, pipelines/driver_ms 43.86, main/busy/ms 39.57, stage/render/prepare_assets 26.20, stage/main/update 19.43, pipelines/wait_ms 16.24, stage/render/render 15.70, render/graph/ms 13.11, encode//sum 12.97 |
| warm-up | 5.28 | 35.0 | pipelines/compile_ms 1077, pipelines/front_ms 1044, render/thread/ms 57.59, render/busy/ms 56.00, main/busy/ms 40.56, pipelines/driver_ms 33.21, stage/render/prepare_assets 27.21, stage/main/update 21.17, pipelines/wait_ms 15.84, encode//sum 13.55, stage/main/post_update 12.02, stage/render/render 10.45 |
| warm-up | 5.31 | 32.3 | pipelines/compile_ms 386, pipelines/front_ms 375, render/thread/ms 48.22, render/busy/ms 46.88, main/busy/ms 36.20, stage/render/prepare_assets 24.72, stage/main/update 18.51, encode//sum 14.81, stage/main/post_update 11.64, pipelines/driver_ms 10.90, stage/render/render 10.00, render/graph/ms 7.90 |
| warm-up | 5.43 | 27.3 | pipelines/compile_ms 504, pipelines/front_ms 489, render/thread/ms 34.24, render/busy/ms 32.78, main/busy/ms 25.75, encode//sum 15.38, stage/main/update 14.94, pipelines/driver_ms 14.63, stage/render/prepare_assets 13.67, stage/render/render 11.46, render/graph/ms 9.60, stream/drive/ms 8.84 |
| warm-up | 5.47 | 31.1 | render/thread/ms 35.31, render/busy/ms 33.96, main/busy/ms 20.47, stage/render/prepare_assets 15.07, encode//sum 14.96, stage/render/render 9.88, stage/main/update 8.69, render/graph/ms 7.77, stage/main/post_update 7.24, stage/render/graph_submit 5.40, encode/finish_wall 3.09, stream/drive/ms 2.93 |
| warm-up | 5.52 | 36.7 | pipelines/compile_ms 144, pipelines/front_ms 142, render/thread/ms 44.08, render/busy/ms 42.90, main/busy/ms 33.31, stage/render/prepare_assets 23.84, stage/main/update 18.87, encode//sum 17.69, stage/render/render 12.47, render/graph/ms 10.83, stage/main/post_update 9.05, stage/render/graph_submit 8.47 |
| warm-up | 5.58 | 30.8 | render/thread/ms 51.62, render/busy/ms 50.01, main/busy/ms 26.62, stage/render/prepare_assets 21.39, encode//sum 18.86, stage/render/render 13.05, render/graph/ms 10.78, stage/main/update 10.12, stage/main/post_update 10.07, stage/render/graph_submit 8.22, encode/submit 4.41, stage/render/specialize 4.08 |
| warm-up | 5.64 | 37.4 | render/thread/ms 48.19, render/busy/ms 46.98, main/busy/ms 29.09, stage/render/prepare_assets 28.40, encode//sum 14.48, stage/main/update 14.16, stage/render/render 9.12, stage/main/post_update 7.81, render/graph/ms 7.31, stream/drive/ms 5.72, stage/render/graph_submit 5.00, encode/submit 2.74 |
| warm-up | 5.69 | 27.1 | render/thread/ms 45.23, render/busy/ms 44.00, stage/render/prepare_assets 27.11, main/busy/ms 23.42, encode//sum 13.61, stage/main/update 10.15, stage/main/post_update 8.60, stage/render/render 8.52, render/graph/ms 6.94, stage/render/graph_submit 5.14, stream/drive/ms 4.23, encode/submit 2.65 |
| warm-up | 5.74 | 27.7 | render/thread/ms 45.76, render/busy/ms 44.26, main/busy/ms 38.94, stage/main/update 19.62, stage/render/prepare_assets 19.25, encode//sum 17.62, stage/main/post_update 13.59, stage/render/render 13.42, render/graph/ms 11.73, stage/render/graph_submit 9.00, stream/drive/ms 5.67, encode/submit 5.52 |
| warm-up | 5.77 | 29.4 | render/thread/ms 48.18, render/busy/ms 46.17, main/busy/ms 40.42, encode//sum 23.31, stage/main/update 19.74, stage/render/render 17.44, render/graph/ms 14.87, stage/render/prepare_assets 14.63, stage/main/post_update 14.21, stage/render/graph_submit 11.13, encode/submit 6.36, stream/drive/ms 5.23 |
| warm-up | 5.80 | 33.4 | render/thread/ms 49.69, render/busy/ms 47.77, main/busy/ms 36.25, encode//sum 26.69, stage/main/post_update 17.29, stage/render/render 17.11, stage/render/prepare_assets 16.58, render/graph/ms 14.50, stage/main/update 12.29, stage/render/graph_submit 10.43, encode/submit 5.87, encode/finish_wall 4.63 |
| warm-up | 5.84 | 33.0 | render/thread/ms 51.17, render/busy/ms 49.40, main/busy/ms 48.08, encode//sum 27.39, stage/main/update 25.88, stage/render/prepare_assets 18.99, stage/render/render 17.28, stage/main/post_update 15.28, render/graph/ms 14.80, stream/drive/ms 13.55, stage/render/graph_submit 10.29, encode/submit 5.80 |
| warm-up | 5.87 | 36.4 | render/thread/ms 46.79, render/busy/ms 45.33, main/busy/ms 42.92, stage/main/update 26.68, stage/render/prepare_assets 24.39, encode//sum 19.61, pipelines/compile_ms 13.90, stream/drive/ms 13.29, pipelines/front_ms 12.94, stage/render/render 12.19, stage/main/post_update 10.24, render/graph/ms 10.05 |
| warm-up | 6.02 | 30.7 | main/busy/ms 37.33, render/thread/ms 28.31, render/busy/ms 27.15, stage/main/update 22.87, encode//sum 16.32, stream/drive/ms 15.54, stage/render/render 11.19, stage/main/post_update 9.94, render/graph/ms 9.76, stage/render/prepare_assets 8.98, stage/render/graph_submit 7.28, encode/submit 3.73 |
| warm-up | 6.32 | 28.3 | main/busy/ms 47.12, render/thread/ms 43.39, render/busy/ms 41.43, encode//sum 27.93, stage/main/update 24.47, stage/render/render 19.40, render/graph/ms 16.65, stage/main/post_update 16.47, stage/render/prepare_assets 13.00, stage/render/graph_submit 12.09, stream/drive/ms 11.92, encode/submit 6.79 |
| warm-up | 6.36 | 48.9 | render/thread/ms 51.36, render/busy/ms 49.77, main/busy/ms 46.41, stage/render/prepare_assets 29.38, stage/main/update 25.68, encode//sum 22.59, stage/render/render 13.63, stage/main/post_update 13.60, stream/drive/ms 12.17, render/graph/ms 11.33, stage/render/graph_submit 8.38, encode/finish_wall 4.47 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_nohold\balmora@2560x1440_off=cascadehold-2 --bench-baseline none --off cascadehold
- env: BEVYRROWIND_DLSS=quality
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 98%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.3 W · 74 °C (max 76) · VRAM 4950 MiB · throttled 100% of samples
