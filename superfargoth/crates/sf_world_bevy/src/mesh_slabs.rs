//! Mesh slabs (ROADMAP 1.22 F4): one vertex slab per vertex stride, sized so a layout never spills into a
//! second slab, plus a log census of the slabs.
//!
//! Bevy's mesh allocator (`bevy_render::mesh::allocator`) packs meshes into slabs keyed by element class and
//! byte stride (vertex), index format (u16 / u32), metadata and morph targets. A batch set's key holds the
//! mesh's slab ids, so meshes of one pipeline split into a new batch set (one more multi-draw and its
//! encode cost, ~14 us of render thread each on the walk) only when they sit in different slabs. A
//! pipeline is already per vertex layout and a stride is per layout, so with one slab per stride the
//! vertex slab costs no batch set the pipeline doesn't. A second slab of the same stride only appears when
//! the first is full at `max_slab_size` (stock 512 MiB): then every pipeline of that layout doubles its
//! batch sets in every view, and the spill slab regrows from 1 MiB in the middle of play.
//!
//! The walk's mesh memory is ~0.55 GB in 16-17 slabs (bench gauge `mesh_allocator_slabs_size`); the
//! largest is likely the lit-static stride, one or two 1.5x growth steps below the stock 512 MiB cap (the
//! census line below says). This plugin raises the cap to [`TARGET_MAX_SLAB_BYTES`] (one arena per layout,
//! the GPU-driven standard: SPUD keeps one vertex and one index arena), clamped to the device's
//! buffer limits. It is a cap, not a reservation: slabs still start at 1 MiB and grow by 1.5x, so no
//! memory is spent until a layout passes 512 MiB. Pre-sizing per layout from a known budget would also
//! save the regrowth copies at load, but the allocator has one `min_slab_size` for every stride (17
//! slabs x a large minimum would waste hundreds of MiB): that needs a per-layout reserve in the vendored
//! allocator (follow-up).
//!
//! Log lines (no picture): `mesh.slabs: cap ...` once, `mesh.slabs: <n> slabs, ...` whenever the slab count
//! changes or the slabs grow by 64 MiB, with a `warn` naming any stride held in more than one general slab.

use std::cmp::Reverse;
use std::fmt::Write as _;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_render::mesh::allocator::{ElementClass, MeshAllocator, MeshAllocatorSettings, allocate_and_free_meshes};
use bevy_render::render_resource::BufferUsages;
use bevy_render::renderer::RenderDevice;
use bevy_render::slab_allocator::{Slab, SlabAllocatorSettings, SlabItemLayout};
use bevy_render::{Render, RenderApp, RenderSystems};

/// The slab cap this plugin asks for: 2 GiB, ~4x the walk's whole mesh memory and a quarter of an 8 GB
/// laptop GPU. Clamped by [`slab_settings`] to what the device can create and bind.
pub const TARGET_MAX_SLAB_BYTES: u64 = 1 << 31;

const MIB: u64 = 1024 * 1024;

/// Logs the slab census and, with [`raise_cap`](Self::raise_cap), sets [`MeshAllocatorSettings`] from
/// [`slab_settings`] on the first render frame (after every plugin added its `extra_buffer_usages`, before
/// the first mesh is allocated). Game-agnostic; `raise_cap: false` keeps stock settings (an A/B) and still
/// logs the census.
#[derive(Debug, Clone, Copy)]
pub struct MeshSlabsPlugin {
    /// Cap slabs at [`TARGET_MAX_SLAB_BYTES`] (device-clamped) instead of stock 512 MiB.
    pub raise_cap: bool,
}

impl Default for MeshSlabsPlugin {
    fn default() -> Self {
        Self { raise_cap: true }
    }
}

impl Plugin for MeshSlabsPlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        if self.raise_cap {
            render_app.add_systems(
                Render,
                (apply_slab_settings, log_slab_census)
                    .chain()
                    .in_set(RenderSystems::PrepareAssets)
                    .before(allocate_and_free_meshes),
            );
        } else {
            log::info!("mesh.slabs: stock settings (cap 512 MiB per slab)");
            render_app.add_systems(Render, log_slab_census.in_set(RenderSystems::PrepareAssets).before(allocate_and_free_meshes));
        }
    }
}

/// Slab settings for a device: [`TARGET_MAX_SLAB_BYTES`] clamped to `max_buffer_size` and, when slabs are
/// also bound as storage buffers (ray tracing, wide wireframes: `STORAGE` in `extra_usages`), to
/// `max_storage_buffer_binding_size`. A mesh that alone fills half a slab still gets its own buffer, as
/// in stock Bevy; minimum size and growth stay stock (1 MiB, 1.5x).
#[must_use]
pub fn slab_settings(max_buffer_size: u64, max_storage_binding: u64, extra_usages: BufferUsages) -> SlabAllocatorSettings {
    let stock = SlabAllocatorSettings::default();
    let mut max = TARGET_MAX_SLAB_BYTES.min(max_buffer_size);
    if extra_usages.contains(BufferUsages::STORAGE) {
        max = max.min(max_storage_binding);
    }
    // Never below stock (a binding limit under 512 MiB keeps stock Bevy's cap and its behaviour).
    let max = max.max(stock.max_slab_size.min(max_buffer_size));
    SlabAllocatorSettings { max_slab_size: max, large_threshold: max / 2, ..stock }
}

fn apply_slab_settings(mut done: Local<bool>, device: Res<RenderDevice>, mut settings: ResMut<MeshAllocatorSettings>) {
    if *done {
        return;
    }
    *done = true;
    let limits = device.limits();
    let wanted = slab_settings(limits.max_buffer_size, limits.max_storage_buffer_binding_size, settings.extra_buffer_usages);
    log::info!(
        "mesh.slabs: cap {} MiB per slab, own slab from {} MiB (stock 512/256; device max_buffer {} MiB, storage binding {} MiB, storage use {})",
        wanted.max_slab_size / MIB,
        wanted.large_threshold / MIB,
        limits.max_buffer_size / MIB,
        limits.max_storage_buffer_binding_size / MIB,
        settings.extra_buffer_usages.contains(BufferUsages::STORAGE),
    );
    settings.slab_allocator_settings = wanted;
}

/// One slab in the census.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlabRow {
    /// `vertex`, `index`, `metadata` or `morph`.
    pub class: &'static str,
    /// Element size in bytes (vertex stride, 2 or 4 for indices).
    pub stride: u64,
    /// Buffer bytes (0 before its first upload).
    pub bytes: u64,
    /// A dedicated large-object slab.
    pub large: bool,
}

/// The census line: slabs per class with stride and size, sorted, and the strides held in more than one
/// general slab (each such stride splits its pipelines' batch sets).
#[must_use]
pub fn census_line(rows: &mut [SlabRow]) -> (String, Vec<(&'static str, u64)>) {
    rows.sort_by_key(|r| (r.class, r.stride, r.large, Reverse(r.bytes)));
    let mut out = format!("{} slabs, {} MiB:", rows.len(), rows.iter().map(|r| r.bytes).sum::<u64>() / MIB);
    let mut split = Vec::new();
    let mut last: Option<(&str, u64)> = None;
    for row in rows.iter() {
        let general = !row.large;
        if general && last == Some((row.class, row.stride)) && split.last() != Some(&(row.class, row.stride)) {
            split.push((row.class, row.stride));
        }
        if general {
            last = Some((row.class, row.stride));
        }
        let tenths = row.bytes * 10 / MIB;
        let large = if row.large { " large" } else { "" };
        let _ = write!(out, " {} {}B {}.{}M{large}", row.class, row.stride, tenths / 10, tenths % 10);
    }
    (out, split)
}

/// The class's name; compared rather than matched, as `MorphTarget` exists only with Bevy's `morph` feature.
fn class_name(class: ElementClass) -> &'static str {
    if class == ElementClass::Vertex {
        "vertex"
    } else if class == ElementClass::Index {
        "index"
    } else if class == ElementClass::Metadata {
        "metadata"
    } else {
        "morph"
    }
}

/// Logs the census when the slab count changes or the slabs grew or shrank by 64 MiB since the last line.
fn log_slab_census(mut seen: Local<(usize, u64)>, allocator: Option<Res<MeshAllocator>>) {
    let Some(allocator) = allocator else { return };
    let (count, bytes) = (allocator.slab_count(), allocator.slabs_size());
    if count == seen.0 && bytes.abs_diff(seen.1) < 64 * MIB {
        return;
    }
    *seen = (count, bytes);
    let mut rows: Vec<SlabRow> = allocator
        .slabs
        .values()
        .map(|slab| SlabRow {
            class: class_name(slab.element_class()),
            stride: slab.element_layout().size(),
            bytes: slab.buffer_size(),
            large: matches!(slab, Slab::LargeObject(_)),
        })
        .collect();
    let (line, split) = census_line(&mut rows);
    log::info!("mesh.slabs: {line}");
    for (class, stride) in split {
        log::warn!("mesh.slabs: {class} stride {stride}B spans several slabs (its pipelines split batch sets)");
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use bevy_render::slab_allocator::{SlabAllocator, SlabItem};

    use super::*;

    /// A slab item keyed by an integer, with a byte stride as its layout (as mesh vertex slabs are).
    struct TestItem;

    impl SlabItem for TestItem {
        type Key = u32;
        type Layout = Stride;

        fn label() -> Cow<'static, str> {
            "test".into()
        }
    }

    #[derive(Clone, PartialEq, Eq, Hash)]
    struct Stride(u64);

    impl SlabItemLayout for Stride {
        fn size(&self) -> u64 {
            self.0
        }

        fn elements_per_slot(&self) -> u32 {
            1
        }

        fn buffer_usages(&self) -> BufferUsages {
            BufferUsages::VERTEX
        }
    }

    /// Slabs per stride after allocating `meshes` of `(stride, bytes)`. Bookkeeping only: the stage is never
    /// committed (that creates the GPU buffers; no device here), so it is forgotten rather than dropped
    /// (its drop reports the uncommitted growth).
    fn slabs_per_stride(meshes: &[(u64, u64)], settings: &SlabAllocatorSettings) -> Vec<(u64, usize)> {
        let mut allocator = SlabAllocator::<TestItem>::new();
        let mut stage = allocator.stage_allocation();
        for (key, &(stride, bytes)) in (0u32..).zip(meshes) {
            stage.allocate(&key, bytes, Stride(stride), settings);
        }
        std::mem::forget(stage);
        let mut strides: Vec<(u64, usize)> = Vec::new();
        for slab in allocator.slabs.values() {
            let stride = slab.element_layout().0;
            match strides.iter_mut().find(|(s, _)| *s == stride) {
                Some((_, n)) => *n += 1,
                None => strides.push((stride, 1)),
            }
        }
        strides.sort_unstable();
        strides
    }

    /// 700 MiB of the lit-static stride (28 B) in 10 MiB meshes, 300 MiB of a 24 B stride and a 12 B one:
    /// stock settings spill the 28 B layout into a second slab at 512 MiB, ours keep one slab per stride.
    #[test]
    fn one_slab_per_stride_past_the_stock_cap() {
        let mut meshes = vec![(28, 10 * MIB); 70];
        meshes.extend([(24, 10 * MIB); 30]);
        meshes.push((12, 3 * MIB));
        assert_eq!(slabs_per_stride(&meshes, &SlabAllocatorSettings::default()), vec![(12, 1), (24, 1), (28, 2)]);
        let ours = slab_settings(u64::MAX, u64::MAX, BufferUsages::empty());
        assert_eq!(slabs_per_stride(&meshes, &ours), vec![(12, 1), (24, 1), (28, 1)]);
    }

    #[test]
    fn settings_clamp_to_the_device() {
        let stock = SlabAllocatorSettings::default();
        // A desktop device: the target, stock min and growth.
        let s = slab_settings(u64::MAX, u64::from(u32::MAX), BufferUsages::empty());
        assert_eq!((s.max_slab_size, s.large_threshold), (TARGET_MAX_SLAB_BYTES, TARGET_MAX_SLAB_BYTES / 2));
        assert_eq!((s.min_slab_size, s.growth_factor), (stock.min_slab_size, stock.growth_factor));
        // Storage-bound slabs (ray tracing) obey the binding limit; vertex-only ones ignore it.
        let limit = 1536 * MIB;
        assert_eq!(slab_settings(u64::MAX, limit, BufferUsages::STORAGE | BufferUsages::BLAS_INPUT).max_slab_size, limit);
        assert_eq!(slab_settings(u64::MAX, limit, BufferUsages::empty()).max_slab_size, TARGET_MAX_SLAB_BYTES);
        // The buffer limit always applies; never below stock unless the device itself is smaller.
        assert_eq!(slab_settings(1024 * MIB, u64::MAX, BufferUsages::empty()).max_slab_size, 1024 * MIB);
        assert_eq!(slab_settings(u64::MAX, 128 * MIB, BufferUsages::STORAGE).max_slab_size, stock.max_slab_size);
        assert_eq!(slab_settings(256 * MIB, u64::MAX, BufferUsages::empty()).max_slab_size, 256 * MIB);
    }

    #[test]
    fn census_flags_a_stride_in_two_slabs() {
        let row = |class, stride, mib, large| SlabRow { class, stride, bytes: mib * MIB, large };
        let mut rows = vec![
            row("vertex", 28, 512, false),
            row("index", 2, 86, false),
            row("vertex", 28, 1, false),
            row("vertex", 24, 57, false),
            row("vertex", 24, 300, true),
            row("metadata", 48, 1, false),
        ];
        let (line, split) = census_line(&mut rows);
        assert_eq!(split, vec![("vertex", 28)]);
        assert_eq!(
            line,
            "6 slabs, 957 MiB: index 2B 86.0M metadata 48B 1.0M vertex 24B 57.0M vertex 24B 300.0M large vertex 28B 512.0M vertex 28B 1.0M"
        );
    }
}
