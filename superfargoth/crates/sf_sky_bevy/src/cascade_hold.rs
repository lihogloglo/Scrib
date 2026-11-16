//! Far sun-cascade hold (ROADMAP 1.22 F3): cascades from
//! [`CascadeHoldConfig::hold_from`] on keep last frame's fit and shadow map
//! until the camera moves or turns, the light turns, or the map gets old. The
//! decision and the padded fit are Bevy-free (`sf_sky::cascade_hold`, which
//! has the why); this module applies them, with public Bevy API only:
//!
//! - **Main world**, [`hold_far_cascades`]: in Bevy's
//!   `SimulationLightSystems::UpdateDirectionalLightCascades` set, after
//!   `build_directional_light_cascades` and so before the light frusta and the
//!   shadow caster visibility. Each held-index cascade of every directional
//!   light with shadow maps is replaced by `sf_sky`'s padded fit: a fresh one
//!   on a redraw, the stored one on a hold. Culling, shading (the cascade's
//!   `clip_from_world` reaches the GPU light) and the map all use the same
//!   matrices. The held views are listed in [`HeldSunCascades`].
//! - **Render world**, [`skip_held_cascades`]: in `PrepareResources` before
//!   `prepare_view_uniforms`, like the vendored lamp-face skip. A held cascade
//!   view whose last drawn texture, layer (pass name) and `clip_from_world`
//!   are the ones it has now is taken out of its camera's `ViewLightEntities`
//!   (no preprocessing, no pass, so its layer of the directional shadow
//!   texture is neither cleared nor drawn) and gets `SkipRootViewRendering`
//!   (no view uniforms, no batching). A new or reallocated texture, a moved
//!   layer or changed matrices draw it, whatever the main world says. Bevy's
//!   `TextureCache` hands the same directional texture back each frame while
//!   its descriptor (size, layer count) is unchanged.
//!
//! Movers in a held cascade (NPCs past the nearest cascade) update every
//! `max_age` frames. Switch: [`SunCascadeHold`] (the game's `--off cascadehold`).

use std::collections::{HashMap, HashSet};

use bevy_app::{App, PostUpdate};
use bevy_camera::Projection;
use bevy_ecs::prelude::*;
use bevy_light::cascade::build_directional_light_cascades;
use bevy_light::{CascadeShadowConfig, Cascades, DirectionalLight, DirectionalLightShadowMap, SimulationLightSystems};
use bevy_math::Mat4;
use bevy_pbr::{LightEntity, ShadowView, ViewLightEntities, ViewShadowBindings};
use bevy_render::render_resource::TextureId;
use bevy_render::sync_world::MainEntity;
use bevy_render::view::{ExtractedView, RetainedViewEntity, SkipRootViewRendering};
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_transform::components::GlobalTransform;
use sf_sky::cascade_hold::{step, CascadeHoldConfig, HoldCapture, HoldDecision, HoldView, Redraw, SliceShape};

/// The hold's settings (on by default; the game turns it off with `--off cascadehold`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct SunCascadeHold(pub CascadeHoldConfig);

/// The cascades held this frame: `(light, camera, cascade index)`.
#[derive(Resource, Clone, Debug, Default)]
pub struct HeldSunCascades(pub Vec<(Entity, Entity, u32)>);

/// Hold counters since startup, for benches: cascade-frames held, and drawn per
/// [`Redraw`] reason (`Redraw::index`).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CascadeHoldStats {
    /// Cascade-frames that kept their map.
    pub held: u64,
    /// Cascade-frames drawn, per reason.
    pub redrawn: [u64; Redraw::ALL.len()],
}

/// Per `(light, camera, cascade index)`: the last draw.
type HoldStates = HashMap<(Entity, Entity, usize), Option<HoldCapture>>;

pub(crate) fn build(app: &mut App) {
    app.init_resource::<SunCascadeHold>()
        .init_resource::<HeldSunCascades>()
        .init_resource::<CascadeHoldStats>()
        .add_systems(
            PostUpdate,
            hold_far_cascades
                .in_set(SimulationLightSystems::UpdateDirectionalLightCascades)
                .after(build_directional_light_cascades),
        );
}

pub(crate) fn finish(app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
    render_app.init_resource::<RenderHeldCascades>().add_systems(ExtractSchedule, extract_held).add_systems(
        Render,
        (skip_held_cascades, ApplyDeferred)
            .chain()
            .in_set(RenderSystems::PrepareResources)
            .before(bevy_render::view::prepare_view_uniforms),
    );
}

/// Replaces held-index cascades with `sf_sky`'s padded fit, held or fresh.
pub fn hold_far_cascades(
    config: Res<SunCascadeHold>,
    map: Option<Res<DirectionalLightShadowMap>>,
    views: Query<(&GlobalTransform, &Projection)>,
    mut lights: Query<(Entity, &GlobalTransform, &DirectionalLight, &CascadeShadowConfig, &mut Cascades)>,
    mut held: ResMut<HeldSunCascades>,
    mut stats: ResMut<CascadeHoldStats>,
    mut captures: Local<HoldStates>,
) {
    held.0.clear();
    let config = config.0;
    let (true, Some(map)) = (config.enabled, map) else {
        captures.clear();
        return;
    };
    let mut seen = HashSet::new();
    for (light, light_transform, directional, cascade_config, mut cascades) in &mut lights {
        if !directional.shadow_maps_enabled {
            continue;
        }
        let light_rotation = light_transform.rotation();
        let overlap = 1.0 - cascade_config.overlap_proportion;
        for (&camera, view_cascades) in &mut cascades.cascades {
            let Ok((camera_transform, projection)) = views.get(camera) else { continue };
            let view = HoldView { world_from_view: camera_transform.to_matrix(), light_rotation };
            for (index, cascade) in view_cascades.iter_mut().enumerate().skip(config.hold_from) {
                let Some(&far) = cascade_config.bounds.get(index) else { continue };
                // Bevy's near bounds: the minimum distance, then the previous far bound less the overlap.
                let near = if index == 0 { cascade_config.minimum_distance } else { overlap * cascade_config.bounds[index - 1] };
                let slice = SliceShape { corners: projection.get_frustum_corners(-near, -far), map_size: u32::try_from(map.size).unwrap_or(u32::MAX) };
                let key = (light, camera, index);
                seen.insert(key);
                let (fit, decision) = step(&config, captures.entry(key).or_default(), &view, &slice);
                cascade.world_from_cascade = fit.world_from_cascade;
                cascade.clip_from_cascade = fit.clip_from_cascade;
                cascade.clip_from_world = fit.clip_from_world;
                cascade.texel_size = fit.texel_size;
                match decision {
                    HoldDecision::Hold => {
                        stats.held += 1;
                        held.0.push((light, camera, u32::try_from(index).unwrap_or(u32::MAX)));
                    }
                    HoldDecision::Redraw(reason) => stats.redrawn[reason.index()] += 1,
                }
            }
        }
    }
    captures.retain(|key, _| seen.contains(key));
}

/// The held cascade views in the render world, by retained view key.
#[derive(Resource, Default)]
struct RenderHeldCascades(HashSet<RetainedViewEntity>);

fn extract_held(mut render: ResMut<RenderHeldCascades>, held: Extract<Res<HeldSunCascades>>) {
    render.0.clear();
    render.0.extend(
        held.0.iter().map(|&(light, camera, index)| RetainedViewEntity::new(MainEntity::from(light), Some(MainEntity::from(camera)), index)),
    );
}

/// What a cascade view was last drawn into and with.
#[derive(Clone, Debug, PartialEq)]
struct DrawnCascade {
    texture: TextureId,
    pass_name: String,
    clip_from_world: Option<Mat4>,
}

/// Takes held cascade views whose map is still valid out of their camera's
/// shadow pass; draws every other cascade view (removing a stale skip).
#[allow(clippy::type_complexity)]
fn skip_held_cascades(
    mut commands: Commands,
    held: Res<RenderHeldCascades>,
    mut cameras: Query<(&mut ViewLightEntities, &ViewShadowBindings)>,
    views: Query<(&ExtractedView, &ShadowView, &LightEntity, Has<SkipRootViewRendering>)>,
    mut drawn: Local<HashMap<RetainedViewEntity, DrawnCascade>>,
) {
    let mut live = HashSet::new();
    for (mut view_lights, bindings) in &mut cameras {
        let texture = bindings.directional_light_depth_texture.id();
        view_lights.lights.retain(|&entity| {
            let Ok((view, shadow, light, skipped)) = views.get(entity) else { return true };
            let LightEntity::Directional { cascade_index, .. } = *light else { return true };
            let key = view.retained_view_entity;
            if key.subview_index as usize != cascade_index {
                return true; // An extra depth view (`sun_view`), not a cascade.
            }
            live.insert(key);
            let unchanged = drawn.get(&key).is_some_and(|d| {
                d.texture == texture && d.pass_name == shadow.pass_name && d.clip_from_world == view.clip_from_world
            });
            let skip = held.0.contains(&key) && unchanged;
            if skip {
                if !skipped {
                    commands.entity(entity).insert(SkipRootViewRendering);
                }
            } else {
                if skipped {
                    commands.entity(entity).remove::<SkipRootViewRendering>();
                }
                if !unchanged {
                    let now = DrawnCascade { texture, pass_name: shadow.pass_name.clone(), clip_from_world: view.clip_from_world };
                    drawn.insert(key, now);
                }
            }
            !skip
        });
    }
    drawn.retain(|key, _| live.contains(key));
}

#[cfg(test)]
#[path = "cascade_hold_tests.rs"]
mod tests;
