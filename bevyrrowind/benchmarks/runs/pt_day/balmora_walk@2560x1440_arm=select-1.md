# `exterior_balmora_walk`

- bench_ground: false · build: release · camera: [-202.6, 90.2, 324.8] -> [-292.6, 2.2, 204.8] · cell: Bevyrrowind - Balmora (4 cells) · focus: clean · gpu: NVIDIA GeForce RTX 4060 Laptop GPU (Vulkan, 617.42) · off: none · prepass: false · present: AutoNoVsync · render: raster dlss=quality · resolution: 2560x1440 · rings: near None far None · tunables: far.handoff_scale=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.bias=1 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_distance_m=24 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); lod.size_cull.min_px=2 (preset: sg.view_distance high; measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level)); render.bloom=on (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.bloom.intensity=0.1 (preset: sg.post high; industry: screen-space-effects research 4.11 (bloom); ROADMAP 1.3 (temporal resolve)); render.gi.probes=on (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.ssao.quality=medium (preset: sg.lighting high; measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level); gi.md slice 1); render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value)); select.gate=on (command line: --set); sf.graphics_preset=custom (preset: derived from its members); sg.effects=high (preset: level requested); sg.lighting=high (preset: level requested); sg.post=custom (preset: derived from its members); sg.shadows=high (preset: level requested); sg.sky=high (preset: level requested); sg.textures=high (preset: level requested); sg.view_distance=high (preset: level requested); sg.water=high (preset: level requested); shadow.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.length_m=0.3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.steps=16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.contact.thickness_m=0.1 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.filter=auto (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.contact=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.count=8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01; framework default 15-25 m)); shadow.lamp.map_size_px=512 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.cascades=3 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.distance_m=205 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.overlap=0.16 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft=on (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.soft_size_texels=10 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); shadow.sun.split_lambda=0.8 (preset: sg.shadows high; measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16); sky.clouds.map_per_sample=on (preset: sg.sky high; industry: weather.md (distant weather)); surface.puddles=on (preset: sg.effects high; industry: surface-weather.md "Budget"); surface.shelter=on (preset: sg.effects high; industry: surface-weather.md "Budget"); texture.max_size_px=2048 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.layers=32 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); texture.parallax.relief_steps=5 (preset: sg.textures high; measured: docs/ledger/textures.md 2026-11-13 (cap); textures.md §9 T4d (parallax)); water.quality=high (preset: sg.water high; industry: water.md "Performance budgets")
- 6078 frames over 65.6 s · **median 10.51 ms (95 fps)** · mean 10.80 · p95 13.37 · p99 16.53 (1% low 60 fps) · worst 48.54 ms · 74 spikes over 1.5× median · 53.0 stutters/min (work 53.9/min)
- warm-up: 325 frames over 9.1 s, worst 2568.7 ms
- hitches (> 26.3 ms): **35** measured, 48 in warm-up
- timed GPU passes: 6.55 ms of a 10.80 ms mean frame (passes Bevy doesn't time, CPU and waits are the rest)
- verdict: FAIL

## Gates

- pass Median frame 10.51 ms, limit 11 ms
- **FAIL** frame_ms: 58 stutters (> 1.5x moving median of 60) in 1.09 min = 53.01/min, limit 1/min
- **FAIL** worst frame 48.54 ms, limit 33 ms

## GPU passes (Bevy-timed)

Per frame = per run x runs per frame (a time-sliced pass runs every Nth frame).

| pass | GPU ms/frame | GPU ms/run | GPU p95/run | runs/frame | CPU ms |
|---|---:|---:|---:|---:|---:|
| dlss_super_resolution | 1.848 | 2.035 | 2.068 | 0.91 | 0.178 |
| main_opaque_pass_3d | 1.469 | 1.618 | 2.034 | 0.91 | 0.068 |
| early prepass | 0.442 | 0.487 | 0.716 | 0.91 | 0.087 |
| bloom | 0.286 | 0.315 | 0.321 | 0.91 | 0.063 |
| atmosphere_luts | 0.276 | 0.304 | 0.366 | 0.91 | 0.003 |
| water_velocity | 0.254 | 0.280 | 0.347 | 0.91 | 0.002 |
| water_sspr | 0.183 | 0.202 | 0.204 | 0.91 | 0.002 |
| ssao | 0.171 | 0.189 | 0.193 | 0.91 | 0.007 |
| auxiliary_views | 0.165 | 0.182 | 0.591 | 0.91 | 0.181 |
| atmos_froxel_inject | 0.132 | 0.146 | 0.147 | 0.91 | 0.001 |
| clouds_trace | 0.128 | 0.141 | 0.233 | 0.91 | 0.001 |
| water_fft | 0.119 | 0.131 | 0.138 | 0.91 | 0.003 |
| uniform_allocation | 0.118 | 0.130 | 0.184 | 0.91 | 0.025 |
| upscaling | 0.106 | 0.117 | 0.124 | 0.91 | 0.004 |
| clouds_composite | 0.096 | 0.106 | 0.109 | 0.91 | 0.002 |
| bin_unpacking | 0.095 | 0.105 | 0.141 | 0.91 | 0.018 |
| tonemapping | 0.077 | 0.085 | 0.089 | 0.91 | 0.004 |
| main_transmissive_pass_3d | 0.077 | 0.085 | 0.178 | 0.91 | 0.009 |
| clustering | 0.058 | 0.064 | 0.068 | 0.91 | 0.019 |
| render_sky | 0.047 | 0.052 | 0.053 | 0.91 | 0.002 |
| clouds_resolve | 0.044 | 0.049 | 0.049 | 0.91 | 0.001 |
| surface_exposure | 0.040 | 0.044 | 0.045 | 0.90 | 0.001 |
| lightprobe_irradiance_map | 0.038 | 0.413 | 0.416 | 0.09 | 0.001 |
| surface_trail | 0.036 | 0.056 | 0.057 | 0.63 | 0.002 |
| surface_exposure_far | 0.034 | 0.042 | 0.045 | 0.81 | 0.002 |
| clouds_shadow | 0.028 | 0.031 | 0.034 | 0.91 | 0.003 |
| early_mesh_preprocessing | 0.028 | 0.030 | 0.047 | 0.91 | 0.005 |
| atmos_froxel_integrate | 0.027 | 0.030 | 0.032 | 0.91 | 0.001 |
| water_snapshot | 0.022 | 0.025 | 0.028 | 0.91 | 0.002 |
| lightprobe_radiance_map | 0.020 | 0.219 | 0.229 | 0.09 | 0.004 |
| water_mips | 0.020 | 0.022 | 0.023 | 0.91 | 0.002 |
| night_sky | 0.013 | 0.014 | 0.034 | 0.91 | 0.002 |
| main_transparent_pass_3d | 0.011 | 0.012 | 0.027 | 0.91 | 0.124 |
| clouds_fog_light | 0.011 | 0.012 | 0.012 | 0.91 | 0.002 |
| per_view_shadow_pass | 0.010 | 0.011 | 0.022 | 0.91 | 0.005 |
| atmosphere_environment | 0.009 | 0.010 | 0.011 | 0.91 | 0.003 |
| sparse buffer updates | 0.006 | 0.006 | 0.007 | 0.90 | 0.003 |
| lightprobe_copy | 0.001 | 0.013 | 0.013 | 0.09 | 0.004 |
| lightprobe_downsampling_first_pass | 0.001 | 0.013 | 0.013 | 0.09 | 0.001 |
| lightprobe_downsampling_second_pass | 0.001 | 0.010 | 0.011 | 0.09 | 0.001 |
| blas_build | 0.000 | 0.001 | 0.002 | 0.02 | 0.006 |

## Per-frame costs (ms)

| counter | mean | p95 | max | total |
|---|---:|---:|---:|---:|
| render/thread/ms | 9.430 | 11.875 | 41.59 | 57317.4 |
| render/busy/ms | 8.878 | 11.331 | 40.96 | 53962.1 |
| main/busy/ms | 8.687 | 10.433 | 26.02 | 52797.3 |
| stage/render/render | 5.559 | 7.241 | 25.59 | 33789.8 |
| stage/main/post_update | 5.461 | 6.834 | 10.38 | 33193.4 |
| render/graph/ms | 4.814 | 6.471 | 24.89 | 29260.2 |
| encode//sum | 3.889 | 5.319 | 26.13 | 23636.0 |
| stage/render/graph_submit | 3.589 | 4.854 | 22.87 | 21816.8 |
| encode/submit | 2.293 | 3.249 | 6.29 | 13935.8 |
| stage/main/update | 1.961 | 2.380 | 19.48 | 11916.9 |
| encode/finish_wall | 1.372 | 1.982 | 19.12 | 8338.1 |
| stage/render/camera_driver | 1.171 | 1.686 | 20.25 | 7115.6 |
| encode/pass:shadow_point_light/sum | 1.021 | 3.459 | 16.47 | 6207.1 |
| encode/pass:shadow_directional_light/sum | 0.836 | 1.522 | 2.96 | 5083.1 |
| stage/render/prepare_write_phase_buffers | 0.774 | 1.111 | 2.29 | 4702.1 |
| encode/pass:main_opaque_pass_3d/sum | 0.770 | 1.091 | 1.92 | 4680.2 |
| encode/pass:early prepass/sum | 0.738 | 1.075 | 1.74 | 4482.5 |
| stage/render/prepare_assets | 0.563 | 0.735 | 11.22 | 3421.3 |
| bigspace/total/ms | 0.505 | 1.235 | 2.41 | 3072.0 |
| render/present/ms | 0.458 | 1.165 | 2.43 | 2782.6 |
| stage/render/prepare_resources | 0.445 | 0.531 | 1.82 | 2702.8 |
| stage/render/prepare_meshes | 0.434 | 0.545 | 8.13 | 2640.2 |
| stage/main/first | 0.395 | 0.522 | 6.29 | 2400.3 |
| stage/main/pre_update | 0.359 | 0.518 | 1.21 | 2181.4 |
| stage/render/specialize | 0.310 | 0.398 | 11.81 | 1883.4 |
| stage/render/queue | 0.280 | 0.411 | 7.72 | 1704.3 |
| stage/render/prepare_bind_groups | 0.280 | 0.496 | 2.15 | 1699.4 |
| controller/feed/ms | 0.268 | 0.336 | 1.05 | 1626.5 |
| encode/pass:main_transparent_pass_3d/sum | 0.237 | 0.352 | 0.95 | 1440.6 |
| stage/main/fixed_main_loop | 0.216 | 0.373 | 1.31 | 1310.6 |
| bigspace/low_precision/ms | 0.209 | 0.594 | 1.96 | 1271.9 |
| stream/drive/ms | 0.204 | 0.286 | 17.46 | 1241.2 |
| stage/render/create_views | 0.196 | 0.254 | 0.99 | 1190.8 |
| stage/render/prepare_batch_phases | 0.189 | 0.270 | 1.08 | 1148.7 |
| stage/render/extract_commands | 0.172 | 0.200 | 0.48 | 1045.3 |
| stage/render/prepare_flush | 0.163 | 0.303 | 1.07 | 991.8 |
| anim/evaluate/ms | 0.158 | 0.249 | 2.14 | 962.2 |
| anim/evaluate_cpu/ms | 0.157 | 0.196 | 0.59 | 952.5 |
| stream/tick/ms | 0.148 | 0.214 | 0.54 | 899.5 |
| controller/move/ms | 0.142 | 0.290 | 0.64 | 860.9 |
| stage/render/prepare_views | 0.138 | 0.171 | 0.36 | 837.2 |
| bigspace/high_precision/ms | 0.136 | 0.516 | 1.63 | 826.4 |
| stage/main/last | 0.117 | 0.161 | 2.59 | 713.1 |
| encode/pass:main_transmissive_pass_3d/sum | 0.098 | 0.159 | 0.75 | 593.5 |
| render/acquire/ms | 0.094 | 0.121 | 0.33 | 572.7 |
| stage/render/prepare_collect_phase_buffers | 0.059 | 0.127 | 0.33 | 357.1 |
| terrain/update/ms | 0.057 | 0.286 | 1.71 | 346.0 |
| pipelines/compile_ms | 0.049 | 0.000 | 126.80 | 297.2 |
| physics/step/ms | 0.047 | 0.088 | 0.57 | 287.0 |
| pipelines/front_ms | 0.047 | 0.000 | 122.89 | 286.0 |
| physics/insert/ms | 0.032 | 0.360 | 0.80 | 192.7 |
| encode/resolve_diagnostics/sum | 0.032 | 0.043 | 0.86 | 192.1 |
| stage/render/cleanup | 0.031 | 0.091 | 0.77 | 191.1 |
| stage/render/phase_sort | 0.030 | 0.051 | 0.24 | 179.9 |
| weather/map/ms | 0.025 | 0.032 | 0.07 | 149.5 |
| terrain/select/ms | 0.024 | 0.255 | 0.83 | 143.1 |
| far/select/ms | 0.016 | 0.167 | 0.62 | 97.2 |
| encode/camera_driver_span_begin/sum | 0.015 | 0.022 | 0.56 | 91.8 |
| anim/apply/ms | 0.012 | 0.023 | 0.29 | 73.0 |
| encode/camera_driver_span_end/sum | 0.012 | 0.020 | 0.59 | 70.2 |
| controller/avoid/ms | 0.011 | 0.015 | 0.09 | 66.4 |
| weather/eval/ms | 0.010 | 0.014 | 0.05 | 61.2 |
| script/update/ms | 0.007 | 0.009 | 0.23 | 41.9 |
| script/input/ms | 0.005 | 0.006 | 0.08 | 31.8 |
| stream/despawn/ms | 0.005 | 0.000 | 1.20 | 30.6 |
| stream/spawn/ms | 0.005 | 0.000 | 1.07 | 28.7 |
| script/ui/ms | 0.004 | 0.005 | 0.03 | 25.6 |
| controller/write/ms | 0.003 | 0.005 | 0.03 | 20.8 |
| stream/deactivate/ms | 0.002 | 0.000 | 0.67 | 14.5 |
| pipelines/driver_ms | 0.002 | 0.000 | 3.91 | 11.2 |
| weather/fx/ms | 0.001 | 0.001 | 0.02 | 5.7 |
| stream/activate/ms | 0.001 | 0.000 | 0.51 | 5.1 |
| shapes/activate/ms | 0.001 | 0.001 | 0.01 | 5.0 |
| stage/render/prepare | 0.001 | 0.003 | 0.05 | 3.2 |
| weather/apply/ms | 0.000 | 0.001 | 0.02 | 2.2 |
| stream/upload/ms | 0.000 | 0.000 | 0.07 | 1.0 |
| surface/cell_upload_ms | 0.000 | 0.000 | 0.04 | 0.2 |
| pipelines/wait_ms | 0.000 | 0.000 | 0.13 | 0.2 |
| pipelines/lock_ms | 0.000 | 0.000 | 0.12 | 0.1 |
| surface/advance_cpu_ms | 0.000 | 0.000 | 0.00 | 0.0 |

## Gauges (counts, sizes)

| gauge | unit | median | max | last |
|---|---|---:|---:|---:|
| anim/a0/count | count | 0 | 2 | 0 |
| anim/a1/count | count | 0 | 3 | 0 |
| anim/a2/count | count | 1 | 9 | 0 |
| anim/a3/count | count | 565 | 579 | 578 |
| anim/characters/count | count | 568 | 579 | 578 |
| anim/evaluated/count | count | 20 | 34 | 20 |
| controller/movers/count | count | 20 | 22 | 20 |
| crowd/c0/count | count | 26 | 42 | 14 |
| crowd/c0/joints/count | count | 1292 | 2218 | 641 |
| crowd/c0/meshes/count | count | 352 | 633 | 174 |
| crowd/c1/count | count | 146 | 155 | 147 |
| crowd/c1/joints/count | count | 5531 | 5936 | 5555 |
| crowd/c1/meshes/count | count | 1166 | 1268 | 1167 |
| crowd/c2/count | count | 386 | 417 | 417 |
| crowd/c2/joints/count | count | 6398 | 6904 | 6904 |
| crowd/c2/meshes/count | count | 1820 | 1980 | 1980 |
| crowd/hidden/count | count | 0 | 0 | 0 |
| crowd/joint_blocks/count | count | 200 | 250 | 238 |
| crowd/joints_extracted/count | count | 4604 | 10.2 k | 6528 |
| crowd/joints_uploaded/count | count | 4582 | 6835 | 6528 |
| crowd/pinned/count | count | 0 | 0 | 0 |
| ecs/entities | count | 59.3 k | 68.3 k | 59.3 k |
| encode//max | ms | 0.41 | 18.94 | 0.45 |
| encode/camera_driver_span_begin/max | ms | 0.02 | 0.56 | 0.02 |
| encode/camera_driver_span_end/max | ms | 0.01 | 0.59 | 0.01 |
| encode/finish_critical | ms | 0.96 | 18.94 | 1.13 |
| encode/pass:early prepass/max | ms | 0.76 | 1.74 | 0.74 |
| encode/pass:main_opaque_pass_3d/max | ms | 0.78 | 1.92 | 1.13 |
| encode/pass:main_transmissive_pass_3d/max | ms | 0.09 | 0.75 | 0.09 |
| encode/pass:main_transparent_pass_3d/max | ms | 0.22 | 0.95 | 0.29 |
| encode/pass:shadow_directional_light/max | ms | 0.79 | 2.96 | 0.74 |
| encode/pass:shadow_point_light/max | ms | 0.81 | 1.77 | 0.85 |
| encode/resolve_diagnostics/max | ms | 0.03 | 0.86 | 0.03 |
| far/entities | count | 2681 | 2681 | 2681 |
| far/forced | count | 0 | 0 | 0 |
| far/impostors | count | 61.0 k | 61.0 k | 61.0 k |
| far/lights | count | 7115 | 7115 | 7115 |
| far/nodes/l1 | count | 1100 | 1100 | 1100 |
| far/nodes/l2 | count | 25 | 25 | 25 |
| far/nodes/l3 | count | 4 | 4 | 4 |
| far/resident/bytes | bytes | 203.19 M | 203.19 M | 203.19 M |
| far/triangles | count | 5.03 M | 5.03 M | 5.03 M |
| frame/work/ms | ms | 9.02 | 40.96 | 8.29 |
| gpu/timed/ms | ms | 7.97 | 9.95 | 8.15 |
| material_allocator_allocations/bevy_pbr::pbr_material::StandardMaterial |  materials | 2613 | 2670 | 2670 |
| material_allocator_slabs/bevy_pbr::pbr_material::StandardMaterial |  slabs | 1 | 1 | 1 |
| material_allocator_slabs_size/bevy_pbr::pbr_material::StandardMaterial |  bytes | 289.2 k | 289.2 k | 289.2 k |
| materials/normal_no_tangent | count | 0 | 0 | 0 |
| materials/normal_no_uv0 | count | 0 | 0 | 0 |
| mesh_allocator_allocations |  meshes | 15.4 k | 16.6 k | 15.6 k |
| mesh_allocator_slabs |  slabs | 16 | 16 | 16 |
| mesh_allocator_slabs_size |  bytes | 555.78 M | 555.78 M | 555.78 M |
| phases/alpha_mask3d/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d/items | count | 810 | 1417 | 617 |
| phases/alpha_mask3d/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d/unbatchable | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/batch_sets | count | 7 | 8 | 7 |
| phases/alpha_mask3d_prepass/bins | count | 195 | 405 | 397 |
| phases/alpha_mask3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/items | count | 810 | 1417 | 617 |
| phases/alpha_mask3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/alpha_mask3d_prepass/pipelines | count | 6 | 7 | 6 |
| phases/alpha_mask3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d/batch_sets | count | 20 | 21 | 20 |
| phases/opaque3d/bins | count | 1286 | 1394 | 1352 |
| phases/opaque3d/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d/items | count | 3397 | 5144 | 3189 |
| phases/opaque3d/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d/pipelines | count | 19 | 20 | 19 |
| phases/opaque3d/unbatchable | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/batch_sets | count | 16 | 18 | 16 |
| phases/opaque3d_prepass/bins | count | 1286 | 1394 | 1352 |
| phases/opaque3d_prepass/instance_batches | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_draws | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/instance_slots | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/items | count | 3397 | 5144 | 3189 |
| phases/opaque3d_prepass/non_mesh | count | 0 | 0 | 0 |
| phases/opaque3d_prepass/pipelines | count | 15 | 17 | 15 |
| phases/opaque3d_prepass/unbatchable | count | 0 | 0 | 0 |
| phases/shadow/batch_sets | count | 23 | 202 | 4 |
| phases/shadow/bins | count | 5060 | 8858 | 2429 |
| phases/shadow/instance_batches | count | 0 | 0 | 0 |
| phases/shadow/instance_draws | count | 0 | 0 | 0 |
| phases/shadow/instance_slots | count | 0 | 0 | 0 |
| phases/shadow/items | count | 11.4 k | 26.2 k | 6382 |
| phases/shadow/non_mesh | count | 0 | 0 | 0 |
| phases/shadow/pipelines | count | 205 | 252 | 29 |
| phases/shadow/unbatchable | count | 0 | 0 | 0 |
| physics/awake | count | 0 | 0 | 0 |
| physics/bodies | count | 2644 | 3268 | 2349 |
| physics/items | count | 222 | 476 | 19 |
| physics/pending | count | 0 | 191 | 0 |
| physics/terrain/chunks | count | 72 | 76 | 64 |
| pipelines/total | count | 329 | 330 | 330 |
| pipelines/waiting | count | 0 | 2 | 0 |
| precache/draw_waiting | count | 0 | 2 | 0 |
| precache/limit | count | 1 | 1 | 1 |
| precache/outstanding | count | 0 | 0 | 0 |
| precache/pending_assets | count | 0 | 0 | 0 |
| precache/throttled | count | 0 | 0 | 0 |
| preprocess/bind_groups | count | 11 | 565 | 6 |
| preprocess/bind_groups_created | count | 11 | 565 | 6 |
| preprocess/bind_groups_materialize | count | 0 | 0 | 0 |
| preprocess/bind_groups_ranges | count | 0 | 0 | 0 |
| preprocess/bind_groups_reused | count | 131 | 270 | 123 |
| process/cpu_usage | % | 26.44 | 40.58 | 23.08 |
| process/mem_usage | GiB | 5.49 | 5.66 | 5.66 |
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
| skin/blocks_changed/count | count | 14 | 130 | 15 |
| skin/blocks_skipped/count | count | 0 | 0 | 0 |
| skin/blocks_walked/count | count | 200 | 250 | 238 |
| skin/joint_bytes_written/bytes | bytes | 363.4 k | 809.5 k | 492.3 k |
| skin/joints_written/count | count | 5678 | 12.6 k | 7692 |
| skin/upload_writes/count | count | 1 | 2 | 1 |
| started_load_count |  | 4919 | 5012 | 5012 |
| statics/bindless | count | 1 | 1 | 1 |
| statics/material_bind_groups | count | 1 | 1 | 1 |
| statics/material_classes | count | 2 | 2 | 2 |
| statics/material_keys | count | 3 | 3 | 3 |
| statics/material_pipelines | count | 3 | 3 | 3 |
| statics/materials | count | 433 | 528 | 396 |
| statics/materials_bound | count | 433 | 528 | 396 |
| statics/materials_unclassed | count | 0 | 0 | 0 |
| stream/activate/queue | count | 0 | 1 | 0 |
| stream/assets/queue | count | 0 | 4 | 0 |
| stream/deactivate/queue | count | 0 | 1 | 0 |
| stream/despawn/queue | count | 0 | 2 | 0 |
| stream/fetch/queue | count | 0 | 3 | 0 |
| stream/latency/s | s | 0.03 | 0.09 | 0.03 |
| stream/model_queue | count | 0 | 45 | 0 |
| stream/models | count | 336 | 429 | 253 |
| stream/oldest_request/s | s | 0 | 0.30 | 0 |
| stream/page_cache/bytes | bytes | 23.68 M | 23.68 M | 23.68 M |
| stream/parked/entities | count | 0 | 0 | 0 |
| stream/parked/worldspaces | count | 0 | 0 | 0 |
| stream/requests | count | 0 | 4 | 0 |
| stream/resident_cells | count | 1128 | 1135 | 1124 |
| stream/spawn/queue | count | 0 | 1 | 0 |
| stream/upload/queue | count | 0 | 0 | 0 |
| system/cpu_usage | % | 31.91 | 44.23 | 29.10 |
| system/mem_usage | % | 54.53 | 55.22 | 55.08 |
| terrain/patches | count | 511 | 523 | 514 |
| textures/gpu_mib | MiB | 2901 | 3012 | 3012 |
| textures/images | count | 2682 | 2735 | 2735 |
| views/all/batch_sets | count | 72 | 251 | 54 |
| views/all/bins | count | 7608 | 11.7 k | 5927 |
| views/all/count | count | 46 | 56 | 4 |
| views/all/instance_batches | count | 0 | 0 | 0 |
| views/all/instance_draws | count | 0 | 0 | 0 |
| views/all/instance_slots | count | 0 | 0 | 0 |
| views/all/items | count | 20.4 k | 38.3 k | 14.0 k |
| views/all/pipelines | count | 252 | 299 | 75 |
| views/all/pipelines_distinct | count | 71 | 74 | 61 |
| views/all/work_items | count | 10.7 k | 35.7 k | 8121 |
| views/cascade/batch_sets | count | 9 | 53 | 4 |
| views/cascade/bins | count | 2804 | 6183 | 2429 |
| views/cascade/count | count | 3 | 7 | 3 |
| views/cascade/instance_batches | count | 0 | 0 | 0 |
| views/cascade/instance_draws | count | 0 | 0 | 0 |
| views/cascade/instance_slots | count | 0 | 0 | 0 |
| views/cascade/items | count | 8898 | 23.2 k | 6382 |
| views/cascade/pipelines | count | 36 | 53 | 29 |
| views/cascade/work_items | count | 413 | 22.8 k | 201 |
| views/lamp_face/batch_sets | count | 0 | 193 | 0 |
| views/lamp_face/bins | count | 2189 | 2889 | 0 |
| views/lamp_face/count | count | 42 | 48 | 0 |
| views/lamp_face/culled | count | 0 | 0 | 0 |
| views/lamp_face/drawn | count | 0 | 48 | 0 |
| views/lamp_face/instance_batches | count | 0 | 0 | 0 |
| views/lamp_face/instance_draws | count | 0 | 0 | 0 |
| views/lamp_face/instance_slots | count | 0 | 0 | 0 |
| views/lamp_face/items | count | 2450 | 3212 | 0 |
| views/lamp_face/pipelines | count | 169 | 202 | 0 |
| views/lamp_face/revealed | count | 0 | 0 | 0 |
| views/lamp_face/work_items | count | 0 | 2923 | 0 |
| views/main/batch_sets | count | 50 | 53 | 50 |
| views/main/bins | count | 2866 | 3562 | 3498 |
| views/main/count | count | 1 | 1 | 1 |
| views/main/instance_batches | count | 0 | 0 | 0 |
| views/main/instance_draws | count | 0 | 0 | 0 |
| views/main/instance_slots | count | 0 | 0 | 0 |
| views/main/items | count | 8403 | 13.1 k | 7612 |
| views/main/pipelines | count | 47 | 49 | 46 |
| views/main/work_items | count | 8568 | 13.3 k | 7920 |
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
| anim/bones_written/count | count | 62.62 | 294 | 380.6 k |
| anim/events/count | count | 0.08 | 3 | 516 |
| assets/Image/added | count | 0.10 | 33 | 592 |
| assets/Image/modified | count | 0.00 | 3 | 18 |
| assets/Image/removed | count | 0.01 | 6 | 44 |
| assets/Mesh/added | count | 0.67 | 201 | 4061 |
| assets/Mesh/removed | count | 0.80 | 1212 | 4842 |
| assets/StandardMaterial/added | count | 0.08 | 35 | 464 |
| assets/StandardMaterial/modified | count | 3.56 | 11 | 21.6 k |
| assets/StandardMaterial/removed | count | 0.07 | 38 | 408 |
| assets/WorldAsset/added | count | 0.00 | 4 | 28 |
| assets/WorldAsset/removed | count | 0.01 | 8 | 86 |
| draws/skipped_late | count | 0 | 0 | 0 |
| ecs/entities_delta | count | 0.77 | 1180 | 4675 |
| encode//count | count | 34.71 | 220 | 210.9 k |
| encode/bind_walk/init_textures | count | 13.3 k | 114.7 k | 80.71 M |
| encode/bind_walk/init_textures_skipped | count | 68.2 k | 151.8 k | 414.35 M |
| encode/bind_walk/inits | count | 465 | 1130 | 2.83 M |
| encode/bind_walk/inits_skipped | count | 111 | 166 | 673.5 k |
| encode/bind_walk/merge_textures | count | 13.2 k | 114.6 k | 80.27 M |
| encode/bind_walk/merge_textures_skipped | count | 5366 | 6012 | 32.61 M |
| encode/bind_walk/merges | count | 101 | 335 | 611.4 k |
| encode/bind_walk/merges_skipped | count | 85.37 | 131 | 518.9 k |
| encode/camera_driver_span_begin/count | count | 1 | 1 | 6078 |
| encode/camera_driver_span_end/count | count | 1 | 1 | 6078 |
| encode/pass:early prepass/count | count | 1 | 1 | 6078 |
| encode/pass:main_opaque_pass_3d/count | count | 1 | 1 | 6078 |
| encode/pass:main_transmissive_pass_3d/count | count | 1 | 1 | 6078 |
| encode/pass:main_transparent_pass_3d/count | count | 1 | 1 | 6078 |
| encode/pass:shadow_directional_light/count | count | 1 | 1 | 6078 |
| encode/pass:shadow_point_light/count | count | 1.69 | 48 | 10.3 k |
| encode/queue/command_buffers | count | 46.41 | 278 | 282.1 k |
| encode/queue/submits | count | 2.02 | 4 | 12.3 k |
| encode/queue/write_buffer/bytes | bytes | 1.88 M | 4.98 M | 11.41 G |
| encode/queue/write_buffer/calls | count | 266 | 843 | 1.61 M |
| encode/queue/write_buffer_with/bytes | bytes | 177.5 k | 3.52 M | 1.08 G |
| encode/queue/write_buffer_with/calls | count | 10.99 | 612 | 66.8 k |
| encode/queue/write_texture/bytes | bytes | 11.8 k | 8.39 M | 71.91 M |
| encode/queue/write_texture/calls | count | 0.09 | 42 | 564 |
| encode/resolve_diagnostics/count | count | 1 | 1 | 6078 |
| physics/inserts | count | 0.37 | 109 | 2260 |
| physics/rebases | count | 0 | 0 | 0 |
| physics/removals | count | 0.38 | 175 | 2335 |
| physics/steps | count | 0.65 | 3 | 3939 |
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
| stream/deactivate/items | count | 0.04 | 35 | 223 |
| stream/despawn/items | count | 0.04 | 6 | 222 |
| stream/fetch/items | count | 0.01 | 3 | 83 |
| stream/spawn/items | count | 0.06 | 20 | 378 |
| stream/upload/bytes | bytes | 5570 | 2.83 M | 33.85 M |
| stream/upload/items | count | 0.01 | 2 | 57 |
| surface/active_cells | count | 0 | 0 | 0 |
| surface/shelter_far_redraws | count | 0.00 | 1 | 29 |
| surface/shelter_redraws | count | 0.01 | 1 | 53 |
| surface/steps | count | 0 | 0 | 0 |
| terrain/upload/bytes | bytes | 1221 | 573.4 k | 7.42 M |
| weather/strikes | count | 0 | 0 | 0 |
| window/hidden | count | 0 | 0 | 0 |
| window/unfocused | count | 0 | 0 | 0 |
| world/origin_shifts | count | 0 | 0 | 0 |

## Hitches (> 26.3 ms; slowest 64, time order; churn = this frame + the one before)

| phase | t s | frame ms | churn |
|---|---:|---:|---|
| warm-up | 2.57 | 2568.7 | main/busy/ms 2538, stage/main/post_update 2433, stage/main/update 95.82, surface/advance_cpu_ms 66.84, stage/main/pre_update 4.92, stage/main/last 1.19, surface/cell_upload_ms 1.02, stream/drive/ms 0.46, stage/main/first 0.46, stage/main/fixed_main_loop 0.38, weather/map/ms 0.16, bigspace/total/ms 0.10 |
| warm-up | 3.36 | 788.4 | main/busy/ms 2682, stage/main/post_update 2457, render/thread/ms 753, render/busy/ms 620, stage/render/prepare_assets 568, stage/main/update 210, stage/render/prepare_views 130, render/acquire/ms 130, surface/advance_cpu_ms 66.84, stage/render/create_views 29.46, stage/render/render 13.45, render/graph/ms 8.50 |
| warm-up | 3.47 | 111.2 | render/thread/ms 846, render/busy/ms 662, stage/render/prepare_assets 596, main/busy/ms 164, stage/render/prepare_views 130, render/acquire/ms 130, stage/main/update 119, pipelines/compile_ms 100, stage/render/render 69.93, pipelines/front_ms 59.97, render/present/ms 54.66, pipelines/driver_ms 40.29 |
| warm-up | 3.51 | 44.6 | pipelines/compile_ms 439, pipelines/front_ms 359, pipelines/wait_ms 174, render/thread/ms 132, stage/render/render 88.93, pipelines/driver_ms 79.53, render/busy/ms 66.06, render/present/ms 65.74, main/busy/ms 64.18, stage/main/post_update 48.43, stage/render/prepare_assets 28.74, terrain/update/ms 23.33 |
| warm-up | 3.55 | 41.2 | pipelines/wait_ms 1219, pipelines/compile_ms 662, pipelines/front_ms 603, main/busy/ms 80.08, render/thread/ms 76.57, stage/main/post_update 67.51, render/busy/ms 59.56, pipelines/driver_ms 58.57, stage/render/render 36.90, render/graph/ms 20.38, render/present/ms 15.57, terrain/update/ms 12.65 |
| warm-up | 3.65 | 92.0 | pipelines/wait_ms 5041, pipelines/compile_ms 1102, pipelines/front_ms 1036, render/thread/ms 122, main/busy/ms 119, render/busy/ms 118, pipelines/driver_ms 66.16, stage/main/update 55.72, stage/main/post_update 49.85, stage/render/prepare_assets 44.10, stage/render/prepare_resources 20.84, stage/render/prepare_write_phase_buffers 17.69 |
| warm-up | 3.72 | 76.9 | pipelines/wait_ms 4217, pipelines/compile_ms 1581, pipelines/front_ms 1497, render/thread/ms 158, render/busy/ms 154, main/busy/ms 112, stage/render/prepare_assets 92.61, pipelines/driver_ms 84.44, stage/main/update 60.68, stage/main/post_update 27.34, stage/render/prepare_write_phase_buffers 18.08, stage/main/pre_update 16.05 |
| warm-up | 3.76 | 33.7 | pipelines/compile_ms 1326, pipelines/front_ms 1269, pipelines/wait_ms 352, render/thread/ms 105, render/busy/ms 103, stage/render/prepare_assets 77.82, pipelines/driver_ms 57.11, main/busy/ms 50.48, stage/main/post_update 17.23, stage/main/update 17.16, terrain/update/ms 10.03, stage/render/render 9.30 |
| warm-up | 3.81 | 50.1 | pipelines/compile_ms 1104, pipelines/front_ms 1056, pipelines/wait_ms 327, render/thread/ms 79.85, render/busy/ms 67.26, main/busy/ms 59.72, pipelines/driver_ms 47.64, stage/render/prepare_assets 42.72, stage/main/post_update 32.02, stage/main/update 16.99, stage/render/render 10.41, stage/render/prepare_views 10.17 |
| warm-up | 3.84 | 31.8 | pipelines/compile_ms 829, pipelines/front_ms 792, pipelines/wait_ms 257, render/thread/ms 78.67, main/busy/ms 67.89, render/busy/ms 65.11, pipelines/driver_ms 36.40, stage/main/post_update 35.65, stage/render/prepare_assets 27.95, stage/main/update 21.50, encode//sum 18.47, stage/render/render 18.10 |
| warm-up | 3.87 | 31.5 | pipelines/compile_ms 686, pipelines/front_ms 664, pipelines/wait_ms 213, render/thread/ms 59.79, render/busy/ms 57.03, main/busy/ms 41.02, stage/render/prepare_assets 22.45, pipelines/driver_ms 21.64, encode//sum 18.97, stage/main/post_update 18.64, stage/main/update 17.67, stage/render/render 17.04 |
| warm-up | 3.91 | 41.2 | pipelines/compile_ms 930, pipelines/front_ms 888, pipelines/wait_ms 153, render/thread/ms 68.79, render/busy/ms 66.96, main/busy/ms 66.95, stage/main/update 52.92, stream/drive/ms 48.69, pipelines/driver_ms 42.44, stage/render/prepare_assets 39.24, stage/render/render 12.90, render/graph/ms 10.71 |
| warm-up | 3.96 | 52.8 | pipelines/compile_ms 1769, pipelines/front_ms 1689, pipelines/driver_ms 80.34, render/thread/ms 64.75, main/busy/ms 64.14, render/busy/ms 63.14, stage/main/update 52.40, stream/drive/ms 47.76, stage/render/prepare_assets 38.04, pipelines/wait_ms 14.07, stage/render/render 11.95, render/graph/ms 10.09 |
| warm-up | 4.00 | 32.9 | pipelines/compile_ms 1397, pipelines/front_ms 1340, pipelines/driver_ms 56.49, render/thread/ms 53.07, render/busy/ms 51.49, stage/render/prepare_assets 34.59, main/busy/ms 25.66, pipelines/wait_ms 17.97, stage/main/update 13.01, stage/render/render 10.53, stream/drive/ms 8.54, render/graph/ms 8.31 |
| warm-up | 4.05 | 32.5 | pipelines/compile_ms 244, pipelines/front_ms 234, render/thread/ms 41.16, render/busy/ms 39.43, stage/render/prepare_assets 20.89, main/busy/ms 16.70, stage/render/render 10.99, pipelines/driver_ms 10.48, render/graph/ms 8.79, stage/main/update 6.72, encode//sum 6.34, stage/render/graph_submit 6.32 |
| warm-up | 4.08 | 37.4 | pipelines/compile_ms 460, pipelines/front_ms 444, render/thread/ms 61.98, render/busy/ms 60.60, main/busy/ms 38.50, stage/render/prepare_assets 25.01, stage/main/post_update 20.31, stage/render/prepare_resources 16.38, pipelines/driver_ms 15.82, stage/main/update 12.20, stage/render/render 11.46, render/graph/ms 9.47 |
| warm-up | 4.14 | 54.9 | pipelines/compile_ms 491, pipelines/front_ms 471, render/thread/ms 74.32, main/busy/ms 73.88, render/busy/ms 72.88, stage/main/update 35.67, stage/render/prepare_assets 27.66, stage/main/post_update 23.72, stage/render/prepare_resources 21.42, pipelines/driver_ms 19.77, stage/render/render 10.83, render/graph/ms 8.92 |
| warm-up | 4.20 | 65.4 | pipelines/compile_ms 1261, pipelines/front_ms 1219, pipelines/wait_ms 94.35, render/thread/ms 81.32, render/busy/ms 79.80, main/busy/ms 71.46, stage/render/prepare_assets 43.60, pipelines/driver_ms 41.92, stage/main/update 35.72, stage/main/post_update 21.92, stage/render/render 12.13, render/graph/ms 9.96 |
| warm-up | 4.28 | 80.0 | pipelines/compile_ms 2121, pipelines/front_ms 2029, pipelines/wait_ms 158, render/thread/ms 119, render/busy/ms 117, pipelines/driver_ms 92.70, main/busy/ms 53.67, stage/render/prepare_assets 49.80, encode//sum 37.76, stage/render/render 37.08, render/graph/ms 34.53, stage/main/update 26.69 |
| warm-up | 4.33 | 49.9 | pipelines/compile_ms 1208, pipelines/front_ms 1136, render/thread/ms 118, render/busy/ms 115, encode//sum 77.00, pipelines/wait_ms 74.73, pipelines/driver_ms 71.82, stage/render/render 54.36, render/graph/ms 51.55, main/busy/ms 37.53, stage/render/graph_submit 32.18, stage/render/prepare_assets 31.74 |
| warm-up | 4.42 | 87.4 | pipelines/compile_ms 1499, pipelines/front_ms 1414, pipelines/wait_ms 152, render/thread/ms 118, render/busy/ms 116, pipelines/driver_ms 84.17, encode//sum 76.91, stage/render/render 45.55, main/busy/ms 44.87, render/graph/ms 42.39, stage/render/graph_submit 33.79, stage/main/update 24.41 |
| warm-up | 4.46 | 41.8 | pipelines/compile_ms 1611, pipelines/front_ms 1532, pipelines/wait_ms 152, render/thread/ms 104, render/busy/ms 102, pipelines/driver_ms 79.64, main/busy/ms 53.38, encode//sum 36.91, stage/main/update 27.88, stage/render/render 26.14, render/graph/ms 23.24, stage/render/prepare_assets 21.94 |
| warm-up | 4.53 | 65.4 | pipelines/compile_ms 1646, pipelines/front_ms 1604, main/busy/ms 105, pipelines/wait_ms 87.92, stage/main/update 83.29, render/thread/ms 69.80, render/busy/ms 68.28, stream/drive/ms 45.81, pipelines/driver_ms 42.56, stage/render/prepare_assets 27.36, stage/render/render 25.75, render/graph/ms 23.62 |
| warm-up | 4.63 | 101.9 | pipelines/compile_ms 1861, pipelines/front_ms 1813, pipelines/wait_ms 129, main/busy/ms 107, stage/main/update 88.71, render/thread/ms 88.60, render/busy/ms 87.00, stream/drive/ms 52.62, pipelines/driver_ms 48.64, stage/render/prepare_assets 45.80, stage/render/render 33.83, render/graph/ms 31.72 |
| warm-up | 4.67 | 41.2 | pipelines/compile_ms 643, pipelines/front_ms 625, main/busy/ms 88.51, render/thread/ms 84.42, render/busy/ms 82.19, stage/main/update 65.06, stream/drive/ms 57.29, pipelines/wait_ms 52.01, stage/render/prepare_assets 31.07, stage/render/render 22.25, render/graph/ms 19.45, pipelines/driver_ms 18.01 |
| warm-up | 4.78 | 105.1 | pipelines/compile_ms 1646, pipelines/front_ms 1562, pipelines/wait_ms 109, main/busy/ms 103, render/thread/ms 102, render/busy/ms 99.37, pipelines/driver_ms 84.71, stage/main/update 68.64, stream/drive/ms 53.59, stage/render/prepare_assets 34.92, stage/main/post_update 21.89, stage/render/render 19.90 |
| warm-up | 4.84 | 67.3 | pipelines/compile_ms 2468, pipelines/front_ms 2360, render/thread/ms 118, render/busy/ms 115, pipelines/wait_ms 110, pipelines/driver_ms 108, main/busy/ms 100, stage/main/update 71.69, stage/render/prepare_assets 53.29, stream/drive/ms 52.45, stage/render/render 19.36, stage/main/post_update 19.27 |
| warm-up | 4.97 | 126.9 | pipelines/compile_ms 2495, pipelines/front_ms 2426, render/thread/ms 136, render/busy/ms 134, main/busy/ms 94.88, pipelines/wait_ms 90.12, stage/main/update 69.14, pipelines/driver_ms 68.87, stage/render/prepare_assets 65.46, stream/drive/ms 50.07, stage/render/render 39.57, render/graph/ms 37.02 |
| warm-up | 5.07 | 78.8 | pipelines/compile_ms 228, pipelines/front_ms 217, main/busy/ms 74.41, stage/main/update 59.21, render/thread/ms 58.53, render/busy/ms 56.98, stream/drive/ms 36.41, stage/render/prepare_assets 24.29, stage/render/render 19.73, render/graph/ms 17.73, stage/render/graph_submit 15.00, encode/submit 12.55 |
| warm-up | 5.20 | 34.4 | pipelines/compile_ms 265, pipelines/front_ms 256, render/thread/ms 54.65, render/busy/ms 53.35, main/busy/ms 31.15, stage/render/prepare_assets 29.09, stage/render/render 11.99, stage/main/post_update 11.05, render/graph/ms 10.18, pipelines/driver_ms 9.87, stage/main/update 9.60, stage/render/graph_submit 7.41 |
| warm-up | 5.33 | 54.7 | pipelines/compile_ms 1029, pipelines/front_ms 991, render/thread/ms 70.71, main/busy/ms 70.11, render/busy/ms 69.11, pipelines/driver_ms 38.45, stage/main/post_update 38.43, pipelines/wait_ms 29.68, stage/main/update 23.65, stage/render/render 16.36, stage/render/prepare_assets 14.42, render/graph/ms 14.04 |
| warm-up | 5.37 | 36.7 | pipelines/compile_ms 1462, pipelines/front_ms 1405, render/thread/ms 76.36, render/busy/ms 74.90, pipelines/wait_ms 63.39, main/busy/ms 61.99, pipelines/driver_ms 56.58, stage/main/post_update 25.44, stage/main/update 23.73, stage/render/prepare_assets 22.67, stage/render/prepare_write_phase_buffers 12.52, stage/render/render 12.44 |
| warm-up | 5.42 | 47.3 | pipelines/compile_ms 1068, pipelines/front_ms 1039, render/thread/ms 61.86, render/busy/ms 60.71, stage/render/prepare_assets 40.09, pipelines/wait_ms 34.10, main/busy/ms 32.97, pipelines/driver_ms 29.57, stage/main/update 13.52, stage/main/post_update 9.85, stage/render/render 9.19, encode//sum 7.72 |
| warm-up | 5.55 | 40.7 | render/thread/ms 51.50, render/busy/ms 50.21, main/busy/ms 40.65, stage/render/prepare_assets 28.43, stage/main/update 19.71, stage/main/post_update 16.41, stage/render/render 11.53, render/graph/ms 9.71, stream/drive/ms 9.68, encode//sum 8.31, stage/render/graph_submit 6.87, encode/submit 4.75 |
| warm-up | 6.15 | 32.7 | render/thread/ms 37.00, render/busy/ms 35.72, main/busy/ms 27.11, stage/render/prepare_assets 15.36, stage/main/update 11.59, stage/render/render 10.39, stage/main/post_update 10.07, render/graph/ms 8.61, encode//sum 8.31, stage/render/graph_submit 6.07, stream/drive/ms 3.62, encode/submit 3.56 |
| warm-up | 6.67 | 32.6 | render/thread/ms 51.22, render/busy/ms 49.70, main/busy/ms 41.56, stage/render/prepare_assets 24.84, stage/main/update 24.49, stream/drive/ms 17.59, stage/render/render 14.45, render/graph/ms 12.23, stage/main/post_update 12.15, encode//sum 10.34, stage/render/graph_submit 8.77, encode/submit 4.82 |
| warm-up | 6.91 | 32.5 | main/busy/ms 39.56, render/thread/ms 29.41, render/busy/ms 28.30, stage/main/update 22.21, pipelines/compile_ms 19.28, pipelines/front_ms 18.90, stage/render/prepare_assets 11.82, stage/main/post_update 10.36, stage/render/render 9.79, encode//sum 8.46, render/graph/ms 8.27, stage/render/graph_submit 5.92 |
| warm-up | 6.97 | 33.7 | render/thread/ms 45.79, render/busy/ms 44.72, main/busy/ms 26.51, stage/render/render 12.66, stage/main/post_update 11.29, render/graph/ms 11.14, stage/main/update 11.05, stage/render/specialize 10.60, encode//sum 9.45, stage/render/graph_submit 8.58, stage/render/queue 8.20, encode/submit 5.15 |
| warm-up | 8.25 | 33.2 | render/thread/ms 38.35, render/busy/ms 37.28, main/busy/ms 18.49, stage/render/render 11.55, stage/main/post_update 10.56, render/graph/ms 10.08, stage/render/specialize 9.58, stage/render/graph_submit 7.82, stage/render/queue 6.79, encode//sum 6.71, stage/main/update 5.51, encode/finish_wall 4.03 |
| measure | 0.41 | 32.1 | render/thread/ms 46.38, render/busy/ms 45.19, stage/render/render 18.77, render/graph/ms 16.95, main/busy/ms 16.82, encode//sum 13.79, stage/render/graph_submit 12.41, encode/pass:shadow_point_light/sum 11.65, stage/main/post_update 10.39, stage/render/specialize 9.14, encode/submit 8.03, stage/render/queue 6.68 |
| measure | 1.71 | 45.1 | render/thread/ms 49.20, render/busy/ms 48.17, main/busy/ms 25.44, stage/render/render 12.55, stage/main/post_update 11.69, render/graph/ms 10.84, stage/main/update 10.55, stage/render/specialize 9.97, encode//sum 8.19, stage/render/prepare_assets 8.18, stage/render/graph_submit 7.81, stage/render/queue 6.75 |
| measure | 4.50 | 30.0 | render/thread/ms 36.53, render/busy/ms 35.53, main/busy/ms 18.46, stage/main/post_update 12.37, stage/render/render 11.88, render/graph/ms 10.47, stage/render/specialize 9.53, encode//sum 8.35, stage/render/graph_submit 7.62, stage/render/queue 5.88, encode/submit 4.83, encode/pass:shadow_point_light/sum 4.55 |
| measure | 9.58 | 31.3 | render/thread/ms 36.89, render/busy/ms 35.74, main/busy/ms 17.98, stage/render/render 11.08, stage/main/post_update 10.69, stage/render/specialize 10.27, render/graph/ms 9.40, encode//sum 8.38, stage/render/graph_submit 7.15, stage/render/queue 6.51, stage/main/update 4.40, encode/submit 3.93 |
| measure | 15.82 | 30.4 | render/thread/ms 39.59, render/busy/ms 38.49, main/busy/ms 19.07, stage/render/render 12.74, stage/main/post_update 11.93, render/graph/ms 11.24, stage/render/specialize 10.24, encode//sum 9.11, stage/render/graph_submit 8.40, stage/render/queue 7.12, encode/pass:shadow_point_light/sum 6.13, encode/submit 5.48 |
| measure | 17.27 | 30.5 | render/thread/ms 40.16, render/busy/ms 39.15, main/busy/ms 19.69, stage/render/render 13.16, stage/main/post_update 12.66, render/graph/ms 11.63, encode//sum 10.37, stage/render/specialize 10.17, stage/render/graph_submit 8.58, stage/render/queue 6.74, encode/submit 5.75, encode/pass:shadow_point_light/sum 4.77 |
| measure | 17.47 | 30.8 | render/thread/ms 40.53, render/busy/ms 39.62, stage/render/render 32.16, render/graph/ms 30.69, encode//sum 30.04, stage/render/graph_submit 27.13, encode/finish_wall 20.50, main/busy/ms 19.91, stage/main/post_update 13.68, encode/submit 6.63, encode/pass:shadow_point_light/sum 6.33, stage/render/camera_driver 3.45 |
| measure | 18.00 | 30.8 | render/thread/ms 40.37, render/busy/ms 39.34, main/busy/ms 18.57, stage/render/render 13.38, render/graph/ms 11.94, stage/main/post_update 11.24, stage/render/specialize 9.49, encode//sum 9.16, stage/render/graph_submit 9.01, stage/render/queue 7.12, encode/pass:shadow_point_light/sum 6.12, encode/submit 5.30 |
| measure | 19.49 | 33.0 | render/thread/ms 44.51, render/busy/ms 43.39, main/busy/ms 21.46, stage/render/render 15.03, render/graph/ms 13.45, encode//sum 11.66, stage/main/post_update 11.46, stage/render/specialize 11.04, stage/render/graph_submit 9.80, encode/pass:shadow_point_light/sum 7.38, stage/render/queue 7.24, encode/submit 6.03 |
| measure | 20.96 | 30.9 | render/thread/ms 38.62, render/busy/ms 37.51, main/busy/ms 18.39, stage/main/post_update 12.02, stage/render/render 12.01, render/graph/ms 10.49, stage/render/specialize 9.67, encode//sum 8.91, stage/render/graph_submit 7.84, stage/render/queue 7.28, encode/pass:shadow_point_light/sum 4.94, encode/submit 4.70 |
| measure | 23.04 | 33.2 | render/thread/ms 43.43, render/busy/ms 42.31, main/busy/ms 22.53, stage/render/render 13.65, render/graph/ms 11.83, stage/render/specialize 11.41, stage/main/post_update 11.41, encode//sum 10.80, stage/render/graph_submit 8.97, stage/render/queue 8.05, encode/pass:shadow_point_light/sum 6.70, stage/main/update 6.23 |
| measure | 24.61 | 33.3 | render/thread/ms 37.47, render/busy/ms 36.43, main/busy/ms 18.20, stage/render/render 11.70, stage/main/post_update 10.28, render/graph/ms 10.25, stage/render/specialize 9.43, encode//sum 8.66, stage/render/graph_submit 7.58, stage/render/queue 6.63, encode/pass:shadow_point_light/sum 6.53, stage/main/update 5.38 |
| measure | 27.28 | 36.5 | render/thread/ms 44.82, render/busy/ms 43.66, main/busy/ms 25.16, stage/render/render 15.45, render/graph/ms 13.60, stage/main/post_update 12.77, stage/render/specialize 11.58, encode//sum 10.79, stage/render/graph_submit 10.69, encode/pass:shadow_point_light/sum 8.22, stage/render/queue 7.31, encode/submit 6.55 |
| measure | 28.76 | 30.8 | render/thread/ms 40.21, render/busy/ms 39.16, main/busy/ms 20.13, stage/render/render 13.41, stage/main/post_update 13.01, render/graph/ms 11.68, stage/render/specialize 10.08, encode//sum 10.03, stage/render/graph_submit 8.74, stage/render/queue 7.12, encode/pass:shadow_point_light/sum 5.98, encode/submit 5.40 |
| measure | 30.62 | 35.5 | render/thread/ms 41.39, render/busy/ms 40.20, main/busy/ms 22.07, stage/render/render 14.12, stage/main/post_update 12.97, render/graph/ms 12.44, stage/render/specialize 10.91, stage/render/graph_submit 9.49, encode//sum 9.23, stage/render/queue 6.89, encode/pass:shadow_point_light/sum 6.78, encode/submit 5.88 |
| measure | 32.08 | 30.5 | render/thread/ms 37.54, render/busy/ms 36.45, main/busy/ms 18.77, stage/main/post_update 11.44, stage/render/render 11.40, render/graph/ms 9.95, stage/render/specialize 9.88, encode//sum 7.96, stage/render/graph_submit 7.58, stage/render/queue 6.75, encode/submit 4.79, stage/main/update 4.10 |
| measure | 35.95 | 32.8 | render/thread/ms 36.97, render/busy/ms 35.86, main/busy/ms 19.28, stage/main/post_update 11.62, stage/render/render 10.48, stage/render/specialize 10.01, render/graph/ms 8.87, stage/render/queue 7.24, encode//sum 7.22, stage/render/graph_submit 6.65, stage/main/update 4.83, encode/submit 3.63 |
| measure | 37.29 | 34.3 | render/thread/ms 43.94, render/busy/ms 42.87, main/busy/ms 22.35, stage/main/post_update 12.68, stage/render/render 11.68, stage/render/specialize 10.79, render/graph/ms 10.16, encode//sum 8.24, stage/render/graph_submit 7.85, stage/render/queue 7.72, stage/main/update 5.89, encode/submit 4.34 |
| measure | 41.12 | 36.5 | render/thread/ms 37.03, render/busy/ms 36.03, main/busy/ms 17.95, stage/main/post_update 10.07, stage/render/specialize 9.91, stage/render/render 9.88, render/graph/ms 8.40, encode//sum 7.10, stage/render/graph_submit 6.66, stage/render/queue 6.59, stage/main/update 4.55, encode/submit 3.98 |
| measure | 43.85 | 30.3 | render/thread/ms 37.10, render/busy/ms 36.10, main/busy/ms 16.51, stage/render/render 10.78, stage/render/specialize 10.22, stage/main/post_update 9.62, render/graph/ms 9.35, stage/render/graph_submit 7.59, encode//sum 7.13, stage/render/queue 6.54, encode/submit 4.57, stage/main/update 4.24 |
| measure | 46.12 | 33.9 | render/thread/ms 41.43, render/busy/ms 40.27, main/busy/ms 18.59, stage/render/render 12.39, render/graph/ms 10.85, stage/render/specialize 10.28, stage/main/post_update 10.28, stage/render/graph_submit 8.50, encode//sum 8.13, stage/render/queue 7.33, stage/main/update 4.89, encode/submit 4.82 |
| measure | 56.44 | 31.5 | render/thread/ms 34.53, render/busy/ms 33.58, main/busy/ms 14.62, stage/render/render 9.47, stage/render/specialize 9.13, stage/main/post_update 8.71, render/graph/ms 8.19, encode//sum 7.52, stage/render/queue 6.77, stage/render/graph_submit 6.28, stage/render/prepare_meshes 3.81, stage/main/update 3.62 |
| measure | 59.24 | 48.5 | render/thread/ms 56.87, render/busy/ms 55.33, main/busy/ms 31.23, stage/render/render 17.28, stage/main/post_update 17.18, render/graph/ms 15.23, encode//sum 12.62, stage/render/graph_submit 12.10, stage/render/specialize 12.04, stage/main/update 8.92, stage/render/prepare_meshes 8.52, encode/submit 7.74 |
| measure | 59.27 | 32.3 | render/thread/ms 61.86, render/busy/ms 60.76, main/busy/ms 24.44, stage/render/prepare_assets 15.96, stage/render/specialize 12.19, stage/main/post_update 11.74, stage/render/render 10.75, render/graph/ms 9.42, stage/render/prepare_meshes 9.33, encode//sum 7.85, stage/render/graph_submit 7.37, stage/render/queue 6.87 |
| measure | 63.20 | 31.0 | render/thread/ms 39.10, render/busy/ms 37.92, main/busy/ms 18.10, stage/render/render 10.98, stage/render/specialize 10.59, render/graph/ms 9.45, stage/main/post_update 9.05, encode//sum 8.11, stage/render/graph_submit 7.26, stage/render/queue 6.75, stage/main/update 6.18, stage/render/prepare_meshes 4.35 |

## Run

- `target/profiling/perftests.exe` view Balmora --bench benchmarks/exterior_balmora_walk.json --set select.gate=on --resolution 2560x1440 --bench-out target/perf\pt_day\balmora_walk@2560x1440_arm=select-1 --bench-baseline none
- env: BEVYRROWIND_DLSS=quality BEVYRROWIND_WINDOW=borderless
- before the run: GPU 0%, CPU 3%
- during the run: other processes 5% CPU (system minus this process)
- other GPU work during the run: none ≥ 15% SM (1260 pmon samples)
- focus: clean
- nvidia-smi over the window (252 samples): GPU load **78%** (min 46%) · 2619 MHz (min 2610) · memory 8101 MHz · 80.8 W · 78 °C (max 83) · VRAM 7056 MiB · throttled 100% of samples

## Tunables

- `far.handoff_scale=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.bias=1 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_distance_m=24 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `lod.size_cull.min_px=2 (preset: sg.view_distance high`
- `measured: docs/ledger/statics.md 2026-10-12 (bias and size culling per level))`
- `render.bloom=on (preset: sg.post high`
- `industry: screen-space-effects research 4.11 (bloom)`
- `ROADMAP 1.3 (temporal resolve))`
- `render.bloom.intensity=0.1 (preset: sg.post high`
- `industry: screen-space-effects research 4.11 (bloom)`
- `ROADMAP 1.3 (temporal resolve))`
- `render.gi.probes=on (preset: sg.lighting high`
- `measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level)`
- `gi.md slice 1)`
- `render.ssao.quality=medium (preset: sg.lighting high`
- `measured: docs/ledger/shadows.md 2026-10-21 (GTAO per level)`
- `gi.md slice 1)`
- `render.upscaling=quality (command line: env BEVYRROWIND_DLSS (deprecated: --set render.upscaling=value))`
- `select.gate=on (command line: --set)`
- `sf.graphics_preset=custom (preset: derived from its members)`
- `sg.effects=high (preset: level requested)`
- `sg.lighting=high (preset: level requested)`
- `sg.post=custom (preset: derived from its members)`
- `sg.shadows=high (preset: level requested)`
- `sg.sky=high (preset: level requested)`
- `sg.textures=high (preset: level requested)`
- `sg.view_distance=high (preset: level requested)`
- `sg.water=high (preset: level requested)`
- `shadow.contact=on (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.contact.length_m=0.3 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.contact.steps=16 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.contact.thickness_m=0.1 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.filter=auto (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.lamp.contact=on (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.lamp.count=8 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.lamp.fade_m=35,45 (mod: mod mw: guess: covers Balmora street blocks without nearby lamp shadow pop-in (visual-checks/shadows.md, audit 2026-11-01`
- `framework default 15-25 m))`
- `shadow.lamp.map_size_px=512 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.cascades=3 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.distance_m=205 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.overlap=0.16 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.soft=on (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.soft_size_texels=10 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `shadow.sun.split_lambda=0.8 (preset: sg.shadows high`
- `measured: docs/ledger/shadows.md 2026-10-19 (cascades), owner 2026-11-01 (lamp contact), SH-12/SH-14/SH-16)`
- `sky.clouds.map_per_sample=on (preset: sg.sky high`
- `industry: weather.md (distant weather))`
- `surface.puddles=on (preset: sg.effects high`
- `industry: surface-weather.md "Budget")`
- `surface.shelter=on (preset: sg.effects high`
- `industry: surface-weather.md "Budget")`
- `texture.max_size_px=2048 (preset: sg.textures high`
- `measured: docs/ledger/textures.md 2026-11-13 (cap)`
- `textures.md §9 T4d (parallax))`
- `texture.parallax.layers=32 (preset: sg.textures high`
- `measured: docs/ledger/textures.md 2026-11-13 (cap)`
- `textures.md §9 T4d (parallax))`
- `texture.parallax.relief_steps=5 (preset: sg.textures high`
- `measured: docs/ledger/textures.md 2026-11-13 (cap)`
- `textures.md §9 T4d (parallax))`
- `water.quality=high (preset: sg.water high`
- `industry: water.md "Performance budgets")`
