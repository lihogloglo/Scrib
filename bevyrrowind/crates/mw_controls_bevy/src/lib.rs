//! The player's gameplay actions in the player's keymap (ROADMAP 2.1b; superfargoth `ui.md`
//! "As built (U6)"): every gameplay input is a rebindable `sf_ui_bevy::UiKeymap` action in the
//! `gameplay` context (profile table `[bindings.gameplay]`), its scope linked to `HUD_SCOPE`
//! (open menu / map live beside play), so one keybinds screen and one profile cover UI and play.
//! `player.rs` and the use context here spawn their `bevy_enhanced_input` bindings from it, and
//! respawn them when it changes (a rebind applies at once).
//!
//! - **Keyboard and mouse** (the owner's modern defaults): WASD move, mouse look, wheel zoom,
//!   Space jump, Left Shift dodge / sprint (tap rolls, hold sprints), Left Ctrl sneak, E use, Tab
//!   first/third person, left mouse attack, F ready weapon (Morrowind's).
//! - **Gamepad** (modern action-RPG convention; combat C1b, owner 2026-10-27): left stick move, right
//!   stick look, South (A / Cross) jump, West (X / Square) use, East (B / Circle) dodge / sprint
//!   (Dark Souls, Elden Ring), right bumper attack, left-stick click sneak, d-pad up first/third
//!   person (right-stick click is kept for the C1c lock-on). No pad zoom or ready-weapon default yet.
//! - **Hold or toggle.** Sprint and sneak hold by default (owner rule: everything holds); each can
//!   switch to toggle per action (`sf_ui::PressMode`, profile `press = "toggle"`, the keybinds
//!   screen's Hold/Toggle cell). Toggle is `bevy_enhanced_input`'s `Toggle` condition on the
//!   action, so the action layer only ever sees a held state.
//! - **Use** lives on its own context entity ([`UseControls`]), because activation also works
//!   in free-camera mode, where there is no player body; `scripts.rs` reads [`use_pressed`];
//!   crates below this one (doors) read [`UsePressed`], published from it each frame.
//! - **Inventory** (I as Skyrim, The Witcher 3, Cyberpunk 2077, Baldur's Gate 3; gamepad North, Y /
//!   Triangle, as Fallout 4's Pip-Boy: Select is the framework's `ui.open_map`, Tab is first/third
//!   person here) shares the use context,
//!   which stays live while a menu pauses the player, so the same key closes the screen;
//!   `inventory_ui.rs` reads [`inventory_pressed`].
//! - **Quicksave / quickload** (F5 / F9, no gamepad default) share the use context too; `saves.rs`
//!   reads [`QuickSaveAction`] / [`QuickLoadAction`].
//! - **Keybinds screen** (dev): F10 opens the stock `sf_ui_screens` keybinds screen (every
//!   context) and closes it; Back closes it too. Bindings save into the selected mod profile
//!   (`profiles/<name>.toml`) when the menu closes (`sf_ui_bevy` `sync_profile`).
//! - Dev keys outside the keymap: F11 player / free camera (`player::TOGGLE_KEY`), F10 here.
//!   TODO(input): the journal's `J` (`dialogue.rs`) moves into the keymap in a later slice.

use bevy_app::{App, PreUpdate, Startup, Update};
use bevy_camera_controller::free_camera::FreeCamera;
use bevy_ecs::prelude::*;
use bevy_ecs::system::EntityCommands;
use bevy_enhanced_input::prelude::*;
use bevy_input::{ButtonInput, keyboard::KeyCode};
use bevy_log::warn;
use bevy_ui::{Node, PositionType, percent};
pub use mw_game_core::player::UsePressed;
use sf_ui::{ActionDef, ActionRef, Keymap, L10nSource, LanguageIdentifier, Layer, Localizer, PressMode, UiRegistry};
use sf_ui_bevy::{
    HUD_SCOPE, ScreenStackRes, UiKeymap, UiLocalizer, UiProfile, UiRegistryRes, UiScreenRoot, keymap_bindings,
    press_condition,
};
use sf_ui_screens::KeybindsScreen;

/// The gameplay keymap context (and profile table).
pub const CONTEXT: &str = "gameplay";
/// Opens / closes the keybinds screen (dev; outside the keymap).
pub const KEYBINDS_KEY: KeyCode = KeyCode::F10;
/// The keybinds screen's id.
pub const KEYBINDS_SCREEN: &str = "keybinds";

/// The keybinds screen record. TODO(HUD slice): with `dialogue.rs`' screens, in the builtin
/// mod's data once more screens land (everything is a mod).
const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "keybinds"
value = { layer = "Menu", implementation = { Template = "keybinds" } }
"#;

/// The player's rebindable gameplay actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Control {
    /// Keys for the move stick, one direction each (rebindable separately, as PC games do).
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,
    /// The move stick on a gamepad.
    Move,
    Look,
    Zoom,
    Jump,
    /// Dodge / Sprint on one button (combat-adapter.md "C1b"): tap rolls on release, hold sprints
    /// (`sf_action_bevy::PlayerAction::DodgeSprint`). Keeps the `sprint` name (saved profiles).
    Sprint,
    Sneak,
    Use,
    ToggleView,
    Inventory,
    /// Light attack (`ActionClass::Light`); while the weapon is sheathed it draws instead.
    Attack,
    /// Ready / sheathe the weapon (Morrowind's F).
    ReadyWeapon,
    /// Save / load the quick slot (`saves.rs`).
    QuickSave,
    QuickLoad,
}

impl Control {
    pub const ALL: [Control; 17] = [
        Control::MoveForward,
        Control::MoveBack,
        Control::MoveLeft,
        Control::MoveRight,
        Control::Move,
        Control::Look,
        Control::Zoom,
        Control::Jump,
        Control::Sprint,
        Control::Sneak,
        Control::Use,
        Control::ToggleView,
        Control::Inventory,
        Control::Attack,
        Control::ReadyWeapon,
        Control::QuickSave,
        Control::QuickLoad,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Control::MoveForward => "move_forward",
            Control::MoveBack => "move_back",
            Control::MoveLeft => "move_left",
            Control::MoveRight => "move_right",
            Control::Move => "move",
            Control::Look => "look",
            Control::Zoom => "zoom",
            Control::Jump => "jump",
            Control::Sprint => "sprint",
            Control::Sneak => "sneak",
            Control::Use => "use",
            Control::ToggleView => "toggle_view",
            Control::Inventory => "inventory",
            Control::Attack => "attack",
            Control::ReadyWeapon => "ready_weapon",
            Control::QuickSave => "quicksave",
            Control::QuickLoad => "quickload",
        }
    }

    pub fn id(self) -> ActionRef {
        ActionRef::new(CONTEXT, self.name())
    }

    /// The definition with its defaults (module docs).
    pub fn def(self) -> ActionDef {
        let name = self.name();
        let button = |defaults: &[&str]| ActionDef::button(CONTEXT, name, defaults.iter().copied());
        let keys = |key: &str| button(&[key]).slots(2, 0);
        match self {
            Control::MoveForward => keys("key:KeyW"),
            Control::MoveBack => keys("key:KeyS"),
            Control::MoveLeft => keys("key:KeyA"),
            Control::MoveRight => keys("key:KeyD"),
            Control::Move => ActionDef::stick(CONTEXT, name, ["stick:Left"]),
            Control::Look => ActionDef::stick(CONTEXT, name, ["mouse_motion", "stick:Right"]).slots(1, 1),
            Control::Zoom => button(&["mouse_wheel"]).slots(1, 0),
            Control::Jump => button(&["key:Space", "pad:South"]),
            Control::Sprint => button(&["key:ShiftLeft", "pad:East"]).press(PressMode::Hold),
            Control::Sneak => button(&["key:ControlLeft", "pad:LeftThumb"]).press(PressMode::Hold),
            Control::Use => button(&["key:KeyE", "pad:West"]),
            Control::ToggleView => button(&["key:Tab", "pad:DPadUp"]),
            Control::Inventory => button(&["key:KeyI", "pad:North"]),
            // Left mouse; the right bumper (souls-likes' R1 light attack).
            Control::Attack => button(&["mouse:Left", "pad:RightTrigger"]),
            Control::ReadyWeapon => keys("key:KeyF"),
            // F5 / F9 as Skyrim, Fallout 4, The Witcher 3 (and Morrowind); no gamepad default.
            Control::QuickSave => button(&["key:F5"]),
            Control::QuickLoad => button(&["key:F9"]),
        }
    }
}

/// Registers the gameplay actions in `keymap` and links their scope to the HUD's.
pub fn register(keymap: &mut Keymap) {
    for control in Control::ALL {
        if keymap.def(&control.id()).is_none() {
            keymap.register(control.def()).expect("gameplay defaults are valid");
        }
    }
    keymap.link_scopes(CONTEXT, HUD_SCOPE);
}

/// The UI actions plus the gameplay actions at their defaults (tests, here and in the app; the app
/// registers into `UiKeymap`).
pub fn keymap() -> Keymap {
    let mut keymap = sf_ui_bevy::ui_keymap();
    register(&mut keymap);
    keymap
}

/// Spawns `control`'s keymap bindings on the action entity `action`, each given its modifiers
/// by `modify` (per binding: a stick's vertical axis feeds Y, mouse and stick scale apart), and
/// the action's hold / toggle condition.
pub fn bind(
    commands: &mut Commands,
    keymap: &Keymap,
    control: Control,
    action: Entity,
    mut modify: impl FnMut(&Binding, &mut EntityCommands<'_>),
) {
    let id = control.id();
    for binding in keymap_bindings(keymap, &id) {
        let mut entity = commands.spawn((BindingOf(action), binding));
        modify(&binding, &mut entity);
    }
    if let Some(toggle) = press_condition(keymap, &id) {
        commands.entity(action).insert(toggle);
    }
}

/// Context of the use action (lives without a player body: free-camera activation).
#[derive(Component, Debug, Default)]
pub struct UseControls;

/// Activate what the focus ray picks.
#[derive(InputAction)]
#[action_output(bool)]
pub struct UseAction;

/// The use action started this frame.
pub fn use_pressed(actions: &Query<&ActionEvents, With<Action<UseAction>>>) -> bool {
    actions.iter().any(|e| e.contains(ActionEvents::START))
}

/// Open / close the inventory (on the use context: live while a menu is open).
#[derive(InputAction)]
#[action_output(bool)]
pub struct InventoryAction;

/// The inventory action started this frame.
pub fn inventory_pressed(actions: &Query<&ActionEvents, With<Action<InventoryAction>>>) -> bool {
    actions.iter().any(|e| e.contains(ActionEvents::START))
}

/// Quicksave (`saves.rs`; on the use context).
#[derive(InputAction)]
#[action_output(bool)]
pub struct QuickSaveAction;

/// Quickload (`saves.rs`; on the use context).
#[derive(InputAction)]
#[action_output(bool)]
pub struct QuickLoadAction;

/// After `dialogue::add` (it adds `SfUiPlugin` when the run has records): the gameplay actions
/// in the keymap (inserted here when there is no UI), the profile's bindings, the use context,
/// rebinding, and F10 for the keybinds screen.
pub fn add(app: &mut App) {
    if !app.is_plugin_added::<EnhancedInputPlugin>() {
        app.add_plugins(EnhancedInputPlugin);
    }
    app.init_resource::<UiKeymap>();
    if !app.world().contains_resource::<UiLocalizer>() {
        app.insert_resource(strings());
    }
    let profile = mw_boot_bevy::mods::bindings_profile();
    let world = app.world_mut();
    let mut keymap = world.resource_mut::<UiKeymap>();
    register(&mut keymap.0);
    if let Some((_, profile)) = &profile {
        for issue in keymap.0.load_from(profile) {
            warn!("bindings: {}: {}", issue.action, issue.message);
        }
    }
    if let Some(mut registry) = world.get_resource_mut::<UiRegistryRes>() {
        registry.0.extend_screens(&screens());
        if let Some((dir, profile)) = profile
            && !world.contains_resource::<UiProfile>()
        {
            world.insert_resource(UiProfile { profile, dir: Some(dir), dirty: false, last_error: None });
        }
    }
    add_use(app);
    app.add_systems(Update, (free_camera_keys, toggle_keybinds, close_keybinds));
}

/// The use context (spawned at startup, respawned on a rebind) and [`UsePressed`] from it.
fn add_use(app: &mut App) {
    app.add_input_context::<UseControls>()
        .init_resource::<UsePressed>()
        .add_systems(Startup, |mut commands: Commands, keymap: Res<UiKeymap>| spawn_use(&mut commands, &keymap.0))
        .add_systems(PreUpdate, publish_use.after(EnhancedInputSystems::Update))
        .add_systems(Update, rebind_use);
}

/// [`UsePressed`] for this frame (crates below this one read it: doors).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn publish_use(uses: Query<&ActionEvents, With<Action<UseAction>>>, mut pressed: ResMut<UsePressed>) {
    pressed.0 = use_pressed(&uses);
}

/// The builtin mod's UI strings (`l10n/en/bevyrrowind.ftl`: keybind labels
/// `keybind-<context>-<action>`). TODO(l10n): from every mod's `l10n/<locale>/<mod>.ftl` through
/// `sf_ui_bevy::insert_from_profile` once the app builds its UI state from the loaded profile.
fn strings() -> UiLocalizer {
    let en: LanguageIdentifier = "en".parse().expect("valid locale");
    let source = L10nSource {
        mod_id: "bevyrrowind".into(),
        locale: en.clone(),
        path: "l10n/en/bevyrrowind.ftl".into(),
        text: include_str!("../l10n/en/bevyrrowind.ftl").into(),
    };
    let (localizer, issues) = Localizer::build(&[en], &[source], false);
    for issue in issues {
        warn!("ui strings: {}: {}", issue.path, issue.message);
    }
    UiLocalizer(localizer)
}

fn screens() -> UiRegistry {
    let mut db = sf_records::RecordDb::new();
    sf_ui::register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    let outcome = sf_records::apply_record_file(&mut db, base, SCREENS, sf_records::RecordFormat::Toml).expect("keybinds screen");
    assert!(outcome.errors.is_empty(), "keybinds screen: {:?}", outcome.errors);
    UiRegistry::build(&db).0
}

fn spawn_use(commands: &mut Commands, keymap: &Keymap) {
    let context = commands.spawn((Name::new("use controls"), UseControls)).id();
    let action = commands.spawn((ActionOf::<UseControls>::new(context), Action::<UseAction>::new())).id();
    bind(commands, keymap, Control::Use, action, |_, _| {});
    let inventory = commands.spawn((ActionOf::<UseControls>::new(context), Action::<InventoryAction>::new())).id();
    bind(commands, keymap, Control::Inventory, inventory, |_, _| {});
    let quicksave = commands.spawn((ActionOf::<UseControls>::new(context), Action::<QuickSaveAction>::new())).id();
    bind(commands, keymap, Control::QuickSave, quicksave, |_, _| {});
    let quickload = commands.spawn((ActionOf::<UseControls>::new(context), Action::<QuickLoadAction>::new())).id();
    bind(commands, keymap, Control::QuickLoad, quickload, |_, _| {});
}

/// A keymap change respawns the use context with the new bindings.
fn rebind_use(
    mut commands: Commands,
    keymap: Res<UiKeymap>,
    mut seen: Local<Option<u64>>,
    contexts: Query<Entity, With<UseControls>>,
) {
    let version = keymap.0.version();
    if seen.replace(version).is_none_or(|v| v == version) {
        return;
    }
    for context in &contexts {
        commands.entity(context).despawn();
    }
    spawn_use(&mut commands, &keymap.0);
}

/// Free-camera fly-up key when Jump has no keyboard binding (the default Jump key).
pub const FREE_CAMERA_UP: KeyCode = KeyCode::Space;
/// Free-camera fly-down key when Sneak has no keyboard binding (the default Sneak key).
pub const FREE_CAMERA_DOWN: KeyCode = KeyCode::ControlLeft;

/// The first keyboard key bound to `control`, or `fallback`.
fn first_key(keymap: &Keymap, control: Control, fallback: KeyCode) -> KeyCode {
    keymap_bindings(keymap, &control.id())
        .into_iter()
        .find_map(|binding| match binding {
            Binding::Keyboard { key, .. } => Some(key),
            _ => None,
        })
        .unwrap_or(fallback)
}

/// Bevy's free camera flies up / down on the Jump / Sneak keys (its own defaults, E / Q, clash
/// with Use), following rebinds; set when the keymap changes or a free camera appears.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn free_camera_keys(keymap: Res<UiKeymap>, mut cams: Query<&mut FreeCamera>) {
    let changed = keymap.is_changed();
    let mut keys = None;
    for mut cam in &mut cams {
        if !changed && !cam.is_added() {
            continue;
        }
        let (up, down) = *keys.get_or_insert_with(|| {
            (first_key(&keymap.0, Control::Jump, FREE_CAMERA_UP), first_key(&keymap.0, Control::Sneak, FREE_CAMERA_DOWN))
        });
        if cam.key_up != up || cam.key_down != down {
            cam.key_up = up;
            cam.key_down = down;
        }
    }
}

/// F10 opens the keybinds screen when no other menu is open and closes it when it is.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn toggle_keybinds(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    stack: Option<ResMut<ScreenStackRes>>,
    registry: Option<Res<UiRegistryRes>>,
) {
    let (Some(mut stack), Some(registry)) = (stack, registry) else { return };
    if !keys.just_pressed(KEYBINDS_KEY) {
        return;
    }
    if stack.0.is_open(KEYBINDS_SCREEN) {
        stack.0.close(KEYBINDS_SCREEN);
        return;
    }
    if stack.0.top(Layer::Menu).is_some() {
        return;
    }
    open_keybinds(&mut commands, &mut stack, &registry);
}

/// Opens the keybinds screen (every context) on top of the stack: F10 here, the options
/// screen's "Key bindings" row (`pause_menu.rs`). Its Back closes it, back to what was below.
pub fn open_keybinds(commands: &mut Commands, stack: &mut ScreenStackRes, registry: &UiRegistryRes) {
    if stack.0.is_open(KEYBINDS_SCREEN) {
        return;
    }
    let centred = Node {
        position_type: PositionType::Absolute,
        left: percent(15),
        top: percent(8),
        ..Node::default()
    };
    commands.spawn((UiScreenRoot(KEYBINDS_SCREEN.into()), centred, KeybindsScreen::new(None)));
    if let Err(e) = stack.0.push(&registry.0, KEYBINDS_SCREEN) {
        warn!("keybinds: {e:?}");
    }
}

/// The keybinds screen's root despawns once it is off the stack (F10, Back).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn close_keybinds(mut commands: Commands, stack: Option<Res<ScreenStackRes>>, roots: Query<(Entity, &UiScreenRoot)>) {
    let Some(stack) = stack else { return };
    for (e, root) in &roots {
        if root.0.as_str() == KEYBINDS_SCREEN && !stack.0.is_open(KEYBINDS_SCREEN) {
            commands.entity(e).despawn();
        }
    }
}

/// The profile file the bindings save into, reloaded (the app's tests: a restart).
pub fn reload(dir: &std::path::Path, name: &str) -> sf_mods::Profile {
    sf_mods::Profile::load(sf_mods::Profile::path_in(dir, name)).expect("profile reloads")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The free camera flies up / down on the Jump / Sneak keys (not E / Q) and follows a rebind.
    #[test]
    fn free_camera_follows_jump_and_sneak() {
        let mut app = App::new();
        app.insert_resource(UiKeymap(keymap())).add_systems(Update, free_camera_keys);
        let cam = app.world_mut().spawn(FreeCamera::default()).id();
        app.update();
        let keys = |app: &App| {
            let cam = app.world().get::<FreeCamera>(cam).expect("free camera");
            (cam.key_up, cam.key_down)
        };
        assert_eq!(keys(&app), (KeyCode::Space, KeyCode::ControlLeft));
        app.world_mut()
            .resource_mut::<UiKeymap>()
            .0
            .set(&Control::Jump.id(), sf_ui::Device::Kbm, &["key:KeyR"])
            .expect("rebind jump");
        app.update();
        assert_eq!(keys(&app), (KeyCode::KeyR, KeyCode::ControlLeft));
    }

    /// [`UsePressed`] (what opens doors) follows the Use binding: E by default; once Use is
    /// rebound to F, F does and E doesn't.
    #[test]
    fn use_pressed_follows_the_use_binding() {
        let mut app = App::new();
        app.add_plugins((bevy_time::TimePlugin, bevy_input::InputPlugin, EnhancedInputPlugin))
            .insert_resource(UiKeymap(keymap()));
        add_use(&mut app);
        // `EnhancedInputPlugin` creates its context registry in `finish` (`App::run` calls it).
        app.finish();
        app.cleanup();
        app.update();
        let tap = |app: &mut App, key: KeyCode| -> bool {
            app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(key);
            app.update();
            let used = app.world().resource::<UsePressed>().0;
            app.world_mut().resource_mut::<ButtonInput<KeyCode>>().release(key);
            app.update();
            assert!(!app.world().resource::<UsePressed>().0, "Use stays pressed after {key:?} is released");
            used
        };
        assert!(tap(&mut app, KeyCode::KeyE), "E (default Use) didn't press Use");
        assert!(!tap(&mut app, KeyCode::KeyF));
        app.world_mut()
            .resource_mut::<UiKeymap>()
            .0
            .set(&Control::Use.id(), sf_ui::Device::Kbm, &["key:KeyF"])
            .expect("rebind use");
        // The use context respawns with the new bindings.
        app.update();
        app.update();
        assert!(!tap(&mut app, KeyCode::KeyE), "E still presses Use after the rebind");
        assert!(tap(&mut app, KeyCode::KeyF), "the rebound key didn't press Use");
    }

    /// Every keybinds row (gameplay and UI actions) has a display name in the builtin strings.
    #[test]
    fn every_action_has_a_label() {
        let strings = strings();
        let keymap = keymap();
        let rows = keymap.rows(None);
        assert!(!rows.is_empty());
        for row in rows {
            assert!(strings.0.has(&row.label), "no string for {}", row.label);
        }
        assert_eq!(strings.0.format("keybind-gameplay-move_forward", None), "Move Forward");
        assert_eq!(strings.0.format("keybind-gameplay-use", None), "Activate");
    }
}
