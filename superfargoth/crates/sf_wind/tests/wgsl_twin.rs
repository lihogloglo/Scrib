//! Runs `sf_wind.wesl` on the GPU (headless wgpu compute) and checks the CPU
//! twin against it at golden points (weather.md "Wind": 1e-4).
//!
//! Uses the default adapter, or the software fallback (WARP on Windows) if
//! there is none; skips with a message when neither exists.

use std::borrow::Cow;

use glam::{DVec3, IVec3, Vec2, Vec3, Vec4};
use sf_coords::WorldPos;
use sf_wind::{WESL, Wind, WindFrame, WindSettings, WindSource, WindState};
use wgpu::util::DeviceExt;

const HARNESS: &str = r"
@group(0) @binding(0) var<uniform> params: WindParams;
@group(0) @binding(1) var<storage, read> sources: array<WindSource>;
@group(0) @binding(2) var<storage, read> points: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read_write> out: array<vec4<f32>>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    if i >= arrayLength(&points) {
        return;
    }
    let p = points[i];
    var v = wind_global(params, p.xyz, p.w);
    for (var k = 0u; k < wind_source_count(params); k++) {
        v += wind_source(sources[k], p.xyz);
    }
    out[i] = vec4<f32>(v, wind_gust(params, p.xz).gust);
}
";

fn floats_to_bytes(f: &[f32]) -> Vec<u8> {
    f.iter().flat_map(|x| x.to_le_bytes()).collect()
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

/// The GPU's `(velocity, gust)` for each `(rel, above_ground)` point.
fn run_gpu(frame: &WindFrame, points: &[Vec4]) -> Option<Vec<Vec4>> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = adapter(&instance)?;
    eprintln!("wgsl twin on {:?}", adapter.get_info().name);
    let (device, queue) =
        futures_lite::future::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).ok()?;

    let source = format!("{WESL}\n{HARNESS}");
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sf_wind twin"),
        source: wgpu::ShaderSource::Wgsl(Cow::Owned(source)),
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("sf_wind twin"),
        layout: None,
        module: &module,
        entry_point: Some("main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });

    let params: Vec<f32> = frame.params.to_array().into_iter().flatten().collect();
    let mut sources: Vec<f32> = frame.sources.iter().flat_map(|s| s.to_array().into_iter().flatten()).collect();
    sources.resize(sources.len().max(12), 0.0);
    let pts: Vec<f32> = points.iter().flat_map(Vec4::to_array).collect();
    let usage = wgpu::BufferUsages::STORAGE;
    let init = |label, data: &[f32], usage| {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: &floats_to_bytes(data),
            usage,
        })
    };
    let params = init("params", &params, wgpu::BufferUsages::UNIFORM);
    let sources = init("sources", &sources, usage);
    let pts_buf = init("points", &pts, usage);
    let size = (pts.len() * 4) as u64;
    let out = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("out"),
        size,
        usage: usage | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let read = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("read"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: params.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: sources.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: pts_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: out.as_entire_binding() },
        ],
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind, &[]);
        pass.dispatch_workgroups(u32::try_from(points.len().div_ceil(64)).unwrap(), 1, 1);
    }
    encoder.copy_buffer_to_buffer(&out, 0, &read, 0, size);
    queue.submit([encoder.finish()]);
    read.map_async(wgpu::MapMode::Read, .., |r| r.expect("map"));
    device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
    let bytes = read.get_mapped_range(..).expect("mapped");
    let floats: Vec<f32> = bytes.as_chunks::<4>().0.iter().map(|c| f32::from_le_bytes(*c)).collect();
    Some(floats.as_chunks::<4>().0.iter().map(|c| Vec4::from_array(*c)).collect())
}

/// Golden points: a scatter over a few hundred meters, heights from below
/// the ground to 400 m, plus points on lattice-cell boundaries and at the
/// source centers.
fn golden_points(frame: &WindFrame) -> Vec<Vec4> {
    let mut pts = Vec::new();
    for i in 0..2048u16 {
        let t = f32::from(i);
        let px = (t * 0.618_034).fract() * 1200.0 - 600.0;
        let pz = (t * 0.754_878).fract() * 1200.0 - 600.0;
        let py = (t * 0.569_840).fract() * 60.0 - 10.0;
        let h = ((t * 0.412_454).fract() * 20.0).powi(2) - 1.0;
        pts.push(Vec4::new(px, py, pz, h));
    }
    let cell = 1.0 / frame.params.octave0.z;
    for k in -4i16..=4 {
        let x = (f32::from(k) - frame.params.octave0.x) * cell;
        pts.push(Vec4::new(x, 0.0, 3.0, 10.0));
        pts.push(Vec4::new(-7.0, 0.0, (f32::from(k) - frame.params.octave0.y) * cell, 10.0));
    }
    for s in frame.sources.iter().filter(|s| s.a.truncate().length() < 2000.0) {
        pts.push(s.a.truncate().extend(10.0));
        pts.push((s.a.truncate() + Vec3::new(0.2, 0.1, -0.3)).extend(10.0));
    }
    pts
}

#[test]
fn cpu_and_wgsl_twins_agree() {
    let settings = WindSettings::default();
    let mut state = WindState::new(Wind { velocity: Vec2::new(-11.0, 6.5), gustiness: 0.7, gust_length: 45.0 });
    let origin = WorldPos::new(IVec3::new(-1203, 0, 877), Vec3::new(17.25, 3.0, -101.5));
    let at = |d: Vec3| origin.offset(d);
    state.add_source(WindSource::radial(at(Vec3::new(30.0, 2.0, -40.0)), 25.0, 60.0).with_inner_radius(8.0));
    state.add_source(WindSource::vortex(at(Vec3::new(-120.0, 0.0, 90.0)), Vec3::new(0.1, 1.0, 0.0), 14.0, 45.0));
    state.add_source(WindSource::directional(at(Vec3::new(200.0, 5.0, 10.0)), Vec3::new(1.0, 0.3, -1.0), 9.0, 120.0));
    state.add_source(WindSource::radial(WorldPos::from_f64(DVec3::new(-5.0e5, 0.0, 3.0e5)), -8.0, 2.0e5));
    for _ in 0..600 {
        state.advance(1.0 / 30.0, &settings);
    }
    let frame = state.frame(&settings, origin);
    let pts = golden_points(&frame);
    let Some(gpu) = run_gpu(&frame, &pts) else {
        eprintln!("no wgpu adapter (not even a software fallback): WGSL twin test skipped");
        return;
    };
    let mut worst = 0.0f32;
    let mut worst_gust = 0.0f32;
    let mut peak = 0.0f32;
    let mut worst_abs = 0.0f32;
    for (p, g) in pts.iter().zip(&gpu) {
        let cpu = frame.sample(p.truncate(), p.w);
        let gust = frame.gust(Vec2::new(p.x, p.z)).gust;
        worst = worst.max((cpu - g.truncate()).length() / cpu.length().max(1.0));
        worst_gust = worst_gust.max((gust - g.w).abs());
        peak = peak.max(cpu.length());
        worst_abs = worst_abs.max((cpu - g.truncate()).length());
    }
    eprintln!("wgsl twin: {} points, worst velocity error {worst:e} (m/s per m/s above 1 m/s; peak {peak} m/s, absolute {worst_abs:e} m/s), worst gust error {worst_gust:e}", pts.len());
    assert!(peak > 10.0, "the golden points should see real wind");
    assert!(worst < 1e-4, "CPU/WGSL twins differ by {worst} (relative)");
    assert!(worst_gust < 1e-4, "gust envelopes differ by {worst_gust}");
}
