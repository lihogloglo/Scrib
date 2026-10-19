//! Runs the weather map resolve (`sf_weather::MAP_WGSL`) on the GPU
//! (headless wgpu compute) and checks every texel against the CPU twin
//! (`WeatherMap`'s own resolve), for a full resolve and for the strips of a
//! recenter (weather.md "`WE4b` as built": ≤ 1e-3, the f16 storage).
//!
//! Uses the default adapter, or the software fallback (WARP on Windows) if
//! there is none; skips with a message when neither exists.

use std::borrow::Cow;

use glam::{DVec2, IVec2};
use sf_weather::{ClimateIx, ClimateRaster, ClimateRow, MAP_LAYERS, MAP_WGSL, MapGpuGeometry, MapTexel, WeatherMap, resolve_rects, row};
use wgpu::util::DeviceExt;

const SIZE: u32 = 64;

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

fn f16_to_f32(h: u16) -> f32 {
    let sign = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exp = i32::from((h >> 10) & 0x1f);
    let man = f32::from(h & 0x3ff);
    sign * match exp {
        0 => man * 2f32.powi(-24),
        31 => f32::INFINITY,
        e => (1.0 + man / 1024.0) * 2f32.powi(e - 15),
    }
}

struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    texture: wgpu::Texture,
}

impl Gpu {
    fn new() -> Option<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let adapter = adapter(&instance)?;
        eprintln!("map twin on {:?}", adapter.get_info().name);
        let (device, queue) = futures_lite::future::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).ok()?;
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("map twin"), source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(MAP_WGSL)) });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("map twin"),
            layout: None,
            module: &module,
            entry_point: Some("resolve"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("map"),
            size: wgpu::Extent3d { width: SIZE, height: SIZE, depth_or_array_layers: MAP_LAYERS },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        Some(Self { device, queue, pipeline, texture })
    }

    fn resolve(&self, geometry: &MapGpuGeometry, map: &WeatherMap, window: IVec2, rects: &[[i32; 4]]) {
        let words = |w: &[u32]| w.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
        let init = |label, contents: &[u8], usage| self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(label), contents, usage });
        let params = init("params", &words(&geometry.params(window, rects)), wgpu::BufferUsages::UNIFORM);
        let raster = init("raster", &words(&map.raster().gpu_cells()), wgpu::BufferUsages::STORAGE);
        let rows: Vec<u8> = map.gpu_rows().iter().flatten().flat_map(|x| x.to_le_bytes()).collect();
        let rows = init("rows", &rows, wgpu::BufferUsages::STORAGE);
        let view = self.texture.create_view(&wgpu::TextureViewDescriptor { dimension: Some(wgpu::TextureViewDimension::D2Array), ..Default::default() });
        let bind = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: params.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: raster.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: rows.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&view) },
            ],
        });
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &bind, &[]);
            pass.dispatch_workgroups(SIZE / 8, SIZE / 8, u32::try_from(rects.len()).unwrap());
        }
        self.queue.submit([encoder.finish()]);
    }

    /// The map's texels in storage order (A, B, C per texel).
    fn read(&self) -> Vec<MapTexel> {
        let row_bytes = SIZE * 8;
        let size = u64::from(row_bytes * SIZE * MAP_LAYERS);
        let read = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("read"),
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        encoder.copy_texture_to_buffer(
            self.texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &read,
                layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row_bytes), rows_per_image: Some(SIZE) },
            },
            wgpu::Extent3d { width: SIZE, height: SIZE, depth_or_array_layers: MAP_LAYERS },
        );
        self.queue.submit([encoder.finish()]);
        read.map_async(wgpu::MapMode::Read, .., |r| r.expect("map"));
        self.device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
        let bytes = read.get_mapped_range(..).expect("mapped");
        let halves: Vec<f32> = bytes.as_chunks::<2>().0.iter().map(|c| f16_to_f32(u16::from_le_bytes(*c))).collect();
        let n = (SIZE * SIZE) as usize;
        let px = |layer: usize, i: usize| -> [f32; 4] {
            let o = (layer * n + i) * 4;
            [halves[o], halves[o + 1], halves[o + 2], halves[o + 3]]
        };
        (0..n).map(|i| MapTexel { a: px(0, i), b: px(1, i), c: px(2, i), d: px(3, i), look: None }).collect()
    }
}

/// Three climates (west, east, an island in the east) over sea, a 200 m band.
fn raster() -> ClimateRaster {
    let (w, h) = (24u32, 20u32);
    let cells: Vec<Option<ClimateIx>> = (0..w * h)
        .map(|i| {
            let (x, z) = (i % w, i / w);
            match (x, z) {
                (0..=1, _) | (_, 0) => None,
                (15..=17, 8..=10) => Some(ClimateIx(9)),
                (2..=10, _) => Some(ClimateIx(3)),
                (13.., _) => Some(ClimateIx(7)),
                _ => None,
            }
        })
        .collect();
    ClimateRaster::new(DVec2::new(-700.3, 512.7), 64.0, w, h, &cells, 200.0).expect("climates")
}

/// Distinct rows: unit channels in 0..1, heights in meters, fog density ~1e-4.
#[allow(clippy::cast_precision_loss)]
fn rows() -> Vec<ClimateRow> {
    (0..3)
        .map(|i| {
            let mut ch = [0.0; row::LEN];
            for (k, c) in ch.iter_mut().enumerate() {
                *c = ((i * 7 + k * 3) % 11) as f32 / 10.0;
            }
            ch[row::BASE] = 900.0 + 650.0 * i as f32;
            ch[row::THICKNESS] = 400.0 + 900.0 * i as f32;
            ch[row::FOG_DENSITY] = 2e-4 * (i + 1) as f32;
            ClimateRow { ch, look: None }
        })
        .collect()
}

fn compare(gpu: &[MapTexel], cpu: &[MapTexel]) -> f32 {
    gpu.iter().zip(cpu).map(|(g, c)| g.max_diff(c)).fold(0.0, f32::max)
}

#[test]
fn gpu_resolve_matches_the_cpu_twin() {
    let Some(gpu) = Gpu::new() else {
        eprintln!("no wgpu adapter; skipping the map twin");
        return;
    };
    let mut map = WeatherMap::new(raster(), SIZE, 32.0);
    map.set_rows(&rows());
    let geometry = map.gpu_geometry();

    // Full resolve around a camera near the three-climate corner.
    let up = map.update_window(DVec2::new(250.0, 1100.0));
    let (window, size) = map.window();
    assert_eq!(up.texels, SIZE * SIZE);
    let rects = resolve_rects(window, size, None);
    gpu.resolve(&geometry, &map, window, &rects);
    let d = compare(&gpu.read(), map.texels());
    eprintln!("full resolve: max diff {d:.2e}");
    assert!(d <= 1e-3, "full resolve differs by {d}");

    // Recenter one step east and one north: only the strips are written.
    let up = map.update_window(DVec2::new(250.0 + 300.0, 1100.0 - 280.0));
    let (moved, _) = map.window();
    assert!(up.recentered && !up.table_changed);
    let strips = resolve_rects(moved, size, Some(window));
    assert_eq!(strips.len(), 2);
    let area: i32 = strips.iter().map(|r| r[2] * r[3]).sum();
    assert_eq!(u32::try_from(area).unwrap(), up.texels, "strips cover exactly the entering texels");
    gpu.resolve(&geometry, &map, moved, &strips);
    let d = compare(&gpu.read(), map.texels());
    eprintln!("strip resolve: max diff {d:.2e}");
    assert!(d <= 1e-3, "strip resolve differs by {d}");
}
