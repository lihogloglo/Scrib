//! GPU resources shared by every view: the field and chunk tables, the scene
//! buffer (masks, then heightmaps), the blue-noise tile, the gust texture, the shared index
//! buffer and the per-frame uniform. Uploaded only when the fields change.

use std::ops::Range; // superfargoth patch (own: partial upload)
use std::sync::Arc;

use bevy::asset::Handle;
use bevy::ecs::prelude::*;
use bevy::log::warn;
use bevy::math::{UVec4, Vec2};
use bevy::render::render_resource::binding_types::{
    sampler, storage_buffer_read_only_sized, storage_buffer_sized, texture_2d, uniform_buffer,
};
use bevy::render::render_resource::{
    AddressMode, BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
    Buffer, BufferInitDescriptor, BufferUsages, Extent3d, FilterMode, PipelineCache, Sampler,
    SamplerBindingType, SamplerDescriptor, ShaderStages, ShaderType, StorageBuffer,
    TextureDataOrder, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType,
    TextureUsages, TextureView, TextureViewDescriptor, UniformBuffer,
};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::view::{ViewUniform, ViewUniforms};
use bevy::shader::Shader;
// superfargoth patch (own: partial upload)
use bevy::render::render_resource::encase::{self, ShaderSize, internal::WriteInto};

use super::compute::ComputePass;
pub(crate) use super::extract::{ExtractedGrass, extract_cameras, extract_grass};
use super::platform;
use super::view::ViewUniformData;
use crate::GrassSettings;
use crate::edit::PatchList;
use crate::fields::{FieldTables, Written}; // superfargoth patch (own: partial upload): `Written`
use crate::gpu_layout::{ChunkGpu, FieldGpu, NO_MAP};
use crate::shader_model::{self, GUST_SIZE, gust_noise};
use crate::wind::GrassWindFrame;

/// The embedded shaders, loaded in the main app and handed to the render app.
#[derive(Resource, Debug, Clone)]
pub(crate) struct GrassShaders {
    pub main: Handle<Shader>,
    pub prepass: Handle<Shader>,
    pub shadow: Handle<Shader>,
    pub generate: Handle<Shader>,
    pub trample: Handle<Shader>,
}

/// Mirrors `Frame` in `common.wesl`.
#[derive(Debug, Clone, Copy, Default, ShaderType)]
pub(crate) struct FrameUniform {
    pub time: f32,
    pub previous_time: f32,
    pub chunk_size: f32,
    pub chunk_count: u32,
    pub lod_rows: UVec4,
}

/// Mirrors `Wind` in `wind.wesl`: [`GrassWindFrame`] as the shaders read it.
///
/// The public frame is plain data and this is its GPU layout, so a new wind
/// term in the shaders changes this struct, not the type every custom wind
/// source writes. Built once a frame, in `extract_grass` and for each
/// far-grass material.
#[derive(Debug, Clone, Copy, Default, PartialEq, ShaderType)]
pub(crate) struct WindUniform {
    pub direction: Vec2,
    pub strength: f32,
    pub gust_scale: f32,
    pub gust_offset: Vec2,
    pub previous_gust_offset: Vec2,
    pub sway: f32,
    pub sway_speed: f32,
    pub flutter: f32,
    pub gust_contrast: f32,
    // superfargoth patch (own: world offset): the world XZ of the render-space
    // origin (`GrassWorldOffset`), added to the blade root only where the gust
    // and patch textures are looked up, so they stay put when a floating
    // origin moves. The offset is a whole multiple of the origin step (256 m),
    // exact in f32; within +-64 km the sum keeps millimetre precision, enough
    // for these lookups. Zero from `From<GrassWindFrame>`.
    pub world_offset: Vec2,
}

impl WindUniform {
    // superfargoth patch (own: world offset)
    /// This frame of wind with the render-space origin at world XZ `world_offset`.
    pub(crate) fn with_world_offset(wind: GrassWindFrame, world_offset: bevy::math::DVec2) -> Self {
        Self {
            world_offset: world_offset.as_vec2(),
            ..wind.into()
        }
    }
}

impl From<GrassWindFrame> for WindUniform {
    fn from(wind: GrassWindFrame) -> Self {
        Self {
            world_offset: Vec2::ZERO, // superfargoth patch (own: world offset)
            direction: wind.direction,
            strength: wind.strength,
            gust_scale: wind.gust_scale,
            gust_offset: wind.gust_offset,
            previous_gust_offset: wind.previous_gust_offset,
            sway: wind.sway,
            sway_speed: wind.sway_speed,
            flutter: wind.flutter,
            gust_contrast: wind.gust_contrast,
        }
    }
}

/// Mirrors `LodTable` in `common.wesl`.
#[derive(Debug, Clone, Copy, Default, ShaderType)]
struct LodTable {
    index_count: UVec4,
    first_index: UVec4,
}

/// Bind group layouts, shared by the pipelines and the bind groups.
#[derive(Resource, Debug, Clone)]
pub(crate) struct GrassLayouts {
    /// Group 0 of the compute passes: everything shared by the views.
    pub shared: BindGroupLayoutDescriptor,
    /// Group 1 of `cull` and `generate`: the view's buffers they fill.
    pub generate_view: BindGroupLayoutDescriptor,
    /// Group 1 of `size_dispatch` and `write_draw_args`: the counters and the
    /// draw arguments made from them.
    pub args_view: BindGroupLayoutDescriptor,
    /// Group 1 of the trample passes: the trample cells and interactors.
    pub trample_view: BindGroupLayoutDescriptor,
    /// Group 2 of `size_dispatch`: the view's indirect dispatch arguments.
    pub dispatch: BindGroupLayoutDescriptor,
    /// Group 2 of every grass draw: one view's instances.
    pub draw: BindGroupLayoutDescriptor,
    /// Group 0 of the shadow pass: the light view's uniform.
    pub shadow_view: BindGroupLayoutDescriptor,
    /// Group 1 of the shadow pass, empty so the grass group stays at 2.
    pub empty: BindGroupLayoutDescriptor,
    // superfargoth patch (own: trample source)
    /// Group 3 of `generate` with an external trample texture (group 2 is
    /// `empty` there).
    pub trample_source: BindGroupLayoutDescriptor,
}

impl GrassLayouts {
    pub(crate) fn new() -> Self {
        let read = || storage_buffer_read_only_sized(false, None);
        let read_write = || storage_buffer_sized(false, None);
        let shared = BindGroupLayoutDescriptor::new(
            "grass_shared",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    uniform_buffer::<FrameUniform>(false),
                    read(),
                    read(),
                    // The scene: masks, then heightmaps.
                    read(),
                    read(),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<LodTable>(false),
                    read(),
                    // The tuft table.
                    read(),
                    uniform_buffer::<WindUniform>(false),
                ),
            ),
        );
        // A view's compute buffers, split by the passes that use them: each
        // storage buffer in a pipeline's layout counts against the device's
        // limit per shader stage, used by the entry point or not. The binding
        // numbers stay those of `generate.wesl` and `trample.wesl`: 0 the
        // view, 1 visible slices, 2 counters, 3 instances, 4 draw arguments,
        // 5 trample cells, 6 interactors.
        let view = || (0, uniform_buffer::<ViewUniformData>(false));
        let generate_view = BindGroupLayoutDescriptor::new(
            "grass_generate_view",
            &BindGroupLayoutEntries::with_indices(
                ShaderStages::COMPUTE,
                (
                    view(),
                    (1, read_write()),
                    (2, read_write()),
                    (3, read_write()),
                    (5, read_write()),
                ),
            ),
        );
        let args_view = BindGroupLayoutDescriptor::new(
            "grass_args_view",
            &BindGroupLayoutEntries::with_indices(
                ShaderStages::COMPUTE,
                (view(), (2, read_write()), (4, read_write())),
            ),
        );
        let trample_view = BindGroupLayoutDescriptor::new(
            "grass_trample_view",
            &BindGroupLayoutEntries::with_indices(
                ShaderStages::COMPUTE,
                (view(), (5, read_write()), (6, read_write())),
            ),
        );
        let dispatch = BindGroupLayoutDescriptor::new(
            "grass_dispatch",
            &BindGroupLayoutEntries::single(ShaderStages::COMPUTE, read_write()),
        );
        let draw = BindGroupLayoutDescriptor::new(
            "grass_draw",
            // Vertex only, but for the view: the blade's look is worked out
            // per vertex, and the fragment shaders read only the view. A
            // fragment-stage storage buffer would add to Bevy's own there.
            &BindGroupLayoutEntries::sequential(
                ShaderStages::VERTEX,
                (
                    uniform_buffer::<FrameUniform>(false),
                    uniform_buffer::<ViewUniformData>(false)
                        .visibility(ShaderStages::VERTEX_FRAGMENT),
                    read(),
                    read(),
                    // The noise the colour patches come from.
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<WindUniform>(false),
                ),
            ),
        );
        let shadow_view = BindGroupLayoutDescriptor::new(
            "grass_shadow_view",
            &BindGroupLayoutEntries::single(
                ShaderStages::VERTEX,
                uniform_buffer::<ViewUniform>(true),
            ),
        );
        let empty = BindGroupLayoutDescriptor::new("grass_empty", &[]);
        Self {
            shared,
            generate_view,
            args_view,
            trample_view,
            dispatch,
            draw,
            shadow_view,
            empty,
            trample_source: super::trample_source::layout(), // superfargoth patch (own: trample source)
        }
    }

    /// The layouts of the compute pipeline for `pass`. `run_generate` binds
    /// the view group that matches.
    pub(crate) fn compute(&self, pass: ComputePass) -> Vec<BindGroupLayoutDescriptor> {
        let view = match pass {
            ComputePass::TrampleDecay | ComputePass::TrampleStamp => &self.trample_view,
            ComputePass::SizeDispatch | ComputePass::WriteDrawArgs => &self.args_view,
            ComputePass::Cull | ComputePass::Generate => &self.generate_view,
            // superfargoth patch (own: trample source)
            ComputePass::GenerateTrampleTexture => &self.generate_view,
        };
        let mut layout = vec![self.shared.clone(), view.clone()];
        if pass == ComputePass::SizeDispatch {
            layout.push(self.dispatch.clone());
        }
        // superfargoth patch (own: trample source)
        if pass == ComputePass::GenerateTrampleTexture {
            layout.push(self.empty.clone());
            layout.push(self.trample_source.clone());
        }
        layout
    }
}

/// The shared GPU buffers.
#[derive(Resource)]
pub(crate) struct GrassShared {
    pub frame: UniformBuffer<FrameUniform>,
    pub wind: UniformBuffer<WindUniform>,
    pub fields: StorageBuffer<Vec<FieldGpu>>,
    chunks: StorageBuffer<Vec<ChunkGpu>>,
    /// Every mask, then every heightmap as `f32` bits: one storage binding
    /// for both, so `generate` stays within 10 storage buffers.
    scene: StorageBuffer<Vec<u32>>,
    tile: Buffer,
    clumps: Buffer,
    tufts: Buffer,
    lods: UniformBuffer<LodTable>,
    /// All three LODs' indices, back to back.
    pub indices: Buffer,
    /// The rows per LOD `indices` was built for.
    blade_rows: [u32; 3],
    pub gust_view: TextureView,
    pub gust_sampler: Sampler,
    uploaded_version: u64,
    // superfargoth patch (own: partial upload): what the buffers hold, so the
    // next version writes only the ranges it changed.
    /// The tables the buffers hold; `None` before the first upload and after
    /// a scene too large to draw.
    uploaded: Option<UploadedState>,
    /// Words in the mask buffer, which edits may write into.
    mask_words: usize,
    /// Chunks in the table; zero means no grass anywhere.
    pub chunk_count: u32,
}

#[cfg(test)]
impl GrassShared {
    /// The scene buffer, masks first, for a test to read back.
    pub(crate) fn masks_buffer(&self) -> Option<&Buffer> {
        self.scene.buffer()
    }
}

/// Bind groups shared by every view, rebuilt when their buffers might move.
#[derive(Resource, Default)]
pub(crate) struct GrassSharedBindGroups {
    pub compute: Option<BindGroup>,
    pub shadow_view: Option<BindGroup>,
    pub empty: Option<BindGroup>,
}

/// Creates every shared resource once the render device exists.
pub(crate) fn init_shared(
    mut commands: Commands,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    let rows = GrassSettings::HIGH.blade_rows;
    let (indices, table) = blade_indices(&device, rows);
    let mut lods = UniformBuffer::from(table);
    lods.write_buffer(&device, &queue);

    let tile_points: Vec<[f32; 2]> = shader_model::blue_noise_tile()
        .iter()
        .map(Vec2::to_array)
        .collect();
    let tile = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("grass_blue_noise_tile"),
        contents: bytemuck::cast_slice(&tile_points),
        usage: BufferUsages::STORAGE,
    });

    let clumps = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("grass_clumps"),
        contents: bytemuck::cast_slice(shader_model::clump_table()),
        usage: BufferUsages::STORAGE,
    });

    let tufts = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("grass_tufts"),
        contents: bytemuck::cast_slice(shader_model::tuft_table()),
        usage: BufferUsages::STORAGE,
    });

    let gust = device.create_texture_with_data(
        &queue,
        &TextureDescriptor {
            label: Some("grass_gust_noise"),
            size: Extent3d {
                width: GUST_SIZE,
                height: GUST_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R8Unorm,
            usage: TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        TextureDataOrder::LayerMajor,
        &gust_noise(GUST_SIZE),
    );
    let gust_view = gust.create_view(&TextureViewDescriptor::default());
    let gust_sampler = device.create_sampler(&SamplerDescriptor {
        label: Some("grass_gust_sampler"),
        address_mode_u: AddressMode::Repeat,
        address_mode_v: AddressMode::Repeat,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        ..Default::default()
    });

    let mut shared = GrassShared {
        frame: UniformBuffer::default(),
        wind: UniformBuffer::default(),
        fields: StorageBuffer::default(),
        chunks: StorageBuffer::default(),
        scene: StorageBuffer::default(),
        tile,
        clumps,
        tufts,
        lods,
        indices,
        blade_rows: rows,
        gust_view,
        gust_sampler,
        uploaded_version: u64::MAX,
        uploaded: None, // superfargoth patch (own: partial upload)
        mask_words: 0,
        chunk_count: 0,
    };
    // A test reads the masks back to check the edits that reach them.
    #[cfg(test)]
    shared.scene.add_usages(BufferUsages::COPY_SRC);
    upload_tables(&mut shared, &FieldTables::default(), &device, &queue);
    commands.insert_resource(GrassLayouts::new());
    commands.insert_resource(shared);
}

/// Storage bindings may not be empty, so an empty table uploads one default
/// element that nothing ever indexes.
fn non_empty<T: Clone + Default>(items: &Arc<Vec<T>>) -> Vec<T> {
    if items.is_empty() {
        vec![T::default()]
    } else {
        items.as_ref().clone()
    }
}

// superfargoth patch (own: partial upload)
/// Bytes of one entry in the fields buffer: its stride as a storage array
/// (every member is 16 bytes, so the size is the stride; so for a chunk).
pub const FIELD_BYTES: u64 = <FieldGpu as ShaderSize>::SHADER_SIZE.get();
/// Bytes of one chunk in the chunks buffer.
pub const CHUNK_BYTES: u64 = <ChunkGpu as ShaderSize>::SHADER_SIZE.get();
const _: () = assert!(FIELD_BYTES % 16 == 0 && CHUNK_BYTES % 16 == 0);

// superfargoth patch (own: partial upload)
/// What the buffers hold: the tables' version and structure version and each
/// table's length (entries, chunks, mask words, height texels).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UploadedState {
    version: u64,
    structure_version: u64,
    lens: [u32; 4],
}

impl UploadedState {
    /// The state once `tables` are uploaded.
    pub(crate) fn of(tables: &FieldTables) -> Self {
        Self {
            version: tables.version,
            structure_version: tables.structure_version,
            lens: [
                tables.fields.len() as u32,
                tables.chunks.len() as u32,
                tables.masks.len() as u32,
                tables.heights.len() as u32,
            ],
        }
    }
}

// superfargoth patch (own: partial upload)
/// How one buffer is brought up to date.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferPlan {
    /// Written whole, at the tables' size.
    Full,
    /// These byte ranges written into the buffer as it is, sorted and merged;
    /// none leaves it untouched.
    Ranges(Vec<Range<u64>>),
}

// superfargoth patch (own: partial upload)
/// How each buffer is brought up to date with a new version of the tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadPlan {
    /// The entries (`FIELD_BYTES` each).
    pub fields: BufferPlan,
    /// The chunks (`CHUNK_BYTES` each).
    pub chunks: BufferPlan,
    /// The masks, then the heightmaps, 4 bytes a word or texel.
    pub scene: BufferPlan,
}

/// `ranges`, sorted, merged where they touch.
fn merged_bytes(ranges: impl IntoIterator<Item = Range<u64>>) -> Vec<Range<u64>> {
    let mut ranges: Vec<Range<u64>> = ranges.into_iter().filter(|r| !r.is_empty()).collect();
    ranges.sort_by_key(|range| range.start);
    let mut merged: Vec<Range<u64>> = Vec::with_capacity(ranges.len());
    for range in ranges {
        match merged.last_mut() {
            Some(last) if last.end >= range.start => last.end = last.end.max(range.end),
            _ => merged.push(range),
        }
    }
    merged
}

// superfargoth patch (own: partial upload)
/// How to upload `tables` over buffers holding `uploaded`.
///
/// A buffer takes only the ranges the latest rebuild wrote when the writes
/// lead from the version it holds and its tables kept their length; it is
/// written whole when nothing was uploaded yet, a table was laid out afresh or
/// changed length (the entries too when the mask words, which shift every
/// heightmap offset, change), or a version came in between that this world
/// never uploaded (two rebuilds in one frame, a skipped extraction), whose
/// writes are unknown.
pub(crate) fn upload_plan(tables: &FieldTables, uploaded: Option<&UploadedState>) -> UploadPlan {
    use BufferPlan::{Full, Ranges};
    let Some(uploaded) = uploaded else {
        return UploadPlan {
            fields: Full,
            chunks: Full,
            scene: Full,
        };
    };
    let lens = UploadedState::of(tables).lens;
    let same = |table: usize| lens[table] == uploaded.lens[table];
    let writes = &tables.writes;
    if writes.since != uploaded.version {
        // The chunks and images change only with the structure version.
        let structure = if tables.structure_version == uploaded.structure_version
            && (1..4).all(same)
        {
            Ranges(Vec::new())
        } else {
            Full
        };
        return UploadPlan {
            fields: Full,
            chunks: structure.clone(),
            scene: structure,
        };
    }
    let bytes = |written: &Written, unit: u64, base: u64| {
        written
            .ranges
            .iter()
            .map(move |r| (base + u64::from(r.start)) * unit..(base + u64::from(r.end)) * unit)
            .collect::<Vec<_>>()
    };
    let fields = if writes.fields.full || !same(0) || !same(2) {
        Full
    } else {
        Ranges(bytes(&writes.fields, FIELD_BYTES, 0))
    };
    let chunks = if writes.chunks.full || !same(1) {
        Full
    } else {
        Ranges(bytes(&writes.chunks, CHUNK_BYTES, 0))
    };
    let scene = if writes.masks.full || writes.heights.full || !same(2) || !same(3) {
        Full
    } else {
        // The heightmaps follow the masks.
        let mask_words = u64::from(lens[2]);
        Ranges(merged_bytes(
            bytes(&writes.masks, 4, 0)
                .into_iter()
                .chain(bytes(&writes.heights, 4, mask_words)),
        ))
    };
    UploadPlan {
        fields,
        chunks,
        scene,
    }
}

/// `field` as the GPU reads it: the heightmaps follow the masks in the scene
/// buffer, so a heightmap's offset counts the mask words before it too.
fn on_gpu(mut field: FieldGpu, mask_words: u32) -> FieldGpu {
    if field.placement.w != NO_MAP {
        field.placement.w += mask_words;
    }
    field
}

/// `values` as their storage array bytes.
fn encoded<T>(values: &[T]) -> Vec<u8>
where
    [T]: ShaderType + WriteInto,
{
    let mut bytes = encase::StorageBuffer::new(Vec::new());
    bytes
        .write(values)
        .expect("a vector of fixed-size GPU structs encodes");
    bytes.into_inner()
}

fn upload_tables(
    shared: &mut GrassShared,
    tables: &FieldTables,
    device: &RenderDevice,
    queue: &RenderQueue,
) {
    shared.uploaded_version = tables.version;
    // superfargoth patch (own: partial upload): each buffer whole or only the
    // ranges the latest rebuild wrote (`upload_plan`).
    let plan = upload_plan(tables, shared.uploaded.as_ref());
    shared.uploaded = None;
    let mask_words = tables.masks.len() as u32;
    match (&plan.fields, shared.fields.buffer()) {
        (BufferPlan::Ranges(ranges), Some(buffer)) => {
            for range in ranges {
                let at = (range.start / FIELD_BYTES) as usize..(range.end / FIELD_BYTES) as usize;
                let entries: Vec<FieldGpu> = tables.fields[at]
                    .iter()
                    .map(|field| on_gpu(*field, mask_words))
                    .collect();
                queue.write_buffer(buffer, range.start, &encoded(&entries));
            }
        }
        _ => {
            let fields = non_empty(&tables.fields)
                .into_iter()
                .map(|field| on_gpu(field, mask_words))
                .collect();
            shared.fields.set(fields);
            shared.fields.write_buffer(device, queue);
        }
    }
    // Showing or hiding a field changes its entries only; the chunks and the
    // images, most of the bytes, stay where they are.
    if plan.chunks == BufferPlan::Full || plan.scene == BufferPlan::Full {
        // A binding larger than the device allows is a validation error that
        // stops Bevy's renderer, so a scene that does not fit draws no grass.
        let limit = device.limits().max_storage_buffer_binding_size;
        if let Some(bytes) =
            platform::scene_too_large(tables.masks.len(), tables.heights.len(), limit)
        {
            warn!(
                "bevy_grass: the density masks and heightmaps of every GrassField take {bytes} \
                 bytes together, and a storage binding on this GPU holds {limit}; no grass is \
                 drawn. Use fewer or smaller masks and heightmaps."
            );
            shared.chunk_count = 0;
            shared.mask_words = 0;
            shared.scene.set(vec![0]);
            shared.scene.write_buffer(device, queue);
            return;
        }
    }
    match (&plan.chunks, shared.chunks.buffer()) {
        (BufferPlan::Ranges(ranges), Some(buffer)) => {
            for range in ranges {
                let at = (range.start / CHUNK_BYTES) as usize..(range.end / CHUNK_BYTES) as usize;
                queue.write_buffer(buffer, range.start, &encoded(&tables.chunks[at]));
            }
        }
        _ => {
            shared.chunk_count = tables.chunks.len() as u32;
            shared.chunks.set(non_empty(&tables.chunks));
            shared.chunks.write_buffer(device, queue);
        }
    }
    match (&plan.scene, shared.scene.buffer()) {
        (BufferPlan::Ranges(ranges), Some(buffer)) => {
            let masks = tables.masks.len();
            for range in ranges {
                let words = (range.start / 4) as usize..(range.end / 4) as usize;
                if words.start < masks {
                    let run = &tables.masks[words.start..words.end.min(masks)];
                    queue.write_buffer(buffer, range.start, bytemuck::cast_slice(run));
                }
                if words.end > masks {
                    let from = words.start.max(masks);
                    let run = &tables.heights[from - masks..words.end - masks];
                    queue.write_buffer(buffer, from as u64 * 4, bytemuck::cast_slice(run));
                }
            }
        }
        _ => {
            shared.mask_words = tables.masks.len();
            let mut scene = Vec::with_capacity(tables.masks.len() + tables.heights.len());
            scene.extend_from_slice(&tables.masks);
            scene.extend(tables.heights.iter().map(|height| height.to_bits()));
            if scene.is_empty() {
                // Storage bindings may not be empty; nothing indexes this word.
                scene.push(0);
            }
            shared.scene.set(scene);
            shared.scene.write_buffer(device, queue);
        }
    }
    shared.uploaded = Some(UploadedState::of(tables));
}

/// Writes the rows edits changed into the masks at the start of the scene
/// buffer, after the tables, so they win over masks the tables still hold from
/// before the edit.
fn write_mask_patches(shared: &GrassShared, patches: &PatchList, queue: &RenderQueue) {
    let Some(buffer) = shared.scene.buffer() else {
        return;
    };
    let mut words = patches.words.as_slice();
    for &(at, len) in &patches.runs {
        let (run, rest) = words.split_at(len as usize);
        words = rest;
        // The main world sends runs only for masks in these tables, so this
        // never fails; it keeps a bad run from being a GPU validation error.
        if (at + len) as usize <= shared.mask_words {
            queue.write_buffer(buffer, u64::from(at) * 4, bytemuck::cast_slice(run));
        }
    }
}

/// The index buffer of blades with `rows` rows per level of detail, all three
/// back to back, and the table of where each begins.
fn blade_indices(device: &RenderDevice, rows: [u32; 3]) -> (Buffer, LodTable) {
    let mut index_data = Vec::new();
    let mut index_count = UVec4::ZERO;
    let mut first_index = UVec4::ZERO;
    for (lod, rows) in rows.iter().enumerate() {
        let lod_indices = shader_model::lod_indices(*rows);
        first_index[lod] = index_data.len() as u32;
        index_count[lod] = lod_indices.len() as u32;
        index_data.extend(lod_indices);
    }
    let indices = device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("grass_indices"),
        contents: bytemuck::cast_slice(&index_data),
        usage: BufferUsages::INDEX,
    });
    (
        indices,
        LodTable {
            index_count,
            first_index,
        },
    )
}

/// Uploads rebuilt tables and this frame's uniform.
pub(crate) fn prepare_shared(
    mut shared: ResMut<GrassShared>,
    mut extracted: ResMut<ExtractedGrass>, // superfargoth patch (own: partial upload): `mut`
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    if extracted.tables.version != shared.uploaded_version {
        upload_tables(&mut shared, &extracted.tables, &device, &queue);
        // superfargoth patch (own: partial upload): nothing in the render
        // world reads the masks or heightmaps after the upload (the ground
        // lookups read the entries and chunks), so let go of them: the main
        // world is then their only owner and its next rebuild writes them in
        // place instead of copying them whole.
        extracted.tables.masks = Arc::default();
        extracted.tables.heights = Arc::default();
    }
    write_mask_patches(&shared, &extracted.mask_patches, &queue);
    // A game that changes the blades' rows gets new meshes, once.
    if extracted.settings.blade_rows != shared.blade_rows {
        let (indices, table) = blade_indices(&device, extracted.settings.blade_rows);
        shared.indices = indices;
        shared.blade_rows = extracted.settings.blade_rows;
        shared.lods.set(table);
        shared.lods.write_buffer(&device, &queue);
    }
    shared.frame.set(extracted.frame);
    shared.frame.write_buffer(&device, &queue);
    shared.wind.set(extracted.wind);
    shared.wind.write_buffer(&device, &queue);
}

/// Rebuilds the shared bind groups. Cheap, and a write can reallocate the
/// uniform and storage buffers, so this runs every frame.
pub(crate) fn prepare_shared_bind_groups(
    mut groups: ResMut<GrassSharedBindGroups>,
    shared: Res<GrassShared>,
    layouts: Res<GrassLayouts>,
    pipeline_cache: Res<PipelineCache>,
    device: Res<RenderDevice>,
    view_uniforms: Res<ViewUniforms>,
) {
    let bindings = (
        shared.frame.binding(),
        shared.fields.binding(),
        shared.chunks.binding(),
        shared.scene.binding(),
        shared.lods.binding(),
        shared.wind.binding(),
    );
    if let (Some(frame), Some(fields), Some(chunks), Some(scene), Some(lods), Some(wind)) = bindings
    {
        groups.compute = Some(device.create_bind_group(
            "grass_shared",
            &pipeline_cache.get_bind_group_layout(&layouts.shared),
            &BindGroupEntries::sequential((
                frame,
                fields,
                chunks,
                scene,
                shared.tile.as_entire_binding(),
                &shared.gust_view,
                &shared.gust_sampler,
                lods,
                shared.clumps.as_entire_binding(),
                shared.tufts.as_entire_binding(),
                wind,
            )),
        ));
    }
    if groups.empty.is_none() {
        groups.empty = Some(device.create_bind_group(
            "grass_empty",
            &pipeline_cache.get_bind_group_layout(&layouts.empty),
            &[],
        ));
    }
    groups.shadow_view = view_uniforms.uniforms.binding().map(|view| {
        device.create_bind_group(
            "grass_shadow_view",
            &pipeline_cache.get_bind_group_layout(&layouts.shadow_view),
            &BindGroupEntries::single(view),
        )
    });
}
