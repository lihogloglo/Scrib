//! Budgeted point-light shadows (spec `docs/specs/sky-lighting.md`,
//! "Local-light shadows").
//!
//! A shadowed point light renders a cube map (six views of every caster in
//! its range), so games give shadows to the few most important local lights
//! and fade them with distance (Unity HDRP's shadow fade distance and
//! max shadows on screen, UE's per-light shadow fade, Skyrim's shadowed-light
//! cap). Bevy 0.20 has the cube maps (`PointLight::shadow_maps_enabled`,
//! `PointLightShadowMap`) but neither a budget nor a per-light shadow
//! strength, so this module:
//!
//! - ranks the lights marked [`BudgetedShadow`] by [`importance`] at the
//!   viewer (spec `docs/specs/shadows.md`, "Light selection"), with
//!   forward-hemisphere weighting outside a light's range and hysteresis,
//!   and lets at most [`PointShadowBudget::max`] hold a shadow map at once;
//! - fades a shadow in and out (distance band and over time) natively in the
//!   PBR fragment shader via [`PointLightShadowFade`] (`mix(1.0, shadow, fade)`),
//!   without mutating [`PointLight::intensity`] or spawning duplicate light
//!   entities.
//!
//! Also works around a Bevy 0.20 leak (shadows.md "Stale cube-face views"):
//! `drop_stale_light_views` despawns the six face views of a light that
//! lost its shadow, which Bevy's `prepare_lights` keeps alive.

use bevy_app::{App, Plugin, PostUpdate};
use bevy_camera::visibility::{InheritedVisibility, Visibility, VisibilitySystems};
use bevy_camera::{Camera, Camera3d, RenderTarget};
use bevy_ecs::prelude::*;
use bevy_light::{PointLight, PointLightShadowMap};
use bevy_pbr::{
    ExtractedPointLight, PointAndSpotLightViewEntities, PointLightShadowFade, prepare_lights,
};
use bevy_render::{Render, RenderApp, RenderSystems};
use bevy_time::Time;
use bevy_transform::TransformSystems;
use bevy_transform::components::GlobalTransform;

/// How many point lights cast shadows, and how they fade. A graphics
/// setting: games map their quality presets onto it.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct PointShadowBudget {
    /// Shadow maps at once (0: no point-light shadows).
    pub max: usize,
    /// Full shadow within this distance of the viewer, meters.
    pub fade_start: f32,
    /// No shadow past this distance, meters.
    pub fade_end: f32,
    /// Seconds for a shadow to fade fully in or out when a light gains or
    /// loses its slot.
    pub fade_time: f32,
    /// A light holding a slot keeps it until a challenger's importance is
    /// higher by `1 / hysteresis` (no flicker between two lights at similar distances).
    pub hysteresis: f32,
    /// Face size of the cube maps (`PointLightShadowMap::size`).
    pub map_size: usize,
    /// Screen-space contact shadows (`PointLight::contact_shadows_enabled`)
    /// on the lights holding a shadow map; needs Bevy's `ContactShadows` on
    /// the camera. One ray march per lit pixel per light, so only the
    /// budgeted few get it, never their twins.
    pub contact_shadows: bool,
}

impl Default for PointShadowBudget {
    /// Off: the budget is opt-in per game and preset.
    fn default() -> Self {
        Self { max: 0, fade_start: 15.0, fade_end: 25.0, fade_time: 0.3, hysteresis: 0.8, map_size: 1024, contact_shadows: false }
    }
}

impl PointShadowBudget {
    /// Shadow strength by distance to the viewer: 1 to `fade_start`, 0 past
    /// `fade_end`, linear between.
    pub fn distance_fade(&self, distance: f32) -> f32 {
        if distance <= self.fade_start {
            1.0
        } else if distance >= self.fade_end || self.fade_end <= self.fade_start {
            0.0
        } else {
            (self.fade_end - distance) / (self.fade_end - self.fade_start)
        }
    }
}

/// How much a point light's shadow matters to a viewer `distance` meters
/// away (spec `docs/specs/shadows.md`, "Light selection"): its strength at
/// the viewer times the share of the view its lit volume covers,
/// `(range / max(d, 1))² · min(1, range / d)²`, times `priority`.
///
/// Strength enters through `range`, not `intensity`: games set the range
/// where the light fades out (Bevy's cutoff), so `range²` tracks the
/// light's power, and it doesn't change with flicker (an animated
/// intensity would reshuffle the shadowed set every frame). Inside its
/// range a light scores its illuminance at the eye (`∝ range² / d²`);
/// outside, that falls off once more by its screen coverage. Clamped below
/// 1 m so the light you stand next to doesn't swamp the ranking.
#[must_use]
pub fn importance(range: f32, distance: f32, priority: f32) -> f32 {
    let near = (range / distance.max(1.0)).powi(2);
    let coverage = (range / distance.max(1e-3)).min(1.0).powi(2);
    priority.max(0.0) * near * coverage
}

/// A point light that may cast shadows within the [`PointShadowBudget`].
/// `priority` multiplies its [`importance`] (a carried torch over a lamp).
#[derive(Component, Clone, Copy, Debug)]
pub struct BudgetedShadow {
    pub priority: f32,
}

impl Default for BudgetedShadow {
    fn default() -> Self {
        Self { priority: 1.0 }
    }
}

/// Budget state of a [`BudgetedShadow`] light, added when it first gets a
/// shadow map.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ShadowSlot {
    /// Current shadow strength, 0..=1.
    pub fade: f32,
    /// Holds one of the budget's shadow maps (fading in, shown or fading out).
    pub held: bool,
}

/// The point-shadow budget on its own (`SkyPlugin` adds it; viewers without
/// a sky add it directly).
pub struct PointShadowPlugin;

impl Plugin for PointShadowPlugin {
    fn build(&self, app: &mut App) {
        add(app);
    }
}

fn add(app: &mut App) {
    app.init_resource::<PointShadowBudget>()
        .add_systems(
            PostUpdate,
            (sync_map_size, budget_shadows)
                .chain()
                .after(TransformSystems::Propagate)
                .before(VisibilitySystems::VisibilityPropagate),
        );
    if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app.add_systems(Render, drop_stale_light_views.in_set(RenderSystems::CreateViews).after(prepare_lights));
    }
}

/// Despawns the cube-face views of point and spot lights that have no
/// shadow map this frame (shadow turned off, or the light hidden).
///
/// Bevy 0.20's `prepare_lights` frees a light's views only while the light
/// sorts among the first `min(point lights, max cubes)` (42 at wgpu's 256
/// layers), shadowed lights first, then by entity. A light that loses its
/// shadow usually sorts far past that among hundreds of unshadowed lights,
/// and a hidden light isn't visited at all, so its six views stay alive and
/// each runs the whole `Core3d` schedule every frame (~0.35 ms of render
/// thread per face). Remove once Bevy frees them itself.
fn drop_stale_light_views(
    mut commands: Commands,
    mut lights: Query<(&mut PointAndSpotLightViewEntities, Option<&ExtractedPointLight>)>,
) {
    for (mut views, light) in &mut lights {
        if views.is_empty() || light.is_some_and(|l| l.shadow_maps_enabled) {
            continue;
        }
        for view in std::mem::take(&mut **views) {
            if let Ok(mut e) = commands.get_entity(view) {
                e.despawn();
            }
        }
    }
}

fn sync_map_size(budget: Res<PointShadowBudget>, map: Option<ResMut<PointLightShadowMap>>) {
    if let Some(mut map) = map
        && budget.is_changed()
        && map.size != budget.map_size
    {
        map.size = budget.map_size;
    }
}

/// The viewer: the highest-order active 3D camera drawing to a window.
fn viewer(cameras: &Query<(&Camera, &GlobalTransform, Option<&RenderTarget>), With<Camera3d>>) -> Option<GlobalTransform> {
    cameras
        .iter()
        .filter(|(c, _, t)| c.is_active && matches!(t, None | Some(RenderTarget::Window(_))))
        .max_by_key(|(c, _, _)| c.order)
        .map(|(_, t, _)| *t)
}

/// Picks the shadowed lights and advances their native shader shadow fades.
#[allow(clippy::type_complexity)]
fn budget_shadows(
    mut commands: Commands,
    budget: Res<PointShadowBudget>,
    time: Res<Time>,
    cameras: Query<(&Camera, &GlobalTransform, Option<&RenderTarget>), With<Camera3d>>,
    mut lights: Query<(
        Entity,
        &mut PointLight,
        Option<&mut ShadowSlot>,
        Option<&mut PointLightShadowFade>,
        &BudgetedShadow,
        &GlobalTransform,
        &Visibility,
        Option<&InheritedVisibility>,
    )>,
) {
    let viewer_tf = viewer(&cameras);
    let eye = viewer_tf.map(|t| t.translation());
    let view_forward = viewer_tf.map(|t| t.forward().as_vec3());

    // The best `max` candidates by importance: shown, inside the fade band;
    // slot holders rank better by 1 / hysteresis. Lights whose lit volume does
    // not reach the viewer (`d > light.range`) are weighted by hemisphere
    // alignment so lamps ahead of the player take priority over distant lamps
    // behind the camera.
    let mut ranked: Vec<(f32, Entity)> = Vec::new();
    if let (Some(eye), Some(forward)) = (eye, view_forward)
        && budget.max > 0
    {
        for (entity, light, slot, _, prio, at, vis, inherited) in &lights {
            let shown = *vis != Visibility::Hidden && inherited.is_none_or(|v| v.get());
            let to_light = at.translation() - eye;
            let d = to_light.length();
            if shown && d < budget.fade_end {
                let held = slot.is_some_and(|s| s.held);
                let view_weight = if d <= light.range || d <= 1e-3 {
                    1.0
                } else {
                    let cos_theta = (to_light / d).dot(forward);
                    0.625 + 0.375 * cos_theta
                };
                let score = importance(light.range, d, prio.priority) * view_weight;
                ranked.push((
                    if held {
                        score / budget.hysteresis.max(1e-3)
                    } else {
                        score
                    },
                    entity,
                ));
            }
        }
    }
    ranked.sort_by(|a, b| b.0.total_cmp(&a.0));
    ranked.truncate(budget.max);

    let target_of = |entity: Entity, at: &GlobalTransform| match eye {
        Some(eye) if ranked.iter().any(|r| r.1 == entity) => {
            budget.distance_fade(at.translation().distance(eye))
        }
        _ => 0.0,
    };

    let step = if budget.fade_time > 0.0 {
        time.delta_secs() / budget.fade_time
    } else {
        1.0
    };

    // First pass: advance existing held slots and release any that have finished
    // fading out, so newly ranked lights can claim freed slots in the same frame.
    let mut held = 0usize;
    for (entity, _, slot, _, _, at, _, _) in &mut lights {
        let Some(mut slot) = slot else {
            continue;
        };
        if !slot.held {
            continue;
        }
        let target = target_of(entity, at);
        let fade = if slot.fade < target {
            (slot.fade + step).min(target)
        } else {
            (slot.fade - step).max(target)
        };
        slot.fade = fade;
        if fade <= 0.0 && target <= 0.0 {
            slot.held = false;
        } else {
            held += 1;
        }
    }

    // Second pass: assign free slots to newly ranked lights and sync components.
    // When `fade_time > 0.0`, stagger new slot acquisitions to at most 1 light per
    // frame so multiple lamps never capture 12+ cubemap faces on the same frame.
    let max_new_slots = if budget.fade_time > 0.0 { 1 } else { budget.max };
    let mut new_slots = 0usize;
    for (entity, mut light, slot, shadow_fade, _, at, _, _) in &mut lights {
        let target = target_of(entity, at);
        let Some(mut slot) = slot else {
            if target > 0.0 && held < budget.max && new_slots < max_new_slots {
                held += 1;
                new_slots += 1;
                let fade = step.min(target);
                let shadows = fade > 0.0;
                if light.shadow_maps_enabled != shadows {
                    light.shadow_maps_enabled = shadows;
                }
                let contact = shadows && budget.contact_shadows;
                if light.contact_shadows_enabled != contact {
                    light.contact_shadows_enabled = contact;
                }
                commands.entity(entity).try_insert((
                    ShadowSlot { fade, held: true },
                    PointLightShadowFade(fade),
                ));
            }
            continue;
        };

        if !slot.held && target > 0.0 && held < budget.max && new_slots < max_new_slots {
            slot.held = true;
            held += 1;
            new_slots += 1;
            slot.fade = (slot.fade + step).min(target);
        }

        let fade = slot.fade;
        let shadows = slot.held && fade > 0.0;
        if light.shadow_maps_enabled != shadows {
            light.shadow_maps_enabled = shadows;
        }
        let contact = shadows && budget.contact_shadows;
        if light.contact_shadows_enabled != contact {
            light.contact_shadows_enabled = contact;
        }
        if let Some(mut shadow_fade) = shadow_fade {
            if (shadow_fade.0 - fade).abs() > 1e-5 {
                shadow_fade.0 = fade;
            }
        } else {
            commands.entity(entity).try_insert(PointLightShadowFade(fade));
        }
    }
}
