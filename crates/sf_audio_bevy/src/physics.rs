//! The physics bridge (`physics` feature): the active world's `PhysicsEvent::Contact` reports of
//! the frame become [`ContactSound`] messages, positioned in the listener's frame (relative to
//! the floating origin's cell, as `GlobalTransform` is). Behind a feature so the audio plugin
//! builds and tests without the physics stack; the app turns it on and adds
//! [`PhysicsAudioPlugin`] next to the physics plugin.

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use bevy_math::IVec3;
use sf_audio::ContactSample;
use sf_physics_bevy::sf_physics::{BodyKind, PhysicsEvent};
use sf_physics_bevy::{Physics, PhysicsEvents, PhysicsSystems};
use sf_world_bevy::WorldOrigin;
use sf_world_bevy::coords::origin_relative;

use crate::{AudioSystems, ContactSound};

/// Feeds the physics contact reports to the audio plugin (add both).
pub struct PhysicsAudioPlugin;

impl Plugin for PhysicsAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ContactSound>()
            .add_systems(Update, contact_sounds.after(PhysicsSystems::WriteBack).before(AudioSystems));
    }
}

/// This frame's contact reports as [`ContactSound`]s. Dynamic bodies are keyed (rate limits and
/// slide loops are per moving body); fixed, held and kinematic ones count as world geometry.
#[allow(clippy::needless_pass_by_value)]
pub fn contact_sounds(events: Res<PhysicsEvents>, physics: Res<Physics>, origin: Option<Res<WorldOrigin>>, mut out: MessageWriter<ContactSound>) {
    let Some(world) = physics.worlds.active_world() else { return };
    let cell = origin.map_or(IVec3::ZERO, |o| o.cell);
    for e in &events.0 {
        let PhysicsEvent::Contact { a, b, impulse, speed, slide, point, surfaces, .. } = *e else { continue };
        let moving = |id| matches!(world.kind(id), Some(BodyKind::Dynamic { .. })).then(|| id.to_bits());
        out.write(ContactSound(ContactSample {
            bodies: [moving(a), moving(b)],
            surfaces: surfaces.map(|s| s.0),
            impulse,
            speed,
            slide,
            at: origin_relative(cell, point).to_array(),
        }));
    }
}
