//! The player's shared pieces (`apps/bevyrrowind/src/player.rs` holds the systems).

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::resource::Resource;
use bevy_ecs::schedule::SystemSet;
use sf_coords::WorldPos;
use sf_ids::PlayerKey;

/// Where `Focus` is updated: E's activation (`scripts.rs`) runs after it.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FocusUpdate;

/// The rebindable Use action (`mw_controls_bevy::Control::Use`) started this frame: written in
/// `PreUpdate` once the input contexts have updated, for readers below the controls crate
/// (`mw_doors_bevy`'s doorless link ends).
#[derive(Resource, Debug, Default)]
pub struct UsePressed(pub bool);

/// The owner's A/B ladder on the player's walk (WALK-0, ROADMAP 3.4c A0): each switch off
/// zeroes one locomotion op's output on the player's graph (the humanoid graph's gate
/// params, `mw_graphs::build::params`), live from the console (`off speed_match` ...).
/// Default: all on, the shipped behaviour. Foot placement is not here: its switch is the
/// foot probes (`ControllerSettings::foot_probes`, key K, `off foot_placement`).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::struct_excessive_bools, reason = "independent A/B switches")]
pub struct LocoAb {
    /// `SpeedMatch`: off plays the clips at their authored rate.
    pub speed_match: bool,
    /// `StrideWarp`: off leaves the steps at their authored length.
    pub stride_warp: bool,
    /// `Lean`: off leaves the trunk as animated.
    pub lean: bool,
    /// Off: "forward lane only", the free-movement direction blends play only their
    /// forward clip (no side or back lane while the body turns); locked-on movement keeps
    /// its directions.
    pub side_lanes: bool,
}

impl Default for LocoAb {
    fn default() -> Self {
        Self { speed_match: true, stride_warp: true, lean: true, side_lanes: true }
    }
}

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

/// The host's character key. TODO(saves): the profile's random `PlayerKey` (save.md "Co-op").
pub const HOST_KEY: PlayerKey = PlayerKey(1);

/// The player character's body.
#[derive(Component, Debug)]
pub struct PlayerBody;

/// The player's character as distinct from its body: an entity that lives for the whole run
/// (the body comes and goes with the free camera) and carries the character's `Stats`
/// (`mw_rpg_bevy::stats`). One per player (`key`).
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerCharacter {
    pub key: PlayerKey,
}

/// The player's dressed, animated character (a child of [`PlayerBody`]; action.md A2 slice 2).
/// NPC systems that pick up every new `Character` (walker movers, dialogue actors) skip it: the
/// action layer moves the player.
#[derive(Component, Debug)]
pub struct PlayerAvatar;
