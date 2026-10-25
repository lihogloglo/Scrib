//! The player's gameplay actions in the player's keymap (ROADMAP 2.1b; superfargoth `ui.md`
//! "As built (U6)"): every gameplay input is a rebindable `sf_ui_bevy::UiKeymap` action in the
//! `gameplay` context (profile table `[bindings.gameplay]`), its scope linked to `HUD_SCOPE`
//! (open menu / map live beside play), so one keybinds screen and one profile cover UI and play.
//! `player.rs` and the use context here spawn their `bevy_enhanced_input` bindings from it, and
//! respawn them when it changes (a rebind applies at once).
//!
//! - **Keyboard and mouse** (the owner's modern defaults): WASD move, mouse look, wheel zoom,
//!   Space jump, Left Shift sprint, Left Ctrl sneak, E use, Tab first/third person.
//! - **Gamepad** (modern action-RPG convention): left stick move, right stick look, South (A /
//!   Cross) jump, West (X / Square) use, left-stick click sprint (Skyrim SE, ESO, Cyberpunk 2077),
//!   East (B / Circle) sneak (Cyberpunk 2077, The Last of Us), right-stick click first/third person
//!   (Skyrim, Fallout 4). No pad zoom yet (the d-pad is kept for quick slots).
//! - **Hold or toggle.** Sprint and sneak hold by default (owner rule: everything holds); each can
//!   switch to toggle per action (`sf_ui::PressMode`, profile `press = "toggle"`, the keybinds
//!   screen's Hold/Toggle cell). Toggle is `bevy_enhanced_input`'s `Toggle` condition on the
//!   action, so the action layer only ever sees a held state.
//! - **Use** lives on its own context entity ([`UseControls`]), because activation also works
//!   in free-camera mode, where there is no player body; `scripts.rs` reads [`use_pressed`].
//! - **Keybinds screen** (dev): F10 opens the stock `sf_ui_screens` keybinds screen (every
//!   context) and closes it; Back closes it too. Bindings save into the selected mod profile
//!   (`profiles/<name>.toml`) when the menu closes (`sf_ui_bevy` `sync_profile`).
//! - Dev keys outside the keymap: F11 player / free camera (`player::TOGGLE_KEY`), F10 here.
//!   TODO(input): the journal's `J` (`dialogue.rs`) moves into the keymap in a later slice.

use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use sf_ui::{ActionDef, ActionRef, Keymap, Layer, PressMode, UiRegistry};
use sf_ui_bevy::{
    HUD_SCOPE, ScreenStackRes, UiKeymap, UiProfile, UiRegistryRes, UiScreenRoot, keymap_bindings, press_condition,
};
use sf_ui_screens::KeybindsScreen;

/// The gameplay keymap context (and profile table).
pub const CONTEXT: &str = "gameplay";
/// Opens / closes the keybinds screen (dev; outside the keymap).
pub const KEYBINDS_KEY: KeyCode = KeyCode::F10;
/// The keybinds screen's id.
const KEYBINDS_SCREEN: &str = "keybinds";

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
    Sprint,
    Sneak,
    Use,
    ToggleView,
}

impl Control {
    pub const ALL: [Control; 12] = [
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
            Control::Sprint => button(&["key:ShiftLeft", "pad:LeftThumb"]).press(PressMode::Hold),
            Control::Sneak => button(&["key:ControlLeft", "pad:East"]).press(PressMode::Hold),
            Control::Use => button(&["key:KeyE", "pad:West"]),
            Control::ToggleView => button(&["key:Tab", "pad:RightThumb"]),
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

/// The UI actions plus the gameplay actions at their defaults (tests; the app registers into `UiKeymap`).
#[cfg(test)]
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

/// After `dialogue::add` (it adds `SfUiPlugin` when the run has records): the gameplay actions
/// in the keymap (inserted here when there is no UI), the profile's bindings, the use context,
/// rebinding, and F10 for the keybinds screen.
pub fn add(app: &mut App) {
    if !app.is_plugin_added::<EnhancedInputPlugin>() {
        app.add_plugins(EnhancedInputPlugin);
    }
    app.init_resource::<UiKeymap>();
    let profile = crate::mods::bindings_profile();
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
    app.add_input_context::<UseControls>()
        .add_systems(Startup, |mut commands: Commands, keymap: Res<UiKeymap>| spawn_use(&mut commands, &keymap.0))
        .add_systems(Update, (rebind_use, toggle_keybinds, close_keybinds));
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
    let centred = Node {
        position_type: PositionType::Absolute,
        left: percent(15),
        top: percent(8),
        ..default()
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

/// The profile file the bindings save into, reloaded (tests: a restart).
#[cfg(test)]
pub fn reload(dir: &std::path::Path, name: &str) -> sf_mods::Profile {
    sf_mods::Profile::load(sf_mods::Profile::path_in(dir, name)).expect("profile reloads")
}
