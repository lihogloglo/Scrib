//! Surface weather W6a, the game's stamp source: every animated character's planted feet print the
//! snow (`sf_surface_bevy::TrailStamp`; the framework keeps those within 40 m of the camera).
//!
//! Any `Animator` with legs in its graph (player avatar, NPCs, creatures; the player body is not a
//! `Mover`, so the animator is the common ground): a foot is planted while its leg's contact curve
//! (the cooked clips' `contact.<foot>`, the one foot placement and the stride monitor read) is
//! above one half; legs without a contact curve count as planted while the effector is within a
//! few cm of the character's ground plane. The foot's world position is the animator entity's
//! transform applied to the effector's model-space position (frames the graph posed, so LOD A3
//! characters don't stamp). A foot stamps once, where and how it planted (as footprint decals spawn
//! at foot-down: a sliding foot leaves one print, a standing one no new work). Feet over something other than
//! terrain (a bridge, a rock, a floor) don't stamp: the ring is 2D and would print the ground
//! underneath. Presentation only: stamps never feed gameplay.
//!
//! Not yet: lying bodies (ragdoll and knockdown capsules, `mw_delirium_bevy`), logged in the
//! surface-weather as built (W6a) for a later slice.

use std::collections::HashMap;

use bevy::prelude::*;
use mw_world::ids;
use sf_anim_bevy::Animator;
use sf_surface_bevy::sf_surface::trail::FOOT_HALF_M;
use sf_surface_bevy::{SurfaceOrigin, SurfaceSettings, TrailStamp};
use sf_terrain_bevy::TerrainHeightsMap;

/// Contact curve value above which a foot is planted.
const PLANTED: f32 = 0.5;
/// Without a contact curve: the effector's model-space height (m, before the body's scale) below
/// which the foot is down.
const DOWN_M: f32 = 0.12;
/// A foot further than this above or below the terrain stands on something else (m).
const ON_TERRAIN_M: f32 = 0.4;
/// The print's centre lies this far ahead of the effector (the ankle), toward the toes (m).
const TOE_SHIFT_M: f32 = 0.05;

/// Writes one stamp per foot-down of every posed animated character over terrain, at the
/// foot's position and facing when it planted (a foot that slides while planted still leaves one
/// print, as footprint decals spawn at foot-down).
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn foot_stamps(
    animators: Query<(Entity, &Animator, &GlobalTransform)>,
    heights: Option<Res<TerrainHeightsMap>>,
    settings: Res<SurfaceSettings>,
    origin: Res<SurfaceOrigin>,
    mut out: MessageWriter<TrailStamp>,
    mut plants: Local<HashMap<(Entity, usize), TrailStamp>>,
) {
    if !settings.trails || !settings.cover || !settings.layer {
        plants.clear();
        return;
    }
    let Some(terrain) = heights.as_deref().and_then(|h| h.0.get(&ids::exterior())) else { return };
    let mut held = HashMap::with_capacity(plants.len());
    for (e, a, global) in &animators {
        if !a.posed() || a.asset.graph.legs.is_empty() {
            // Frames the graph skipped (LOD) keep the plants they had.
            for k in 0..a.asset.graph.legs.len() {
                if let Some(p) = plants.get(&(e, k)) {
                    held.insert((e, k), *p);
                }
            }
            continue;
        }
        let (scale, rotation, _) = global.to_scale_rotation_translation();
        let forward3 = rotation * Vec3::Z;
        let forward = Vec2::new(forward3.x, forward3.z);
        let half_m = FOOT_HALF_M * Vec2::new(scale.z, scale.x);
        for (k, leg) in a.asset.graph.legs.iter().enumerate() {
            let model = a.output.model.translation(usize::from(leg.effector()));
            let planted = match leg.contact {
                Some(c) => a.output.curves.get(usize::from(c)).copied().unwrap_or(0.0) > PLANTED,
                None => model.y < DOWN_M,
            };
            if !planted {
                continue;
            }
            if let Some(p) = plants.get(&(e, k)) {
                held.insert((e, k), *p);
                continue;
            }
            let at = origin.0.offset(global.transform_point(model + Vec3::Z * TOE_SHIFT_M));
            let Some(ground) = terrain.height(at) else { continue };
            if (at.to_f64().y - f64::from(ground)).abs() > f64::from(ON_TERRAIN_M) {
                continue;
            }
            let stamp = TrailStamp { at, forward, half_m, depth: 1.0 };
            held.insert((e, k), stamp);
            out.write(stamp);
        }
    }
    *plants = held;
}
