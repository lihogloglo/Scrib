//! Live texture and material gauge (feature `textures`): proof that normal and ORM maps reach the
//! GPU, and what the loaded textures cost there.
//!
//! [`TextureGaugePlugin`] scans `Assets<StandardMaterial>`, `Assets<Image>`, `Assets<Mesh>` and the
//! shown `Mesh3d` + `MeshMaterial3d<StandardMaterial>` entities every [`SCAN_SECS`] (cheap: a
//! 0.25 Hz walk over handles and descriptors) and logs two lines:
//!
//! - `textures.live: N images, X MiB GPU (bc1 n/X MiB, ...), K normal maps sampled as sRGB`:
//!   bytes from each image's texture descriptor (format block size × mip chain × layers, what
//!   wgpu allocates, not the CPU copy, which Bevy drops after upload). A normal map in an sRGB
//!   format is decoded with the wrong curve.
//! - `materials.live: N StandardMaterial (normal a, metallic_roughness b, occlusion c, depth d);
//!   E shown entities, F normal-mapped, G normal map without tangents, H normal map without UV0`:
//!   Bevy's PBR shader only applies a normal map when the mesh carries `ATTRIBUTE_TANGENT` (and
//!   UV0), silently otherwise, so G and H are the "normals don't work" diagnostics.
//!
//! The lines are printed at each [`Reveal`] (half a second later, once the render world has
//! sampled the meshes) and again when a count moves by more than [`CHANGE_SHARE`], at most every
//! [`MIN_PRINT_SECS`]. With [`DiagCounters`] the main numbers are also gauges
//! (`textures/gpu_mib`, `textures/images`, `materials/normal_no_tangent`, `materials/normal_no_uv0`).
//!
//! Meshes uploaded with `RenderAssetUsages::RENDER_WORLD` keep no attributes in the main world,
//! so the render world samples `RenderAssets<RenderMesh>` vertex layouts on request
//! ([`MeshAttributeProbe`]). A mesh neither readable nor prepared yet counts as `unread`.
//! Only `StandardMaterial` is counted (extended or custom materials, such as terrain, are not).

use std::sync::{Arc, Mutex, PoisonError};

use bevy_app::{App, Plugin, Update};
use bevy_asset::{AssetId, Assets};
use bevy_camera::visibility::InheritedVisibility;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_image::Image;
use bevy_mesh::{Mesh, Mesh3d};
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_platform::collections::HashSet;
use bevy_render::mesh::RenderMesh;
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::{TextureDimension, TextureFormat};
use bevy_render::{Render, RenderApp, RenderSystems};
use bevy_time::{Real, Time};

use crate::{DiagCounters, Reveal, RevealWatchPlugin};

/// Seconds between scans.
pub const SCAN_SECS: f64 = 4.0;
/// Seconds between asking the render world for mesh layouts and scanning.
pub const SAMPLE_LAG_SECS: f64 = 0.5;
/// Minimum seconds between two change-triggered prints (a reveal always prints).
pub const MIN_PRINT_SECS: f64 = 10.0;
/// Relative change of any count that triggers a print.
pub const CHANGE_SHARE: f64 = 0.05;

/// Texture format families of [`TextureStats::families`], in order.
pub const FAMILIES: [&str; 7] = ["bc1", "bc3", "bc4", "bc5", "bc7", "other compressed", "uncompressed"];

/// The family index of a format in [`FAMILIES`].
#[must_use]
pub fn family(format: TextureFormat) -> usize {
    match format {
        TextureFormat::Bc1RgbaUnorm | TextureFormat::Bc1RgbaUnormSrgb => 0,
        TextureFormat::Bc3RgbaUnorm | TextureFormat::Bc3RgbaUnormSrgb => 1,
        TextureFormat::Bc4RUnorm | TextureFormat::Bc4RSnorm => 2,
        TextureFormat::Bc5RgUnorm | TextureFormat::Bc5RgSnorm => 3,
        TextureFormat::Bc7RgbaUnorm | TextureFormat::Bc7RgbaUnormSrgb => 4,
        f if f.is_compressed() => 5,
        _ => 6,
    }
}

/// GPU bytes of an image from its texture descriptor: every mip level, rounded up to whole
/// blocks, times the array layers (or the shrinking depth of a 3D texture).
#[must_use]
pub fn gpu_bytes(image: &Image) -> u64 {
    let d = &image.texture_descriptor;
    let (bw, bh) = d.format.block_dimensions();
    // Depth/stencil formats have no single copy size; 4 bytes is what drivers allocate for most.
    let block = u64::from(d.format.block_copy_size(None).unwrap_or(4));
    let d3 = d.dimension == TextureDimension::D3;
    (0..d.mip_level_count.max(1))
        .map(|level| {
            let w = (d.size.width >> level).max(1);
            let h = (d.size.height >> level).max(1);
            let depth = if d3 { (d.size.depth_or_array_layers >> level).max(1) } else { d.size.depth_or_array_layers.max(1) };
            u64::from(w.div_ceil(bw)) * u64::from(h.div_ceil(bh)) * u64::from(depth) * block
        })
        .sum()
}

/// One scan's numbers.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextureStats {
    pub images: usize,
    pub gpu_bytes: u64,
    /// Per [`FAMILIES`] entry: image count and GPU bytes.
    pub families: [(usize, u64); 7],
    /// Images some material uses as a normal map whose format is sRGB (decoded wrong).
    pub normal_srgb: usize,
    pub materials: usize,
    pub with_normal: usize,
    pub with_metallic_roughness: usize,
    pub with_occlusion: usize,
    pub with_depth: usize,
    /// Shown entities with `Mesh3d` + `MeshMaterial3d<StandardMaterial>`.
    pub shown: usize,
    /// Of those, the ones whose material has a normal map.
    pub normal_mapped: usize,
    /// Normal-mapped shown entities whose mesh has no tangents (Bevy ignores the normal map).
    pub normal_no_tangent: usize,
    /// Normal-mapped shown entities whose mesh has no UV0.
    pub normal_no_uv0: usize,
    /// Normal-mapped shown entities whose mesh attributes are unknown (extracted, not prepared yet).
    pub unread: usize,
}

impl TextureStats {
    /// The `textures.live:` line.
    #[must_use]
    pub fn texture_line(&self) -> String {
        let families: Vec<String> = FAMILIES
            .iter()
            .zip(&self.families)
            .filter(|(_, (n, _))| *n > 0)
            .map(|(name, (n, bytes))| format!("{name} {n}/{:.1} MiB", mib(*bytes)))
            .collect();
        format!(
            "textures.live: {} images, {:.1} MiB GPU ({}), {} normal maps sampled as sRGB",
            self.images,
            mib(self.gpu_bytes),
            families.join(", "),
            self.normal_srgb
        )
    }

    /// The `materials.live:` line.
    #[must_use]
    pub fn material_line(&self) -> String {
        format!(
            "materials.live: {} StandardMaterial (normal {}, metallic_roughness {}, occlusion {}, depth {}); {} shown entities, {} normal-mapped, {} normal map without tangents, {} normal map without UV0, {} unread",
            self.materials,
            self.with_normal,
            self.with_metallic_roughness,
            self.with_occlusion,
            self.with_depth,
            self.shown,
            self.normal_mapped,
            self.normal_no_tangent,
            self.normal_no_uv0,
            self.unread
        )
    }

    fn keys(&self) -> [u64; 13] {
        let n = |v: usize| v as u64;
        [
            n(self.images),
            self.gpu_bytes,
            n(self.normal_srgb),
            n(self.materials),
            n(self.with_normal),
            n(self.with_metallic_roughness),
            n(self.with_occlusion),
            n(self.with_depth),
            n(self.shown),
            n(self.normal_mapped),
            n(self.normal_no_tangent),
            n(self.normal_no_uv0),
            n(self.unread),
        ]
    }

    /// Whether any count moved by more than [`CHANGE_SHARE`] from `prev` (any change from 0).
    #[must_use]
    #[allow(clippy::cast_precision_loss)] // counts and bytes, a share
    pub fn changed_from(&self, prev: &Self) -> bool {
        self.keys().iter().zip(prev.keys()).any(|(&now, before)| now.abs_diff(before) as f64 > CHANGE_SHARE * before as f64)
    }
}

#[allow(clippy::cast_precision_loss)] // bytes to MiB for a log line
fn mib(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

/// What the render world saw of the prepared meshes' vertex layouts, at the last request.
#[derive(Default, Debug)]
pub struct MeshLayouts {
    /// The main world asked for a fresh sample.
    pub requested: bool,
    pub prepared: HashSet<AssetId<Mesh>>,
    pub no_tangent: HashSet<AssetId<Mesh>>,
    pub no_uv0: HashSet<AssetId<Mesh>>,
}

/// Shared between the worlds: the main world requests, the render world fills.
#[derive(Resource, Clone, Default)]
pub struct MeshAttributeProbe(pub Arc<Mutex<MeshLayouts>>);

impl MeshAttributeProbe {
    pub fn lock(&self) -> std::sync::MutexGuard<'_, MeshLayouts> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// The last scan, readable by tools (BRP status).
#[derive(Resource, Clone, Debug, Default)]
pub struct TextureGauge(pub Option<TextureStats>);

/// The main-world inputs of a scan.
#[derive(SystemParam)]
pub struct GaugeInputs<'w, 's> {
    materials: Option<Res<'w, Assets<StandardMaterial>>>,
    images: Option<Res<'w, Assets<Image>>>,
    meshes: Option<Res<'w, Assets<Mesh>>>,
    entities: Query<'w, 's, (&'static Mesh3d, &'static MeshMaterial3d<StandardMaterial>, Option<&'static InheritedVisibility>)>,
}

impl GaugeInputs<'_, '_> {
    /// Counts everything; `layouts` answers for meshes whose attributes left the main world.
    #[must_use]
    pub fn collect(&self, layouts: &MeshLayouts) -> TextureStats {
        let mut s = TextureStats::default();
        let mut normal_materials: HashSet<AssetId<StandardMaterial>> = HashSet::default();
        let mut normal_images: HashSet<AssetId<Image>> = HashSet::default();
        if let Some(materials) = &self.materials {
            for (id, m) in materials.iter() {
                s.materials += 1;
                if let Some(n) = &m.normal_map_texture {
                    s.with_normal += 1;
                    normal_materials.insert(id);
                    normal_images.insert(n.id());
                }
                s.with_metallic_roughness += usize::from(m.metallic_roughness_texture.is_some());
                s.with_occlusion += usize::from(m.occlusion_texture.is_some());
                s.with_depth += usize::from(m.depth_map.is_some());
            }
        }
        if let Some(images) = &self.images {
            for (id, image) in images.iter() {
                let bytes = gpu_bytes(image);
                let format = image.texture_descriptor.format;
                s.images += 1;
                s.gpu_bytes += bytes;
                let f = &mut s.families[family(format)];
                f.0 += 1;
                f.1 += bytes;
                s.normal_srgb += usize::from(format.is_srgb() && normal_images.contains(&id));
            }
        }
        for (mesh, material, shown) in &self.entities {
            if shown.is_some_and(|v| !v.get()) {
                continue;
            }
            s.shown += 1;
            if !normal_materials.contains(&material.0.id()) {
                continue;
            }
            s.normal_mapped += 1;
            let id = mesh.0.id();
            let main = self.meshes.as_ref().and_then(|m| m.get(id)).and_then(|m| {
                Some((m.try_contains_attribute(Mesh::ATTRIBUTE_TANGENT).ok()?, m.try_contains_attribute(Mesh::ATTRIBUTE_UV_0).ok()?))
            });
            let (tangent, uv0) = match main {
                Some(both) => both,
                None if layouts.prepared.contains(&id) => (!layouts.no_tangent.contains(&id), !layouts.no_uv0.contains(&id)),
                None => {
                    s.unread += 1;
                    continue;
                }
            };
            s.normal_no_tangent += usize::from(!tangent);
            s.normal_no_uv0 += usize::from(!uv0);
        }
        s
    }
}

/// See the module docs. Adds [`RevealWatchPlugin`] if missing (for the [`Reveal`] messages).
#[derive(Default)]
pub struct TextureGaugePlugin;

impl Plugin for TextureGaugePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<RevealWatchPlugin>() {
            app.add_plugins(RevealWatchPlugin);
        }
        app.init_resource::<MeshAttributeProbe>().init_resource::<TextureGauge>().add_systems(Update, gauge);
    }

    fn finish(&self, app: &mut App) {
        let probe = app.world().resource::<MeshAttributeProbe>().clone();
        let Some(render) = app.get_sub_app_mut(RenderApp) else { return };
        render.insert_resource(probe).add_systems(Render, sample_layouts.in_set(RenderSystems::Cleanup));
    }
}

/// The render-world half: on request, the prepared meshes and which lack tangents or UV0.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn sample_layouts(probe: Res<MeshAttributeProbe>, meshes: Option<Res<RenderAssets<RenderMesh>>>) {
    let mut l = probe.lock();
    if !l.requested {
        return;
    }
    l.requested = false;
    l.prepared.clear();
    l.no_tangent.clear();
    l.no_uv0.clear();
    let Some(meshes) = meshes else { return };
    for (id, mesh) in meshes.iter() {
        l.prepared.insert(id);
        if !mesh.layout.0.contains(Mesh::ATTRIBUTE_TANGENT) {
            l.no_tangent.insert(id);
        }
        if !mesh.layout.0.contains(Mesh::ATTRIBUTE_UV_0) {
            l.no_uv0.insert(id);
        }
    }
}

/// When the next request, scan and print happen.
#[derive(Default)]
struct Schedule {
    next_request: f64,
    scan_at: Option<f64>,
    force_print: bool,
    printed: Option<(f64, TextureStats)>,
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn gauge(
    inputs: GaugeInputs,
    probe: Res<MeshAttributeProbe>,
    mut reveals: MessageReader<Reveal>,
    time: Res<Time<Real>>,
    counters: Option<Res<DiagCounters>>,
    mut last: ResMut<TextureGauge>,
    mut state: Local<Schedule>,
) {
    let now = time.elapsed_secs_f64();
    let revealed = reveals.read().count() > 0;
    if revealed || now >= state.next_request {
        probe.lock().requested = true;
        state.next_request = now + SCAN_SECS;
        state.scan_at = Some(now + SAMPLE_LAG_SECS);
        state.force_print |= revealed;
    }
    if state.scan_at.is_some_and(|t| now >= t) {
        state.scan_at = None;
        let scan = inputs.collect(&probe.lock());
        let print = state.force_print
            || match &state.printed {
                None => scan.materials > 0 || scan.images > 0,
                Some((at, prev)) => now - at >= MIN_PRINT_SECS && scan.changed_from(prev),
            };
        if print {
            log::info!("{}", scan.texture_line());
            log::info!("{}", scan.material_line());
            state.printed = Some((now, scan.clone()));
            state.force_print = false;
        }
        last.0 = Some(scan);
    }
    if let (Some(counters), Some(s)) = (counters, &last.0) {
        #[allow(clippy::cast_precision_loss)] // counts as gauge values
        {
            counters.set_named("textures/gpu_mib", "MiB", mib(s.gpu_bytes));
            counters.set_named("textures/images", sf_diag::unit::COUNT, s.images as f64);
            counters.set_named("materials/normal_no_tangent", sf_diag::unit::COUNT, s.normal_no_tangent as f64);
            counters.set_named("materials/normal_no_uv0", sf_diag::unit::COUNT, s.normal_no_uv0 as f64);
        }
    }
}
