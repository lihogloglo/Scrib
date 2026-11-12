//! Crowd LOD, slice CROWD-1 (superfargoth `docs/specs/crowd-lod.md`): the ceiling switch and the
//! `crowd/*` gauges, no tier machinery yet.
//!
//! - [`CrowdSettings::hide_far`]: characters past the `C0` threshold (below 20 reference pixels,
//!   back above 23 px; `sf_character::crowd`) hide at their root, so their rig, skins, joints and
//!   shadows leave the frame while the animator and gameplay keep running. Pinned characters
//!   ([`crowd_pinned`]) always show. This is the A/B that measures what CROWD-2 can win.
//! - Gauges per frame (perf reports, `sf_diag_bevy`): characters per would-be tier
//!   (`crowd/c0|c1|c2/count`), their skinned meshes (`crowd/cN/meshes/count`) and palette joints
//!   (`crowd/cN/joints/count`, one palette per distinct joint binding: the P5 block), pinned and
//!   switch-hidden counts; from the render world, the skin extract's joints walked
//!   (`crowd/joints_extracted/count`), joints uploaded (`crowd/joints_uploaded/count`) and joint
//!   blocks (`crowd/joint_blocks/count`).

use bevy_app::{App, PostUpdate};
use bevy_camera::visibility::{Visibility, VisibilitySystems};
use bevy_ecs::prelude::*;
use bevy_mesh::skinning::SkinnedMesh;
use sf_anim_bevy::{AnimLod, AnimSystems, PhysicsPose};
use sf_character::crowd::{CrowdBand, far_hidden};
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::unit::COUNT;

use crate::CharacterBody;

/// Crowd switches (the host sets them; `--off farnpcs` in the game).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CrowdSettings {
    /// Hide characters past the `C0` threshold (the CROWD-1 ceiling A/B).
    pub hide_far: bool,
}

/// Forces `C0` on a character root (player, dialogue, combat, lock-on, scripted scene): the host
/// adds it. Forced anim tiers, significance above 1 and ragdolls pin too ([`crowd_pinned`]).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct CrowdPin;

/// On a root the far-hide switch hid (its `Visibility` was not `Hidden` before).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct CrowdHidden;

/// Per-frame crowd counts (the gauges' main-world values; read by tests and viewers).
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct CrowdStats {
    /// Characters per band (`C0`, `C1`, `C2`), plain thresholds, pins in `C0`.
    pub characters: [usize; 3],
    /// Skinned meshes of those characters, per band.
    pub meshes: [usize; 3],
    /// Palette joints per band (one palette per distinct joint binding).
    pub joints: [usize; 3],
    pub pinned: usize,
    /// Roots the far-hide switch keeps hidden.
    pub hidden: usize,
}

/// Whether a body's state pins its character to `C0`: a forced anim tier (the player, viewers),
/// significance above 1 (combat, the player's target), a ragdoll, or [`CrowdPin`] on the root.
pub fn crowd_pinned(lod: &AnimLod, ragdoll: bool, pin: bool) -> bool {
    pin || ragdoll || lod.forced.is_some() || lod.significance > 1.0
}

/// Gauge names per band: characters, skinned meshes, palette joints.
const BAND_GAUGES: [[&str; 3]; 3] = [
    ["crowd/c0/count", "crowd/c0/meshes/count", "crowd/c0/joints/count"],
    ["crowd/c1/count", "crowd/c1/meshes/count", "crowd/c1/joints/count"],
    ["crowd/c2/count", "crowd/c2/meshes/count", "crowd/c2/joints/count"],
];

pub(crate) fn add(app: &mut App) {
    app.init_resource::<CrowdSettings>().init_resource::<CrowdStats>().add_systems(
        PostUpdate,
        crowd_update.after(AnimSystems::Lod).before(VisibilitySystems::VisibilityPropagate),
    );
}

/// Render-world gauges (needs `DiagCounters`, i.e. `SfDiagPlugin`, and a render app).
pub(crate) fn finish(app: &mut App) {
    let Some(counters) = app.world().get_resource::<DiagCounters>().cloned() else { return };
    let Some(render_app) = app.get_sub_app_mut(bevy_render::RenderApp) else { return };
    render_app
        .insert_resource(counters)
        .add_systems(bevy_render::Render, skin_gauges.in_set(bevy_render::RenderSystems::Cleanup));
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn crowd_update(
    mut commands: Commands,
    settings: Res<CrowdSettings>,
    mut stats: ResMut<CrowdStats>,
    counters: Option<Res<DiagCounters>>,
    mut roots: Query<(Entity, &CharacterBody, &mut Visibility, Has<CrowdHidden>, Has<CrowdPin>)>,
    bodies: Query<(&AnimLod, Has<PhysicsPose>)>,
    skins: Query<&SkinnedMesh>,
) {
    let mut next = CrowdStats::default();
    for (root, cb, mut visibility, hidden, pin) in &mut roots {
        let Ok((lod, ragdoll)) = bodies.get(cb.body) else { continue };
        let pinned = crowd_pinned(lod, ragdoll, pin);
        let band = CrowdBand::of(lod.pixels, pinned).index();
        next.characters[band] += 1;
        next.pinned += usize::from(pinned);
        let mut palettes: Vec<&SkinnedMesh> = Vec::new();
        for skin in cb.meshes.iter().filter_map(|&m| skins.get(m).ok()) {
            next.meshes[band] += 1;
            if !palettes.iter().any(|p| p.inverse_bindposes.id() == skin.inverse_bindposes.id() && p.joints == skin.joints) {
                next.joints[band] += skin.joints.len();
                palettes.push(skin);
            }
        }
        let hide = settings.hide_far && far_hidden(lod.pixels, pinned, hidden);
        if hide && !hidden && *visibility != Visibility::Hidden {
            *visibility = Visibility::Hidden;
            commands.entity(root).try_insert(CrowdHidden);
            next.hidden += 1;
        } else if hidden && !hide {
            *visibility = Visibility::Inherited;
            commands.entity(root).try_remove::<CrowdHidden>();
        } else if hidden {
            next.hidden += 1;
        }
    }
    #[allow(clippy::cast_precision_loss)]
    if let Some(c) = counters {
        for band in CrowdBand::ALL {
            let i = band.index();
            let [count, meshes, joints] = BAND_GAUGES[i];
            c.set_named(count, COUNT, next.characters[i] as f64);
            c.set_named(meshes, COUNT, next.meshes[i] as f64);
            c.set_named(joints, COUNT, next.joints[i] as f64);
        }
        c.set_named("crowd/pinned/count", COUNT, next.pinned as f64);
        c.set_named("crowd/hidden/count", COUNT, next.hidden as f64);
    }
    *stats = next;
}

#[allow(clippy::needless_pass_by_value)]
fn skin_gauges(skins: Option<Res<bevy_pbr::SkinUniforms>>, counters: Res<DiagCounters>) {
    let Some(skins) = skins else { return };
    #[allow(clippy::cast_precision_loss)]
    {
        counters.set_named("crowd/joints_extracted/count", COUNT, skins.blocks.joints_visited() as f64);
        counters.set_named("crowd/joints_uploaded/count", COUNT, skins.blocks.total_joints() as f64);
        counters.set_named("crowd/joint_blocks/count", COUNT, skins.blocks.block_count() as f64);
    }
}
