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
    let prm = FilterParams { octaves: settings.runtime_octaves, ..FilterParams::default() };
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
    let reqs = requests(t);
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
