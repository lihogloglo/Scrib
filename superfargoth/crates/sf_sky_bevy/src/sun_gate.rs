//! The sun and moon shadow gate on the sky's lights (ROADMAP 1.25 CW-8; the rule and its
//! state machine are `sf_sky::sun_gate`). Off by default ([`SunShadowGating`]): the lights
//! then carry neither [`ShadowGateState`] nor `DirectionalLightShadowFade` and
//! [`crate::SkyPlugin`] writes them exactly as before.
//!
//! On: each frame `apply_lights` asks [`gate_light`] for the light the elevation rule lets
//! cast. Its shadow contrast on a receiver facing it (`sf_sky::sun_gate::contrasts`: direct light
//! times the [`SunLightOcclusion`] of its shadow texture, against both lights' sky fill, the
//! other light and the flat ambient) gives a target strength; the strength fades toward it
//! over seconds and goes to `bevy_pbr::DirectionalLightShadowFade` (vendored patch "CW-8":
//! the shader blends the cascades, the material's far-shadow term and contact shadows toward
//! unshadowed). At strength 0 the light's shadow maps go off: no cascades, no caster scan, and
//! the terrain's far shadows and everything else keyed on `shadow_maps_enabled` follow.

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_pbr::DirectionalLightShadowFade;
use sf_sky::sun_gate::{GateState, SunShadowGate, TUNABLES};
use sf_tunables_bevy::TunablesAppExt;

/// The gate's settings (`sf_sky::sun_gate` tunables `shadow.sun.gate*`). The game turns it on.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct SunShadowGating(pub SunShadowGate);

/// Mean share of the sun's direct light that a shadow texture on it lets through (a cloud
/// shadow map, `DirectionalLightTexture`), `[0, 1]`; 1 = none. Written by the plugin that
/// owns the texture; the gate counts it against the sky's fill, which no such texture dims.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct SunLightOcclusion(pub f32);

impl Default for SunLightOcclusion {
    fn default() -> Self {
        Self(1.0)
    }
}

/// A gated light's state (present only while the gate is on).
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct ShadowGateState(pub GateState);

/// Last frame's contrasts and strengths, for logs and diagnostics (zero while off).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct SunShadowGateStats {
    pub sun_contrast: f32,
    pub moon_contrast: f32,
    pub sun_strength: f32,
    pub moon_strength: f32,
}

pub(crate) fn build(app: &mut App) {
    app.init_resource::<SunShadowGating>()
        .init_resource::<SunLightOcclusion>()
        .init_resource::<SunShadowGateStats>()
        .register_tunables(TUNABLES);
}

/// A light's gate components, as `apply_lights` queries them.
pub(crate) type GatedLight<'a> = (Entity, Option<Mut<'a, ShadowGateState>>, Option<Mut<'a, DirectionalLightShadowFade>>);

/// One light's gate this frame: `base` is the elevation rule's answer (and the light is
/// outdoors on the raster path), `contrast` its shadow contrast. Returns whether the gate
/// drops its maps and the strength to report. Off: removes leftover gate components (a live
/// switch) and drops nothing.
pub(crate) fn gate_light(
    gate: &SunShadowGate,
    (base, contrast): (bool, f32),
    dt: f32,
    (entity, state, fade): GatedLight<'_>,
    commands: &mut Commands,
) -> (bool, f32) {
    if !gate.enabled {
        if state.is_some() || fade.is_some() {
            commands.entity(entity).remove::<(ShadowGateState, DirectionalLightShadowFade)>();
        }
        return (false, if base { 1.0 } else { 0.0 });
    }
    let previous = state.as_ref().map_or_else(GateState::default, |s| s.0);
    let next = gate.step(previous, base, contrast, dt);
    // Lights the elevation rule leaves dark keep a neutral fade (their contact shadows as before).
    let value = DirectionalLightShadowFade(if base { next.strength } else { 1.0 });
    match state {
        Some(mut s) => {
            s.set_if_neq(ShadowGateState(next));
        }
        None => {
            commands.entity(entity).insert(ShadowGateState(next));
        }
    }
    match fade {
        Some(mut f) => {
            f.set_if_neq(value);
        }
        None => {
            commands.entity(entity).insert(value);
        }
    }
    (base && !next.held, next.strength)
}
