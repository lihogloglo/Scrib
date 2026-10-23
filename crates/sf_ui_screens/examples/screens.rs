//! Every stock screen over synthetic data (`--features demo`), unthemed (the fallback look).
//!
//! `cargo run -p sf_ui_screens --features demo --example screens` opens a window with the HUD.
//! F1 dialogue, F2 journal, F3 inventory (1,000 stacks), F4 keybinds; on a pad, Start (Esc)
//! cycles them from the HUD and View (M) opens the inventory. B (Esc) closes a screen.
//! Hold H to drain the HUD's health bar.
//!
//! `--autotest [dir]` opens each screen in turn with a synthetic gamepad, moves the focus, and
//! exits non-zero if a screen didn't build its parts or the focus didn't land on one of them;
//! with a directory it saves a screenshot of each (`<dir>/<screen>.png`).

use bevy::input::gamepad::{Gamepad, GamepadButton};
use bevy::input_focus::{InputFocus, InputFocusVisible};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy_enhanced_input::prelude::EnhancedInputPlugin;
use sf_rpg_ui::{Alphabet, JournalScreen, JournalSeen};
use sf_ui::Layer;
use sf_ui_bevy::{
    ScreenStackRes, SfUiPlugin, UiAction, UiActionEvent, UiKeymap, UiRegistryRes, UiScreenRoot,
    UiTextPlugin,
};
use sf_ui_screens::demo::{self, DemoInventory, DemoWorld};
use sf_ui_screens::{
    DialogueScreen, HudScreen, HudView, InventoryScreen, JournalUi, JournalUiPlugin,
    KeybindsScreen, ScreenSystems, SfScreensPlugin,
};

const MENUS: [&str; 4] = ["dialogue", "journal", "inventory", "keybinds"];

fn main() -> AppExit {
    let args: Vec<String> = std::env::args().collect();
    let autotest = args
        .iter()
        .position(|a| a == "--autotest")
        .map(|i| args.get(i + 1).cloned());
    let registry = demo::registry();
    let mut stack = ScreenStackRes::default();
    stack.0.push(&registry, "hud").expect("hud screen");
    let mut keymap = UiKeymap::default();
    demo::register_gameplay(&mut keymap.0);

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "sf_ui_screens: screens".into(),
                ..default()
            }),
            ..default()
        }),
        EnhancedInputPlugin,
        SfUiPlugin,
        UiTextPlugin,
        SfScreensPlugin,
        JournalUiPlugin::<DemoWorld>::default(),
    ))
    .insert_resource(UiRegistryRes(registry))
    .insert_resource(stack)
    .insert_resource(keymap)
    .insert_resource(InputFocusVisible(true))
    .insert_resource(demo::world(24, 40))
    .insert_resource(DemoInventory::new(1000))
    .insert_resource(demo::hud())
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        (demo::host_dialogue, demo::host_inventory).after(ScreenSystems::Input),
    )
    .add_systems(
        Update,
        (open_screens, show_open_screens, drain_health).chain(),
    );
    if let Some(shots) = autotest {
        app.insert_resource(Autotest { shots, ..default() })
            .add_systems(Update, drive_autotest.after(ScreenSystems::Sync));
    }
    app.run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((UiScreenRoot("hud".into()), HudScreen));
}

/// The menu root of `id`, centred.
fn centred(commands: &mut Commands, id: &str) -> Entity {
    commands
        .spawn((
            UiScreenRoot(id.into()),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(8.0),
                top: Val::Percent(8.0),
                ..default()
            },
        ))
        .id()
}

/// Opens (respawning) a menu screen with fresh demo data.
fn open(
    commands: &mut Commands,
    id: &str,
    stack: &mut ScreenStackRes,
    registry: &UiRegistryRes,
    roots: &Query<(Entity, &UiScreenRoot)>,
    world: &mut DemoWorld,
    inventory: &DemoInventory,
) {
    for (entity, root) in roots {
        if root.0.as_str() == id {
            commands.entity(entity).despawn();
        }
    }
    let root = centred(commands, id);
    match id {
        "dialogue" => {
            let (talk, view) = demo::open_dialogue(&mut world.world);
            commands.insert_resource(talk);
            commands.entity(root).insert(DialogueScreen::new(view));
        }
        "journal" => {
            let screen = JournalScreen::new(Alphabet::default(), 2, JournalSeen::default());
            commands.entity(root).insert(JournalUi::new(screen));
        }
        "inventory" => {
            let labels = vec![
                ("damage".into(), "Damage".into()),
                ("armor".into(), "Armor".into()),
            ];
            commands
                .entity(root)
                .insert(InventoryScreen::new(inventory.view()).with_stat_labels(labels));
        }
        _ => {
            commands.entity(root).insert(KeybindsScreen::new(None));
        }
    }
    let _ = stack.0.push(&registry.0, id);
}

#[allow(clippy::too_many_arguments)]
fn open_screens(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut actions: MessageReader<UiActionEvent>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    roots: Query<(Entity, &UiScreenRoot)>,
    mut world: ResMut<DemoWorld>,
    inventory: Res<DemoInventory>,
    mut next: Local<usize>,
) {
    let mut wanted = None;
    for (key, id) in [KeyCode::F1, KeyCode::F2, KeyCode::F3, KeyCode::F4]
        .into_iter()
        .zip(MENUS)
    {
        if keys.just_pressed(key) {
            wanted = Some(id);
        }
    }
    for event in actions.read() {
        if event.screen.as_str() != "hud" {
            continue;
        }
        match event.action {
            UiAction::OpenMenu => {
                wanted = Some(MENUS[*next % MENUS.len()]);
                *next += 1;
            }
            UiAction::OpenMap => wanted = Some("inventory"),
            _ => {}
        }
    }
    if let Some(id) = wanted {
        // One menu at a time.
        while stack.0.top(Layer::Menu).is_some() {
            stack.0.pop(Layer::Menu);
        }
        open(
            &mut commands,
            id,
            &mut stack,
            &registry,
            &roots,
            &mut world,
            &inventory,
        );
    }
}

/// Screens not on the stack are hidden (an app's job: it decides whether closed screens
/// despawn or wait).
fn show_open_screens(stack: Res<ScreenStackRes>, mut roots: Query<(&UiScreenRoot, &mut Node)>) {
    for (root, mut node) in &mut roots {
        let open = stack.0.is_open(root.0.as_str());
        let display = if open { Display::Flex } else { Display::None };
        if node.display != display {
            node.display = display;
        }
    }
}

fn drain_health(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut hud: ResMut<HudView>) {
    if keys.pressed(KeyCode::KeyH) {
        let bar = &mut hud.bars[0];
        bar.current = (bar.current - 30.0 * time.delta_secs()).max(0.0);
    } else if hud.bars[0].current < hud.bars[0].max {
        let bar = &mut hud.bars[0];
        bar.current = (bar.current + 10.0 * time.delta_secs()).min(bar.max);
    }
}

#[derive(Resource, Default)]
struct Autotest {
    frame: u32,
    pad: Option<Entity>,
    /// Screenshot directory.
    shots: Option<String>,
}

/// Each menu for 60 frames: open, press down and right, check the parts and the focus.
#[allow(clippy::too_many_arguments)]
fn drive_autotest(
    mut commands: Commands,
    mut test: ResMut<Autotest>,
    mut pads: Query<&mut Gamepad>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    roots: Query<(Entity, &UiScreenRoot)>,
    children: Query<&Children>,
    parents: Query<&ChildOf>,
    focus: Res<InputFocus>,
    mut world: ResMut<DemoWorld>,
    inventory: Res<DemoInventory>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(pad) = test.pad else {
        test.pad = Some(commands.spawn(Gamepad::default()).id());
        return;
    };
    test.frame += 1;
    let (screen, step) = ((test.frame / 60) as usize, test.frame % 60);
    if screen >= MENUS.len() {
        info!("autotest passed");
        exit.write(AppExit::Success);
        return;
    }
    let id = MENUS[screen];
    let Ok(mut gamepad) = pads.get_mut(pad) else {
        return;
    };
    match step {
        1 => {
            while stack.0.top(Layer::Menu).is_some() {
                stack.0.pop(Layer::Menu);
            }
            open(
                &mut commands,
                id,
                &mut stack,
                &registry,
                &roots,
                &mut world,
                &inventory,
            );
        }
        10 | 20 => press(&mut gamepad, GamepadButton::DPadDown, true),
        11 | 21 => press(&mut gamepad, GamepadButton::DPadDown, false),
        30 => press(&mut gamepad, GamepadButton::DPadRight, true),
        31 => press(&mut gamepad, GamepadButton::DPadRight, false),
        36 => {
            if let Some(dir) = &test.shots {
                let path = format!("{dir}/{id}.png");
                commands
                    .spawn(Screenshot::primary_window())
                    .observe(save_to_disk(path));
            }
        }
        59 => {
            let Some((root, _)) = roots.iter().find(|(_, r)| r.0.as_str() == id) else {
                error!("{id}: no screen root");
                exit.write(AppExit::error());
                return;
            };
            match children.get(root) {
                Ok(parts) if !parts.is_empty() => {}
                _ => {
                    error!("{id}: no parts");
                    exit.write(AppExit::error());
                    return;
                }
            }
            let inside = focus.get().is_some_and(|f| {
                std::iter::successors(Some(f), |e| parents.get(*e).ok().map(ChildOf::parent))
                    .any(|e| e == root)
            });
            // The dialogue's focus may sit on a text link (no node focus); others must focus a part.
            if !inside && id != "dialogue" {
                error!("{id}: focus {:?} is not on the screen", focus.get());
                exit.write(AppExit::error());
                return;
            }
            info!("{id}: ok");
        }
        _ => {}
    }
}

fn press(gamepad: &mut Gamepad, button: GamepadButton, down: bool) {
    if down {
        gamepad.digital_mut().press(button);
    } else {
        gamepad.digital_mut().release(button);
    }
    gamepad
        .analog_mut()
        .set(button, if down { 1.0 } else { 0.0 });
}
