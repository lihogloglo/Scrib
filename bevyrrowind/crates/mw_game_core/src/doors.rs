//! Door travel's shared pieces (`apps/bevyrrowind/src/doors.rs` holds the systems).

use bevy_ecs::component::Component;

/// Scaffold `ARRIVAL_EYE_M`: the free camera arrives this far above the
/// marker. A marker is where the traveler's feet go before the engine drops
/// them to the floor; vanilla markers float about 1.2 m above the door sill
/// (`.sessions/doorpairs.md`, world-adapter.md "Portal frames:
/// feasibility"), so this gives a ~1.7 m eye in the common case and keeps
/// the camera above the floor where a marker sits low. Removal path: the
/// character controller (M3) grounds the player and puts the camera at its
/// eye height.
pub const ARRIVAL_EYE_M: f32 = 0.5;

/// The camera that travels (and activates doors).
#[derive(Component)]
pub struct DoorUser;
