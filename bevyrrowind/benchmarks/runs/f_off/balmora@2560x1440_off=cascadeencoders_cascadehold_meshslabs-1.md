# `exterior_balmora`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: meshslabs,cascadeencoders,cascadehold · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None
- 1399 frames over 15.0 s · **median 10.60 ms (94 fps)** · mean 10.73 · p95 12.11 · p99 13.34 (1% low 75 fps) · worst 17.46 ms · 2 spikes over 1.5× median · 8.0 stutters/min (work 4.0/min)
- warm-up: 280 frames over 8.6 s, worst 2603.1 ms
- hitches (> 26.5 ms): **0** measured, 49 in warm-up
- timed GPU passes: 8.92 ms of a 10.73 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: PASS

## Gates

- pass Median frame 10.60 ms, limit 11 ms
- pass worst frame 17.46 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| main_opaque_pass_3d | 2.896 | 2.941 | 3.423 | 0.98 | 0.066 |
| dlss_super_resolution | 2.046 | 2.077 | 2.331 | 0.98 | 0.190 |
| early prepass | 0.795 | 0.807 | 0.994 | 0.98 | 0.090 |
| atmosphere_luts | 0.419 | 0.425 | 0.689 | 0.98 | 0.004 |
| bloom | 0.319 | 0.324 | 0.338 | 0.98 | 0.070 |
| main_transmissive_pass_3d | 0.280 | 0.284 | 0.297 | 0.98 | 0.009 |
| water_velocity | 0.244 | 0.248 | 0.268 | 0.98 | 0.002 |
| water_sspr | 0.212 | 0.215 | 0.216 | 0.98 | 0.002 |
| ssao | 0.211 | 0.214 | 0.357 | 0.98 | 0.008 |
| clouds_trace | 0.150 | 0.152 | 0.185 | 0.98 | 0.001 |
| uniform_allocation | 0.149 | 0.151 | 0.154 | 0.98 | 0.030 |
| atmos_froxel_inject | 0.147 | 0.149 | 0.152 | 0.98 | 0.002 |
| water_fft | 0.138 | 0.140 | 0.140 | 0.98 | 0.003 |
| upscaling | 0.117 | 0.119 | 0.124 | 0.98 | 0.005 |
| clouds_composite | 0.113 | 0.114 | 0.116 | 0.98 | 0.002 |
| bin_unpacking | 0.110 | 0.112 | 0.116 | 0.98 | 0.022 |
| tonemapping | 0.087 | 0.088 | 0.094 | 0.98 | 0.004 |
| clustering | 0.066 | 0.067 | 0.079 | 0.98 | 0.023 |
| lightprobe_irradiance_map | 0.052 | 0.424 | 0.419 | 0.12 | 0.001 |
| render_sky | 0.052 | 0.053 | 0.054 | 0.98 | 0.002 |
| clouds_resolve | 0.050 | 0.051 | 0.053 | 0.98 | 0.001 |
| early_mesh_preprocessing | 0.039 | 0.040 | 0.041 | 0.98 | 0.006 |
| clouds_shadow | 0.031 | 0.031 | 0.035 | 0.98 | 0.003 |
| atmos_froxel_integrate | 0.029 | 0.030 | 0.031 | 0.98 | 0.001 |
| lightprobe_radiance_map | 0.028 | 0.223 | 0.233 | 0.12 | 0.005 |
| main_transparent_pass_3d | 0.026 | 0.026 | 0.030 | 0.98 | 0.078 |
| water_mips | 0.026 | 0.026 | 0.024 | 0.98 | 0.002 |
| water_snapshot | 0.025 | 0.025 | 0.028 | 0.98 | 0.002 |
| per_view_shadow_pass | 0.023 | 0.023 | 0.031 | 0.98 | 0.008 |
| atmosphere_environment | 0.012 | 0.012 | 0.014 | 0.98 | 0.003 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.98 | 0.002 |
| sparse buffer updates | 0.006 | 0.006 | 0.008 | 0.98 | 0.004 |
| night_sky | 0.004 | 0.004 | 0.006 | 0.98 | 0.002 |
| lightprobe_copy | 0.002 | 0.015 | 0.016 | 0.12 | 0.004 |
| auxiliary_views | 0.002 | 0.002 | 0.003 | 0.98 | 0.004 |
| lightprobe_downsampling_first_pass | 0.002 | 0.014 | 0.014 | 0.12 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.011 | 0.012 | 0.12 | 0.001 |
| blas_build | 0.000 | 0.002 | 0.002 | 0.00 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.764 | 11.160 | 16.42 | 13659.4 |
| encode//sum | 8.402 | 12.104 | 25.09 | 11754.2 |
| render/busy/ms | 8.238 | 9.964 | 15.59 | 11525.4 |
| main/busy/ms | 7.724 | 9.205 | 15.85 | 10805.8 |
| stage/render/render | 5.789 | 7.437 | 12.17 | 8098.4 |
| render/graph/ms | 5.047 | 6.502 | 11.14 | 7060.9 |
| stage/main/post_update | 4.635 | 5.931 | 9.66 | 6484.6 |
| stage/render/graph_submit | 3.964 | 5.371 | 9.34 | 5545.9 |
| encode/submit | 2.017 | 2.983 | 6.47 | 2821.7 |
| encode/finish_wall | 1.998 | 2.617 | 5.56 | 2795.4 |
| stage/main/update | 1.947 | 2.351 | 5.48 | 2723.6 |
| stage/render/prepare_views | 1.076 | 2.200 | 3.33 | 1505.1 |
| render/acquire/ms | 1.027 | 2.157 | 3.29 | 1437.1 |
| stage/render/camera_driver | 1.018 | 1.234 | 1.81 | 1423.7 |
| stage/render/prepare_write_phase_buffers | 0.738 | 0.917 | 1.41 | 1032.5 |
| render/present/ms | 0.498 | 0.633 | 1.85 | 696.9 |
| stage/render/prepare_assets | 0.470 | 0.617 | 1.83 | 658.0 |
| bigspace/total/ms | 0.421 | 1.030 | 3.51 | 589.1 |
| stage/render/prepare_resources | 0.379 | 0.483 | 2.99 | 529.5 |
| stage/main/pre_update | 0.330 | 0.454 | 1.13 | 461.2 |
| stage/main/first | 0.304 | 0.382 | 1.68 | 425.3 |
| terrain/update/ms | 0.239 | 0.313 | 0.60 | 334.7 |
| stage/render/prepare_bind_groups | 0.214 | 0.344 | 0.73 | 299.7 |
| terrain/select/ms | 0.208 | 0.271 | 0.57 | 290.3 |
| stage/main/fixed_main_loop | 0.203 | 0.385 | 1.07 | 283.6 |
| stream/drive/ms | 0.200 | 0.308 | 0.51 | 279.5 |
| stage/render/prepare_meshes | 0.195 | 0.260 | 0.49 | 272.5 |
| stage/render/prepare_flush | 0.194 | 0.382 | 1.09 | 271.2 |
| controller/feed/ms | 0.182 | 0.249 | 0.53 | 254.3 |
| stage/render/extract_commands | 0.175 | 0.219 | 0.38 | 244.5 |
| stage/render/prepare_batch_phases | 0.173 | 0.233 | 0.52 | 241.9 |
| bigspace/high_precision/ms | 0.172 | 0.592 | 3.23 | 240.5 |
| far/select/ms | 0.171 | 0.254 | 0.52 | 238.9 |
| stream/tick/ms | 0.170 | 0.267 | 0.48 | 238.1 |
| stage/render/create_views | 0.153 | 0.203 | 0.45 | 213.7 |
| controller/move/ms | 0.146 | 0.293 | 0.50 | 203.9 |
| stage/main/last | 0.120 | 0.168 | 0.53 | 167.8 |
| stage/render/queue | 0.111 | 0.148 | 0.41 | 154.7 |
| anim/evaluate/ms | 0.109 | 0.230 | 2.24 | 152.9 |
| bigspace/low_precision/ms | 0.100 | 0.214 | 2.05 | 139.8 |
| stage/render/specialize | 0.097 | 0.133 | 0.42 | 135.2 |
| anim/evaluate_cpu/ms | 0.061 | 0.081 | 0.36 | 85.3 |
| stage/render/phase_sort | 0.053 | 0.113 | 0.26 | 74.6 |
| stage/render/prepare_collect_phase_buffers | 0.053 | 0.084 | 0.35 | 73.7 |
| encode/resolve_diagnostics/sum | 0.044 | 0.081 | 0.43 | 61.0 |
| physics/step/ms | 0.031 | 0.059 | 0.32 | 43.8 |
| weather/map/ms | 0.026 | 0.035 | 0.19 | 36.8 |
| stage/render/cleanup | 0.024 | 0.034 | 0.28 | 33.9 |
| encode/camera_driver_span_begin/sum | 0.021 | 0.037 | 0.33 | 29.8 |
| encode/camera_driver_span_end/sum | 0.017 | 0.035 | 0.08 | 23.6 |
| weather/eval/ms | 0.012 | 0.016 | 0.05 | 16.4 |
| script/update/ms | 0.008 | 0.012 | 0.03 | 10.7 |
| controller/avoid/ms | 0.006 | 0.008 | 0.04 | 8.6 |
| script/ui/ms | 0.006 | 0.008 | 0.02 | 8.0 |
| script/input/ms | 0.005 | 0.006 | 0.02 | 7.3 |
| anim/apply/ms | 0.004 | 0.006 | 0.03 | 6.3 |
| controller/write/ms | 0.002 | 0.002 | 0.02 | 2.5 |
| weather/fx/ms | 0.001 | 0.002 | 0.17 | 1.7 |
| stage/render/prepare | 0.001 | 0.004 | 0.27 | 1.4 |
| shapes/activate/ms | 0.001 | 0.001 | 0.03 | 1.0 |
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
| crowd/c1/count | count | 123 | 123 | 123 |
| crowd/c1/joints/count | count | 4880 | 4880 | 4880 |
| crowd/c1/meshes/count | count | 1088 | 1088 | 1088 |
| crowd/c2/count | count | 283 | 283 | 283 |
| crowd/c2/joints/count | count | 4916 | 4916 | 4916 |
| crowd/c2/meshes/count | count | 1373 | 1373 | 1373 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 167 | 167 | 167 |
| crowd/joints_extracted/count | count | 4212 | 4212 | 4212 |
| crowd/joints_uploaded/count | count | 4212 | 4212 | 4212 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 47.1 k | 47.1 k | 47.1 k |
| encode//max | ms | 1.68 | 5.38 | 1.65 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.33 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.08 | 0.02 |
| encode/finish_critical | ms | 1.68 | 5.38 | 1.65 |
| encode/resolve_diagnostics/max | ms | 0.04 | 0.43 | 0.03 |
| far/entities | count | 2319 | 2319 | 2319 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 58.5 k | 58.5 k | 58.5 k |
| far/lights | count | 6750 | 6750 | 6750 |
| far/nodes/l1 | count | 946 | 946 | 946 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 189.08 M | 189.08 M | 189.08 M |
| far/triangles | count | 4.64 M | 4.64 M | 4.64 M |
| frame/work/ms | ms | 9.62 | 15.85 | 9.66 |
| gpu/timed/ms | ms | 9.60 | 11.50 | 9.66 |
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
| phases/opaque3d/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 20 | 20 | 20 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 17 | 17 | 17 |
| phases/opaque3d_prepass/bins | count | 1307 | 1307 | 1307 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3767 | 3767 | 3767 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 16 | 16 | 16 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 19 | 22 | 19 |
| phases/shadow/bins | count | 1893 | 2200 | 1893 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 5194 | 5520 | 5194 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 19 | 22 | 19 |
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
| preprocess/bind_groups_reused | count | 165 | 165 | 165 |
| process/cpu_usage | % | 29.33 | 41.35 | 36.15 |
| process/mem_usage | GiB | 4.63 | 4.71 | 4.70 |
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
| system/cpu_usage | % | 36.57 | 58.18 | 45.39 |
| system/mem_usage | % | 66.82 | 67.54 | 67.14 |
| terrain/patches | count | 370 | 370 | 370 |
| textures/gpu_mib | MiB | 2544 | 2544 | 2544 |
| textures/images | count | 2009 | 2009 | 2009 |
| views/all/batch_sets | count | 71 | 74 | 71 |
| views/all/bins | count | 5067 | 5374 | 5067 |
| views/all/count | count | 4 | 6 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 14.5 k | 14.8 k | 14.5 k |
| views/all/pipelines | count | 67 | 70 | 67 |
| views/all/pipelines_distinct | count | 63 | 63 | 63 |
| views/all/work_items | count | 14.7 k | 15.0 k | 14.7 k |
| views/cascade/batch_sets | count | 19 | 22 | 19 |
| views/cascade/bins | count | 1893 | 2200 | 1893 |
| views/cascade/count | count | 3 | 5 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 5194 | 5520 | 5194 |
| views/cascade/pipelines | count | 19 | 22 | 19 |
| views/cascade/work_items | count | 5194 | 5520 | 5194 |
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
| views/main/bins | count | 3174 | 3174 | 3174 |
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
| anim/events/count | count | 0.09 | 2 | 127 |
| assets/Image/added | count | 0 | 0 | 0 |
| assets/Image/modified | count | 0.00 | 1 | 4 |
| assets/Image/removed | count | 0 | 0 | 0 |
| assets/Mesh/added | count | 0 | 0 | 0 |
| assets/Mesh/removed | count | 0 | 0 | 0 |
| assets/StandardMaterial/added | count | 0 | 0 | 0 |
| assets/StandardMaterial/modified | count | 3.55 | 11 | 4961 |
| assets/WorldAsset/added | count | 0 | 0 | 0 |
| assets/WorldAsset/removed | count | 0 | 0 | 0 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.00 | 1 | 2 |
| encode//count | count | 31.25 | 33 | 43.7 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 1399 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 1399 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 1399 |
| physics/inserts | count | 0 | 0 | 0 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0 | 0 | 0 |
| physics/steps | count | 0.64 | 1 | 901 |
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
| warm-up | 2.60 | 2603.1 | main/busy/ms 2574, stage/main/post_update 2474, stage/main/update 93.66, surface/advance_cpu_ms 65.96, stage/main/pre_update 2.22, stage/main/last 1.13, surface/cell_upload_ms 0.96, stage/main/first 0.60, stage/main/fixed_main_loop 0.44, stream/drive/ms 0.31, weather/map/ms 0.08, bigspace/total/ms 0.08 |
| warm-up | 3.36 | 752.4 | main/busy/ms 2714, stage/main/post_update 2498, render/thread/ms 722, render/busy/ms 572, stage/render/prepare_assets 525, stage/main/update 204, stage/render/prepare_views 148, render/acquire/ms 147, surface/advance_cpu_ms 65.96, stage/render/create_views 26.95, encode//sum 12.41, stage/render/render 12.19 |
| warm-up | 3.44 | 86.4 | render/thread/ms 792, render/busy/ms 637, stage/render/prepare_assets 575, main/busy/ms 164, stage/render/prepare_views 148, render/acquire/ms 148, stage/main/update 115, pipelines/compile_ms 85.25, pipelines/front_ms 81.08, stage/main/post_update 37.13, stage/render/create_views 27.15, stage/render/render 22.72 |
| warm-up | 3.54 | 84.3 | pipelines/wait_ms 1436, pipelines/compile_ms 656, pipelines/front_ms 591, render/thread/ms 74.67, render/busy/ms 71.76, pipelines/driver_ms 65.59, main/busy/ms 35.83, stage/render/prepare_resources 25.49, stage/main/post_update 20.75, stage/render/prepare_assets 20.08, terrain/update/ms 13.76, stage/render/prepare_bind_groups 7.89 |
| warm-up | 3.63 | 95.1 | pipelines/wait_ms 3591, pipelines/compile_ms 1472, pipelines/front_ms 1372, render/thread/ms 147, render/busy/ms 145, main/busy/ms 132, pipelines/driver_ms 99.70, stage/main/update 63.67, stage/render/prepare_resources 57.95, stage/main/pre_update 35.74, stage/render/prepare_bind_groups 35.29, stage/render/prepare_assets 29.26 |
| warm-up | 3.76 | 122.3 | pipelines/wait_ms 5950, pipelines/compile_ms 2800, pipelines/front_ms 2672, render/thread/ms 168, render/busy/ms 166, main/busy/ms 158, pipelines/driver_ms 127, stage/render/prepare_assets 76.97, stage/main/update 69.24, stage/main/pre_update 51.40, stage/render/prepare_resources 35.35, stage/render/prepare_bind_groups 28.27 |
| warm-up | 3.80 | 42.2 | pipelines/wait_ms 3838, pipelines/compile_ms 2390, pipelines/front_ms 2281, render/thread/ms 121, render/busy/ms 119, pipelines/driver_ms 109, stage/render/prepare_assets 94.02, main/busy/ms 64.54, stage/main/pre_update 21.14, stage/main/post_update 17.34, stage/render/render 17.05, render/graph/ms 14.61 |
| warm-up | 3.83 | 28.6 | pipelines/compile_ms 489, pipelines/front_ms 465, render/thread/ms 62.73, render/busy/ms 60.99, pipelines/wait_ms 45.47, stage/render/prepare_assets 43.35, main/busy/ms 32.03, pipelines/driver_ms 24.27, stage/main/post_update 14.12, encode//sum 10.19, stage/render/render 9.75, terrain/update/ms 9.71 |
| warm-up | 3.86 | 35.4 | pipelines/compile_ms 351, pipelines/front_ms 332, pipelines/wait_ms 145, encode//sum 99.84, render/thread/ms 61.04, render/busy/ms 58.76, main/busy/ms 36.08, stage/render/render 27.17, stage/main/post_update 25.90, render/graph/ms 24.30, stage/render/graph_submit 19.56, pipelines/driver_ms 18.47 |
| warm-up | 3.92 | 38.8 | pipelines/compile_ms 330, pipelines/front_ms 312, pipelines/wait_ms 132, render/thread/ms 53.42, render/busy/ms 50.87, main/busy/ms 31.35, stage/render/prepare_assets 27.56, pipelines/driver_ms 18.27, stage/render/render 16.52, render/graph/ms 13.83, stage/main/update 12.39, stage/main/post_update 12.26 |
| warm-up | 4.00 | 69.8 | pipelines/compile_ms 1365, pipelines/front_ms 1310, pipelines/driver_ms 55.53, main/busy/ms 51.52, render/thread/ms 48.49, render/busy/ms 46.83, stage/main/update 40.99, stream/drive/ms 36.82, stage/render/prepare_assets 31.96, stage/render/render 10.30, render/graph/ms 8.30, encode//sum 7.14 |
| warm-up | 4.03 | 29.2 | pipelines/compile_ms 737, pipelines/front_ms 706, render/thread/ms 61.99, render/busy/ms 60.20, main/busy/ms 43.90, pipelines/driver_ms 31.09, stage/main/update 30.04, stage/render/prepare_assets 29.18, stream/drive/ms 25.03, stage/render/render 13.06, encode//sum 11.14, render/graph/ms 10.80 |
| warm-up | 4.10 | 63.3 | pipelines/compile_ms 329, pipelines/front_ms 318, render/thread/ms 74.49, render/busy/ms 72.45, main/busy/ms 44.02, stage/render/prepare_assets 30.38, stage/main/update 29.96, stream/drive/ms 25.73, stage/render/render 22.88, render/graph/ms 20.24, encode//sum 12.99, pipelines/driver_ms 11.60 |
| warm-up | 4.15 | 55.8 | pipelines/compile_ms 482, pipelines/front_ms 465, render/thread/ms 97.12, render/busy/ms 94.71, main/busy/ms 58.57, stage/render/prepare_assets 47.79, stage/render/render 33.29, render/graph/ms 30.40, stage/main/post_update 27.12, stage/main/update 25.71, stream/drive/ms 21.10, stage/render/camera_driver 19.11 |
| warm-up | 4.24 | 87.7 | pipelines/compile_ms 1425, pipelines/front_ms 1380, render/thread/ms 104, render/busy/ms 102, main/busy/ms 86.17, stage/render/prepare_assets 59.33, stage/main/update 50.26, pipelines/driver_ms 44.76, stage/main/post_update 28.44, stage/render/render 27.64, render/graph/ms 24.87, stream/drive/ms 22.04 |
| warm-up | 4.39 | 147.2 | pipelines/compile_ms 1863, pipelines/front_ms 1809, render/thread/ms 165, render/busy/ms 163, main/busy/ms 128, stage/main/update 107, stage/render/prepare_assets 98.50, stream/drive/ms 65.19, pipelines/driver_ms 53.50, stage/render/prepare_write_phase_buffers 35.52, stage/render/render 18.89, render/graph/ms 15.89 |
| warm-up | 4.49 | 102.1 | pipelines/compile_ms 2377, pipelines/front_ms 2312, pipelines/wait_ms 265, render/thread/ms 184, render/busy/ms 182, main/busy/ms 108, stage/render/prepare_assets 106, stage/main/update 88.90, pipelines/driver_ms 65.36, stream/drive/ms 65.13, stage/render/prepare_write_phase_buffers 35.57, stage/render/render 26.74 |
| warm-up | 4.60 | 110.8 | pipelines/compile_ms 2755, pipelines/front_ms 2677, pipelines/wait_ms 485, render/thread/ms 154, render/busy/ms 152, stage/render/prepare_assets 101, pipelines/driver_ms 78.41, main/busy/ms 52.43, stage/main/update 30.82, stage/render/render 22.71, render/graph/ms 20.51, stream/drive/ms 13.43 |
| warm-up | 4.64 | 41.5 | pipelines/compile_ms 1321, pipelines/front_ms 1283, pipelines/wait_ms 267, render/thread/ms 119, render/busy/ms 118, main/busy/ms 77.09, stage/render/prepare_assets 55.57, stage/main/update 40.22, pipelines/driver_ms 37.94, stage/main/post_update 28.88, stage/render/prepare_resources 15.43, stage/render/render 13.68 |
| warm-up | 4.71 | 68.0 | pipelines/compile_ms 1340, pipelines/front_ms 1307, pipelines/wait_ms 188, render/thread/ms 94.01, render/busy/ms 92.56, main/busy/ms 66.30, pipelines/driver_ms 33.63, stage/main/update 30.80, stage/main/post_update 27.16, stage/render/prepare_assets 23.82, stage/render/prepare_resources 16.77, stage/render/prepare_flush 13.31 |
| warm-up | 4.74 | 35.2 | pipelines/compile_ms 1790, pipelines/front_ms 1747, pipelines/wait_ms 235, render/thread/ms 81.09, render/busy/ms 79.33, pipelines/driver_ms 43.07, main/busy/ms 41.46, stage/render/prepare_assets 32.39, stage/main/update 19.48, stage/main/post_update 14.04, encode//sum 12.88, stage/render/render 11.75 |
| warm-up | 4.78 | 36.7 | pipelines/compile_ms 941, pipelines/front_ms 916, pipelines/wait_ms 184, render/thread/ms 58.39, render/busy/ms 55.56, main/busy/ms 43.27, stage/render/prepare_assets 26.98, pipelines/driver_ms 24.93, stage/main/post_update 21.63, stage/main/update 15.94, stage/render/render 14.49, encode//sum 12.89 |
| warm-up | 4.81 | 30.2 | pipelines/compile_ms 875, pipelines/front_ms 850, pipelines/wait_ms 206, render/thread/ms 58.02, render/busy/ms 55.11, main/busy/ms 49.02, stage/main/post_update 25.69, pipelines/driver_ms 24.60, stage/render/prepare_assets 20.37, stage/main/update 17.65, stage/render/render 16.48, render/graph/ms 12.99 |
| warm-up | 4.84 | 30.8 | pipelines/compile_ms 875, pipelines/front_ms 853, pipelines/wait_ms 109, main/busy/ms 108, stage/main/update 90.01, stream/drive/ms 78.82, render/thread/ms 49.26, render/busy/ms 47.11, pipelines/driver_ms 21.97, stage/render/render 18.39, render/graph/ms 15.75, stage/render/prepare_assets 13.09 |
| warm-up | 4.93 | 87.8 | pipelines/compile_ms 1278, pipelines/front_ms 1248, pipelines/wait_ms 225, main/busy/ms 97.07, stage/main/update 84.64, stream/drive/ms 78.90, render/thread/ms 44.74, render/busy/ms 42.74, pipelines/driver_ms 29.86, stage/render/render 18.06, stage/render/prepare_assets 16.13, render/graph/ms 15.64 |
| warm-up | 5.01 | 52.3 | pipelines/compile_ms 315, pipelines/front_ms 303, main/busy/ms 79.07, render/thread/ms 55.22, render/busy/ms 51.18, stage/main/update 41.57, stage/render/render 26.76, render/graph/ms 22.26, stage/render/graph_submit 14.74, encode//sum 14.62, stage/main/post_update 12.13, pipelines/driver_ms 11.52 |
| warm-up | 5.09 | 84.5 | pipelines/compile_ms 266, pipelines/front_ms 255, render/thread/ms 85.07, main/busy/ms 83.22, render/busy/ms 81.17, stage/main/update 66.32, stage/render/prepare_assets 34.07, stage/render/render 23.08, render/graph/ms 18.50, pipelines/wait_ms 18.00, encode//sum 15.07, stage/render/graph_submit 14.12 |
| warm-up | 5.14 | 50.7 | pipelines/compile_ms 1674, pipelines/front_ms 1633, render/thread/ms 92.14, render/busy/ms 90.40, main/busy/ms 69.66, stage/render/prepare_assets 46.02, pipelines/driver_ms 41.22, stage/main/update 35.82, pipelines/wait_ms 33.40, stage/main/fixed_main_loop 15.91, stage/render/render 15.71, stage/main/post_update 14.41 |
| warm-up | 5.18 | 36.7 | pipelines/compile_ms 2104, pipelines/front_ms 2047, render/thread/ms 72.02, render/busy/ms 70.19, main/busy/ms 57.76, pipelines/driver_ms 56.64, stage/render/prepare_assets 26.00, pipelines/wait_ms 23.05, stage/main/update 17.10, stage/main/fixed_main_loop 15.68, stage/main/post_update 15.19, encode//sum 13.46 |
| warm-up | 5.21 | 35.1 | pipelines/compile_ms 690, pipelines/front_ms 667, render/thread/ms 55.83, render/busy/ms 54.09, main/busy/ms 45.31, pipelines/driver_ms 23.41, stage/main/update 22.00, stage/render/prepare_assets 15.39, encode//sum 15.18, stage/render/render 12.13, stage/main/post_update 11.08, render/graph/ms 9.95 |
| warm-up | 5.28 | 43.2 | pipelines/compile_ms 323, pipelines/front_ms 312, render/thread/ms 54.41, render/busy/ms 52.45, main/busy/ms 45.21, stage/main/update 27.32, stream/drive/ms 17.09, stage/render/prepare_resources 15.00, encode//sum 13.98, stage/render/render 13.88, stage/main/post_update 12.27, stage/render/prepare_assets 11.67 |
| warm-up | 5.32 | 43.0 | pipelines/compile_ms 236, pipelines/front_ms 229, render/thread/ms 72.09, render/busy/ms 70.15, main/busy/ms 50.16, pipelines/wait_ms 28.62, stage/main/update 27.68, stage/render/prepare_assets 25.63, stage/render/prepare_resources 15.96, stage/main/post_update 15.33, encode//sum 15.13, stage/render/render 12.99 |
| warm-up | 5.37 | 49.4 | pipelines/compile_ms 964, pipelines/front_ms 937, render/thread/ms 74.71, render/busy/ms 72.78, main/busy/ms 70.50, stage/main/update 49.07, stage/render/prepare_assets 38.71, pipelines/wait_ms 31.62, pipelines/driver_ms 27.54, encode//sum 16.21, stage/render/render 15.03, stage/main/post_update 13.15 |
| warm-up | 5.43 | 52.1 | pipelines/compile_ms 1856, pipelines/front_ms 1791, render/thread/ms 73.15, render/busy/ms 71.14, main/busy/ms 67.11, pipelines/driver_ms 65.02, stage/main/update 38.03, stage/render/prepare_assets 34.28, stage/main/post_update 21.58, stage/render/render 15.79, encode//sum 14.84, render/graph/ms 12.35 |
| warm-up | 5.46 | 30.6 | pipelines/compile_ms 1422, pipelines/front_ms 1367, render/thread/ms 63.20, render/busy/ms 60.75, pipelines/driver_ms 54.72, main/busy/ms 47.49, stage/render/prepare_assets 22.19, stage/main/post_update 20.68, encode//sum 20.61, stage/main/update 18.75, stage/render/render 15.55, render/graph/ms 12.66 |
| warm-up | 5.50 | 38.5 | pipelines/compile_ms 346, pipelines/front_ms 334, render/thread/ms 53.21, render/busy/ms 50.99, main/busy/ms 36.39, encode//sum 20.64, stage/render/prepare_assets 20.51, stage/main/update 19.14, stage/render/render 16.17, render/graph/ms 13.62, pipelines/driver_ms 12.06, stage/render/graph_submit 10.65 |
| warm-up | 5.53 | 31.3 | pipelines/compile_ms 78.31, pipelines/front_ms 75.64, render/thread/ms 51.62, render/busy/ms 49.98, main/busy/ms 32.30, stage/render/prepare_assets 25.47, stage/main/update 17.60, encode//sum 15.45, stage/render/render 14.65, render/graph/ms 12.56, stage/render/graph_submit 10.09, stage/main/post_update 8.08 |
| warm-up | 5.75 | 35.9 | render/thread/ms 53.09, render/busy/ms 51.90, stage/render/prepare_assets 32.90, main/busy/ms 26.32, stage/main/update 13.07, encode//sum 12.86, stage/render/render 8.89, stage/main/post_update 8.49, render/graph/ms 7.20, stage/render/graph_submit 5.47, stream/drive/ms 4.44, encode/submit 2.96 |
| warm-up | 5.80 | 33.9 | pipelines/compile_ms 284, pipelines/front_ms 275, render/thread/ms 46.84, render/busy/ms 45.36, main/busy/ms 30.17, encode//sum 18.92, stage/render/prepare_assets 17.85, stage/render/render 15.32, stage/main/update 13.22, render/graph/ms 13.05, stage/main/post_update 10.41, stage/render/graph_submit 10.11 |
| warm-up | 5.84 | 30.5 | pipelines/compile_ms 143, pipelines/front_ms 138, render/thread/ms 53.40, render/busy/ms 52.15, main/busy/ms 31.20, stage/render/prepare_assets 28.33, encode//sum 16.80, stage/main/update 14.18, stage/render/render 10.79, stage/main/post_update 9.73, render/graph/ms 8.89, stage/render/graph_submit 6.54 |
| warm-up | 5.87 | 39.5 | render/thread/ms 56.55, render/busy/ms 55.42, stage/render/prepare_assets 35.55, main/busy/ms 26.89, encode//sum 15.48, stage/main/update 12.22, stage/render/render 10.04, stage/main/post_update 9.31, render/graph/ms 8.50, stage/render/graph_submit 6.55, stream/drive/ms 5.11, encode/submit 3.37 |
| warm-up | 5.93 | 32.2 | render/thread/ms 46.26, render/busy/ms 45.03, main/busy/ms 32.69, stage/render/prepare_assets 27.61, stage/main/update 15.26, encode//sum 13.07, stage/main/post_update 11.15, stage/render/render 8.87, render/graph/ms 7.18, stage/render/graph_submit 5.30, stream/drive/ms 3.56, encode/finish_wall 2.81 |
| warm-up | 5.96 | 30.7 | render/thread/ms 54.12, render/busy/ms 52.47, main/busy/ms 39.08, stage/render/prepare_assets 27.83, stage/main/update 17.89, encode//sum 16.50, stage/main/post_update 14.93, stage/render/render 12.54, render/graph/ms 10.14, stage/render/graph_submit 7.50, stream/drive/ms 5.93, encode/submit 3.98 |
| warm-up | 5.99 | 30.8 | render/thread/ms 48.88, render/busy/ms 46.75, main/busy/ms 37.36, encode//sum 21.59, stage/main/post_update 18.01, stage/render/prepare_assets 17.74, stage/render/render 16.83, render/graph/ms 13.79, stage/main/update 12.69, stage/render/graph_submit 10.03, stream/drive/ms 6.15, encode/finish_wall 5.28 |
| warm-up | 6.03 | 37.1 | render/thread/ms 46.65, render/busy/ms 44.97, main/busy/ms 32.36, stage/render/prepare_assets 22.76, encode//sum 18.57, stage/main/post_update 14.47, stage/render/render 14.01, render/graph/ms 11.75, stage/main/update 10.98, stage/render/graph_submit 8.71, encode/finish_wall 4.56, encode/submit 4.20 |
| warm-up | 6.06 | 29.0 | render/thread/ms 45.67, render/busy/ms 43.72, main/busy/ms 41.29, stage/main/update 25.85, stage/render/prepare_assets 23.07, encode//sum 16.66, stream/drive/ms 14.81, stage/render/render 14.20, render/graph/ms 11.89, stage/render/graph_submit 9.78, stage/main/post_update 9.59, pipelines/compile_ms 8.46 |
| warm-up | 6.22 | 26.8 | main/busy/ms 33.54, render/thread/ms 27.50, render/busy/ms 26.30, stage/main/update 22.22, stream/drive/ms 15.41, encode//sum 14.98, stage/render/render 11.71, render/graph/ms 10.19, stage/render/prepare_assets 8.84, stage/render/graph_submit 7.45, stage/main/post_update 7.36, encode/submit 3.87 |
| warm-up | 6.49 | 28.7 | render/thread/ms 35.33, main/busy/ms 34.16, render/busy/ms 33.66, encode//sum 27.94, stage/render/render 21.01, render/graph/ms 18.77, stage/main/post_update 15.35, stage/render/graph_submit 14.63, stage/main/update 14.46, encode/submit 7.80, encode/finish_wall 6.98, stage/render/prepare_assets 6.36 |
| warm-up | 6.56 | 44.3 | render/thread/ms 48.52, render/busy/ms 46.87, main/busy/ms 42.26, stage/render/prepare_assets 27.70, encode//sum 22.63, stage/main/update 21.02, stage/main/post_update 14.26, stage/render/render 13.67, stream/drive/ms 12.53, render/graph/ms 11.59, stage/render/graph_submit 8.57, encode/finish_wall 5.09 |

## Run

- `D:\Gamedev\Bevyrrowind\bevyrrowind\target\profiling\orch-f123.exe` view Balmora --bench benchmarks/exterior_balmora.json --resolution 2560x1440 --bench-out target/perf\f_off\balmora@2560x1440_off=cascadeencoders_cascadehold_meshslabs-1 --bench-baseline none --off cascadeencoders,cascadehold,meshslabs
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_SUN_CASCADES=3:205
- before the run: GPU 0%, CPU 12%
- during the run: other processes 7% CPU (system minus this process)
- focus: clean
- nvidia-smi over the window (57 samples): GPU load **98%** (min 93%) · 2580 MHz (min 2580) · memory 8001 MHz · 94.0 W · 74 °C (max 76) · VRAM 5687 MiB · throttled 100% of samples
