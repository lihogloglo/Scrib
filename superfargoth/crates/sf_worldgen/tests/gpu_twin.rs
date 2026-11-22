//! The GPU twin of the runtime tiles (worldgen.md "Pipeline" stage 7, slice G2): `tiles.wgsl` composed with the
//! clipmap fill passes (`sf_terrain::gpu_fill`) as the runtime builds it, validated by naga, then run headless on
//! wgpu (the default adapter, or the WARP software fallback) over clipmap pieces at several levels and compared with
//! the CPU reference (`GuideTiles::height`, the tiles' own function, and `clipmap::fill_normals` over the CPU
//! tiles). Skips the GPU half with a message when there is no adapter at all.
//!
//! Tolerance: the CPU splits positions in f64 and sums the base in f64, the GPU stays f32 with integer cell
//! splits and runs its own sin / cos / atan2; the test prints the worst height difference and asserts it under
//! [`HEIGHT_TOL_M`].

// As the crate (lib.rs): grid maths in the usual letters, small index casts.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::many_single_char_names
)]

use std::borrow::Cow;

use sf_terrain::TerrainTiles;
use sf_terrain::clipmap::{self, SampleRect};
use sf_terrain::gpu_fill::{self, FillKind, FillPlan, FillRequest, FillTarget};
use sf_terrain::{HeightTile, TileCoord, sample_meters};
use sf_world::WorldspaceId;
use sf_worldgen::filter::FilterParams;
use sf_worldgen::guide::Guide;
use sf_worldgen::stamps::{Stamp, StampSet};
use sf_worldgen::tiles::{GuideTiles, TileSettings};
use wgpu::util::DeviceExt;

/// Largest height difference GPU vs CPU accepted, metres: float rounding of a 1.5 km relief with 20 m of detail.
const HEIGHT_TOL_M: f32 = 0.02;

/// A cone with ridges and a shore: steep enough everywhere for every octave, part of it under the sea.
fn guide(w: usize, h: usize, spacing_m: f64, origin_m: [f64; 2]) -> Guide {
    let mut heights = vec![0.0f32; w * h];
    let (cx, cy) = (w as f64 * spacing_m * 0.5, h as f64 * spacing_m * 0.5);
    for r in 0..h {
        for c in 0..w {
            let (x, y) = (c as f64 * spacing_m - cx, r as f64 * spacing_m - cy);
            let d2 = (x * x + y * y) / (2.0 * (0.3 * cx).powi(2));
            let v = 1500.0 * (-d2).exp() + 90.0 * (x / 310.0).sin() * (y / 470.0).cos() - 60.0;
            heights[r * w + c] = v as f32;
        }
    }
    Guide { w, h, spacing_m, sea_level_m: 0.0, fill_m: None, origin_m, heights, regions: None }
}

fn tiles(g: Guide) -> GuideTiles {
    let settings = TileSettings::default();
    let prm = FilterParams { octaves: settings.runtime_octaves, ..FilterParams::default().with_sharp_defaults() };
    GuideTiles::new(g, WorldspaceId::new("twin"), prm, settings)
}

fn shader(t: &GuideTiles) -> String {
    gpu_fill::compose(&t.gpu_field().expect("guide tiles have a GPU field").wgsl)
}

#[test]
fn composed_fill_shader_validates() {
    let src = shader(&tiles(guide(64, 64, 20.0, [-640.37, 640.21])));
    let module = naga::front::wgsl::parse_str(&src).unwrap_or_else(|e| panic!("parse: {}", e.emit_to_string(&src)));
    naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
        .validate(&module)
        .unwrap_or_else(|e| panic!("validate: {e:?}"));
}

fn adapter(instance: &wgpu::Instance) -> Option<wgpu::Adapter> {
    for fallback in [false, true] {
        let options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: fallback,
            compatible_surface: None,
            ..wgpu::RequestAdapterOptions::default()
        };
        if let Ok(a) = futures_lite::future::block_on(instance.request_adapter(&options)) {
            return Some(a);
        }
    }
    None
}

/// Runs both fill passes over `plan` and returns the output buffer's words and the GPU wall time (ms).
fn run_gpu(t: &GuideTiles, plan: &FillPlan) -> Option<(Vec<u32>, f64)> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = adapter(&instance)?;
    eprintln!("gpu twin on {:?}", adapter.get_info().name);
    let (device, queue) = futures_lite::future::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        required_limits: adapter.limits(),
        ..wgpu::DeviceDescriptor::default()
    }))
    .ok()?;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("gpu twin"),
        source: wgpu::ShaderSource::Wgsl(Cow::Owned(shader(t))),
    });
    let storage = |read_only| wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Storage { read_only },
        has_dynamic_offset: false,
        min_binding_size: None,
    };
    let entries: Vec<wgpu::BindGroupLayoutEntry> = [true, true, false, false]
        .iter()
        .enumerate()
        .map(|(b, ro)| wgpu::BindGroupLayoutEntry {
            binding: b as u32,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: storage(*ro),
            count: None,
        })
        .collect();
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: None, entries: &entries });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[Some(&bgl)],
        immediate_size: 0,
    });
    let pipeline = |entry: &str| {
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(entry),
            layout: Some(&layout),
            module: &module,
            entry_point: Some(entry),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        })
    };
    let (eval, pack) = (pipeline("fill_eval"), pipeline("fill_pack"));
    let words = |w: &[u32]| -> Vec<u8> { w.iter().flat_map(|v| v.to_le_bytes()).collect() };
    let data = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("field_data"),
        contents: &words(&t.gpu_field().unwrap().data),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let jobs = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("jobs"),
        contents: &plan.job_bytes(),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let sized = |label, len: u32, usage| {
        device.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size: u64::from(len.max(1)) * 4, usage, mapped_at_creation: false })
    };
    let scratch = sized("scratch", plan.scratch_len, wgpu::BufferUsages::STORAGE);
    let out = sized("out", plan.out_len, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC);
    let read = sized("read", plan.out_len, wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST);
    let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: data.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: jobs.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: scratch.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: out.as_entire_binding() },
        ],
    });
    let started = std::time::Instant::now();
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    for (pipeline, threads) in [(&eval, plan.threads[0]), (&pack, plan.threads[1])] {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind, &[]);
        let [x, y] = FillPlan::groups(threads);
        if x > 0 {
            pass.dispatch_workgroups(x, y, 1);
        }
    }
    encoder.copy_buffer_to_buffer(&out, 0, &read, 0, u64::from(plan.out_len.max(1)) * 4);
    queue.submit([encoder.finish()]);
    read.map_async(wgpu::MapMode::Read, .., |r| r.expect("map"));
    device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
    let ms = started.elapsed().as_secs_f64() * 1e3;
    let bytes = read.get_mapped_range(..).expect("mapped");
    Some((bytes.as_chunks::<4>().0.iter().map(|c| u32::from_le_bytes(*c)).collect(), ms))
}

fn rect(x: i64, z: i64, w: i64, h: i64) -> SampleRect {
    SampleRect { min: [x, z], size: [w, h] }
}

/// The CPU tiles of `level` covering `r` plus one sample of margin.
fn cpu_tiles(t: &GuideTiles, level: u8, r: SampleRect) -> Vec<HeightTile> {
    let per = 512i64;
    let (x0, z0) = ((r.min[0] - 1).div_euclid(per), (r.min[1] - 1).div_euclid(per));
    let (x1, z1) = ((r.min[0] + r.size[0]).div_euclid(per), (r.min[1] + r.size[1]).div_euclid(per));
    let mut out = Vec::new();
    for z in z0..=z1 {
        for x in x0..=x1 {
            out.push(t.heights(TileCoord::new(level, x as i32, z as i32)));
        }
    }
    out
}

/// Pieces at a fine, a middle and a coarse level, a macro piece, across the guide's slopes and its corner.
fn requests(t: &GuideTiles) -> Vec<FillRequest> {
    let [x0, z0, x1, z1] = t.extent_m();
    // Level-0 global index of a world position (sample_meters inverted), then at `level`.
    let at = |x: f64, z: f64, level: u8| -> [i64; 2] {
        let g = |v: f64| ((v + 128.0) * 2.0).floor() as i64 >> level;
        [g(x), g(z)]
    };
    let mid = [f64::midpoint(x0, x1), f64::midpoint(z0, z1)];
    let flank = [x0 + (x1 - x0) * 0.35, z0 + (z1 - z0) * 0.4];
    let mut out = Vec::new();
    for (level, p, size) in [(0u8, flank, [37, 21]), (0, [x1 - 30.0, z1 - 30.0], [40, 9]), (2, mid, [64, 33]), (5, flank, [50, 50]), (9, mid, [8, 8])] {
        let m = at(p[0], p[1], level);
        out.push(FillRequest { kind: FillKind::Heights, level, rect: rect(m[0], m[1], size[0], size[1]), texel: [0, 0] });
    }
    let m = at(flank[0], flank[1], 2);
    out.push(FillRequest { kind: FillKind::Macro, level: 1, rect: rect(m[0], m[1], 30, 7), texel: [0, 0] });
    out
}

/// Worst height and normal differences, and the share of normal components more than one snorm step apart.
fn compare(t: &GuideTiles, reqs: &[FillRequest], plan: &FillPlan, out: &[u32]) -> (f32, i32, f32) {
    let (mut worst_h, mut worst_n, mut far_n, mut n_total) = (0.0f32, 0i32, 0usize, 0usize);
    let mut copies = plan.copies.iter();
    for r in reqs {
        let (w, h) = (r.rect.size[0] as usize, r.rect.size[1] as usize);
        match r.kind {
            FillKind::Heights => {
                let hc = copies.next().unwrap();
                let nc = copies.next().unwrap();
                assert_eq!((hc.target, nc.target), (FillTarget::Heights, FillTarget::Normals));
                let held = cpu_tiles(t, r.level, r.rect);
                let tile = |c: TileCoord| held.iter().find(|x| x.coord == c);
                let mut cpu_h = vec![0.0f32; w * h];
                clipmap::fill_heights(r.level, r.rect, &tile, None, 0.0, &mut cpu_h);
                let mut cpu_n = vec![[0i8; 2]; w * h];
                clipmap::fill_normals(r.level, r.rect, &tile, None, 0.0, &mut cpu_n);
                for j in 0..h {
                    for i in 0..w {
                        let k = j * w + i;
                        // The CPU tiles are the reference function itself, bit for bit.
                        let gx = (r.rect.min[0] + i as i64) << r.level;
                        let gz = (r.rect.min[1] + j as i64) << r.level;
                        let reference = t.height(sample_meters(gx), sample_meters(gz), r.level);
                        assert_eq!(reference.to_bits(), cpu_h[k].to_bits(), "CPU tile and point query differ");
                        let gpu = f32::from_bits(out[(hc.offset / 4) as usize + j * (hc.bytes_per_row / 4) as usize + i]);
                        assert!(gpu.is_finite(), "GPU height not finite at level {} ({i}, {j})", r.level);
                        worst_h = worst_h.max((gpu - cpu_h[k]).abs());
                        let word = out[(nc.offset / 4) as usize + j * (nc.bytes_per_row / 4) as usize + i / 2];
                        let bytes = word.to_le_bytes();
                        let g = if i % 2 == 0 { [bytes[0], bytes[1]] } else { [bytes[2], bytes[3]] };
                        for a in 0..2 {
                            let d = (i32::from(g[a].cast_signed()) - i32::from(cpu_n[k][a])).abs();
                            worst_n = worst_n.max(d);
                            far_n += usize::from(d > 1);
                            n_total += 1;
                        }
                    }
                }
            }
            FillKind::Macro => {
                let mc = copies.next().unwrap();
                assert_eq!(mc.target, FillTarget::Macro);
                // Macro sample m of level l: tile m.div_euclid(256), index m - 256 tile in its 257² grid.
                let mut held: Vec<(TileCoord, Vec<[u8; 4]>)> = Vec::new();
                for j in 0..h {
                    for i in 0..w {
                        let (mx, mz) = (r.rect.min[0] + i as i64, r.rect.min[1] + j as i64);
                        let c = TileCoord::new(r.level, mx.div_euclid(256) as i32, mz.div_euclid(256) as i32);
                        if !held.iter().any(|(k, _)| *k == c) {
                            held.push((c, t.macro_albedo(c).expect("macro").expect("macro").rgba));
                        }
                        let cpu = &held.iter().find(|(k, _)| *k == c).unwrap().1;
                        let want = cpu[(mz - i64::from(c.z) * 256) as usize * 257 + (mx - i64::from(c.x) * 256) as usize];
                        let px = out[(mc.offset / 4) as usize + j * (mc.bytes_per_row / 4) as usize + i].to_le_bytes();
                        for a in 0..4 {
                            assert!(px[a].abs_diff(want[a]) <= 2, "macro texel ({i}, {j}) channel {a}: GPU {} CPU {}", px[a], want[a]);
                        }
                    }
                }
            }
        }
    }
    (worst_h, worst_n, far_n as f32 / n_total.max(1) as f32)
}

fn check(name: &str, t: &GuideTiles) {
    check_with(name, t, &requests(t));
}

fn check_with(name: &str, t: &GuideTiles, reqs: &[FillRequest]) {
    let reqs = reqs.to_vec();
    let plan = gpu_fill::plan(&reqs);
    let Some((out, ms)) = run_gpu(t, &plan) else {
        eprintln!("no wgpu adapter (not even a software fallback): GPU twin test skipped");
        return;
    };
    let (dh, dn, far) = compare(t, &reqs, &plan, &out);
    eprintln!(
        "gpu twin {name}: {} texels in {ms:.1} ms (submit to readback), worst |dh| {dh:e} m, worst normal {dn} snorm steps, {:.3}% > 1 step",
        plan.texels,
        far * 100.0
    );
    assert!(dh < HEIGHT_TOL_M, "{name}: GPU and CPU heights differ by {dh} m");
    assert!(far < 0.01, "{name}: {}% of normal components more than one step apart", far * 100.0);
}

#[test]
fn gpu_fill_matches_the_cpu_tiles() {
    // Guide spacing a multiple of 0.5 m (integer cell splits), origin off the eighth-metre grid (as V3).
    check("integer cells", &tiles(guide(400, 380, 20.0, [-4009.008, 3699.712])));
    // Spacing that isn't (the f32 affine fallback).
    check("f32 cells", &tiles(guide(300, 300, 7.3, [-1100.4, 1095.9])));
    // The steep-face gully fade (LF-2b): the cone's flanks run 0-60 deg, so the fade's whole ramp is crossed.
    let t = tiles(guide(400, 380, 20.0, [-4009.008, 3699.712]));
    let fade = [30f32.to_radians().tan(), 45f32.to_radians().tan()];
    check("steep fade", &t.with_params(FilterParams { steep_fade: Some(fade), ..*t.params() }));
}

/// A stamp centred on engine (x, z): weight 1 inside, a C1 smootherstep to 0 across `band_m` at its edges,
/// heights a tilted plateau `top_m` with a ripple (so the GPU's bilinear sees slopes).
fn stamp_at(x: f64, z: f64, dims: [usize; 2], spacing_m: f64, band_m: f64, top_m: f32) -> Stamp {
    let [w, h] = dims;
    let origin_m = [x - (w - 1) as f64 * spacing_m * 0.5, -z + (h - 1) as f64 * spacing_m * 0.5];
    let (mut heights, mut weights) = (Vec::with_capacity(w * h), Vec::with_capacity(w * h));
    for r in 0..h {
        for c in 0..w {
            let d = (c.min(w - 1 - c).min(r).min(h - 1 - r)) as f64 * spacing_m;
            let t = (d / band_m).clamp(0.0, 1.0);
            weights.push((t * t * t * (t * (6.0 * t - 15.0) + 10.0)) as f32);
            heights.push(top_m + 0.03 * c as f32 - 0.02 * r as f32 + 1.5 * ((c as f32) * 0.37).sin() * ((r as f32) * 0.23).cos());
        }
    }
    Stamp { name: "twin".into(), origin_m, dims, spacing_m, heights, weights }
}

#[test]
fn gpu_fill_matches_the_cpu_tiles_with_a_stamp() {
    let base = tiles(guide(400, 380, 20.0, [-4009.008, 3699.712]));
    let [x0, z0, x1, z1] = base.extent_m();
    let (cx, cz) = (x0 + (x1 - x0) * 0.35, z0 + (z1 - z0) * 0.4);
    let top = base.height(cx, cz, 0) + 25.0;
    let set = StampSet { stamps: vec![stamp_at(cx, cz, [80, 60], 1.8288, 30.0, top)] };
    let t = tiles(guide(400, 380, 20.0, [-4009.008, 3699.712])).with_stamps(set);
    let idx = t.stamps().expect("stamped");
    let at = |x: f64, z: f64, level: u8| -> [i64; 2] {
        let g = |v: f64| ((v + 128.0) * 2.0).floor() as i64 >> level;
        [g(x), g(z)]
    };
    // Level 0 over the stamp's west half and beyond (inside, band, outside), coarser levels over all of it.
    let mut reqs = Vec::new();
    for (level, size) in [(0u8, [260i64, 300i64]), (2, [100, 80]), (4, [24, 20])] {
        let m = at(cx - 100.0, cz - 75.0, level);
        reqs.push(FillRequest { kind: FillKind::Heights, level, rect: rect(m[0], m[1], size[0], size[1]), texel: [0, 0] });
    }
    // Every class of sample is in the pieces.
    let (mut inside, mut band, mut outside) = (0, 0, 0);
    for r in &reqs {
        for j in 0..r.rect.size[1] {
            for i in 0..r.rect.size[0] {
                let gx = (r.rect.min[0] + i) << r.level;
                let gz = (r.rect.min[1] + j) << r.level;
                match idx.sample(sample_meters(gx), -sample_meters(gz)) {
                    Some((_, w)) if w >= 1.0 => inside += 1,
                    Some((_, w)) if w > 0.0 => band += 1,
                    _ => outside += 1,
                }
            }
        }
    }
    eprintln!("gpu twin stamp: {inside} samples inside, {band} in the band, {outside} outside; field data {:.2} MiB", t.gpu_data_bytes() as f64 / f64::from(1u32 << 20));
    assert!(inside > 100 && band > 100 && outside > 100, "{inside} / {band} / {outside}");
    assert_eq!(t.gpu_data_bytes(), t.gpu_field().unwrap().data.len() * 4);
    check_with("stamp", &t, &reqs);
}

/// The layer macro look (WG-X3): a global 73 m grid and a 7.3 m stamp grid, their mean-albedo pyramids on the GPU
/// against `GuideTiles::macro_albedo`, at fine and coarse macro levels inside, across and outside the stamp.
#[test]
fn gpu_macro_matches_the_cpu_with_layers() {
    use sf_terrain::{LayerInfo, LayerTable, layer_id};
    use sf_worldgen::layers::{GuideLayerGrid, GuideLayers, NO_LAYER};
    let g = || guide(400, 380, 20.0, [-4009.008, 3699.712]);
    let base = tiles(g());
    let [x0, z0, x1, z1] = base.extent_m();
    let (cx, cz) = (x0 + (x1 - x0) * 0.35, z0 + (z1 - z0) * 0.4);
    let st = stamp_at(cx, cz, [160, 120], 1.8288, 40.0, base.height(cx, cz, 0) + 25.0);
    let [sx0, _, _, sy1] = st.rect_m();
    let names: Vec<String> = (0..6).map(|n| format!("twin_layer_{n}")).collect();
    let global = GuideLayerGrid {
        origin_m: [-4100.0, 3800.0],
        dims: [120, 110],
        spacing_m: 73.152,
        blend_m: 54.864,
        texels: (0..120 * 110).map(|k| if k % 97 == 5 { NO_LAYER } else { (((k % 120) / 2 + (k / 120) / 3) % 4) as u16 }).collect(),
        tint: None,
    };
    let stamp_grid = GuideLayerGrid {
        origin_m: [sx0 - 10.0, sy1 + 10.0],
        dims: [45, 36],
        spacing_m: 7.3152,
        blend_m: 5.4864,
        texels: (0..45 * 36).map(|k| 4 + ((k % 45) / 3 + (k / 45) / 2) as u16 % 2).collect(),
        tint: None,
    };
    let layers = GuideLayers { palette: names.clone(), default_layer: "twin_default".into(), global, stamps: vec![(0, stamp_grid)] };
    let info = |record: &str, n: usize| LayerInfo {
        record: record.into(),
        id: layer_id(record),
        tiling_m: 4.0,
        triplanar: false,
        displacement_m: 0.0,
        mean_albedo: [0.05 + 0.1 * n as f32, 0.4 - 0.05 * n as f32, 0.1 + 0.03 * n as f32],
        mean_height: 0.5,
        p90_height: 0.6,
        source_edge: 64,
    };
    let mut table: Vec<LayerInfo> = names.iter().enumerate().map(|(n, r)| info(r, n)).collect();
    table.push(info("twin_default", 6));
    let table = LayerTable { layers: table, fill_layer: Some(layer_id("twin_default")), fill_height: 0.0 };
    let (t, missing) = tiles(g())
        .with_stamps(StampSet { stamps: vec![st] })
        .with_layers(layers)
        .unwrap()
        .with_layer_arrays(&table, std::sync::Arc::new(|_, _| None))
        .unwrap();
    assert!(missing.is_empty());
    let at = |x: f64, z: f64, shift: u8| -> [i64; 2] {
        let g = |v: f64| ((v + 128.0) * 2.0).floor() as i64 >> shift;
        [g(x), g(z)]
    };
    // Macro level l samples every 2^(l+1) level-0 indices.
    let mut reqs = Vec::new();
    for (level, size) in [(0u8, [300i64, 260i64]), (1, [160, 140]), (3, [60, 60]), (6, [40, 40]), (9, [8, 8])] {
        let m = at(cx - 150.0, cz - 130.0, level + 1);
        reqs.push(FillRequest { kind: FillKind::Macro, level, rect: rect(m[0], m[1], size[0], size[1]), texel: [0, 0] });
    }
    assert_eq!(t.gpu_data_bytes(), t.gpu_field().unwrap().data.len() * 4);
    check_with("layers macro", &t, &reqs);
}

/// The real island when it exists (`target/worldgen/vo/V3`), with its large coordinates.
#[test]
#[ignore = "measurement: needs target/worldgen/vo/V3"]
fn measure_gpu_twin_on_v3() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../target/worldgen/vo/V3");
    let t = GuideTiles::open(&dir, WorldspaceId::new("worldgen")).expect("V3 guide");
    check("V3", &t);
    // A full 512² level of each kind: what a burst costs (wall time includes submit and readback).
    let full = |kind, level| FillRequest { kind, level, rect: rect(40_000 >> level, -30_000 >> level, 512, 512), texel: [0, 0] };
    for reqs in [vec![full(FillKind::Heights, 0)], vec![full(FillKind::Heights, 4)], vec![full(FillKind::Macro, 0)]] {
        let plan = gpu_fill::plan(&reqs);
        if let Some((_, ms)) = run_gpu(&t, &plan) {
            let (_, again) = run_gpu(&t, &plan).unwrap();
            eprintln!("gpu twin V3 full {:?} level {}: {} texels, {ms:.1} ms first, {again:.1} ms again", reqs[0].kind, reqs[0].level, plan.texels);
        }
    }
}
