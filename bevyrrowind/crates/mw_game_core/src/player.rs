//! The player's shared pieces (`apps/bevyrrowind/src/player.rs` holds the systems).

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::resource::Resource;
use bevy_ecs::schedule::SystemSet;
use sf_coords::WorldPos;

/// Where `Focus` is updated: E's activation (`scripts.rs`) runs after it.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FocusUpdate;

/// Player or free camera.
#[derive(Resource, Debug)]
pub struct PlayerMode {
    /// The player is wanted (it spawns once the start has streamed in).
    pub on: bool,
    /// Controls suspended and the cursor free (menus, dialogue): the body stands.
    pub paused: bool,
    /// The spawned body (player.rs's own state).
    pub body: Option<Entity>,
    /// The last grounded position (player.rs's own state: the fall guard's way back).
    pub last_ground: Option<WorldPos>,
}

/// The player character's body.
#[derive(Component, Debug)]
pub struct PlayerBody;
