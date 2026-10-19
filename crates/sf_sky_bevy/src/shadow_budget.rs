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
//!   viewer (spec `docs/specs/shadows.md`, "Light selection"), lights out
//!   of view included, with hysteresis, and lets at most
//!   [`PointShadowBudget::max`] hold a shadow map at once;
//! - fades a shadow in and out (distance band and over time) by splitting
//!   the light: the shadowed light carries `fade` of its power and an
//!   unshadowed twin the rest, `I (1 - s + s v) = s I v + (1 - s) I`. The
//!   twin exists only for budgeted lights and is hidden outside a fade.
//!
//! Game code keeps writing the full intensity in `Update`: the split is
//! applied in `PostUpdate` and undone in `First`.

use bevy_app::{App, First, Plugin, PostUpdate};
use bevy_camera::visibility::{InheritedVisibility, Visibility, VisibilitySystems};
use bevy_camera::{Camera, Camera3d, RenderTarget};
use bevy_ecs::prelude::*;
use bevy_light::{PointLight, PointLightShadowMap};
use bevy_time::Time;
use bevy_transform::TransformSystems;
use bevy_transform::components::{GlobalTransform, Transform};

use crate::NoLightProxy;

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
#[derive(Component, Clone, Copy, Debug)]
pub struct ShadowSlot {
    /// Current shadow strength, 0..=1.
    pub fade: f32,
    /// Holds one of the budget's shadow maps (fading in, shown or fading out).
    pub held: bool,
    /// The light's full intensity this frame (restored in `First`).
    full: f32,
    /// The unshadowed part of the light while it fades.
    twin: Entity,
}

/// The unshadowed twin of a budgeted light (a child of it).
#[derive(Component, Clone, Copy, Debug)]
pub struct ShadowTwin;

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
        .add_systems(First, restore_split)
        .add_systems(
            PostUpdate,
            (sync_map_size, budget_shadows)
                .chain()
                .after(TransformSystems::Propagate)
                .before(VisibilitySystems::VisibilityPropagate),
        );
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

/// Picks the shadowed lights, advances their fades and splits them.
#[allow(clippy::type_complexity)]
fn budget_shadows(
    mut commands: Commands,
    budget: Res<PointShadowBudget>,
    time: Res<Time>,
    cameras: Query<(&Camera, &GlobalTransform, Option<&RenderTarget>), With<Camera3d>>,
    mut lights: Query<
        (Entity, &mut PointLight, Option<&mut ShadowSlot>, &BudgetedShadow, &GlobalTransform, &Visibility, Option<&InheritedVisibility>),
        Without<ShadowTwin>,
    >,
    mut twins: Query<(&mut PointLight, &mut Visibility), With<ShadowTwin>>,
) {
    let eye = viewer(&cameras).map(|t| t.translation());
    // The best `max` candidates by importance: shown (in view or not, so
    // turning the camera changes nothing), inside the fade band; slot
    // holders rank better by 1 / hysteresis.
    let mut ranked: Vec<(f32, Entity)> = Vec::new();
    if let Some(eye) = eye
        && budget.max > 0
    {
        for (entity, light, slot, prio, at, vis, inherited) in &lights {
            let shown = *vis != Visibility::Hidden && inherited.is_none_or(|v| v.get());
            let d = at.translation().distance(eye);
            if shown && d < budget.fade_end {
                let held = slot.is_some_and(|s| s.held);
                let score = importance(light.range, d, prio.priority);
                ranked.push((if held { score / budget.hysteresis.max(1e-3) } else { score }, entity));
            }
        }
    }
    ranked.sort_by(|a, b| b.0.total_cmp(&a.0));
    ranked.truncate(budget.max);
    let target_of = |entity: Entity, at: &GlobalTransform| match eye {
        Some(eye) if ranked.iter().any(|r| r.1 == entity) => budget.distance_fade(at.translation().distance(eye)),
        _ => 0.0,
    };
    let mut held = lights.iter().filter(|l| l.2.as_ref().is_some_and(|s| s.held)).count();
    let step = if budget.fade_time > 0.0 { time.delta_secs() / budget.fade_time } else { 1.0 };
    for (entity, mut light, slot, _, at, _, _) in &mut lights {
        let target = target_of(entity, at);
        let Some(mut slot) = slot else {
            // First slot: the twin is spawned now, the fade starts next frame.
            if target > 0.0 && held < budget.max {
                held += 1;
                let twin = commands
                    .spawn((PointLight { shadow_maps_enabled: false, contact_shadows_enabled: false, ..*light }, ShadowTwin, NoLightProxy, Transform::IDENTITY, Visibility::Hidden, ChildOf(entity)))
                    .id();
                commands.entity(entity).try_insert(ShadowSlot { fade: 0.0, held: true, full: light.intensity, twin });
            }
            continue;
        };
        // A light takes a slot only when one is free (a fading-out light
        // keeps its map until it reaches 0): never more than `max` maps.
        if !slot.held && target > 0.0 && held < budget.max {
            slot.held = true;
            held += 1;
        }
        let goal = if slot.held { target } else { 0.0 };
        let fade = if slot.fade < goal { (slot.fade + step).min(goal) } else { (slot.fade - step).max(goal) };
        slot.fade = fade;
        if slot.held && fade <= 0.0 && target <= 0.0 {
            slot.held = false;
            held -= 1;
        }
        let full = light.intensity;
        slot.full = full;
        let shadows = slot.held && fade > 0.0;
        if light.shadow_maps_enabled != shadows {
            light.shadow_maps_enabled = shadows;
        }
        let contact = shadows && budget.contact_shadows;
        if light.contact_shadows_enabled != contact {
            light.contact_shadows_enabled = contact;
        }
        let split = shadows && fade < 1.0;
        if split {
            light.intensity = full * fade;
        }
        if let Ok((mut twin, mut vis)) = twins.get_mut(slot.twin) {
            vis.set_if_neq(if split { Visibility::Inherited } else { Visibility::Hidden });
            if split {
                *twin = PointLight { intensity: full * (1.0 - fade), shadow_maps_enabled: false, contact_shadows_enabled: false, ..*light };
            }
        }
    }
}

/// Gives the shadowed part of a split light back its full intensity, so
/// game code in `Update` (flicker, budgets) sees what it wrote.
fn restore_split(mut lights: Query<(&mut PointLight, &ShadowSlot), Without<ShadowTwin>>) {
    for (mut light, slot) in &mut lights {
        if light.shadow_maps_enabled && slot.fade < 1.0 {
            light.intensity = slot.full;
        }
    }
}
