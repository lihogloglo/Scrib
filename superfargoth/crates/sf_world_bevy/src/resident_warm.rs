//! Pipeline warm-up W4b3 (`docs/specs/pipeline-warmup.md` §5.3 door row, §5.4): the reveal's
//! shown set, the resident meshes within the reveal radius (all of them, not only the
//! camera-visible ones), precached under [`RESIDENT_TAG`]; the reveal gate holds on it.
//!
//! While a reveal holds, every frame, each mesh entity drawn with a registered material
//! (`StandardMaterial` by default; a game adds its own with [`add_resident_material`]) that is
//! within [`ResidentWarmup::radius_m`] of the hold's focus ([`RevealGate::focus`]) and not hidden
//! becomes a [`Specimen`] from its real mesh, material and entity flags ([`specimen_flags`]:
//! shadow caster, visibility-range dither, lightmap, previous skin/morph when skinned/morphed, so
//! `plan_specimen` plans the previous-frame subsets), once per distinct (mesh, material, flags) per
//! hold. A `StandardMaterial` whose textures still load waits (counted pending); one whose texture
//! failed is skipped (its material never prepares). The tag of the previous reveal is released when
//! a new hold begins (its pipelines stay compiled; a capped reveal's leftovers keep compiling in
//! play until then). Content the catalog covered is a hash hit, so the shown set costs only what
//! the catalog has not compiled yet; the catalog itself compiles in the background
//! ([`crate::Precache::set_background`]).
//!
//! GPU-batched statics (`GpuBatchedMesh3d`, no `Mesh3d`) are not scanned: their batch entities draw
//! in every view, so the gate's draw terms already hold on them.
//! Industry: UE5 PSO precaching (`PSOPrecache` at component load; the loading screen waits for the
//! "required" PSOs of what it shows).

use std::collections::HashSet;

use bevy_app::{App, PostUpdate};
use bevy_asset::{AssetId, AssetServer, Assets, UntypedAssetId};
use bevy_camera::visibility::{InheritedVisibility, VisibilityRange};
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_light::NotShadowCaster;
use bevy_math::Vec3;
use bevy_mesh::morph::MeshMorphWeights;
use bevy_mesh::skinning::SkinnedMesh;
use bevy_mesh::{Mesh, Mesh3d};
use bevy_pbr::{Lightmap, Material, MeshMaterial3d, StandardMaterial};
use bevy_transform::components::GlobalTransform;

use crate::catalog_warm::textures;
use crate::precache::{Precache, Specimen, specimen_flags};
use crate::reveal::RevealGate;

/// The precache tag of the reveal's shown set.
pub const RESIDENT_TAG: &str = "resident";

/// The scan systems (`PostUpdate`, after transform propagation).
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResidentScan;

/// The shown set of the reveal in progress (see the module docs). `WorldStreamingPlugin` adds it.
#[derive(Resource, Debug, Default)]
pub struct ResidentWarmup {
    /// Meshes this close to the reveal's focus are requested, meters; `None`: the gate's
    /// [`RevealGate::radius_m`].
    pub radius_m: Option<f32>,
    hold: Option<ResidentHold>,
}

#[derive(Debug, Default)]
struct ResidentHold {
    /// A reveal holds (false: the last hold's counts, kept for the log until the next one).
    active: bool,
    seen: HashSet<(AssetId<Mesh>, UntypedAssetId, u8)>,
    requested: usize,
    /// Entities in range whose material textures were still loading, this frame's scan.
    textures_waiting: usize,
    /// Distinct specimens skipped for a failed texture.
    failed: usize,
}

impl ResidentWarmup {
    /// Shown-set specimens a reveal waits for: materials whose textures still load, then the
    /// [`RESIDENT_TAG`]'s specimens waiting for assets, queued or compiling (`0` with no hold).
    #[must_use]
    pub fn pending(&self, precache: Option<&Precache>) -> usize {
        let Some(hold) = self.hold.as_ref().filter(|h| h.active) else { return 0 };
        let tag = precache.map_or(0, |p| {
            let s = p.status(RESIDENT_TAG);
            (s.pending_assets + s.queued + s.compiling) as usize
        });
        hold.textures_waiting + tag
    }

    /// Distinct specimens requested in the hold in progress (or the last one, until a new hold).
    #[must_use]
    pub fn requested(&self) -> usize {
        self.hold.as_ref().map_or(0, |h| h.requested)
    }

    /// Specimens skipped in this hold because a texture failed to load.
    #[must_use]
    pub fn failed(&self) -> usize {
        self.hold.as_ref().map_or(0, |h| h.failed)
    }

    /// Whether a hold's shown set is being gathered.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.hold.as_ref().is_some_and(|h| h.active)
    }
}

/// Scans entities drawn with `M` for the shown set (`StandardMaterial` is registered by
/// `WorldStreamingPlugin`; a game adds its own mesh materials, e.g. terrain, water).
pub fn add_resident_material<M: Material>(app: &mut App) {
    app.add_systems(PostUpdate, scan_resident::<M>.in_set(ResidentScan).run_if(resident_holding));
}

pub(crate) fn resident_holding(warm: Option<Res<ResidentWarmup>>) -> bool {
    warm.is_some_and(|w| w.is_active())
}

/// Starts a hold's shown set when a reveal hold begins (releasing the previous reveal's tag) and
/// ends it when the hold ends (its counts stay for the log until the next one); resets the
/// per-frame counts. Before [`ResidentScan`].
#[allow(clippy::needless_pass_by_value)] // system params
pub(crate) fn begin_resident_frame(mut warm: ResMut<ResidentWarmup>, gate: Option<Res<RevealGate>>, precache: Option<Res<Precache>>) {
    let holding = gate.is_some_and(|g| g.is_holding());
    if holding && !warm.is_active() {
        if let Some(precache) = &precache {
            precache.release(RESIDENT_TAG);
        }
        warm.hold = Some(ResidentHold { active: true, ..ResidentHold::default() });
    } else if let Some(hold) = warm.hold.as_mut() {
        hold.textures_waiting = 0;
        hold.active &= holding;
    }
}

type ResidentQuery<'w, 's, M> = Query<
    'w,
    's,
    (
        &'static Mesh3d,
        &'static MeshMaterial3d<M>,
        &'static GlobalTransform,
        &'static InheritedVisibility,
        Has<NotShadowCaster>,
        Option<&'static VisibilityRange>,
        Option<&'static Lightmap>,
        Has<SkinnedMesh>,
        Has<MeshMorphWeights>,
    ),
>;

/// One frame of the shown set for entities drawn with `M` (`StandardMaterial`: textures checked).
#[allow(clippy::needless_pass_by_value)] // system params
pub(crate) fn scan_resident<M: Material>(
    mut warm: ResMut<ResidentWarmup>,
    gate: Option<Res<RevealGate>>,
    precache: Option<Res<Precache>>,
    query: ResidentQuery<M>,
    standard: Option<Res<Assets<StandardMaterial>>>,
    images: Option<Res<Assets<Image>>>,
    server: Option<Res<AssetServer>>,
) {
    let Some(gate) = gate.filter(|g| g.is_holding()) else { return };
    let Some(focus) = gate.focus().map(Vec3::from_array) else { return };
    let radius = warm.radius_m.unwrap_or(gate.radius_m);
    let Some(hold) = warm.hold.as_mut().filter(|h| h.active) else { return };
    let r2 = radius * radius;
    let mut specimens = Vec::new();
    for (mesh, material, at, inherited, no_shadow, range, lightmap, skinned, morphed) in &query {
        if !inherited.get() || at.translation().distance_squared(focus) > r2 {
            continue;
        }
        let flags = specimen_flags(!no_shadow, range, lightmap, skinned, morphed);
        let material = material.id().untyped();
        let key = (mesh.id(), material, flags.bits());
        if hold.seen.contains(&key) {
            continue;
        }
        // `StandardMaterial` textures: loading waits, failed skips; other materials: as they are.
        let state = match (material.try_typed::<StandardMaterial>(), &standard, &images, &server) {
            (Ok(id), Some(standard), Some(images), Some(server)) => standard.get(id).map_or(Some(true), |m| textures(m, images, server)),
            _ => Some(true),
        };
        match state {
            None => hold.textures_waiting += 1,
            Some(ok) => {
                hold.seen.insert(key);
                if ok {
                    specimens.push(Specimen { mesh: mesh.id(), material, flags });
                } else {
                    hold.failed += 1;
                }
            }
        }
    }
    if specimens.is_empty() {
        return;
    }
    hold.requested += specimens.len();
    if let Some(precache) = precache {
        precache.request(specimens, RESIDENT_TAG);
    }
}
