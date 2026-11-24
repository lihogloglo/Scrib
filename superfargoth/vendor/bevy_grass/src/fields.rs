//! The CPU's whole steady-state job: describing the fields.
//!
//! When a field, its style, its mask or its heightmap changes, the field
//! table, the chunk table and the packed masks and heights are rebuilt and
//! handed to the render world. Nothing here runs per blade or per frame; the
//! GPU decides which blades exist (`shaders/generate.wesl`).

use std::collections::{HashMap, HashSet};
use std::ops::Range; // superfargoth patch (own: stable tables)
use std::sync::Arc;
use std::time::Duration; // superfargoth patch (own: rebuild phases)

use bevy::app::{App, PostUpdate};
use bevy::asset::{Asset, AssetEvent, AssetEventSystems, AssetId, Assets};
use bevy::camera::visibility::{InheritedVisibility, RenderLayers, VisibilitySystems};
use bevy::color::{ColorToComponents, LinearRgba};
use bevy::ecs::message::MessageReader;
use bevy::ecs::prelude::*;
use bevy::image::Image;
use bevy::log::warn;
use bevy::math::{DVec2, EulerRot, Quat, UVec2, UVec4, Vec2, Vec3, Vec4}; // superfargoth patch (own: world offset): DVec2
use bevy::platform::time::Instant;
use bevy::transform::TransformSystems;
use bevy::transform::components::GlobalTransform;

use crate::api::{
    GrassBlade, GrassField, GrassFieldOverrides, GrassGround, GrassSpecies, GrassStyle,
};
use crate::edit::{self, EditLayers};
use crate::gpu_layout::{ChunkGpu, FieldGpu, NO_MAP};
use crate::ground::{HOLE, MeshGrounds, bake_mesh_grounds};
use crate::packed::{self, PackedImages};
use crate::shader_model::{self, ChunkLayout};
use crate::stats::{RebuildPhases, TableStats}; // superfargoth patch (own: rebuild phases): `RebuildPhases`
use crate::style::MAX_SPECIES;
use crate::{GrassStats, GrassSystems};

/// Side of a chunk, in metres.
///
/// One size for every view, because the chunk table is shared. T2 in the plan
/// suggested 8 m chunks for orthographic views; per-blade frustum culling
/// rejects what a larger chunk lets through, so this stays 16 m until a
/// measurement says otherwise.
pub(crate) const CHUNK_SIZE: f32 = 16.0;

/// Entries beyond this index cannot be addressed by an instance's 8-bit
/// field index. Every species of a field is an entry.
const MAX_ENTRIES: usize = 256;

impl FieldGpu {
    /// The field's X axis in world XZ. An entry built by hand, with no axis,
    /// is unturned.
    pub(crate) fn x_axis(&self) -> Vec2 {
        let axis = Vec2::new(self.axis.x, self.axis.y);
        if axis == Vec2::ZERO { Vec2::X } else { axis }
    }

    /// A point of the field, in metres along its axes from its minimum
    /// corner, in world XZ. Twin of `field_to_world` in `generate.wesl`.
    /// Only the debug gizmos and the tests draw from the GPU table.
    #[cfg(any(test, feature = "debug"))]
    pub(crate) fn world_of(&self, local: Vec2) -> Vec2 {
        Vec2::new(self.min_corner.x, self.min_corner.z) + turn(self.x_axis(), local)
    }

    /// The point of the field at `point` (world XZ), in metres along its axes
    /// from its minimum corner.
    pub(crate) fn local_of(&self, point: Vec2) -> Vec2 {
        let axis = self.x_axis();
        let offset = point - Vec2::new(self.min_corner.x, self.min_corner.z);
        Vec2::new(offset.dot(axis), offset.dot(axis.perp()))
    }
}

/// `v`, given along the axes of a field whose X axis is `axis`, in world XZ.
/// Unturned (`axis` = X) it is `v` exactly.
fn turn(axis: Vec2, v: Vec2) -> Vec2 {
    axis * v.x + axis.perp() * v.y
}

/// Everything the GPU needs to know about the fields. Replaced, never edited,
/// so the render world can keep the previous version while a new one builds.
#[derive(Resource, Debug, Default, Clone)]
pub(crate) struct FieldTables {
    /// Bumped on every change; the render world uploads when it changes.
    pub version: u64,
    /// Bumped when the chunks, masks or heightmaps change, which is most of
    /// the upload. A field only shown or hidden bumps `version` alone.
    pub structure_version: u64,
    pub fields: Arc<Vec<FieldGpu>>,
    /// Where each field's entries are in `fields`, and what they were built
    /// from.
    pub slots: Arc<Vec<FieldSlot>>,
    /// What each mask in `masks` is, in order.
    pub mask_ids: Arc<Vec<MaskKey>>,
    /// The image of each heightmap in `heights`, in order.
    pub height_ids: Arc<Vec<AssetId<Image>>>,
    pub chunks: Arc<Vec<ChunkGpu>>,
    /// Every mask, two R8G8 texels per word, low half first.
    pub masks: Arc<Vec<u32>>,
    /// Every heightmap, one normalized height per texel.
    pub heights: Arc<Vec<f32>>,
    /// The optional shading terms some field uses (`LookFeatures`).
    pub features: LookFeatures,
    // superfargoth patch (own: stable tables): where each table has room, so
    // a field keeps its place in every table while it is unchanged, and what
    // the latest rebuild wrote.
    /// The free ranges of each table. A field's ranges are in its slot.
    pub layout: TableLayout,
    /// What changed since version `writes.since`, per table.
    pub writes: TableWrites,
}

// superfargoth patch (own: stable tables)
/// The free ranges of one table, so a field keeps its place while it is
/// unchanged and a removed field's place goes to the next field that fits.
///
/// A sorted free list with best fit, not `offset-allocator` (in the lock file
/// for Bevy's mesh allocator): a table holds tens of ranges, mostly of one size
/// (streamed fields are alike, so a new field takes a removed one's place
/// exactly), and this list grows in place at its end, where an
/// `offset_allocator::Allocator` has a fixed size and growing it would move
/// every field. It also needs no new dependency of the vendored crate.
#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct FreeList {
    /// Sorted, disjoint and never touching.
    free: Vec<Range<u32>>,
    /// The table's length.
    end: u32,
}

impl FreeList {
    pub(crate) fn end(&self) -> u32 {
        self.end
    }

    /// The start of a free range of `len`: the smallest free range that fits
    /// (the lowest of equals), or else the end, the table growing to at least
    /// `grow_to`; `None` past `cap`.
    fn take(&mut self, len: u32, grow_to: u32, cap: u32) -> Option<u32> {
        if len == 0 {
            return Some(0);
        }
        let best = self
            .free
            .iter()
            .enumerate()
            .filter(|(_, range)| range.end - range.start >= len)
            .min_by_key(|(_, range)| range.end - range.start)
            .map(|(index, _)| index);
        if let Some(index) = best {
            let at = self.free[index].start;
            self.free[index].start += len;
            if self.free[index].is_empty() {
                self.free.remove(index);
            }
            return Some(at);
        }
        let start = match self.free.last() {
            Some(tail) if tail.end == self.end => tail.start,
            _ => self.end,
        };
        let need = start.checked_add(len).filter(|&need| need <= cap)?;
        let grown = need.max(grow_to.min(cap));
        self.release(self.end..grown);
        self.end = grown;
        self.take(len, grow_to, cap)
    }

    /// Gives `range` back.
    fn release(&mut self, range: Range<u32>) {
        if range.is_empty() {
            return;
        }
        let at = self.free.partition_point(|free| free.start < range.start);
        self.free.insert(at, range);
        if at + 1 < self.free.len() && self.free[at].end == self.free[at + 1].start {
            self.free[at].end = self.free.remove(at + 1).end;
        }
        if at > 0 && self.free[at - 1].end == self.free[at].start {
            self.free[at - 1].end = self.free.remove(at).end;
        }
    }
}

// superfargoth patch (own: stable tables)
/// The free ranges of the four tables, in their own units: entries, chunks,
/// mask words and height texels.
#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct TableLayout {
    pub entries: FreeList,
    pub chunks: FreeList,
    pub masks: FreeList,
    pub heights: FreeList,
}

// superfargoth patch (own: stable tables)
/// What one table's latest change wrote, in its units.
#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct Written {
    /// The table was laid out afresh or grew: all of it is new.
    pub full: bool,
    /// The ranges written, sorted and merged (meaningful when not `full`).
    pub ranges: Vec<Range<u32>>,
}

impl Written {
    fn add(&mut self, range: Range<u32>) {
        if !range.is_empty() {
            self.ranges.push(range);
        }
    }

    /// Nothing was written.
    pub(crate) fn is_empty(&self) -> bool {
        !self.full && self.ranges.is_empty()
    }

    fn merged(mut self) -> Self {
        self.ranges.sort_by_key(|range| range.start);
        let mut merged: Vec<Range<u32>> = Vec::with_capacity(self.ranges.len());
        for range in self.ranges {
            match merged.last_mut() {
                Some(last) if last.end >= range.start => last.end = last.end.max(range.end),
                _ => merged.push(range),
            }
        }
        self.ranges = merged;
        self
    }
}

// superfargoth patch (own: stable tables)
/// What changed in each table from version `since` to this one, for an upload
/// of only those ranges. A gap of versions means: upload everything.
#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) struct TableWrites {
    pub since: u64,
    pub fields: Written,
    pub chunks: Written,
    pub masks: Written,
    pub heights: Written,
}

// superfargoth patch (own: stable tables)
/// A chunk no field owns: a hole in the chunk table, skipped by `cull`.
const UNUSED_CHUNK: ChunkGpu = ChunkGpu {
    bounds: Vec4::ZERO,
    ids: UVec4::new(NO_MAP, 0, 0, 0),
};

// superfargoth patch (own: stable tables)
/// One table being rebuilt: the old one, copied on its first write.
struct Table<T> {
    data: Arc<Vec<T>>,
    written: Written,
}

impl<T: Clone + PartialEq> Table<T> {
    /// `old` at `len` (new texels are `fill`), or a new table of `len` if
    /// `fresh`. Taken, not shared, so a table nothing else holds is written in
    /// place.
    fn new(old: Arc<Vec<T>>, len: u32, fill: T, fresh: bool) -> Self {
        let mut data = if fresh { Arc::new(Vec::new()) } else { old };
        let full = fresh || data.len() != len as usize;
        if data.len() != len as usize {
            Arc::make_mut(&mut data).resize(len as usize, fill);
        }
        Self {
            data,
            written: Written {
                full,
                ranges: Vec::new(),
            },
        }
    }

    /// Writes `values` at `at`; with `unless_same`, only if they differ.
    fn write(&mut self, at: u32, values: &[T], unless_same: bool) {
        let range = at as usize..at as usize + values.len();
        if unless_same && self.data[range.clone()] == *values {
            return;
        }
        Arc::make_mut(&mut self.data)[range].clone_from_slice(values);
        self.written.add(at..at + values.len() as u32);
    }

    fn fill(&mut self, range: Range<u32>, value: &T) {
        let values = vec![value.clone(); (range.end - range.start) as usize];
        self.write(range.start, &values, true);
    }
}

impl FieldTables {
    /// The highest ground under `point` (world X and Z) in the chunk of the
    /// field that holds it, or `None` outside every field. A flat field is
    /// exact; on a heightmap it is the top of the 16 m chunk.
    pub(crate) fn highest_ground_at(&self, point: Vec2) -> Option<f32> {
        for slot in self.slots.iter() {
            let field = self.fields.get(slot.first)?;
            let size = Vec2::new(field.extent.x, field.extent.y);
            let counts = (size / CHUNK_SIZE).ceil().as_uvec2().max(UVec2::ONE);
            let local = field.local_of(point);
            if local.cmpge(Vec2::ZERO).all() && local.cmplt(size).all() {
                let cell = (local / CHUNK_SIZE).as_uvec2().min(counts - UVec2::ONE);
                // superfargoth patch (own: stable tables): the field's chunks
                // start where its slot says, not after the previous field's.
                let index = slot.chunks.start + cell.y * counts.x + cell.x;
                return slot
                    .chunks
                    .contains(&index)
                    .then(|| self.chunks.get(index as usize))
                    .flatten()
                    .map(|chunk| chunk.bounds.w);
            }
        }
        None
    }
}

impl FieldTables {
    /// The ground below `point` (world X and Z) as [`highest_ground_at`]
    /// finds it, or, outside every field, the first field's height, or 0.
    ///
    /// [`highest_ground_at`]: FieldTables::highest_ground_at
    pub(crate) fn ground_below(&self, point: Vec2) -> f32 {
        // superfargoth patch (own: stable tables): the first field's entry,
        // which a hole may have pushed off index 0.
        self.highest_ground_at(point)
            .or_else(|| {
                let first = self.slots.first()?;
                self.fields.get(first.first).map(|field| field.min_corner.y)
            })
            .unwrap_or(0.0)
    }
}

/// What one mask in the tables is. An image keeps its key when it is edited,
/// so a field's first edit does not move anything in the tables; a field
/// without an image gets a mask of its own on its first edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum MaskKey {
    /// The field's density map, edited or not.
    Image(AssetId<Image>),
    /// The edit layer of a field that has no density map.
    Edited(Entity),
}

/// The entries of one field in [`FieldTables::fields`], and what they were
/// built from, so a rebuild can tell what has to be built again.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FieldSlot {
    pub entity: Entity,
    pub first: usize,
    pub entries: usize,
    // superfargoth patch (own: stable tables): the field's ranges in the
    // chunk, mask (words) and height (texels) tables.
    pub chunks: Range<u32>,
    pub mask: Option<Range<u32>>,
    pub height: Option<Range<u32>>,
    entry: EntryKey,
    chunk: ChunkKey,
}

/// Everything a field's entries depend on besides the contents of its style:
/// while this and the style are the same, so are the entries.
#[derive(Debug, Clone, PartialEq)]
struct EntryKey {
    style: AssetId<GrassStyle>,
    size: Vec2,
    corner: Vec3,
    axis: Vec2,
    access: u32,
    /// The mask, and its offset in the mask table, in texels.
    mask: Option<(MaskKey, u32)>,
    /// The heightmap, its offset in the height table and its scale.
    height: Option<(AssetId<Image>, u32, f32)>,
    species: usize,
    overrides: GrassFieldOverrides,
    // superfargoth patch (own: world offset): the seed hangs on the world
    // offset too, so a change of the offset alone rewrites the entries.
    seed: u32,
}

/// Everything a field's chunks depend on besides the contents of its
/// heightmap.
#[derive(Debug, Clone, PartialEq)]
struct ChunkKey {
    size: Vec2,
    base: f32,
    height: Option<(AssetId<Image>, f32)>,
    // superfargoth patch (own: stable tables): no `first` (the index of the
    // field's first entry): a moved entry only repoints the chunks, which
    // `rebuild` does without scanning the heightmap again.
}

/// Optional shading terms, compiled into the grass shaders only when some
/// field's style uses them, so the others cost nothing.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct LookFeatures(u8);

impl LookFeatures {
    pub(crate) const GROUND_TINT: Self = Self(1);
    pub(crate) const SHEEN: Self = Self(2);
    pub(crate) const TOON: Self = Self(4);
    /// Blade shape: twist, droop, fold and heads.
    pub(crate) const SHAPE: Self = Self(8);
    /// Colour variation: patches, dry tips and dead blades.
    pub(crate) const VARIATION: Self = Self(16);

    /// The terms a field of `style` uses for its species `look` (the style
    /// itself for its own blades).
    pub(crate) fn of(style: &GrassStyle, look: &GrassBlade) -> Self {
        let mut features = Self::default();
        if style.ground_tint.amount > 0.0 {
            features.0 |= Self::GROUND_TINT.0;
        }
        if look.specular > 0.0 {
            features.0 |= Self::SHEEN.0;
        }
        if style.toon_bands > 0 {
            features.0 |= Self::TOON.0;
        }
        if look.twist > 0.0 || look.droop > 0.0 || look.fold > 0.0 || look.head_length > 0.0 {
            features.0 |= Self::SHAPE.0;
        }
        let variation = &style.variation;
        if variation.patch_amount > 0.0 || variation.dry_tips > 0.0 || variation.dead_share > 0.0 {
            features.0 |= Self::VARIATION.0;
        }
        features
    }

    /// The terms that change where blades are, which is all a shadow map
    /// needs.
    pub(crate) fn geometry(self) -> Self {
        Self(self.0 & Self::SHAPE.0)
    }

    pub(crate) fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    /// Every optional look term with the shader definition that compiles it
    /// in: the one list `shader_defs` and the tests both read, so a term
    /// cannot be given a definition in one place and not the other.
    pub(crate) const TERMS: [(Self, &'static str); 5] = [
        (Self::GROUND_TINT, "GRASS_GROUND_TINT"),
        (Self::SHEEN, "GRASS_SHEEN"),
        (Self::TOON, "GRASS_TOON"),
        (Self::SHAPE, "GRASS_SHAPE"),
        (Self::VARIATION, "GRASS_VARIATION"),
    ];

    /// The shader definitions that compile the terms in.
    pub(crate) fn shader_defs(self) -> impl Iterator<Item = &'static str> {
        Self::TERMS
            .into_iter()
            .filter(move |(feature, _)| self.contains(*feature))
            .map(|(_, def)| def)
    }
}

impl std::ops::BitOr for LookFeatures {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Registers the tables and the systems that keep them current, in
/// [`GrassSystems`] order.
pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<FieldTables>()
        .init_resource::<TablesDirty>()
        .init_resource::<MeshGrounds>()
        .init_resource::<GrassStats>()
        .init_resource::<PackedImages>()
        // superfargoth patch (own: world offset)
        .init_resource::<GrassWorldOffset>()
        .configure_sets(
            PostUpdate,
            (
                GrassSystems::DetectChanges,
                GrassSystems::Edit,
                GrassSystems::Rebuild,
            )
                .chain()
                .after(TransformSystems::Propagate)
                .after(VisibilitySystems::VisibilityPropagate)
                .after(AssetEventSystems),
        )
        .add_systems(
            PostUpdate,
            (
                (
                    warn_unsupported,
                    warn_missing_style,
                    (bake_mesh_grounds, detect_changes).chain(),
                )
                    .in_set(GrassSystems::DetectChanges),
                (rebuild, edit::collect_patches)
                    .chain()
                    .in_set(GrassSystems::Rebuild),
            ),
        );
    edit::plugin(app);
}

// superfargoth patch (own: world offset): `GrassWorldOffset` and `field_seed`.
/// The world XZ position of the render-space origin, in metres.
///
/// An app with a floating origin writes it whenever its origin moves, so a
/// field's blades follow its world position rather than its render-space
/// one; other apps never touch it (it stays zero).
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
pub struct GrassWorldOffset(pub DVec2);

/// The layout seed of a field whose minimum corner is at `corner` (render
/// space XZ, in metres), with the render-space origin at `world_offset`.
///
/// The world corner is rounded to whole centimetres, so an origin shift that
/// moves the corner by a whole number of centimetres keeps the seed.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn field_seed(corner: Vec2, world_offset: DVec2) -> u32 {
    let world = corner.as_dvec2() + world_offset;
    let cm = |metres: f64| (metres * 100.0).round() as i64;
    let (x, z) = (cm(world.x), cm(world.y));
    // Both halves of each axis go in, so far-apart fields don't collide.
    shader_model::hash2(
        shader_model::hash2(x as u32, (x >> 32) as u32),
        shader_model::hash2(z as u32, (z >> 32) as u32),
    )
}

/// What the tables need.
#[derive(Resource, Debug, Default)]
pub(crate) struct TablesDirty {
    /// A field was added, removed, changed or moved.
    fields: bool,
    /// Styles in use that changed.
    styles: HashSet<AssetId<GrassStyle>>,
    /// Masks and heightmaps in use that changed.
    images: HashSet<AssetId<Image>>,
    /// Which fields are drawn changed, which only the entries show.
    access: bool,
}

impl TablesDirty {
    /// Marks the fields as changed: a field's ground was baked, say.
    pub(crate) fn mark_fields(&mut self) {
        self.fields = true;
    }

    /// Whether the tables have to be looked at again, not merely patched.
    fn rebuild(&self) -> bool {
        self.fields || !self.styles.is_empty() || !self.images.is_empty()
    }
}

/// A field whose settings or position changed. `With<GrassField>` matters:
/// without it, every moving camera or character would rebuild the tables.
type FieldChanged = (
    With<GrassField>,
    Or<(
        Changed<GrassField>,
        Changed<GlobalTransform>,
        Changed<GrassFieldOverrides>,
    )>,
);

/// A field shown, hidden, or moved to other render layers.
type AccessChanged = (
    With<GrassField>,
    Or<(Changed<InheritedVisibility>, Changed<RenderLayers>)>,
);

/// The number of render layers a view or field can name.
const LAYER_BITS: usize = 32;

/// How many frames a field may have no style before it is worth a warning:
/// long enough for a field spawned in `Startup` to have its style added, short
/// enough to answer "why is nothing drawing?" while the user is looking.
const MISSING_STYLE_FRAMES: u8 = 3;

/// `layers` as the bit mask the shaders test; a missing component is layer 0,
/// as for meshes. Layers beyond 31 do not fit and are left out.
pub(crate) fn layer_mask(layers: Option<&RenderLayers>) -> u32 {
    layers.map_or(1, |layers| {
        layers
            .iter()
            .filter(|&layer| layer < LAYER_BITS)
            .fold(0, |mask, layer| mask | 1 << layer)
    })
}

/// Whether `layers` names a layer the shaders cannot test.
pub(crate) fn has_unsupported_layer(layers: Option<&RenderLayers>) -> bool {
    layers.is_some_and(|layers| layers.iter().any(|layer| layer >= LAYER_BITS))
}

/// What a field's `access` word holds: its layers, or nothing while hidden.
fn access(visible: bool, layers: Option<&RenderLayers>) -> u32 {
    if visible { layer_mask(layers) } else { 0 }
}

/// A field's size on the ground: `size` times the transform's X and Z scale.
fn scaled_size(field: &GrassField, transform: &GlobalTransform) -> Vec2 {
    let scale = transform.scale();
    field.size * Vec2::new(scale.x, scale.z).abs()
}

/// Where a field lies on the ground, worked out from its components exactly
/// as the tables work it out, for code that cannot wait for a rebuild.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FieldFrame {
    /// The minimum corner (the field's own −X, −Z corner), in world XZ.
    pub corner: Vec2,
    /// The field's X axis in world XZ.
    pub axis: Vec2,
    /// Its size on the ground, scale applied.
    pub size: Vec2,
}

impl FieldFrame {
    pub(crate) fn of(field: &GrassField, transform: &GlobalTransform) -> Self {
        let size = scaled_size(field, transform);
        let axis = field_axis(transform);
        let origin = transform.translation();
        Self {
            corner: Vec2::new(origin.x, origin.z) + turn(axis, -size * 0.5),
            axis,
            size,
        }
    }

    /// A point of the field, in metres along its axes from its minimum
    /// corner, in world XZ.
    pub(crate) fn world_of(&self, local: Vec2) -> Vec2 {
        self.corner + turn(self.axis, local)
    }

    /// The point of the field at `point` (world XZ), in metres along its axes
    /// from its minimum corner.
    pub(crate) fn local_of(&self, point: Vec2) -> Vec2 {
        let offset = point - self.corner;
        Vec2::new(offset.dot(self.axis), offset.dot(self.axis.perp()))
    }
}

/// A field's X axis in world XZ: its transform's turn about Y. Grass grows
/// upright, so a tilt is left out.
fn field_axis(transform: &GlobalTransform) -> Vec2 {
    let (yaw, _, _) = transform.rotation().to_euler(EulerRot::YXZ);
    let (sin, cos) = yaw.sin_cos();
    // An unturned field gets exactly X, so it is placed exactly as before.
    if sin == 0.0 && cos > 0.0 {
        Vec2::X
    } else {
        Vec2::new(cos, -sin)
    }
}

/// Whether a transform tilts the field out of the XZ plane, which a field
/// does not follow.
fn is_tilted(transform: &GlobalTransform) -> bool {
    let rotation = transform.rotation();
    let (yaw, _, _) = rotation.to_euler(EulerRot::YXZ);
    rotation.angle_between(Quat::from_rotation_y(yaw)) > 1e-3
}

/// A field whose transform or layers changed, for `warn_unsupported`.
type TransformOrLayersChanged = (
    With<GrassField>,
    Or<(Changed<GlobalTransform>, Changed<RenderLayers>)>,
);

/// True the first time `unsupported` holds for `entity`, so each field is
/// warned about once; a field that fixes the problem and breaks it again is
/// warned about again.
fn first_time(seen: &mut HashSet<Entity>, entity: Entity, unsupported: bool) -> bool {
    if unsupported {
        seen.insert(entity)
    } else {
        seen.remove(&entity);
        false
    }
}

/// Warns, once per field, about the things a field's entity can ask for that
/// are not supported.
pub(crate) fn warn_unsupported(
    fields: Query<(Entity, &GlobalTransform, Option<&RenderLayers>), TransformOrLayersChanged>,
    mut tilted: Local<HashSet<Entity>>,
    mut layered: Local<HashSet<Entity>>,
) {
    for (entity, transform, layers) in &fields {
        if first_time(&mut tilted, entity, is_tilted(transform)) {
            warn!(
                "bevy_grass: the GrassField {entity} is tilted; grass grows upright, so only \
                 its turn about Y is used. For a slope, use GrassGround::Heightmap or \
                 GrassGround::Mesh."
            );
        }
        if first_time(&mut layered, entity, has_unsupported_layer(layers)) {
            warn!(
                "bevy_grass: the GrassField {entity} is on a render layer above 31; grass \
                 supports layers 0 to 31, so the others are ignored."
            );
        }
    }
}

/// Warns, once per field, when a field still has the empty default style
/// handle after a few frames. Such a handle never loads, so the field waits
/// forever and nothing is drawn; the warning says how to give it a style.
pub(crate) fn warn_missing_style(
    fields: Query<(Entity, &GrassField)>,
    styles: Res<Assets<GrassStyle>>,
    mut waiting: Local<HashMap<Entity, u8>>,
) {
    waiting.retain(|entity, _| fields.contains(*entity));
    let default = AssetId::<GrassStyle>::default();
    for (entity, field) in &fields {
        let missing = field.style.id() == default && !styles.contains(&field.style);
        if !missing {
            waiting.remove(&entity);
            continue;
        }
        let frames = waiting.entry(entity).or_insert(0);
        *frames = frames.saturating_add(1);
        if *frames == MISSING_STYLE_FRAMES {
            warn!(
                "bevy_grass: the GrassField {entity} has no style, so nothing is drawn; use \
                 styles.add(GrassStyle::MEADOW) or load a .grass.ron, or construct it with \
                 GrassField::new(size, GrassStyle::MEADOW, &mut styles)."
            );
        }
    }
}

/// superfargoth patch (own: no echo rebuild): the asset whose content changed,
/// `None` for an asset that only appeared or went unused. `gather` already
/// waits until every image and style a field uses has loaded (the tables stay
/// dirty until then), so `Added` and `LoadedWithDependencies` carry nothing new;
/// marking them re-read and re-joined every new field's images one frame after
/// the rebuild that first read them (GC-3b4a: 5.5 ms twice per block move).
fn changed_id<A: Asset>(event: &AssetEvent<A>) -> Option<AssetId<A>> {
    match event {
        AssetEvent::Modified { id } | AssetEvent::Removed { id } => Some(*id),
        AssetEvent::Added { .. }
        | AssetEvent::LoadedWithDependencies { .. }
        | AssetEvent::Unused { .. } => None,
    }
}

/// Marks the tables for a rebuild when anything they depend on changes.
pub(crate) fn detect_changes(
    mut dirty: ResMut<TablesDirty>,
    changed: Query<(), FieldChanged>,
    access_changed: Query<(), AccessChanged>,
    (mut removed, mut removed_overrides): (
        RemovedComponents<GrassField>,
        RemovedComponents<GrassFieldOverrides>,
    ),
    mut styles: MessageReader<AssetEvent<GrassStyle>>,
    mut images: MessageReader<AssetEvent<Image>>,
    fields: Query<&GrassField>,
) {
    // Only styles and images some field uses matter.
    // superfargoth patch (own: no echo rebuild): `changed_id`, was `event_id`.
    for event in styles.read() {
        let Some(id) = changed_id(event) else { continue };
        if fields.iter().any(|field| field.style.id() == id) {
            dirty.styles.insert(id);
        }
    }
    for event in images.read() {
        let Some(id) = changed_id(event) else { continue };
        let used = fields.iter().any(|field| {
            field
                .density_map
                .as_ref()
                .is_some_and(|mask| mask.id() == id)
                || matches!(&field.ground, GrassGround::Heightmap { image, .. } if image.id() == id)
        });
        if used {
            dirty.images.insert(id);
        }
    }
    let (gone, overrides_gone) = (removed.read().count(), removed_overrides.read().count());
    if !changed.is_empty() || gone > 0 || overrides_gone > 0 {
        dirty.fields = true;
    }
    if !access_changed.is_empty() {
        dirty.access = true;
    }
}

/// What `rebuild` reads of each field.
type FieldInputs<'a> = (
    Entity,
    &'a GrassField,
    &'a GlobalTransform,
    &'a InheritedVisibility,
    Option<&'a RenderLayers>,
    Option<&'a GrassFieldOverrides>,
);

/// Where a field is and which views draw it.
struct Placement {
    size: Vec2,
    /// The minimum corner (the field's own −X, −Z corner), at the field's
    /// base height.
    corner: Vec3,
    /// The field's X axis in world XZ.
    axis: Vec2,
    access: u32,
    // superfargoth patch (own: world offset): the layout seed, from the
    // corner's world position (`field_seed`).
    seed: u32,
}

impl Placement {
    fn of(
        field: &GrassField,
        transform: &GlobalTransform,
        visibility: InheritedVisibility,
        layers: Option<&RenderLayers>,
        world_offset: DVec2, // superfargoth patch (own: world offset)
    ) -> Self {
        let frame = FieldFrame::of(field, transform);
        Self {
            size: frame.size,
            corner: Vec3::new(frame.corner.x, transform.translation().y, frame.corner.y),
            axis: frame.axis,
            access: access(visibility.get(), layers),
            seed: field_seed(frame.corner, world_offset), // superfargoth patch (own: world offset)
        }
    }
}

/// Shows or hides the fields already in the tables, without touching their
/// chunks or images.
fn patch_access(tables: &mut FieldTables, fields: &Query<FieldInputs>) {
    let mut entries = tables.fields.as_ref().clone();
    let mut slots = tables.slots.as_ref().clone();
    let mut written = Written::default(); // superfargoth patch (own: stable tables)
    for slot in &mut slots {
        let Ok((_, _, _, visibility, layers, _)) = fields.get(slot.entity) else {
            continue;
        };
        let bits = access(visibility.get(), layers);
        slot.entry.access = bits;
        // superfargoth patch (own: stable tables): the entries that changed.
        let range = slot.first..slot.first + slot.entries;
        if entries[range.clone()].iter().any(|entry| entry.access.x != bits) {
            written.add(range.start as u32..range.end as u32);
        }
        for entry in &mut entries[range] {
            entry.access.x = bits;
        }
    }
    tables.fields = Arc::new(entries);
    tables.slots = Arc::new(slots);
    // superfargoth patch (own: stable tables)
    tables.writes = TableWrites {
        since: tables.version,
        fields: written.merged(),
        ..TableWrites::default()
    };
    tables.version += 1;
}

/// What `rebuild` reads of each field, once its style and images have loaded.
struct Input<'a> {
    entity: Entity,
    placement: Placement,
    overrides: GrassFieldOverrides,
    style_id: AssetId<GrassStyle>,
    style: &'a GrassStyle,
    mask: Option<MaskInput<'a>>,
    height: Option<(AssetId<Image>, &'a Image, f32)>,
}

/// A field's mask: its image, its edit layer, or both, in which case the
/// layer's texels are the ones drawn.
struct MaskInput<'a> {
    key: MaskKey,
    size: UVec2,
    image: Option<&'a Image>,
    edited: Option<&'a [u32]>,
}

impl MaskInput<'_> {
    /// The image the mask was read from, if any.
    fn image_id(&self) -> Option<AssetId<Image>> {
        match self.key {
            MaskKey::Image(id) => Some(id),
            MaskKey::Edited(_) => None,
        }
    }

    /// The texels, packed (see `PackedImages::mask`).
    fn words(&self, cache: &mut PackedImages) -> Arc<Vec<u32>> {
        match (self.edited, self.image, self.key) {
            (Some(words), ..) => Arc::new(words.to_vec()),
            (None, Some(image), MaskKey::Image(id)) => cache.mask(id, image),
            _ => Arc::default(),
        }
    }
}

/// An input placed in the tables: which entries it gets, and where its images
/// sit in the mask and height tables.
struct Plan<'a> {
    input: &'a Input<'a>,
    /// The style's species that fit, after its own blades.
    species: &'a [GrassSpecies],
    /// The index of the field's first entry.
    first: usize,
    /// Offsets of its mask and heightmap, in texels.
    mask_offset: u32,
    height_offset: u32,
    // superfargoth patch (own: stable tables)
    /// Its slot in the old tables, if it was in them (and they are kept).
    old: Option<usize>,
    /// Its ranges in the chunk, mask (words) and height (texels) tables.
    chunks: Range<u32>,
    mask: Option<Range<u32>>,
    height: Option<Range<u32>>,
    /// Which of its ranges were placed anew, and so must be written.
    placed: [bool; 4],
}

// superfargoth patch (own: stable tables): indices into `Plan::placed`.
const ENTRIES: usize = 0;
const CHUNKS: usize = 1;
const MASK: usize = 2;
const HEIGHT: usize = 3;

impl Plan<'_> {
    fn entry_key(&self) -> EntryKey {
        let input = self.input;
        EntryKey {
            style: input.style_id,
            size: input.placement.size,
            corner: input.placement.corner,
            axis: input.placement.axis,
            access: input.placement.access,
            mask: input.mask.as_ref().map(|mask| (mask.key, self.mask_offset)),
            height: input
                .height
                .map(|(id, _, scale)| (id, self.height_offset, scale)),
            species: self.species.len(),
            overrides: input.overrides,
            seed: input.placement.seed, // superfargoth patch (own: world offset)
        }
    }

    fn chunk_key(&self) -> ChunkKey {
        ChunkKey {
            size: self.input.placement.size,
            base: self.input.placement.corner.y,
            height: self.input.height.map(|(id, _, scale)| (id, scale)),
        }
    }

    // superfargoth patch (own: stable tables): what each table needs.
    /// Its entries, chunks, mask words and height texels.
    fn needs(&self) -> [u32; 4] {
        let counts = (self.input.placement.size / CHUNK_SIZE).ceil().as_uvec2();
        let mask = self
            .input
            .mask
            .as_ref()
            .map_or(0, |mask| packed::mask_words(mask.size.x * mask.size.y));
        let height = self.input.height.map_or(0, |(_, image, _)| {
            let size = image.size();
            size.x * size.y
        });
        [1 + self.species.len() as u32, counts.x * counts.y, mask, height]
    }

    /// Whether the field's heightmap is one of `changed`.
    fn uses_height(&self, changed: &HashSet<AssetId<Image>>) -> bool {
        self.input
            .height
            .is_some_and(|(id, ..)| changed.contains(&id))
    }

    /// Whether the field's mask or heightmap is one of `changed`.
    fn uses(&self, changed: &HashSet<AssetId<Image>>) -> bool {
        let mask = self.input.mask.as_ref().and_then(MaskInput::image_id);
        mask.is_some_and(|id| changed.contains(&id))
            || self
                .input
                .height
                .is_some_and(|(id, ..)| changed.contains(&id))
    }
}

/// Reads each field with its style, mask and heightmap, or `None` while any of
/// them has not loaded (or a mesh ground has not been baked), so a field
/// never appears at the wrong density or height and then jumps. An edited
/// field's mask is its edit layer.
fn gather<'a>(
    sorted: &[FieldInputs<'a>],
    styles: &'a Assets<GrassStyle>,
    images: &'a Assets<Image>,
    grounds: &MeshGrounds,
    edits: &'a EditLayers,
    world_offset: DVec2, // superfargoth patch (own: world offset)
) -> Option<Vec<Input<'a>>> {
    let mut inputs = Vec::with_capacity(sorted.len());
    for (entity, field, transform, visibility, layers, overrides) in sorted {
        let layer = edits.layer_of(*entity, field);
        let mask = match &field.density_map {
            Some(handle) => {
                let image = images.get(handle)?;
                Some(MaskInput {
                    key: MaskKey::Image(handle.id()),
                    size: image.size(),
                    image: Some(image),
                    edited: layer
                        .filter(|layer| layer.size == image.size())
                        .map(|layer| layer.words.as_slice()),
                })
            }
            None => layer.map(|layer| MaskInput {
                key: MaskKey::Edited(*entity),
                size: layer.size,
                image: None,
                edited: Some(layer.words.as_slice()),
            }),
        };
        let height = match &field.ground {
            GrassGround::Flat => None,
            GrassGround::Heightmap {
                image,
                height_scale,
            } => Some((image.id(), images.get(image)?, *height_scale)),
            // A baked ground is in the mesh's units, which the transform's Y
            // scale turns into metres, as it does for the mesh itself.
            GrassGround::Mesh { .. } => {
                let image = grounds.image(*entity)?;
                Some((image.id(), images.get(image)?, transform.scale().y))
            }
        };
        inputs.push(Input {
            entity: *entity,
            // superfargoth patch (own: world offset)
            placement: Placement::of(field, transform, **visibility, *layers, world_offset),
            overrides: overrides.copied().unwrap_or_default(),
            style_id: field.style.id(),
            style: styles.get(&field.style)?,
            mask,
            height,
        });
    }
    Some(inputs)
}

/// What the limits left out.
#[derive(Debug, Default, Clone, Copy)]
struct Dropped {
    fields: usize,
    species: usize,
}

/// Which entries each field gets, and where its images sit in the tables.
/// Fields beyond the table's 256 entries are left out.
fn plan<'a>(inputs: &'a [Input<'a>]) -> (Vec<Plan<'a>>, Dropped) {
    let mut plans = Vec::with_capacity(inputs.len());
    let mut dropped = Dropped::default();
    let mut entry_cursor = 0usize; // superfargoth patch (own: stable tables): no image offsets
    for (index, input) in inputs.iter().enumerate() {
        let species = &input.style.species[..input.style.species.len().min(MAX_SPECIES - 1)];
        dropped.species += input.style.species.len() - species.len();
        if species.len() < input.style.species.len() {
            warn!(
                "bevy_grass mixes at most {MAX_SPECIES} species in a field; {} of a style's {} \
                 species are drawn",
                species.len(),
                input.style.species.len()
            );
        }
        if entry_cursor + 1 + species.len() > MAX_ENTRIES {
            warn!(
                "bevy_grass draws at most {MAX_ENTRIES} fields, each species of a field \
                 counting as one; some fields are not drawn"
            );
            dropped.fields = inputs.len() - index;
            break;
        }
        // superfargoth patch (own: stable tables): `place` sets the offsets;
        // this only counts the entries against the limit.
        plans.push(Plan {
            input,
            species,
            first: 0,
            mask_offset: 0,
            height_offset: 0,
            old: None,
            chunks: 0..0,
            mask: None,
            height: None,
            placed: [false; 4],
        });
        entry_cursor += 1 + species.len();
    }
    (plans, dropped)
}

// superfargoth patch (own: stable tables)
/// Where `place` put the fields.
struct Placed {
    layout: TableLayout,
    /// The tables were laid out afresh (the entries no longer fit around the
    /// holes): every field is placed anew.
    fresh: bool,
    /// Entry and chunk ranges no field holds any more, to be marked unused.
    /// Mask and height ranges are left as they are: no entry addresses them.
    freed_entries: Vec<Range<u32>>,
    freed_chunks: Vec<Range<u32>>,
}

// superfargoth patch (own: stable tables)
/// Gives each field its ranges in the four tables: the ones it had while their
/// sizes are the same, otherwise the best free range, the tables growing (by
/// half at least) when none fits. Removed and resized fields give theirs back
/// first, so a new field of the same size takes a removed one's place. When the
/// entries no longer fit in `MAX_ENTRIES` around the holes, the tables are laid
/// out afresh.
fn place(plans: &mut [Plan], old: &FieldTables) -> Placed {
    place_in(plans, old, false).unwrap_or_else(|| {
        place_in(plans, old, true).unwrap_or_else(|| {
            unreachable!("`plan` keeps the fields' entries within MAX_ENTRIES")
        })
    })
}

fn place_in(plans: &mut [Plan], old: &FieldTables, fresh: bool) -> Option<Placed> {
    let mut placed = Placed {
        layout: if fresh {
            TableLayout::default()
        } else {
            old.layout.clone()
        },
        fresh,
        freed_entries: Vec::new(),
        freed_chunks: Vec::new(),
    };
    // Laid out afresh, the tables take exactly what they need.
    let ends = if fresh {
        [0; 4]
    } else {
        [
            old.layout.entries.end(),
            old.layout.chunks.end(),
            old.layout.masks.end(),
            old.layout.heights.end(),
        ]
    };
    let previous: HashMap<Entity, usize> = if fresh {
        HashMap::new()
    } else {
        old.slots
            .iter()
            .enumerate()
            .map(|(index, slot)| (slot.entity, index))
            .collect()
    };
    let mut live = vec![false; old.slots.len()];
    let Placed {
        layout,
        freed_entries,
        freed_chunks,
        ..
    } = &mut placed;
    // Keep what still fits; give back what does not, and every removed field's.
    for plan in plans.iter_mut() {
        let needs = plan.needs();
        plan.old = previous.get(&plan.input.entity).copied();
        plan.placed = [true; 4];
        let Some(index) = plan.old else { continue };
        live[index] = true;
        let slot = &old.slots[index];
        if slot.entries as u32 == needs[ENTRIES] {
            plan.first = slot.first;
            plan.placed[ENTRIES] = false;
        } else {
            let range = slot.first as u32..(slot.first + slot.entries) as u32;
            layout.entries.release(range.clone());
            freed_entries.push(range);
        }
        if slot.chunks.end - slot.chunks.start == needs[CHUNKS] {
            plan.chunks = slot.chunks.clone();
            plan.placed[CHUNKS] = false;
        } else {
            layout.chunks.release(slot.chunks.clone());
            freed_chunks.push(slot.chunks.clone());
        }
        let wanted = [plan.input.mask.is_some(), plan.input.height.is_some()];
        for (kind, had, list, keep) in [
            (MASK, &slot.mask, &mut layout.masks, wanted[0]),
            (HEIGHT, &slot.height, &mut layout.heights, wanted[1]),
        ] {
            let Some(range) = had else { continue };
            if keep && range.end - range.start == needs[kind] {
                plan.placed[kind] = false;
                if kind == MASK {
                    plan.mask = Some(range.clone());
                } else {
                    plan.height = Some(range.clone());
                }
            } else {
                list.release(range.clone());
            }
        }
    }
    // Laid out afresh, the old slots hold nothing in the new layout.
    let removed = old.slots.iter().zip(&live).filter(|(_, live)| !fresh && !**live);
    for (slot, _) in removed {
        let entries = slot.first as u32..(slot.first + slot.entries) as u32;
        layout.entries.release(entries.clone());
        freed_entries.push(entries);
        layout.chunks.release(slot.chunks.clone());
        freed_chunks.push(slot.chunks.clone());
        for (had, list) in [
            (&slot.mask, &mut layout.masks),
            (&slot.height, &mut layout.heights),
        ] {
            if let Some(range) = had {
                list.release(range.clone());
            }
        }
    }
    // Place the rest.
    let grow = |end: u32| end.saturating_add(end / 2);
    let cap = MAX_ENTRIES as u32;
    for plan in plans.iter_mut() {
        let needs = plan.needs();
        if plan.placed[ENTRIES] {
            plan.first = layout.entries.take(needs[ENTRIES], grow(ends[ENTRIES]), cap)? as usize;
        }
        if plan.placed[CHUNKS] {
            let at = layout.chunks.take(needs[CHUNKS], grow(ends[CHUNKS]), u32::MAX)?;
            plan.chunks = at..at + needs[CHUNKS];
        }
        if plan.placed[MASK] && plan.input.mask.is_some() {
            let at = layout.masks.take(needs[MASK], grow(ends[MASK]), u32::MAX)?;
            plan.mask = Some(at..at + needs[MASK]);
        }
        if plan.placed[HEIGHT] && plan.input.height.is_some() {
            let at = layout.heights.take(needs[HEIGHT], grow(ends[HEIGHT]), u32::MAX)?;
            plan.height = Some(at..at + needs[HEIGHT]);
        }
        plan.mask_offset = plan.mask.as_ref().map_or(0, |mask| mask.start * 2);
        plan.height_offset = plan.height.as_ref().map_or(0, |height| height.start);
    }
    Some(placed)
}

// superfargoth patch (own: stable tables): the tables are written in place,
// field by field, instead of joined afresh: a field that did not change keeps
// its bytes at the same offsets, and only what changed is written (and
// recorded in `Written`).

/// The chunks: a field's are worked out again when it is new, was placed
/// anew, resized, moved up or down or its heightmap changed, and only
/// repointed when its entries moved. A removed field's are marked unused.
fn write_chunks(
    plans: &[Plan],
    old: &mut FieldTables,
    placed: &Placed,
    changed: &HashSet<AssetId<Image>>,
    cache: &mut PackedImages,
) -> Table<ChunkGpu> {
    let end = placed.layout.chunks.end();
    let mut chunks = Table::new(std::mem::take(&mut old.chunks), end, UNUSED_CHUNK, placed.fresh);
    for range in &placed.freed_chunks {
        chunks.fill(range.clone(), &UNUSED_CHUNK);
    }
    let mut scratch = Vec::new();
    for plan in plans {
        let slot = plan.old.map(|index| &old.slots[index]);
        let rescan = plan.placed[CHUNKS]
            || slot.is_none_or(|slot| slot.chunk != plan.chunk_key())
            || plan.uses_height(changed);
        scratch.clear();
        if rescan {
            let heights = plan
                .input
                .height
                .map(|(id, image, _)| cache.heights(id, image));
            field_chunks(plan, heights.as_ref().map(|h| h.as_slice()), &mut scratch);
        } else if slot.is_some_and(|slot| slot.first != plan.first) {
            let range = plan.chunks.start as usize..plan.chunks.end as usize;
            scratch.extend_from_slice(&chunks.data[range]);
            for chunk in &mut scratch {
                chunk.ids.x = plan.first as u32;
            }
        } else {
            continue;
        }
        chunks.write(plan.chunks.start, &scratch, true);
    }
    chunks
}

/// The heightmaps of the fields that are new, were placed anew, or whose
/// heightmap is another or changed.
fn write_heights(
    plans: &[Plan],
    old: &mut FieldTables,
    placed: &Placed,
    changed: &HashSet<AssetId<Image>>,
    cache: &mut PackedImages,
) -> Table<f32> {
    let end = placed.layout.heights.end();
    let mut heights = Table::new(std::mem::take(&mut old.heights), end, 0.0, placed.fresh);
    for plan in plans {
        let (Some((id, image, _)), Some(range)) = (plan.input.height, &plan.height) else {
            continue;
        };
        let slot = plan.old.map(|index| &old.slots[index]);
        let same = !plan.placed[HEIGHT]
            && slot.is_some_and(|slot| slot.entry.height.is_some_and(|(was, ..)| was == id))
            && !changed.contains(&id);
        if !same {
            let texels = cache.heights(id, image);
            let count = texels.len().min((range.end - range.start) as usize);
            heights.write(range.start, &texels[..count], true);
        }
    }
    heights
}

/// Whether a field's mask must be written: it is new, was placed anew, is
/// another mask, or its image changed.
fn mask_stale(plan: &Plan, old: &FieldTables, changed: &HashSet<AssetId<Image>>) -> bool {
    let Some(mask) = &plan.input.mask else {
        return false;
    };
    let slot = plan.old.map(|index| &old.slots[index]);
    plan.placed[MASK]
        || slot.is_none_or(|slot| slot.entry.mask.is_none_or(|(key, _)| key != mask.key))
        || mask.image_id().is_some_and(|id| changed.contains(&id))
}

/// The stale masks (`mask_stale`), and with `refresh` every edited one too:
/// edits reach the GPU as patches, so the masks here lag behind them, which is
/// fine until the masks are uploaded with something else.
fn write_masks(
    plans: &[Plan],
    old: &mut FieldTables,
    placed: &Placed,
    changed: &HashSet<AssetId<Image>>,
    (cache, refresh): (&mut PackedImages, bool),
) -> Table<u32> {
    let end = placed.layout.masks.end();
    let mut masks = Table::new(std::mem::take(&mut old.masks), end, 0, placed.fresh);
    for plan in plans {
        let (Some(mask), Some(range)) = (&plan.input.mask, &plan.mask) else {
            continue;
        };
        if mask_stale(plan, old, changed) || (refresh && mask.edited.is_some()) {
            let words = mask.words(cache);
            let count = words.len().min((range.end - range.start) as usize);
            // Always written: the GPU's copy may hold a removed field's
            // edits that this copy never saw.
            masks.write(range.start, &words[..count], false);
        }
    }
    masks
}

/// The field table, with the slots that say what each entry was built from.
struct Built {
    entries: Table<FieldGpu>,
    slots: Vec<FieldSlot>,
    features: LookFeatures,
    /// Entries worked out again, not kept.
    rewritten: u64,
}

/// Each field's entries: kept when its style, place and images are the same
/// as when they were built, worked out again otherwise. A removed field's are
/// cleared (hidden everywhere).
fn write_entries(
    plans: &[Plan],
    old: &mut FieldTables,
    placed: &Placed,
    changed_styles: &HashSet<AssetId<GrassStyle>>,
    changed_images: &HashSet<AssetId<Image>>,
) -> Built {
    let end = placed.layout.entries.end();
    let mut built = Built {
        entries: Table::new(std::mem::take(&mut old.fields), end, FieldGpu::default(), placed.fresh),
        slots: Vec::with_capacity(plans.len()),
        features: LookFeatures::default(),
        rewritten: 0,
    };
    for range in &placed.freed_entries {
        built.entries.fill(range.clone(), &FieldGpu::default());
    }
    for plan in plans {
        let (entry, chunk) = (plan.entry_key(), plan.chunk_key());
        let count = 1 + plan.species.len();
        let kept = !plan.placed[ENTRIES]
            && plan.old.is_some_and(|index| {
                let slot = &old.slots[index];
                slot.entry == entry
                    && slot.entries == count
                    && !changed_styles.contains(&plan.input.style_id)
                    && !plan.uses(changed_images)
            });
        if !kept {
            built.entries.write(plan.first as u32, &field_entries(plan), true);
            built.rewritten += count as u64;
        }
        built.slots.push(FieldSlot {
            entity: plan.input.entity,
            first: plan.first,
            entries: count,
            chunks: plan.chunks.clone(),
            mask: plan.mask.clone(),
            height: plan.height.clone(),
            entry,
            chunk,
        });
        let style = plan.input.style;
        for look in std::iter::once(&style.blade).chain(plan.species.iter().map(|s| &s.blade)) {
            built.features = built.features | LookFeatures::of(style, look);
        }
    }
    built
}

/// Brings the tables up to date with what changed, building only what that
/// needs, once every input has loaded.
///
/// - A mask or heightmap is read from its image only when that image changed
///   (`PackedImages`).
/// - A field's entries are rewritten only when its style changed, or it moved,
///   or its images did.
/// - The chunks, masks and heightmaps are kept (and not uploaded again) while
///   no field resized, moved up or down, or changed images. A style change
///   alone, a colour changing every frame for a season or the time of day,
///   rewrites the entries of the fields using it and nothing else.
/// - A pass that finds the tables unchanged changes nothing.
/// - superfargoth patch (own: stable tables): every field keeps its ranges in
///   the four tables while their sizes hold (`place`), so adding or removing
///   a field writes that field's ranges only (`FieldTables::writes`).
pub(crate) fn rebuild(
    mut dirty: ResMut<TablesDirty>,
    mut tables: ResMut<FieldTables>,
    fields: Query<FieldInputs>,
    (styles, images, grounds): (
        Res<Assets<GrassStyle>>,
        Res<Assets<Image>>,
        Res<MeshGrounds>,
    ),
    (mut cache, mut edits): (ResMut<PackedImages>, ResMut<EditLayers>),
    mut stats: ResMut<GrassStats>,
    world_offset: Res<GrassWorldOffset>, // superfargoth patch (own: world offset)
) {
    // superfargoth patch (own: world offset): a new offset alone re-derives
    // every field's seed (`EntryKey::seed` rewrites the entries it changed).
    if world_offset.is_changed() {
        dirty.fields = true;
    }
    if !dirty.rebuild() {
        if dirty.access {
            dirty.access = false;
            patch_access(&mut tables, &fields);
        }
        return;
    }
    let gathering = Instant::now(); // superfargoth patch (own: rebuild phases)
    let mut sorted: Vec<_> = fields.iter().collect();
    sorted.sort_by_key(|(entity, ..)| *entity);
    let edits_ahead = edits.ahead;
    // superfargoth patch (own: world offset): `world_offset.0`
    let Some(inputs) = gather(&sorted, &styles, &images, &grounds, &edits, world_offset.0) else {
        return;
    };
    // superfargoth patch (own: rebuild phases): each phase's time.
    let mut phases = RebuildPhases {
        gather: gathering.elapsed(),
        ..RebuildPhases::default()
    };
    let mut lap = Instant::now();
    let mut split = |phase: &mut Duration| {
        let now = Instant::now();
        *phase = now - lap;
        lap = now;
    };
    // This pass reads every field's visibility too, and takes care of
    // everything that changed.
    let changed_styles = std::mem::take(&mut dirty.styles);
    let changed_images = std::mem::take(&mut dirty.images);
    dirty.fields = false;
    dirty.access = false;

    let started = Instant::now();
    cache.forget(&changed_images);
    // superfargoth patch (own: stable tables): taken, not cloned, so the
    // tables are written in place when nothing else (the render world) still
    // holds them; put back unchanged when nothing changed.
    let mut old = std::mem::take(tables.bypass_change_detection());
    split(&mut phases.clone); // superfargoth patch (own: rebuild phases)
    // superfargoth patch (own: stable tables): each field placed (`place`),
    // then only what changed written in place.
    let (mut plans, dropped) = plan(&inputs);
    let placed = place(&mut plans, &old);
    split(&mut phases.plan); // superfargoth patch (own: rebuild phases)
    let chunks = write_chunks(&plans, &mut old, &placed, &changed_images, &mut cache);
    split(&mut phases.chunks); // superfargoth patch (own: rebuild phases)
    let heights = write_heights(&plans, &mut old, &placed, &changed_images, &mut cache);
    // The masks go to the GPU with the chunks and heightmaps, so the edits
    // that reached it as patches are written here first.
    let structure = !chunks.written.is_empty()
        || !heights.written.is_empty()
        || plans.iter().any(|plan| mask_stale(plan, &old, &changed_images));
    let refresh = edits_ahead && structure;
    let masks = write_masks(
        &plans,
        &mut old,
        &placed,
        &changed_images,
        (&mut cache, refresh),
    );
    let mask_ids: Vec<_> = plans
        .iter()
        .filter_map(|plan| plan.input.mask.as_ref().map(|mask| mask.key))
        .collect();
    let height_ids: Vec<_> = plans
        .iter()
        .filter_map(|plan| plan.input.height.map(|(id, ..)| id))
        .collect();
    let mask_images: Vec<_> = plans
        .iter()
        .filter_map(|plan| plan.input.mask.as_ref().and_then(MaskInput::image_id))
        .collect();
    cache.retain(&mask_images, &height_ids);
    split(&mut phases.images); // superfargoth patch (own: rebuild phases)
    let built = write_entries(&plans, &mut old, &placed, &changed_styles, &changed_images);
    split(&mut phases.entries); // superfargoth patch (own: rebuild phases)
    drop(plans);
    drop(inputs);
    if refresh {
        // The edited masks were written from the layers as they are now.
        edits.ahead = false;
    }

    let layout_changed =
        !(chunks.written.is_empty() && masks.written.is_empty() && heights.written.is_empty());
    let mut counted = stats.tables;
    counted.entries_rewritten += built.rewritten;
    counted.images_read = cache.reads;
    if !layout_changed
        && built.entries.written.is_empty()
        && built.slots == *old.slots
        && built.features == old.features
    {
        // Something changed that no field shows: another style, say. There is
        // nothing to upload.
        if counted != stats.tables {
            stats.tables = counted;
        }
        // superfargoth patch (own: stable tables): the same tables back.
        *tables.bypass_change_detection() = FieldTables {
            fields: built.entries.data,
            chunks: chunks.data,
            masks: masks.data,
            heights: heights.data,
            ..old
        };
        return;
    }
    let fields_len = built.slots.len();
    let chunks_len = built
        .slots
        .iter()
        .map(|slot| (slot.chunks.end - slot.chunks.start) as usize)
        .sum();
    *tables = FieldTables {
        version: old.version + 1,
        structure_version: old.structure_version + u64::from(layout_changed),
        fields: built.entries.data,
        slots: Arc::new(built.slots),
        chunks: chunks.data,
        masks: masks.data,
        heights: heights.data,
        mask_ids: if *old.mask_ids == mask_ids {
            old.mask_ids.clone()
        } else {
            Arc::new(mask_ids)
        },
        height_ids: if *old.height_ids == height_ids {
            old.height_ids.clone()
        } else {
            Arc::new(height_ids)
        },
        features: built.features,
        layout: placed.layout,
        writes: TableWrites {
            since: old.version,
            fields: built.entries.written.merged(),
            chunks: chunks.written.merged(),
            masks: masks.written.merged(),
            heights: heights.written.merged(),
        },
    };
    split(&mut phases.finish); // superfargoth patch (own: rebuild phases)
    stats.tables = TableStats {
        fields: fields_len,
        chunks: chunks_len,
        fields_dropped: dropped.fields,
        species_dropped: dropped.species,
        rebuilds: counted.rebuilds + 1,
        layout_rebuilds: counted.layout_rebuilds + u64::from(layout_changed),
        last_rebuild: started.elapsed(),
        last_rebuild_phases: phases, // superfargoth patch (own: rebuild phases)
        ..counted
    };
}

/// The entry of one look of a field: `field_entry` (where the field is and
/// what belongs to the whole field) with `look`'s blades. `style` is the
/// field's own style, whose density, toon bands and patch
/// colour every species shares.
fn look_entry(
    field_entry: &FieldGpu,
    style: &GrassStyle,
    look: &GrassBlade,
    overrides: &GrassFieldOverrides,
    mix: Vec4,
) -> FieldGpu {
    let (root, tip) = (linear(look.root_color), linear(look.tip_color));
    let patch = linear(style.variation.patch_color).truncate();
    let tint = overrides
        .tint
        .map_or(Vec3::ONE, |tint| linear(tint).truncate());
    let height = overrides.height_scale.max(0.0);
    FieldGpu {
        shape: Vec4::new(
            look.height.x * height,
            look.height.y * height,
            look.width,
            style.density * overrides.density_scale.max(0.0),
        ),
        motion: Vec4::new(
            look.lean,
            look.stiffness,
            1.0 - overrides.wind_scale.max(0.0),
            look.root_occlusion,
        ),
        root_color: (root.truncate() * tint).extend(root.w),
        // The tip colour's alpha carries the translucency.
        tip_color: (tip.truncate() * tint).extend(look.translucency),
        look: Vec4::new(
            look.specular.clamp(0.0, 1.0),
            style.toon_bands as f32,
            0.0,
            0.0,
        ),
        mix,
        form: Vec4::new(
            look.twist.max(0.0),
            look.droop.clamp(0.0, 1.0),
            look.fold.clamp(0.0, 1.0),
            look.head_length.clamp(0.0, 0.9),
        ),
        head: linear(look.head_color)
            .truncate()
            .extend(look.head_width.max(0.0)),
        // In a patch the whole blade is scaled by how far its tip colour
        // moves toward the patch colour.
        patches: (patch / tip.truncate().max(Vec3::splat(1e-3)))
            .extend(style.variation.patch_amount.clamp(0.0, 1.0)),
        ..*field_entry
    }
}

/// A field's entries: its own blades first, then each species, all repeating
/// the field's placement.
fn field_entries(plan: &Plan) -> Vec<FieldGpu> {
    let Input {
        placement,
        style,
        mask,
        height,
        ..
    } = plan.input;
    let Placement {
        size,
        corner,
        axis,
        access,
        seed, // superfargoth patch (own: world offset)
    } = *placement;
    let overrides = &plan.input.overrides;
    let layout =
        ChunkLayout::for_density(CHUNK_SIZE, style.density * overrides.density_scale.max(0.0));
    let (mask_offset, mask_size) = match mask {
        Some(mask) => (plan.mask_offset, mask.size),
        None => (NO_MAP, UVec2::ZERO),
    };
    let (height_offset, height_size, height_scale) = match height {
        Some((_, image, scale)) => (plan.height_offset, image.size(), *scale),
        None => (NO_MAP, UVec2::ZERO, 0.0),
    };

    let variation = &style.variation;
    let field_entry = FieldGpu {
        min_corner: corner.extend(0.0),
        extent: Vec4::new(size.x, size.y, height_scale, layout.tile_size),
        // The seed comes from where the field is, not from its entity, so
        // the same field looks the same however the scene was spawned.
        // superfargoth patch (own: world offset): from the corner's world
        // position (`field_seed`), not its render-space f32 bits.
        placement: UVec4::new(
            seed,
            layout.tiles_per_side,
            mask_offset,
            height_offset,
        ),
        map_sizes: UVec4::new(mask_size.x, mask_size.y, height_size.x, height_size.y),
        ground: linear(style.ground_tint.color)
            .truncate()
            .extend(style.ground_tint.amount.clamp(0.0, 1.0)),
        variation: Vec4::new(
            variation.patch_size.max(0.01),
            variation.dry_tips.clamp(0.0, 1.0),
            variation.dead_share.clamp(0.0, 1.0),
            0.0,
        ),
        tufts: Vec4::new(
            variation.tufts.clamp(0.0, 1.0),
            variation.tuft_spacing.max(0.01),
            if variation.bare_share > 0.0 {
                shader_model::bare_threshold(variation.bare_share)
            } else {
                -1.0
            },
            variation.bare_patch_size.max(0.01),
        ),
        access: UVec4::new(access, 0, 0, 0),
        axis: axis.extend(0.0).extend(0.0),
        ..FieldGpu::default()
    };
    let tallest = plan
        .species
        .iter()
        .map(|species| species.blade.height.y)
        .fold(style.blade.height.y, f32::max)
        * overrides.height_scale.max(0.0);
    let chances = species_chances(plan.species);
    let looks = std::iter::once((&style.blade, 1.0, 0.0)).chain(
        plan.species
            .iter()
            .zip(&chances)
            .map(|(species, chance)| (&species.blade, *chance, species.clustering)),
    );
    looks
        .map(|(look, chance, clustering)| {
            let mix = Vec4::new(
                chance,
                clustering.clamp(0.0, 1.0),
                (1 + plan.species.len()) as f32,
                tallest,
            );
            look_entry(&field_entry, style, look, overrides, mix)
        })
        .collect()
}

/// A field's chunks: the 16 m squares of it, each with the lowest and highest
/// ground in it. `heights` is the field's heightmap, if it has one.
fn field_chunks(plan: &Plan, heights: Option<&[f32]>, out: &mut Vec<ChunkGpu>) {
    let Placement { size, corner, .. } = plan.input.placement;
    let height_scale = plan.input.height.map_or(0.0, |(_, _, scale)| scale);
    let height_size = plan
        .input
        .height
        .map_or(UVec2::ZERO, |(_, image, _)| image.size());
    let counts = (size / CHUNK_SIZE).ceil().as_uvec2();
    for chunk_z in 0..counts.y {
        for chunk_x in 0..counts.x {
            let local = Vec2::new(chunk_x as f32, chunk_z as f32) * CHUNK_SIZE;
            let (low, high) = match heights {
                None => (corner.y, corner.y),
                Some(heights) => {
                    let (low, high) =
                        height_range(heights, height_size, local / size, CHUNK_SIZE / size);
                    (
                        corner.y + low * height_scale,
                        corner.y + high * height_scale,
                    )
                }
            };
            out.push(ChunkGpu {
                bounds: Vec4::new(local.x, local.y, low, high),
                ids: UVec4::new(plan.first as u32, chunk_x, chunk_z, 0),
            });
        }
    }
}

fn linear(color: bevy::color::Color) -> Vec4 {
    Vec4::from_array(LinearRgba::from(color).to_f32_array())
}

/// Each species' chance of taking a blade that the species listed before it
/// left. Shares adding up to more than 1 are scaled down to 1.
fn species_chances(species: &[GrassSpecies]) -> Vec<f32> {
    let total: f32 = species.iter().map(|s| s.share.max(0.0)).sum();
    let scale = if total > 1.0 { 1.0 / total } else { 1.0 };
    let mut left = 1.0;
    species
        .iter()
        .map(|s| {
            let share = s.share.max(0.0) * scale;
            let chance = if left > 1e-6 {
                (share / left).min(1.0)
            } else {
                0.0
            };
            left -= share;
            chance
        })
        .collect()
}

/// Lowest and highest normalized height over a UV rectangle, one texel wider
/// on every side so bilinear sampling at the edges is covered. Holes in a
/// mesh ground hold no blades, so they are left out.
fn height_range(heights: &[f32], size: UVec2, uv_min: Vec2, uv_size: Vec2) -> (f32, f32) {
    let texels = size.as_vec2();
    let lo = ((uv_min * texels).floor().as_ivec2() - 1)
        .max(bevy::math::IVec2::ZERO)
        .as_uvec2();
    let hi = (((uv_min + uv_size) * texels).ceil().as_ivec2() + 1)
        .min(size.as_ivec2() - 1)
        .max(bevy::math::IVec2::ZERO)
        .as_uvec2();
    let mut range = (f32::MAX, f32::MIN);
    for y in lo.y..=hi.y {
        for x in lo.x..=hi.x {
            let h = heights[(y * size.x + x) as usize];
            if h > HOLE * 0.5 {
                range = (range.0.min(h), range.1.max(h));
            }
        }
    }
    if range.0 > range.1 { (0.0, 0.0) } else { range }
}

// superfargoth patch (own: stable tables): what the tests of the crate that
// vendors this one read of the tables (`superfargoth_test`); cargo cannot run
// this crate's own tests there.
/// One field in the tables: its ranges, in each table's units (entries,
/// chunks, mask words, height texels), and the words the GPU reads for it.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldView {
    /// The field.
    pub entity: Entity,
    /// Its entries.
    pub entries: Range<u32>,
    /// Its chunks.
    pub chunks: Range<u32>,
    /// Its mask, in words.
    pub mask: Option<Range<u32>>,
    /// Its heightmap, in texels.
    pub height: Option<Range<u32>>,
    /// Its entries, chunks, mask words and height texels, as 32-bit words.
    pub words: Vec<u32>,
}

/// One table's latest writes (`Written`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WrittenView {
    /// All of the table is new.
    pub full: bool,
    /// The ranges written, sorted and merged.
    pub ranges: Vec<Range<u32>>,
}

/// The tables as the tests see them.
#[derive(Debug, Clone, PartialEq)]
pub struct TablesView {
    /// `FieldTables::version`.
    pub version: u64,
    /// `FieldTables::structure_version`.
    pub structure_version: u64,
    /// The version the writes lead from.
    pub since: u64,
    /// In entity order.
    pub fields: Vec<FieldView>,
    /// Entries, chunks, masks and heights.
    pub writes: [WrittenView; 4],
    /// The length of each table, in the same order.
    pub lens: [u32; 4],
}

fn vec_words(v: Vec4) -> [u32; 4] {
    v.to_array().map(f32::to_bits)
}

fn entry_words(entry: &FieldGpu, out: &mut Vec<u32>) {
    let FieldGpu {
        min_corner,
        extent,
        shape,
        motion,
        root_color,
        tip_color,
        placement,
        map_sizes,
        ground,
        look,
        mix,
        form,
        head,
        patches,
        variation,
        tufts,
        access,
        axis,
    } = *entry;
    for v in [min_corner, extent, shape, motion, root_color, tip_color] {
        out.extend(vec_words(v));
    }
    out.extend(placement.to_array());
    out.extend(map_sizes.to_array());
    for v in [ground, look, mix, form, head, patches, variation, tufts] {
        out.extend(vec_words(v));
    }
    out.extend(access.to_array());
    out.extend(vec_words(axis));
}

/// The tables in `world`, as the tests see them.
pub fn tables_view(world: &World) -> TablesView {
    let tables = world.resource::<FieldTables>();
    let view = |written: &Written| WrittenView {
        full: written.full,
        ranges: written.ranges.clone(),
    };
    let fields = tables
        .slots
        .iter()
        .map(|slot| {
            let mut words = Vec::new();
            for entry in &tables.fields[slot.first..slot.first + slot.entries] {
                entry_words(entry, &mut words);
            }
            for chunk in &tables.chunks[slot.chunks.start as usize..slot.chunks.end as usize] {
                words.extend(vec_words(chunk.bounds));
                words.extend(chunk.ids.to_array());
            }
            if let Some(mask) = &slot.mask {
                words.extend_from_slice(&tables.masks[mask.start as usize..mask.end as usize]);
            }
            if let Some(height) = &slot.height {
                let texels = &tables.heights[height.start as usize..height.end as usize];
                words.extend(texels.iter().map(|h| h.to_bits()));
            }
            FieldView {
                entity: slot.entity,
                entries: slot.first as u32..(slot.first + slot.entries) as u32,
                chunks: slot.chunks.clone(),
                mask: slot.mask.clone(),
                height: slot.height.clone(),
                words,
            }
        })
        .collect();
    let writes = &tables.writes;
    TablesView {
        version: tables.version,
        structure_version: tables.structure_version,
        since: writes.since,
        fields,
        writes: [
            view(&writes.fields),
            view(&writes.chunks),
            view(&writes.masks),
            view(&writes.heights),
        ],
        lens: [
            tables.fields.len() as u32,
            tables.chunks.len() as u32,
            tables.masks.len() as u32,
            tables.heights.len() as u32,
        ],
    }
}

#[cfg(test)]
mod tests;
