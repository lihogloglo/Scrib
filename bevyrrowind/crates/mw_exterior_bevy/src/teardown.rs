//! Leaving the world (menus.md "### 1. App phases", slice F2a): `OnExit(WorldAlive)` tears the
//! world down, so the next `OnEnter(WorldAlive)` builds it fresh.
//!
//! - **Entities.** Every worldspace root goes, recursively: the streamed cells (and their batches,
//!   bodies, actors), the origin proxy, the world camera and the player anchor (`exterior.rs`
//!   `setup` spawns both under the start root; doors reparent them to another root), the player
//!   body and the level's props, dummies and lights. Our free-standing world entities (the sea
//!   scaffold, the offscreen render's UI camera and image) carry `DespawnOnExit(WorldAlive)` at
//!   spawn; the water's ocean and pool go through `mw_environment_bevy::water::despawn_world`.
//! - **Resources.** Those holding world `Entity`s or per-world state get fresh values:
//!   `Streaming` (same config), `WorldspaceRegistry` (empty, layers from 1 again), the static
//!   scene (`sf_scene_bevy::reset_scene`: its batches are not under the roots, and its cells
//!   would stay committed, since replacing `Streaming` streams nothing out),
//!   `ActiveWorldspace` and the player's start (as the app built them), `WorldOverlay`,
//!   `FadeTransition`, the player's mode (no body), `RestoreBody`, `ArrivalSpot`, `Focus`; and
//!   `Settled` is removed, so the next world settles again.
//! - **Not here** (F2b): the RPG state (stats, inventory, journal), the clock, script globals and
//!   a fresh save identity. The app-lifetime framework entities (sky lights and atmosphere,
//!   precipitation views, the HUD root, input contexts) stay, as in the first front end.

use bevy::prelude::*;
use mw_game_core::player::PlayerMode;
use sf_app_bevy::WorldAlive;
use sf_app_bevy::bevy_state::prelude::OnExit;
use sf_world_bevy::{ActiveWorldspace, FadeTransition, Streaming, WorldOverlay, WorldspaceRegistry, WorldspaceRoot};

/// What a new world starts from: the values the app was built with.
#[derive(Resource)]
struct FreshWorld {
    active: ActiveWorldspace,
    player_start: Option<mw_player_bevy::PlayerStart>,
    /// [`PlayerMode::on`] at the start (the player wanted, or the free camera).
    player_on: Option<bool>,
}

/// Adds the teardown; call once every world resource is in (the end of `exterior::run`), so the
/// snapshot of the start values is the app's.
pub(crate) fn add(app: &mut App) {
    let world = app.world();
    let fresh = FreshWorld {
        active: world.get_resource::<ActiveWorldspace>().cloned().unwrap_or_default(),
        player_start: world.get_resource::<mw_player_bevy::PlayerStart>().cloned(),
        player_on: world.get_resource::<PlayerMode>().map(|m| m.on),
    };
    app.insert_resource(fresh).add_systems(OnExit(WorldAlive), tear_down);
}

/// `OnExit(WorldAlive)`: despawns the world and resets the world resources (module docs).
fn tear_down(world: &mut World) {
    // Parked worldspaces (CW-9, `sf_world_bevy` park.rs) are disabled: named so they go too.
    let roots: Vec<Entity> = world.query_filtered::<Entity, (With<WorldspaceRoot>, Allow<bevy::ecs::entity_disabling::Disabled>)>().iter(world).collect();
    let count = roots.len();
    for root in roots {
        world.despawn(root);
    }
    mw_environment_bevy::water::despawn_world(world);

    let config = world.resource::<Streaming>().config().clone();
    world.insert_resource(Streaming::new(config));
    // With `Streaming` replaced, no cell streams out through the scene hook: reset the scene too.
    sf_scene_bevy::reset_scene(world);
    world.insert_resource(WorldspaceRegistry::default());
    world.insert_resource(WorldOverlay::default());
    world.insert_resource(FadeTransition::default());
    world.remove_resource::<mw_game_core::exterior::Settled>();
    let fresh = world.resource::<FreshWorld>();
    let (active, player_start, player_on) = (fresh.active.clone(), fresh.player_start.clone(), fresh.player_on);
    world.insert_resource(active);
    if let Some(start) = player_start {
        world.insert_resource(start);
    }
    if let Some(on) = player_on
        && let Some(mut mode) = world.get_resource_mut::<PlayerMode>()
    {
        *mode = PlayerMode { on, paused: mode.paused, body: None, last_ground: None };
        world.insert_resource(mw_player_bevy::RestoreBody::default());
        world.insert_resource(mw_player_bevy::ArrivalSpot::default());
        world.insert_resource(mw_player_bevy::Focus::default());
    }
    info!("app phase: world torn down ({count} worldspace roots)");
}

#[cfg(test)]
mod tests {
    use sf_app_bevy::AppPhase;
    use sf_app_bevy::bevy_state::prelude::{NextState, State};
    use sf_world_bevy::StreamingConfig;

    use super::*;
    use mw_game_core::exterior::ExteriorCamera;

    use crate::exterior::test_world_setup;

    /// A reduced app (no Morrowind install): the phase wiring of `phase.rs` (release start), the
    /// world resources `WorldStreamingPlugin` adds, the viewer's real `setup` and the teardown.
    fn app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .init_asset::<Image>()
            .insert_resource(Streaming::new(StreamingConfig::default()))
            .init_resource::<WorldspaceRegistry>()
            .init_resource::<WorldOverlay>()
            .init_resource::<FadeTransition>()
            .insert_resource(ActiveWorldspace(None))
            .insert_resource(PlayerMode { on: true, paused: false, body: None, last_ground: None })
            .init_resource::<mw_player_bevy::PlayerStart>()
            .init_resource::<mw_player_bevy::RestoreBody>()
            .init_resource::<mw_player_bevy::ArrivalSpot>()
            .init_resource::<mw_player_bevy::Focus>()
            // The static scene's per-run state, as `ScenePlugin` adds it (F3).
            .init_resource::<sf_scene_bevy::SceneSpaces>()
            .init_resource::<sf_scene_bevy::SceneInbox>()
            .init_resource::<sf_scene_bevy::SceneUploads>();
        crate::phase::add(&mut app, true);
        test_world_setup(&mut app);
        add(&mut app);
        app
    }

    fn go(app: &mut App, to: AppPhase) {
        app.world_mut().resource_mut::<NextState<AppPhase>>().set(to);
        app.update();
        assert_eq!(*app.world().resource::<State<AppPhase>>().get(), to);
    }

    /// Entities that are not resources (Bevy 0.20 stores resources as entities, and the first
    /// round trip inserts some, e.g. `PreviousState<WorldAlive>`).
    fn entities(app: &mut App) -> usize {
        app.world_mut().query_filtered::<Entity, Without<bevy::ecs::resource::IsResource>>().iter(app.world()).count()
    }

    fn cameras(app: &mut App) -> usize {
        app.world_mut().query_filtered::<Entity, With<ExteriorCamera>>().iter(app.world()).count()
    }

    /// F2a: `Frontend` → `Loading` → `InGame` → `Frontend` twice; each return leaves the first front
    /// end's entity count and an empty registry, each world has setup's camera exactly once.
    #[test]
    fn leaving_the_world_tears_it_down() {
        let mut app = app();
        app.update();
        assert_eq!(*app.world().resource::<State<AppPhase>>().get(), AppPhase::Frontend);
        app.update();
        let frontend = entities(&mut app);
        assert_eq!(app.world().resource::<WorldspaceRegistry>().iter().count(), 0, "no world in the front end");
        for round in 0..2 {
            go(&mut app, AppPhase::Loading);
            go(&mut app, AppPhase::InGame);
            app.update();
            assert_eq!(cameras(&mut app), 1, "round {round}: setup's camera once");
            assert!(app.world().resource::<WorldspaceRegistry>().iter().count() > 0, "round {round}: a world");
            // A stale body from the first world must not survive into the next.
            app.world_mut().resource_mut::<PlayerMode>().body = Some(Entity::PLACEHOLDER);
            // F3: a scene batch (top-level, not under a root) and its queued zero-fill.
            let batch = app.world_mut().spawn(sf_scene_bevy::SceneBatch { space: 0, batch: 0 }).id();
            let fill = sf_scene_bevy::BatchZeroFill { space: 0, batch: 0, entity: batch, slots: 1 };
            app.world_mut().resource_mut::<sf_scene_bevy::SceneUploads>().zero_fills.push(fill);
            go(&mut app, AppPhase::Frontend);
            app.update();
            assert_eq!(entities(&mut app), frontend, "round {round}: back to the front end's entities");
            assert_eq!(cameras(&mut app), 0);
            assert_eq!(app.world().resource::<WorldspaceRegistry>().iter().count(), 0, "round {round}: registry empty");
            assert!(app.world().resource::<PlayerMode>().body.is_none(), "round {round}: no stale body");
            assert!(app.world().resource::<sf_scene_bevy::SceneUploads>().zero_fills.is_empty(), "round {round}: scene reset");
        }
    }
}
