//! Saves, part 1 (ROADMAP 2.5; superfargoth `docs/specs/save.md`, `net.md` "Saves"): the game's
//! sections on `sf_save_bevy` and quicksave / quickload.
//!
//! - **Sections** (one file per save, versioned sections, stable ids only, never entity bits):
//!   - `sf/overlay`: the placement overlay ([`WorldOverlay`]): taken or removed placements by
//!     `PlacementId` (and physics' moved items, its blobs).
//!   - `sf/rpg`: quests, journal, known topics (`RpgState::world`).
//!   - `sf/clock`: the sky clock (day and hour, time scale) and the sim tick.
//!   - `sf/weather`: the world weather's seed and overrides (`sf_weather_bevy::WorldWeather`;
//!     DECISIONS we1: scripted `ChangeWeather` / `ModRegion` changes are timestamped overrides
//!     in it), and the surface state (`sf_weather_bevy::WorldSurface`: wetness, puddles, snow and
//!     ash cover; v1 saves without it warm up from the weather). The wind's live sources are not
//!     saved (`wind: None`): they are presentation.
//!   - `sf/character/<host key>`: the host character: worldspace + feet position, facing, the
//!     gameplay body (`ActionState`, `Locomotion`; restored through `RestoreBody`), the
//!     inventory (`RpgState::pc`), and an `mw` extension blob ([`PlayerExt`]: first / third
//!     person).
//!   - Effects: the player's in its `CharacterSave` (`effects`, v4), an NPC's or creature's in
//!     the overlay `effects` blob beside its `stats` blob (`mw_rpg_bevy::effects`); `sf/effects`
//!     holds the effect instance counter. Older saves load with none active.
//!   - `sf/services`: merchant purses (`Rpg::services`; placement-id actors only). Old saves
//!     without it load with none: each merchant restocks to its base on its next open. Open
//!     trade sessions are never saved and close on load.
//!   - `mw/rpg` ([`MwRpgSave`]): the rest of `RpgState`: NPC dialogue state (disposition changes,
//!     greetings), container inventories, script locals and globals. Actor keys that are not
//!     placement ids (entity bits of spawned characters: `dialogue.rs` `actor_key`) are left out.
//! - **Files**: `<user data>/saves/single/<world id>/<n>-quick.sfsave` (save.md "Slots and
//!   files"; `<user data>` is the mods user folder, `mw_mods::UserDirs`, also holding
//!   `profiles/`). Two quicksaves rotate.
//! - **Keys**: quicksave and quickload are rebindable keymap actions (`controls.rs`,
//!   [`mw_controls_bevy::Control::QuickSave`]): F5 / F9, the modern PC convention (Skyrim,
//!   Fallout 4, Oblivion, The Witcher 3; Morrowind had F5 quicksave and F9 quickload too). No
//!   gamepad default (consoles have none; the save menu is part 2).
//! - **Identity** (`sf/meta`): content id, profile, load order and profile fingerprint from the
//!   profile the boot resolved ([`GameProfile`], from `mw_boot_bevy::mods::profile_identity`),
//!   copied into `SaveIdentity` whenever it changes. Load does not compare them yet (save.md
//!   "Mod-list changes", S3).
//! - **Quickload** restores in the running game behind a blackout (`load_behind_fade`): at the
//!   fade's cut every section is replaced (save.md "Loading"), and the character section's
//!   `StartTravel { reload: true }` moves the player to the saved spot (the door-travel path, so
//!   another worldspace works) and respawns every resident cell from the loaded overlay in the same
//!   frame: items taken after the save come back, moved ones go back, removed ones stay out.
//!   With no save made or loaded in this run (a fresh start), quickload takes the newest
//!   quicksave of any world, so "save, quit, start, F9" resumes.
//! - **Save/load screen** (part 2): [`menu`], from the pause menu and the main menu.
//! - **Continue** (the main menu): [`newest_save`] of any reason, quickload's search.
//! - **Notices**: "Quicksaved" / "Saved" / "Loaded" toasts (`sf_ui_bevy::Toasts`, when the UI runs), else log.
//! - Not yet (leftovers): the load-side identity check and mod diff (S3), `sf/records` (no
//!   runtime records in the game yet), a save during dialogue, the host's profile `PlayerKey`
//!   (a constant for now), localized notices.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, PoisonError};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_math::prelude::*;
use bevy_transform::prelude::*;
use bevy_enhanced_input::prelude::*;
use mw_rpg::NpcStates;
use serde::{Deserialize, Serialize};
use sf_camera::ViewChoice;
use sf_camera_bevy::PlayerCamera;
use sf_coords::WorldPos;
use sf_records::RecordId;
use sf_rpg::service::WorldServices;
use sf_rpg::{ActorKey, CharacterRpg};
use mw_boot_bevy::mods::ProfileIdentity;
use sf_save_bevy::sf_save::{ModRef, SaveMode, SaveReason, SectionKey, SlotDir, list_worlds};
use sf_save_bevy::{
    LoadDone, LoadRequest, LoadTransition, SaveAppExt, SaveConfig, SaveDone, SaveGate, SaveIdentity, SaveRequest, SaveSection, SaveSession,
    SaveStorage, SfSavePlugin, sections,
};
use sf_sky_bevy::GameClock;
use sf_weather_bevy::{WeatherRecords, WorldSurface, WorldWeather};
use sf_world::{Location, WorldspaceId};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{ActiveWorldspace, FadeCutHooks, FadeTransition, StartBlackout, StartTravel, WorldOverlay, from_grid};

use mw_controls_bevy::{QuickLoadAction, QuickSaveAction};
use mw_game_core::doors::{ARRIVAL_EYE_M, DoorUser};
use mw_game_core::player::PlayerBody;
use mw_player_bevy::RestoreBody;
use mw_rpg_bevy::dialogue::{Rpg, RpgState};

pub mod menu;
pub use menu::{OpenSaveLoad, SAVELOAD};

/// The adapter's section owner.
pub const OWNER: &str = "mw";
/// `mw/rpg`: [`MwRpgSave`].
pub const RPG: &str = "rpg";
pub const RPG_VERSION: u32 = 1;
/// Version of the host character's `mw` extension blob ([`PlayerExt`]).
pub const PLAYER_EXT_VERSION: u32 = 1;
/// The host's character key (`mw_game_core::player::HOST_KEY`).
pub use mw_game_core::player::HOST_KEY;
/// The save gate's blocker while a door fade runs (save.md "Saving" 1: never during a crossing).
const FADE_BLOCKER: &str = "door fade";

/// `mw/rpg`: the part of [`RpgState`] the framework sections don't hold.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MwRpgSave {
    pub npcs: NpcStates,
    pub inventories: BTreeMap<ActorKey, CharacterRpg>,
    pub locals: BTreeMap<(ActorKey, RecordId), f32>,
    pub globals: BTreeMap<RecordId, f32>,
}

/// The host character's `mw` extension blob.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerExt {
    pub view: ViewChoice,
}

/// A placement id, not entity bits (`dialogue.rs` keys characters without a placement by their
/// entity's 64 bits; placement ids are 128-bit hashes).
fn stable(key: ActorKey) -> bool {
    key.0 > u128::from(u64::MAX)
}

/// `sf/services`: the merchant purses of placement-id actors (as [`MwRpgSave`], spawned
/// characters' entity-bit keys are left out). Purses of actors gone from the content are not
/// dropped on load yet: the actor registry fills as cells stream (leftover).
fn services_save(services: &WorldServices) -> WorldServices {
    let mut copy = services.clone();
    copy.drop_missing(stable);
    copy
}

impl MwRpgSave {
    pub fn of(state: &RpgState) -> Self {
        Self {
            npcs: state.npcs.clone(),
            inventories: state.inventories.iter().filter(|(k, _)| stable(**k)).map(|(k, v)| (*k, v.clone())).collect(),
            locals: state.locals.iter().filter(|((k, _), _)| stable(*k)).map(|(k, v)| (k.clone(), *v)).collect(),
            globals: state.globals.clone(),
        }
    }

    pub fn apply(self, state: &mut RpgState) {
        state.npcs = self.npcs;
        state.inventories = self.inventories;
        state.locals = self.locals;
        state.globals = self.globals;
    }
}

/// The content the game runs (`mw_boot_bevy::mods::ProfileIdentity`): `sf/meta`'s identity.
/// The app inserts it from the boot's resolved profile; a change rewrites [`SaveIdentity`].
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct GameProfile(pub ProfileIdentity);

/// Copies the profile into the save identity (content id, profile, load order, fingerprint);
/// the party and versions stay as they are.
fn fill_identity(identity: &mut SaveIdentity, profile: &ProfileIdentity) {
    mw_boot_bevy::mods::CONTENT_ID.clone_into(&mut identity.content_id);
    env!("CARGO_PKG_VERSION").clone_into(&mut identity.build_id);
    identity.profile.clone_from(&profile.profile);
    identity.load_order =
        profile.load_order.iter().map(|m| ModRef { id: m.id.clone(), version: m.version.clone(), manifest_hash: m.manifest_hash }).collect();
    identity.profile_fingerprint = profile.fingerprint;
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn sync_identity(profile: Option<Res<GameProfile>>, mut identity: ResMut<SaveIdentity>) {
    if let Some(profile) = profile
        && profile.is_changed()
    {
        fill_identity(&mut identity, &profile.0);
    }
}

/// Registers the game's sections (after `SfSavePlugin`; the app and the headless test) and the
/// identity sync from [`GameProfile`].
pub fn register(app: &mut App) {
    app.add_systems(First, sync_identity);
    app.on_load_reset(close_menus);
    // NPC and creature sheets ride the overlay (`mw_rpg_bevy::stats`).
    app.world_mut().resource_mut::<sections::BlobVersions>().declare(mw_rpg_bevy::stats::STATS_BLOB, mw_rpg_bevy::stats::STATS_BLOB_VERSION);
    // Their effects too (`mw_rpg_bevy::effects`); effect output in flight is dropped on load.
    app.world_mut().resource_mut::<sections::BlobVersions>().declare(mw_rpg_bevy::effects::EFFECTS_BLOB, mw_rpg_bevy::effects::EFFECTS_BLOB_VERSION);
    app.on_load_reset(|w: &mut World| {
        if let Some(mut pending) = w.get_resource_mut::<sf_effects_bevy::PendingEffectOut>() {
            pending.0.clear();
        }
    });
    app.add_save_section(sections::overlay(
        snapshot_overlay,
        set_overlay,
    ))
    .add_save_section(sections::rpg(
        |w| w.get_resource::<Rpg>().map(|r| r.state.world.clone()),
        |w, v| {
            if let Some(mut rpg) = w.get_resource_mut::<Rpg>() {
                rpg.state.world = v;
            }
        },
    ))
    .add_save_section(sections::services(
        |w| w.get_resource::<Rpg>().map(|r| services_save(&r.services)).filter(|s| !s.purses.is_empty()),
        |w, v| {
            if let Some(mut rpg) = w.get_resource_mut::<Rpg>() {
                rpg.services = v;
                rpg.close_service_sessions();
            }
        },
    ))
    .add_save_section(sections::clock(
        |w| w.get_resource::<GameClock>().copied(),
        |w, v| {
            // The saved instant and scale; the dev freeze (`paused`) stays this session's.
            if let Some(mut t) = w.get_resource_mut::<GameClock>() {
                let paused = t.paused;
                *t = v;
                t.paused = paused;
            }
        },
    ))
    .add_save_section(sections::weather(
        |w| {
            w.get_resource::<WorldWeather>().map(|ww| sections::WeatherSave {
                weather: ww.0.clone(),
                wind: None,
                surface: surface_save(w),
            })
        },
        |w, v| {
            if let Some(mut ww) = w.get_resource_mut::<WorldWeather>() {
                ww.0 = v.weather;
            }
            restore_surface(w, v.surface.as_ref());
        },
    ))
    .add_save_section(sections::effects(
        |w| w.get_resource::<sf_effects_bevy::EffectIds>().map(|ids| sections::EffectsWorldSave { ids: ids.0 }),
        |w, v| {
            if let Some(mut ids) = w.get_resource_mut::<sf_effects_bevy::EffectIds>() {
                ids.0 = v.ids;
            }
        },
    ))
    .add_save_section(sections::characters(host_character, restore_characters))
    .add_save_section(SaveSection::value(
        SectionKey::new(OWNER, RPG).expect("valid key"),
        RPG_VERSION,
        |w: &World| w.get_resource::<Rpg>().map(|r| MwRpgSave::of(&r.state)),
        |w: &mut World, v: MwRpgSave| {
            if let Some(mut rpg) = w.get_resource_mut::<Rpg>() {
                v.apply(&mut rpg.state);
            }
        },
    ));
}

/// A copy of the live overlay with the resident items physics moved written in, as their
/// deactivation would (save.md "One path for unload and save"): a crate knocked over and
/// still resident saves where it lies.
fn snapshot_overlay(world: &World) -> Option<sf_world::PlacementOverlay> {
    let mut copy = world.get_resource::<WorldOverlay>()?.0.read().unwrap_or_else(PoisonError::into_inner).clone();
    sf_physics_bevy::stream::persist_moved_items(world, &mut copy);
    mw_rpg_bevy::stats::persist_actor_stats(world, &mut copy);
    Some(copy)
}

/// The loaded overlay replaces the live one. The resident cells respawn from it at the
/// travel's cut (`StartTravel::reload`, `Streaming::reload_resident`): taken items come back,
/// moved ones go back, removed ones stay out.
fn set_overlay(world: &mut World, overlay: sf_world::PlacementOverlay) {
    let Some(live) = world.get_resource::<WorldOverlay>() else { return };
    *live.0.write().unwrap_or_else(PoisonError::into_inner) = overlay;
}

/// Load is a full replace (save.md "Loading"): every menu and modal closes (dialogue,
/// journal, inventory, keybinds, message boxes), so the controls and the camera resume when
/// the fade ends (`dialogue.rs` pauses them only while a menu is open).
fn close_menus(world: &mut World) {
    if let Some(mut stack) = world.get_resource_mut::<sf_ui_bevy::ScreenStackRes>() {
        stack.0.clear(sf_ui::Layer::Modal);
        stack.0.clear(sf_ui::Layer::Menu);
    }
}

/// The player's gameplay body: the action state and the mover state.
fn player_body(world: &World) -> Option<sections::CharacterBody> {
    let mut q = world.try_query_filtered::<(&sf_action_bevy::ActionCharacter, &sf_action_bevy::ActionBody), With<PlayerBody>>()?;
    let (character, body) = q.iter(world).next()?;
    Some(sections::CharacterBody { action: character.state.clone(), locomotion: body.loco.clone() })
}

/// Where the player stands: the body's feet and facing, else the free camera's eye lowered to
/// feet height. `facing` is radians about +y with 0 = -z (`CharacterSave`).
fn player_pose(world: &World) -> Option<(Location, f32, ViewChoice)> {
    let worldspace = world.get_resource::<ActiveWorldspace>()?.0.clone()?;
    let view = world.try_query::<&PlayerCamera>().and_then(|mut q| q.iter(world).next().map(PlayerCamera::view)).unwrap_or_default();
    if let Some(body) = world.try_query_filtered::<&sf_action_bevy::ActionBody, With<PlayerBody>>().and_then(|mut q| q.iter(world).next().map(|b| (b.loco.render_position(), b.loco.motion.yaw)))
    {
        // Body yaw is `Look`'s (0 = +z).
        return Some((Location::new(worldspace, body.0), wrap(body.1 + std::f32::consts::PI), view));
    }
    let mut cams = world.try_query_filtered::<(&CellCoord, &Transform), With<DoorUser>>()?;
    let (cell, transform) = cams.iter(world).next()?;
    let eye = from_grid(cell, transform);
    let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
    Some((Location::new(worldspace, eye.offset(Vec3::NEG_Y * ARRIVAL_EYE_M)), wrap(yaw), view))
}

fn wrap(a: f32) -> f32 {
    let tau = std::f32::consts::TAU;
    (a + std::f32::consts::PI).rem_euclid(tau) - std::f32::consts::PI
}

/// The surface state for `sf/weather` (`None` without the weather plugin).
fn surface_save(world: &World) -> Option<sf_weather_bevy::sf_surface::SurfaceSave> {
    let book = &world.get_resource::<WeatherRecords>()?.0;
    Some(world.get_resource::<WorldSurface>()?.save(book))
}

/// Restores the surface state of `sf/weather` (`None`: a v1 save; it warms up from the weather).
fn restore_surface(world: &mut World, save: Option<&sf_weather_bevy::sf_surface::SurfaceSave>) {
    let Some(book) = world.get_resource::<WeatherRecords>().map(|r| r.0.clone()) else { return };
    if let Some(mut s) = world.get_resource_mut::<WorldSurface>() {
        s.restore(save, &book);
    }
}

/// The host character (one until co-op): saved whenever the RPG host runs, pose or not.
fn host_character(world: &World) -> Vec<sections::CharacterSave> {
    let Some(rpg) = world.get_resource::<Rpg>() else { return Vec::new() };
    let (location, facing, view) =
        player_pose(world).unwrap_or_else(|| (Location::new(WorldspaceId::new(""), WorldPos::default()), 0.0, ViewChoice::default()));
    let bytes = sf_save_bevy::sf_save::encode(&PlayerExt { view });
    let ext = BTreeMap::from([(OWNER.to_owned(), sections::VersionedBlob { version: PLAYER_EXT_VERSION, bytes })]);
    vec![sections::CharacterSave {
        player: HOST_KEY,
        name: "Player".to_owned(),
        location,
        facing,
        body: player_body(world),
        rpg: rpg.state.pc.clone(),
        stats: mw_rpg_bevy::stats::player_sheet_save(world, HOST_KEY),
        effects: mw_rpg_bevy::effects::player_effects_save(world, HOST_KEY),
        ext,
    }]
}

/// The host's inventory and stats sheet back, and the player to the saved spot through the door fade.
fn restore_characters(world: &mut World, all: Vec<sections::CharacterSave>) {
    let Some(host) = all.into_iter().find(|c| c.player == HOST_KEY) else { return };
    if let Some(mut rpg) = world.get_resource_mut::<Rpg>() {
        rpg.state.pc = host.rpg;
    }
    mw_rpg_bevy::stats::restore_player_sheet(world, HOST_KEY, host.stats.as_ref(), host.effects.as_ref());
    let ext =host.ext.get(OWNER).filter(|b| b.version == PLAYER_EXT_VERSION).and_then(|b| sf_save_bevy::sf_save::decode::<PlayerExt>(&b.bytes).ok());
    if host.location.worldspace.as_str().is_empty() {
        return;
    }
    let Some(cam) = world.try_query_filtered::<Entity, With<DoorUser>>().and_then(|mut q| q.iter(world).next()) else { return };
    if let (Some(ext), Some(mut camera)) = (ext, world.get_mut::<PlayerCamera>(cam))
        && camera.view() != ext.view
    {
        camera.set_view(ext.view);
    }
    let to = Location::new(host.location.worldspace, host.location.pos.offset(Vec3::Y * ARRIVAL_EYE_M));
    info!("load: player to {:.1?} in {}", to.pos.to_f64(), to.worldspace.as_str());
    if let Some(body) = host.body
        && let Some(mut restore) = world.get_resource_mut::<RestoreBody>()
    {
        restore.0 = Some((body.action, body.locomotion));
    }
    // Runs at the blackout's cut (`load_behind_fade`): the fade executes this travel in the same
    // frame, and `reload` respawns the resident cells from the loaded overlay (save.md "Loading").
    world.write_message(StartTravel { traveler: cam, to, rotation: Quat::from_rotation_y(host.facing), reload: true });
}

/// The app: the plugin with the user-data folder, the sections, keys, notices and the gate.
pub fn add(app: &mut App) {
    let root = save_root();
    info!("saves: {}", root.display());
    app.add_plugins(SfSavePlugin { config: SaveConfig { root, ..Default::default() } });
    if let Some(profile) = mw_boot_bevy::mods::profile_identity() {
        app.insert_resource(GameProfile(profile));
    } else {
        warn!("saves: no resolved mod profile; saves carry no content identity");
    }
    register(app);
    load_behind_fade(app);
    app.add_systems(Update, (gate_fade, keys, notices).chain());
    menu::add(app);
}

/// A decoded load applies at a fade's cut (save.md "Loading" step 4): the save runner starts a
/// blackout (queued behind a door fade if one runs), whose cut hook applies the load; the
/// character section's `StartTravel { reload }` then moves the player and reloads the resident
/// cells in that same frame.
fn load_behind_fade(app: &mut App) {
    app.insert_resource(LoadTransition {
        begin: Some(Arc::new(|world: &mut World| {
            world.write_message(StartBlackout);
        })),
    });
    app.world_mut().get_resource_or_init::<FadeCutHooks>().add(|world| {
        sf_save_bevy::apply_pending_load(world);
    });
}

/// `<user data>` (the mods user folder), else `./userdata`.
fn save_root() -> PathBuf {
    mw_boot_bevy::mods::user_data().unwrap_or_else(|| PathBuf::from("userdata"))
}

fn gate_fade(fade: Option<Res<FadeTransition>>, mut gate: ResMut<SaveGate>) {
    let active = fade.is_some_and(|f| f.is_active());
    if active != gate.blockers().any(|b| b == FADE_BLOCKER) {
        if active {
            gate.block(FADE_BLOCKER);
        } else {
            gate.unblock(FADE_BLOCKER);
        }
    }
}

#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn keys(
    save_keys: Query<&ActionEvents, With<Action<QuickSaveAction>>>,
    load_keys: Query<&ActionEvents, With<Action<QuickLoadAction>>>,
    fade: Option<Res<FadeTransition>>,
    session: Res<SaveSession>,
    config: Res<SaveConfig>,
    storage: Res<SaveStorage>,
    mut saves: MessageWriter<SaveRequest>,
    mut loads: MessageWriter<LoadRequest>,
    mut toasts: Option<ResMut<sf_ui_bevy::Toasts>>,
) {

    if started(&save_keys) {
        saves.write(SaveRequest::quick());
    }
    if !started(&load_keys) {
        return;
    }
    if fade.is_some_and(|f| f.is_active()) {
        notice(toasts.as_deref_mut(), "Can't load during a transition");
        return;
    }
    if session.world_id != 0 {
        loads.write(LoadRequest::latest(Some(SaveReason::Quick)));
        return;
    }
    // A fresh run: the newest quicksave of any world.
    match newest_save(&storage, &config, Some(SaveReason::Quick)) {
        Some(path) => {
            loads.write(LoadRequest::path(path));
        }
        None => notice(toasts.as_deref_mut(), "No quicksave"),
    }
}

/// The newest single-player save of any world, by the wall clock it was written at (then its
/// sequence), of `reason` only when given: quickload's fresh-run target (`Quick`) and the main
/// menu's Continue (`None`: any reason, as `OpenMW` `apps/openmw/mwgui/mainmenu.cpp` at 675146bd,
/// "return" loads the newest save). Saves whose meta can't be read are skipped.
pub fn newest_save(storage: &SaveStorage, config: &SaveConfig, reason: Option<SaveReason>) -> Option<PathBuf> {
    let fs = &*storage.0;
    list_worlds(fs, &config.root, SaveMode::Single)
        .into_iter()
        .filter_map(|(_, dir): (u128, SlotDir)| dir.latest(fs, reason))
        .filter_map(|e| e.meta.as_ref().ok().map(|m| ((m.wall_clock, e.sequence), e.path.clone())))
        .max_by_key(|(t, _)| *t)
        .map(|(_, path)| path)
}

/// `A` started this frame.
fn started<A: InputAction>(actions: &Query<&ActionEvents, With<Action<A>>>) -> bool {
    actions.iter().any(|e| e.contains(ActionEvents::START))
}

fn notices(mut saved: MessageReader<SaveDone>, mut loaded: MessageReader<LoadDone>, mut toasts: Option<ResMut<sf_ui_bevy::Toasts>>) {
    for done in saved.read() {
        match &done.0 {
            Ok(r) => {
                info!("saved {} ({} B, {} sections)", r.path.display(), r.bytes, r.sections);
                match r.reason {
                    SaveReason::Quick => notice(toasts.as_deref_mut(), "Quicksaved"),
                    SaveReason::Manual => notice(toasts.as_deref_mut(), "Saved"),
                    _ => {}
                }
            }
            Err(e) => notice(toasts.as_deref_mut(), &format!("Save failed: {e}")),
        }
    }
    for done in loaded.read() {
        match &done.0 {
            Ok(r) => {
                info!("loaded {} (prepare {:?}, apply {:?})", r.path.display(), r.prepare, r.apply);
                notice(toasts.as_deref_mut(), "Loaded");
            }
            Err(e) => notice(toasts.as_deref_mut(), &format!("Load failed: {e}")),
        }
    }
}

fn notice(toasts: Option<&mut sf_ui_bevy::Toasts>, text: &str) {
    info!("saves: {text}");
    if let Some(t) = toasts {
        t.push(text);
    }
}

#[cfg(test)]
mod tests {
    //! Save, fresh state, load on real data (skips without a Morrowind install).

    use bevy_ecs::message::Messages;
    use mw_install::MorrowindInstall;
    use mw_rpg::NoSheets;
    use sf_rpg::PlayerId;
    use sf_rpg_ui::DialogueRequest;
    use sf_world::PlacementId;

    use super::*;

    const FARGOTH: ActorKey = ActorKey(0xF00 << 64 | 1);
    const BARREL: ActorKey = ActorKey(0xF00 << 64 | 7);
    const TAKEN: PlacementId = PlacementId(0xF00 << 64 | 9);

    fn app(root: PathBuf, rpg: Rpg) -> App {
        let mut app = App::new();
        app.add_plugins(SfSavePlugin { config: SaveConfig { root, background: false, ..Default::default() } });
        register(&mut app);
        app.insert_resource(rpg).insert_resource(WorldOverlay::default()).insert_resource(GameClock::default());
        app
    }

    fn temp_root(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("bw-saves-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        root
    }

    /// The registration alone (no install): sections over whatever resources the test inserts.
    fn bare_app(root: PathBuf) -> App {
        let mut app = App::new();
        app.add_plugins(SfSavePlugin { config: SaveConfig { root, background: false, ..Default::default() } });
        register(&mut app);
        app
    }

    fn save(app: &mut App) {
        app.world_mut().write_message(SaveRequest::quick());
        app.update();
        let saved: Vec<_> = app.world_mut().resource_mut::<Messages<SaveDone>>().drain().collect();
        saved[0].0.as_ref().expect("saved");
    }

    fn load(app: &mut App) {
        app.world_mut().write_message(LoadRequest::latest(Some(SaveReason::Quick)));
        app.update();
        let loaded: Vec<_> = app.world_mut().resource_mut::<Messages<LoadDone>>().drain().collect();
        loaded[0].0.as_ref().expect("loaded");
    }

    #[test]
    fn a_save_carries_the_game_profile_identity() {
        let root = temp_root("identity");
        let mut app = bare_app(root.clone());
        let profile = ProfileIdentity {
            profile: "default".to_owned(),
            load_order: vec![
                mw_boot_bevy::mods::ModIdentity { id: "mw".to_owned(), version: "0.1.0".to_owned(), manifest_hash: [1; 32] },
                mw_boot_bevy::mods::ModIdentity { id: "morrowind-data".to_owned(), version: "1.6.1820".to_owned(), manifest_hash: [2; 32] },
            ],
            fingerprint: [7; 32],
        };
        app.insert_resource(GameProfile(profile));
        save(&mut app);

        let storage = app.world().resource::<SaveStorage>().0.clone();
        let (_, dir) = list_worlds(&*storage, &root, SaveMode::Single).into_iter().next().expect("one world");
        let entry = dir.latest(&*storage, None).expect("a save");
        let meta = entry.meta.as_ref().expect("readable meta");
        assert_eq!(meta.content_id, mw_boot_bevy::mods::CONTENT_ID);
        assert_eq!(meta.profile, "default");
        assert_eq!(meta.load_order.iter().map(|m| m.id.as_str()).collect::<Vec<_>>(), ["mw", "morrowind-data"]);
        assert_eq!(meta.load_order[1].manifest_hash, [2; 32]);
        assert_eq!(meta.profile_fingerprint, [7; 32]);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// `sf/services` through the game's registration (no install): a save without purses writes
    /// no section, and loading it (like a save from before the section) leaves no purses; a
    /// placement-id merchant's purse round-trips, an entity-bits key is left out.
    #[test]
    fn merchant_purses_round_trip_and_a_save_without_them_loads_empty() {
        use sf_rpg::GameTime;
        use sf_rpg::service::Purse;

        let root = temp_root("services");
        let mut db = sf_records::RecordDb::new();
        mw_records::register_kinds(&mut db).unwrap();
        let mut app = app(root.clone(), Rpg::new(Arc::new(db)).unwrap());
        let purse = Purse { gold: 137, last_restock: GameTime { day: 2, minute: 615 } };
        let spawned = ActorKey(42);
        let set_purses = |app: &mut App| {
            let mut rpg = app.world_mut().resource_mut::<Rpg>();
            rpg.services.purses.insert(FARGOTH, purse);
            rpg.services.purses.insert(spawned, purse);
        };

        save(&mut app);
        set_purses(&mut app);
        load(&mut app);
        assert_eq!(app.world().resource::<Rpg>().services, WorldServices::default(), "no section: no purses");

        set_purses(&mut app);
        save(&mut app);
        app.world_mut().resource_mut::<Rpg>().services = WorldServices::default();
        load(&mut app);
        let loaded = &app.world().resource::<Rpg>().services;
        assert_eq!(loaded.purses.iter().map(|(k, p)| (*k, *p)).collect::<Vec<_>>(), [(FARGOTH, purse)]);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn scripted_weather_and_the_seed_round_trip() {
        let root = temp_root("weather");
        let mut app = bare_app(root.clone());
        let mut state = sf_weather_bevy::sf_weather::WeatherState::new(0xBEEF);
        assert!(mw_weather::script::change_weather(&mut state, "Bitter Coast Region", 4, 30.5));
        assert!(!state.overrides.is_empty());
        app.insert_resource(WorldWeather(state.clone()));
        save(&mut app);

        app.insert_resource(WorldWeather::default());
        load(&mut app);
        assert_eq!(app.world().resource::<WorldWeather>().0, state, "seed and the ChangeWeather override are back");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn the_ring_barrel_and_fargoths_quest_round_trip() {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return;
        };
        let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
        let db = Arc::new(mw_records::load(&vfs, &install.game_files()).unwrap().db);
        let barrel = RecordId::new("chargen barrel fatigue");
        let quest = RecordId::new("ms_fargothring");
        let root = std::env::temp_dir().join(format!("bw-saves-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);

        // Take the ring barrel, finish Fargoth's quest, take a loose item, move the clock.
        let mut rpg = Rpg::new(db.clone()).unwrap();
        assert!(!mw_rpg_bevy::inventory::take_all(&mut rpg.state, &db, PlayerId::HOST, BARREL, &barrel).is_empty());
        assert!(rpg.add_actor(FARGOTH, &RecordId::new("fargoth"), "Seyda Neen"));
        let (mut talk, mut view) = rpg.open(FARGOTH, &NoSheets).unwrap();
        rpg.request(&mut talk, &mut view, &DialogueRequest::Ask(RecordId::new("ring")), &NoSheets);
        let yes = view.choices()[1].value;
        rpg.request(&mut talk, &mut view, &DialogueRequest::Choose(yes), &NoSheets);
        assert_eq!(rpg.state.world.quests.stage(&quest), 100);
        let before = rpg.state.clone();
        let mut app = app(root.clone(), rpg);
        app.world().resource::<WorldOverlay>().0.write().unwrap().remove(TAKEN);
        // Day 3, 18:00 (clock.md: integer game ms).
        app.world_mut().resource_mut::<GameClock>().now = sf_sky_bevy::GameInstant(3 * 86_400_000 + 64_800_000);

        app.world_mut().write_message(SaveRequest::quick());
        app.update();
        let saved: Vec<_> = app.world_mut().resource_mut::<Messages<SaveDone>>().drain().collect();
        let report = saved[0].0.as_ref().expect("saved");
        assert!(report.per_owner.contains_key(OWNER) && report.per_owner.contains_key("sf"), "{:?}", report.per_owner);

        // Fresh state, then quickload.
        app.insert_resource(Rpg::new(db.clone()).unwrap()).insert_resource(WorldOverlay::default()).insert_resource(GameClock::default());
        app.world_mut().write_message(LoadRequest::latest(Some(SaveReason::Quick)));
        app.update();
        let loaded: Vec<_> = app.world_mut().resource_mut::<Messages<LoadDone>>().drain().collect();
        loaded[0].0.as_ref().expect("loaded");

        let mut rpg = app.world_mut().resource_mut::<Rpg>();
        assert_eq!(rpg.state, before, "the whole RPG state round-trips");
        assert_eq!(rpg.state.world.quests.stage(&quest), 100);
        assert!(rpg.state.world.journal.for_quest(&quest).next().is_some(), "journal entries are back");
        assert!(mw_rpg_bevy::inventory::take_all(&mut rpg.state, &db, PlayerId::HOST, BARREL, &barrel).is_empty(), "the barrel stays empty");
        assert!(app.world().resource::<WorldOverlay>().0.read().unwrap().is_removed(TAKEN));
        assert_eq!(app.world().resource::<GameClock>().now, sf_sky_bevy::GameInstant(3 * 86_400_000 + 64_800_000));
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Stats through the game's save path (stats.md "Saves"): the player's sheet in its
    /// `CharacterSave`, a touched NPC's in the overlay (`stats` blob) and back on its next spawn;
    /// an untouched NPC writes nothing; a character section without a sheet (a v2 save) rebuilds
    /// the player's from the record.
    #[test]
    fn player_and_npc_sheets_round_trip() {
        use bevy_ecs::system::RunSystemOnce;
        use sf_character_bevy::{Character, CharacterPlacement};
        use sf_stats_bevy::sf_stats::{Cause, CauseKind, StatOp, StatOwner};
        use sf_stats_bevy::{StatRules, StatTableRes, Stats};

        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return;
        };
        let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
        let db = Arc::new(mw_records::load(&vfs, &install.game_files()).unwrap().db);
        let root = temp_root("stats");
        let mut app = app(root.clone(), Rpg::new(db.clone()).unwrap());
        mw_rpg_bevy::stats::add(&mut app, Some(db.clone()));
        let (fargoth, guard) = (PlacementId(0xF00 << 64 | 0x2A), PlacementId(0xF00 << 64 | 0x2B));
        let npc = |id: &str, p: PlacementId| (Character::record("NPC_", RecordId::new(id)), CharacterPlacement(p));
        let spawn_npcs = |app: &mut App| {
            let a = app.world_mut().spawn(npc("fargoth", fargoth)).id();
            let b = app.world_mut().spawn(npc("imperial guard", guard)).id();
            app.world_mut().run_system_once(mw_rpg_bevy::stats::attach_actor_stats).unwrap();
            (a, b)
        };
        let (f, _) = spawn_npcs(&mut app);

        // Touch the player and Fargoth.
        let table = app.world().resource::<StatTableRes>().0.clone();
        let rules = app.world().resource::<StatRules>().0.clone();
        let t = app.world().resource::<mw_rpg_bevy::stats::MwStats>().table.clone();
        let cause = Cause { kind: CauseKind::Debug, source: None, tick: 1 };
        let touch = [
            StatOp::SetBase { stat: t.idx.attributes[0], value: 77.0 },
            StatOp::PoolAdjust { pool: t.idx.health, delta: -5.0, below_zero: false, above_max: false },
            StatOp::UseSkill { skill: t.idx.skills[5], use_kind: 0, scale: 1.0 },
        ];
        let player = |app: &mut App| {
            let mut q = app.world_mut().query::<(&mw_game_core::player::PlayerCharacter, &Stats)>();
            q.single(app.world()).unwrap().1.sheet.to_save(&table)
        };
        let fresh_player = player(&mut app);
        {
            let mut q = app.world_mut().query::<&mut Stats>();
            for mut s in q.iter_mut(app.world_mut()) {
                if matches!(s.owner, StatOwner::Player(_)) || s.owner == StatOwner::Actor(ActorKey(fargoth.0)) {
                    assert_eq!(s.apply(&table, &rules, &touch, cause, &mut Vec::new()), 0);
                }
            }
        }
        let (player_before, fargoth_before) = (player(&mut app), app.world().get::<Stats>(f).unwrap().sheet.to_save(&table));
        assert_ne!(player_before, fresh_player);
        save(&mut app);

        // Fresh state: new player sheet, no NPCs, empty overlay; then load and respawn.
        restore_player_sheet_fresh(&mut app);
        assert_eq!(player(&mut app), fresh_player);
        let mut q = app.world_mut().query_filtered::<Entity, With<CharacterPlacement>>();
        for e in q.iter(app.world()).collect::<Vec<_>>() {
            app.world_mut().despawn(e);
        }
        app.insert_resource(WorldOverlay::default());
        load(&mut app);
        assert_eq!(player(&mut app), player_before, "the player's sheet round-trips");
        let overlay = app.world().resource::<WorldOverlay>().0.read().unwrap().clone();
        assert!(overlay.state(fargoth, mw_rpg_bevy::stats::STATS_BLOB).is_some());
        assert!(overlay.state(guard, mw_rpg_bevy::stats::STATS_BLOB).is_none(), "an untouched NPC saves nothing");
        let (f, g) = spawn_npcs(&mut app);
        assert_eq!(app.world().get::<Stats>(f).unwrap().sheet.to_save(&table), fargoth_before, "Fargoth's sheet comes back on spawn");
        assert!(!app.world().get::<Stats>(g).unwrap().sheet.changed_since_init());

        // A character section from before stats (v2 → v3: `stats: None`) rebuilds the sheet.
        let mut old = host_character(app.world());
        old[0].stats = None;
        restore_characters(app.world_mut(), old);
        assert_eq!(player(&mut app), fresh_player);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Effects through the game's save path, no install (effects.md "Saves", E4): the player's
    /// fortify and drain in its `CharacterSave`, a creature's fortify in its overlay `effects`
    /// blob; after a load the modified values and the remaining durations are equal and the
    /// effects still expire on time; a character section without effects (a v3 save) and a
    /// placement without the blob load with none active.
    #[test]
    #[allow(clippy::float_cmp, clippy::too_many_lines, reason = "exact values: saves are logic; one round trip")]
    fn a_fortify_and_a_drain_survive_a_save_round_trip() {
        use std::time::Duration;

        use bevy_ecs::system::RunSystemOnce;
        use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
        use sf_character_bevy::{Character, CharacterPlacement};
        use sf_effects_bevy::sf_effects::{ArgRef, EffectEntry, EffectOp, InstanceFlags, Range, RollSeed, SourceClass, SourceRef};
        use sf_effects_bevy::{EffectCommand, EffectIds, Effects};
        use sf_stats_bevy::sf_stats::StatOwner;
        use sf_stats_bevy::{StatTableRes, Stats};

        let root = temp_root("effects");
        let db = mw_rpg_bevy::effects::test_records::db(&[("rat", 20)]);
        let mut app = app(root.clone(), Rpg::new(db.clone()).unwrap());
        let step = |app: &mut App, seconds: f64| {
            app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(seconds)));
        };
        app.add_plugins(TimePlugin).insert_resource(Time::<Fixed>::from_hz(60.0));
        step(&mut app, 1.0 / 60.0);
        mw_rpg_bevy::stats::add(&mut app, Some(db));
        app.update();
        let rat_at = PlacementId(0xF00 << 64 | 0x2C);
        let spawn_rat = |app: &mut App| {
            let e = app.world_mut().spawn((Character::record("CREA", RecordId::new("rat")), CharacterPlacement(rat_at))).id();
            app.world_mut().run_system_once(mw_rpg_bevy::stats::attach_actor_stats).unwrap();
            app.world_mut().flush();
            e
        };
        let rat = spawn_rat(&mut app);
        let (hero, rat_owner) = (StatOwner::Player(HOST_KEY), StatOwner::Actor(ActorKey(rat_at.0)));

        let fortify = sf_effects_bevy::sf_effects::EffectKey::new("FortifyAttribute");
        let drain = sf_effects_bevy::sf_effects::EffectKey::new("DrainAttribute");
        let entry = |effect: &sf_effects_bevy::sf_effects::EffectKey, seconds: f32, magnitude: f32| EffectEntry {
            effect: effect.clone(),
            arg: Some(ArgRef::Stat("strength".into())),
            range: Range::OnSelf,
            area: 0.0,
            duration: seconds,
            magnitude: (magnitude, magnitude),
        };
        let inflict = |record: &str, entries: Vec<EffectEntry>| EffectOp::Inflict {
            source: SourceRef { record: RecordId::new(record), item: None },
            class: SourceClass::Consumable,
            caster: None,
            entries: entries.into_iter().collect(),
            flags: InstanceFlags::default(),
            rolls: RollSeed(3),
        };
        app.world_mut().write_message(EffectCommand::new(hero, vec![inflict("p_hero", vec![entry(&fortify, 2.0, 10.0), entry(&drain, 1.0, 5.0)])]));
        app.world_mut().write_message(EffectCommand::new(rat_owner, vec![inflict("p_rat", vec![entry(&fortify, 2.0, 7.0)])]));
        for _ in 0..10 {
            app.update();
        }

        let table = app.world().resource::<StatTableRes>().0.clone();
        let str_idx = table.find("strength").unwrap();
        let read = |app: &mut App, owner: StatOwner| {
            let mut q = app.world_mut().query::<(&Stats, &Effects)>();
            let (s, fx) = q.iter(app.world()).find(|(s, _)| s.owner == owner).expect("the character");
            let left: Vec<_> = fx.active.sources().iter().flat_map(|s| &s.effects).map(|e| e.duration.map(|d| d - e.elapsed)).collect();
            (s.view(&table).modified(str_idx), left)
        };
        let (hero_before, rat_before) = (read(&mut app, hero), read(&mut app, rat_owner));
        let base_str = read(&mut app, hero).0 - 5.0;
        assert_eq!(rat_before.0, 27.0);
        assert_eq!(hero_before.1, [Some(110), Some(50)]);
        let ids_before = *app.world().resource::<EffectIds>();

        // Save with time stopped, then a fresh state: new player, no creature, empty overlay.
        step(&mut app, 0.0);
        save(&mut app);
        restore_player_sheet_fresh(&mut app);
        assert!(app.world_mut().query::<&Effects>().iter(app.world()).all(|fx| fx.owner != hero || fx.active.is_empty()));
        app.world_mut().despawn(rat);
        app.insert_resource(WorldOverlay::default()).insert_resource(EffectIds::default());
        load(&mut app);
        assert!(app.world().resource::<WorldOverlay>().0.read().unwrap().state(rat_at, mw_rpg_bevy::effects::EFFECTS_BLOB).is_some());
        spawn_rat(&mut app);
        assert_eq!(read(&mut app, hero), hero_before, "the player's modified Strength and remaining durations");
        assert_eq!(read(&mut app, rat_owner), rat_before, "the creature's, from its overlay blob");
        assert_eq!(*app.world().resource::<EffectIds>(), ids_before, "the counter is back");

        // Time runs again: the drain ends 50 ticks on, the fortifies 110.
        step(&mut app, 1.0 / 60.0);
        for _ in 0..49 {
            app.update();
        }
        assert_eq!(read(&mut app, hero).0, base_str + 5.0);
        app.update();
        assert_eq!(read(&mut app, hero).0, base_str + 10.0, "the drain ended on time");
        for _ in 0..60 {
            app.update();
        }
        assert_eq!((read(&mut app, hero).0, read(&mut app, rat_owner).0), (base_str, 20.0), "the fortifies ended on time");

        // A v3 character section (no effects) loads with none active.
        let mut old = host_character(app.world());
        old[0].effects = None;
        restore_characters(app.world_mut(), old);
        assert!(read(&mut app, hero).1.is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    #[allow(clippy::float_cmp, reason = "exact values: saves are logic")]
    fn abilities_survive_a_save_round_trip_once() {
        use std::time::Duration;

        use bevy_ecs::system::RunSystemOnce;
        use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
        use mw_rpg_bevy::effects::test_records::{NPC, SIGN};
        use sf_character_bevy::{Character, CharacterPlacement};
        use sf_effects_bevy::{EffectIds, Effects};
        use sf_stats_bevy::sf_stats::StatOwner;
        use sf_stats_bevy::{StatTableRes, Stats};

        let root = temp_root("abilities");
        let db = mw_rpg_bevy::effects::test_records::spell_db(&[]);
        let mut app = app(root.clone(), Rpg::new(db.clone()).unwrap());
        let step = |app: &mut App, seconds: f64| {
            app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(seconds)));
        };
        app.add_plugins(TimePlugin).insert_resource(Time::<Fixed>::from_hz(60.0));
        app.insert_resource(mw_rpg_bevy::stats::PlayerBirthsign(Some(RecordId::new(SIGN))));
        step(&mut app, 1.0 / 60.0);
        mw_rpg_bevy::stats::add(&mut app, Some(db));
        app.update();
        let npc_at = PlacementId(0xF00 << 64 | 0x3D);
        let spawn_npc = |app: &mut App| {
            let e = app.world_mut().spawn((Character::record("NPC_", RecordId::new(NPC)), CharacterPlacement(npc_at))).id();
            app.world_mut().run_system_once(mw_rpg_bevy::stats::attach_actor_stats).unwrap();
            app.world_mut().flush();
            e
        };
        let npc = spawn_npc(&mut app);
        for _ in 0..3 {
            app.update();
        }
        let (hero, npc_owner) = (StatOwner::Player(HOST_KEY), StatOwner::Actor(ActorKey(npc_at.0)));
        let table = app.world().resource::<StatTableRes>().0.clone();
        let idx = |k: &str| table.find(k).unwrap();
        // Strength (effective base, modified), magicka (max, current), Resist Fire, active
        // sources, book.
        let read = |app: &mut App, owner: StatOwner| {
            let mut q = app.world_mut().query::<(&Stats, &Effects)>();
            let (s, fx) = q.iter(app.world()).find(|(s, _)| s.owner == owner).expect("the character");
            let v = s.view(&table);
            let records: Vec<_> = fx.active.sources().iter().map(|s| s.source.record.clone()).collect();
            let values = [v.effective_base(idx("strength")), v.modified(idx("strength")), v.max(idx("magicka")), v.current(idx("magicka")), v.modified(idx("resistfire"))];
            (values, records, fx.book.known().to_vec())
        };
        let (hero_before, npc_before) = (read(&mut app, hero), read(&mut app, npc_owner));
        assert_eq!(hero_before.0, [50.0, 50.0, 60.0, 60.0, 50.0], "race and birthsign abilities");
        assert_eq!(npc_before.0[..2], [45.0, 45.0], "race and record abilities");
        assert_eq!((hero_before.1.len(), npc_before.1.len()), (2, 2));

        // Save, then a fresh state (a new player relearns its start spells; no NPC, empty
        // overlay), then load: the saved sets come back, nothing is learned on top.
        step(&mut app, 0.0);
        save(&mut app);
        restore_player_sheet_fresh(&mut app);
        app.world_mut().despawn(npc);
        app.insert_resource(WorldOverlay::default()).insert_resource(EffectIds::default());
        load(&mut app);
        let blob = app.world().resource::<WorldOverlay>().0.read().unwrap().state(npc_at, mw_rpg_bevy::stats::STATS_BLOB).is_some();
        assert!(!blob, "the NPC's sheet is untouched: only its effects blob is saved");
        spawn_npc(&mut app);
        assert_eq!(read(&mut app, hero), hero_before, "the player's Strength 50, not 60");
        assert_eq!(read(&mut app, npc_owner), npc_before, "the NPC's Strength 45, not 50");
        step(&mut app, 1.0 / 60.0);
        for _ in 0..3 {
            app.update();
        }
        assert_eq!((read(&mut app, hero), read(&mut app, npc_owner)), (hero_before, npc_before), "and stays so");
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Pool fortifies through the game's save path: an ability's Fortify Magicka learned in play
    /// (a `Permanent` modifier on the derived pool, re-registered without shifting again) with
    /// magicka partly spent, and a temporary Fortify Health (the current over the max): equal
    /// after a load, the fortify still ends on time, and `Forget` still takes the 20 off.
    #[test]
    #[allow(clippy::float_cmp, reason = "exact values: saves are logic")]
    fn pool_fortifies_survive_a_save_round_trip() {
        use std::time::Duration;

        use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
        use mw_rpg_bevy::effects::test_records::POOL_ABILITY;
        use sf_effects_bevy::sf_effects::{EffectEntry, EffectKey, EffectOp, InstanceFlags, Range, RollSeed, SourceClass, SourceRef};
        use sf_effects_bevy::{EffectCommand, EffectIds, Effects};
        use sf_stats_bevy::sf_stats::{CauseKind, StatOp, StatOwner};
        use sf_stats_bevy::{StatCommand, StatTableRes, Stats};

        let root = temp_root("pool-fortify");
        let db = mw_rpg_bevy::effects::test_records::spell_db(&[]);
        let mut app = app(root.clone(), Rpg::new(db.clone()).unwrap());
        let step = |app: &mut App, seconds: f64| {
            app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(seconds)));
        };
        app.add_plugins(TimePlugin).insert_resource(Time::<Fixed>::from_hz(60.0));
        step(&mut app, 1.0 / 60.0);
        mw_rpg_bevy::stats::add(&mut app, Some(db));
        app.update();
        let hero = StatOwner::Player(HOST_KEY);
        let table = app.world().resource::<StatTableRes>().0.clone();
        let (magicka, health) = (table.find("magicka").unwrap(), table.find("health").unwrap());
        // (magicka max, current, health max, current, active source records)
        let read = |app: &mut App| {
            let mut q = app.world_mut().query::<(&Stats, &Effects)>();
            let (s, fx) = q.iter(app.world()).find(|(s, _)| s.owner == hero).expect("the player");
            let v = s.view(&table);
            let records: Vec<_> = fx.active.sources().iter().map(|s| s.source.record.to_string()).collect();
            ([v.max(magicka), v.current(magicka), v.max(health), v.current(health)], records)
        };
        let full = read(&mut app).0;
        assert_eq!(full[..2], [40.0, 40.0]);

        let fortify = EffectEntry {
            effect: EffectKey::new("FortifyHealth"),
            arg: None,
            range: Range::OnSelf,
            area: 0.0,
            duration: 2.0,
            magnitude: (30.0, 30.0),
        };
        let potion = EffectOp::Inflict {
            source: SourceRef { record: RecordId::new("p_hero"), item: None },
            class: SourceClass::Consumable,
            caster: None,
            entries: [fortify].into_iter().collect(),
            flags: InstanceFlags::default(),
            rolls: RollSeed(3),
        };
        let learn = EffectOp::Learn { source: RecordId::new(POOL_ABILITY) };
        app.world_mut().write_message(EffectCommand::new(hero, vec![learn, potion]));
        app.update();
        let spend = StatOp::PoolSetCurrent { pool: magicka, value: 30.0 };
        app.world_mut().write_message(StatCommand::new(hero, CauseKind::Debug, vec![spend]));
        for _ in 0..9 {
            app.update();
        }
        let before = read(&mut app);
        assert_eq!(before.0, [60.0, 30.0, full[2], full[3] + 30.0]);

        step(&mut app, 0.0);
        save(&mut app);
        restore_player_sheet_fresh(&mut app);
        app.insert_resource(WorldOverlay::default()).insert_resource(EffectIds::default());
        load(&mut app);
        assert_eq!(read(&mut app), before, "magicka 30 / 60 and health over its max, not shifted twice");

        // The fortify ends 110 ticks on; the ability stays until forgotten.
        step(&mut app, 1.0 / 60.0);
        for _ in 0..110 {
            app.update();
        }
        assert_eq!(read(&mut app).0, [60.0, 30.0, full[2], full[3]]);
        let forget = EffectOp::Forget { source: RecordId::new(POOL_ABILITY) };
        app.world_mut().write_message(EffectCommand::new(hero, vec![forget]));
        app.update();
        assert_eq!(read(&mut app).0, [40.0, 10.0, full[2], full[3]]);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Adapter A5a: a power cast through the host's `CastRequest` records its use on the world
    /// clock; the use survives a save round trip, so a recast right after the load is refused
    /// (24 game hours, `spells.cpp` `canUsePower`) and spends nothing.
    #[test]
    #[allow(clippy::float_cmp, reason = "exact values: saves are logic")]
    fn a_used_power_still_recharges_after_a_save_round_trip() {
        use std::time::Duration;

        use bevy_time::{Fixed, Time, TimePlugin, TimeUpdateStrategy};
        use mw_rpg_bevy::cast::CastRequest;
        use mw_rpg_bevy::effects::test_records::RACE_POWER;
        use sf_effects_bevy::sf_effects::GameMinutes;
        use sf_effects_bevy::{EffectClock, EffectIds, Effects};
        use sf_rpg::PlayerId;
        use sf_stats_bevy::sf_stats::StatOwner;
        use sf_stats_bevy::{StatTableRes, Stats};

        let root = temp_root("power-recharge");
        let db = mw_rpg_bevy::effects::test_records::spell_db(&[]);
        let mut app = app(root.clone(), Rpg::new(db.clone()).unwrap());
        app.add_plugins(TimePlugin)
            .insert_resource(Time::<Fixed>::from_hz(60.0))
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)));
        mw_rpg_bevy::stats::add(&mut app, Some(db));
        app.update();
        let hero = StatOwner::Player(HOST_KEY);
        let power = RecordId::new(RACE_POWER);
        let table = app.world().resource::<StatTableRes>().0.clone();
        let magicka = table.find("magicka").unwrap();
        // (magicka current, the power's last use)
        let read = |app: &mut App| {
            let mut q = app.world_mut().query::<(&Stats, &Effects)>();
            let (s, fx) = q.iter(app.world()).find(|(s, _)| s.owner == hero).expect("the player");
            (s.view(&table).current(magicka), fx.book.power_used(&power))
        };
        let cast = |app: &mut App| {
            let request = CastRequest { player: PlayerId::HOST, source: power.clone(), target: None };
            app.world_mut().write_message(request);
            for _ in 0..3 {
                app.update();
            }
        };
        assert_eq!(read(&mut app), (40.0, None));
        cast(&mut app);
        let clock = app.world().resource::<EffectClock>().0;
        let used = read(&mut app);
        assert_eq!(used, (35.0, Some(clock)), "cost 5, used now");

        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO));
        save(&mut app);
        restore_player_sheet_fresh(&mut app);
        app.insert_resource(WorldOverlay::default()).insert_resource(EffectIds::default());
        load(&mut app);
        assert_eq!(read(&mut app), used, "the use is kept");

        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(1.0 / 60.0)));
        cast(&mut app);
        assert_eq!(read(&mut app), used, "refused while recharging: nothing spent");
        assert!(app.world().resource::<EffectClock>().0 < GameMinutes(clock.0 + GameMinutes::DAY.0));
        let _ = std::fs::remove_dir_all(&root);
    }

    fn restore_player_sheet_fresh(app: &mut App) {
        mw_rpg_bevy::stats::restore_player_sheet(app.world_mut(), HOST_KEY, None, None);
    }
}
