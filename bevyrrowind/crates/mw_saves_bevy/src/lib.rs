//! Saves, part 1 (ROADMAP 2.5; superfargoth `docs/specs/save.md`, `net.md` "Saves"): the game's
//! sections on `sf_save_bevy` and quicksave / quickload.
//!
//! - **Sections** (one file per save, versioned sections, stable ids only, never entity bits):
//!   - `sf/overlay`: the placement overlay ([`WorldOverlay`]): taken or removed placements by
//!     `PlacementId` (and physics' moved items, its blobs).
//!   - `sf/rpg`: quests, journal, known topics (`RpgState::world`).
//!   - `sf/clock`: the sky clock (day and hour, time scale) and the sim tick.
//!   - `sf/character/<host key>`: the host character: worldspace + feet position, facing, the
//!     inventory (`RpgState::pc`), and an `mw` extension blob ([`PlayerExt`]: first / third
//!     person). The gameplay body (`ActionState`, `Locomotion`) is not saved yet (`body: None`):
//!     load drops the player standing at the saved spot.
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
//! - **Quickload** restores in the running game: a full replace of every section (save.md
//!   "Loading"), then the player goes to the saved spot through the door-travel fade
//!   (`StartTravel`, the same path as a door, so another worldspace works). Resident placements the
//!   loaded overlay removes are taken out now (`inventory::take_placement`). With no save made or
//!   loaded in this run (a fresh start), quickload takes the newest quicksave of any world, so
//!   "save, quit, start, F9" resumes.
//! - **Save/load screen** (part 2): [`menu`], from the pause menu and the main menu.
//! - **Continue** (the main menu): [`newest_save`] of any reason, quickload's search.
//! - **Notices**: "Quicksaved" / "Saved" / "Loaded" toasts (`sf_ui_bevy::Toasts`, when the UI runs), else log.
//! - Not yet (leftovers): items taken after the save reappear only when their cell reloads (no
//!   streaming refresh of resident cells), the body's motion state, a save during dialogue,
//!   the host's profile `PlayerKey` (a constant for now), localized notices.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::PoisonError;

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
use sf_rpg::{ActorKey, CharacterRpg, PlayerKey};
use sf_save_bevy::sf_save::{SaveMode, SaveReason, SectionKey, SlotDir, list_worlds};
use sf_save_bevy::{
    LoadDone, LoadRequest, SaveAppExt, SaveConfig, SaveDone, SaveGate, SaveRequest, SaveSection, SaveSession, SaveStorage,
    SfSavePlugin, sections,
};
use sf_sky_bevy::SkyTime;
use sf_world::{Location, WorldspaceId};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{ActiveWorldspace, FadeTransition, StartTravel, WorldOverlay, from_grid};

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
/// The host's character key. TODO(saves): the profile's random `PlayerKey` (save.md "Co-op").
pub const HOST_KEY: PlayerKey = PlayerKey(1);
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

/// Registers the game's sections (after `SfSavePlugin`; the app and the headless test).
pub fn register(app: &mut App) {
    app.on_load_reset(close_menus);
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
    .add_save_section(sections::clock(
        |w| w.get_resource::<SkyTime>().map(|t| t.0),
        |w, v| {
            if let Some(mut t) = w.get_resource_mut::<SkyTime>() {
                t.0 = v;
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
        ext,
    }]
}

/// The host's inventory back, and the player to the saved spot through the door fade.
fn restore_characters(world: &mut World, all: Vec<sections::CharacterSave>) {
    let Some(host) = all.into_iter().find(|c| c.player == HOST_KEY) else { return };
    if let Some(mut rpg) = world.get_resource_mut::<Rpg>() {
        rpg.state.pc = host.rpg;
    }
    let ext = host.ext.get(OWNER).filter(|b| b.version == PLAYER_EXT_VERSION).and_then(|b| sf_save_bevy::sf_save::decode::<PlayerExt>(&b.bytes).ok());
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
    // `reload`: the resident cells respawn from the loaded overlay at the cut (save.md "Loading").
    world.write_message(StartTravel { traveler: cam, to, rotation: Quat::from_rotation_y(host.facing), reload: true });
}

/// The app: the plugin with the user-data folder, the sections, keys, notices and the gate.
pub fn add(app: &mut App) {
    let root = save_root();
    info!("saves: {}", root.display());
    app.add_plugins(SfSavePlugin { config: SaveConfig { root, ..Default::default() } });
    register(app);
    app.add_systems(Update, (gate_fade, keys, notices).chain());
    menu::add(app);
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
    use std::sync::Arc;

    use bevy_ecs::message::Messages;
    use mw_install::MorrowindInstall;
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
        app.insert_resource(rpg).insert_resource(WorldOverlay::default()).insert_resource(SkyTime::default());
        app
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
        let (mut talk, mut view) = rpg.open(FARGOTH).unwrap();
        rpg.request(&mut talk, &mut view, &DialogueRequest::Ask(RecordId::new("ring")));
        let yes = view.choices()[1].value;
        rpg.request(&mut talk, &mut view, &DialogueRequest::Choose(yes));
        assert_eq!(rpg.state.world.quests.stage(&quest), 100);
        let before = rpg.state.clone();
        let mut app = app(root.clone(), rpg);
        app.world().resource::<WorldOverlay>().0.write().unwrap().remove(TAKEN);
        app.world_mut().resource_mut::<SkyTime>().0.days = 3.75;

        app.world_mut().write_message(SaveRequest::quick());
        app.update();
        let saved: Vec<_> = app.world_mut().resource_mut::<Messages<SaveDone>>().drain().collect();
        let report = saved[0].0.as_ref().expect("saved");
        assert!(report.per_owner.contains_key(OWNER) && report.per_owner.contains_key("sf"), "{:?}", report.per_owner);

        // Fresh state, then quickload.
        app.insert_resource(Rpg::new(db.clone()).unwrap()).insert_resource(WorldOverlay::default()).insert_resource(SkyTime::default());
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
        assert!((app.world().resource::<SkyTime>().0.days - 3.75).abs() < 1e-9);
        let _ = std::fs::remove_dir_all(&root);
    }
}
