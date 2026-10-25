//! Gamepad-only navigation across a HUD, a menu and a modal (ROADMAP M5 U1 Verify).
//!
//! `cargo run -p sf_ui_bevy --example ui_nav` opens a window: Start (Esc) opens the menu, the d-pad,
//! left stick or arrows move focus (held directions repeat), A (Enter) activates, B (Esc) goes back,
//! LB/RB (Q/E) tab, View (M) "map". "Quit" opens a confirm modal; navigation stays inside it.
//! The HUD shows the last-used device and the glyphs of the bound buttons.
//!
//! `--autotest [screenshot.png]` drives the same app with a synthetic gamepad (a `Gamepad` entity
//! whose buttons the script presses), checks every step, saves a screenshot with the modal open if a
//! path is given, and exits non-zero on the first failed check.

use bevy::input::gamepad::{Gamepad, GamepadButton};
use bevy::input_focus::{InputFocus, InputFocusVisible};
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::ui_widgets::{Activate, Button};
use bevy_enhanced_input::prelude::EnhancedInputPlugin;
use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_ui::{Layer, UiRegistry, register_kinds};
use sf_ui_bevy::{
    Device, GamepadFamily, InputMode, ScreenStackRes, SfUiPlugin, UiAction, UiActionEvent,
    UiDefaultFocus, UiFocusable, UiKeymap, UiRegistryRes, UiScreenRoot, glyph,
};

const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "hud"
value = { layer = "Hud", implementation = { Template = "hud" } }

[[define]]
kind = "ui.screen"
id = "menu"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Template = "menu" } }

[[define]]
kind = "ui.screen"
id = "confirm"
value = { layer = "Modal", input = "modal", implementation = { Template = "confirm" } }
"#;

const MENU: [&str; 6] = ["Inventory", "Magic", "Map", "Journal", "Options", "Quit"];

fn main() -> AppExit {
    let args: Vec<String> = std::env::args().collect();
    let autotest = args
        .iter()
        .position(|a| a == "--autotest")
        .map(|i| args.get(i + 1).cloned());

    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let source = db.add_source("example", &[]);
    apply_record_file(&mut db, source, SCREENS, RecordFormat::Toml).unwrap();
    let (registry, _) = UiRegistry::build(&db);
    let mut stack = ScreenStackRes::default();
    stack.0.push(&registry, "hud").unwrap();

    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "sf_ui_bevy: ui_nav".into(),
                ..default()
            }),
            ..default()
        }),
        EnhancedInputPlugin,
        SfUiPlugin,
    ))
    .insert_resource(UiRegistryRes(registry))
    .insert_resource(stack)
    .insert_resource(InputFocusVisible(true))
    .add_systems(Startup, |mut commands: Commands| {
        commands.spawn(Camera2d);
    })
    .add_systems(
        Update,
        (game_reacts, spawn_screens, highlight_focus, hud_text)
            .chain()
            .after(sf_ui_bevy::UiSystems::Dispatch),
    );
    if let Some(shot) = autotest {
        app.insert_resource(Script { shot, ..default() })
            .add_systems(Update, run_script.before(sf_ui_bevy::UiSystems::Dispatch));
    }
    app.run()
}

/// The game's side of the actions: open/close the menu, log the rest.
fn game_reacts(
    mut events: MessageReader<UiActionEvent>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
) {
    for event in events.read() {
        info!("action {:?} on `{}`", event.action, event.screen.0);
        if event.action == UiAction::OpenMenu {
            if stack.0.is_open("menu") {
                stack.0.close("menu");
            } else {
                stack.0.push(&registry.0, "menu").unwrap();
            }
        }
    }
}

#[derive(Component)]
struct HudText;

/// Spawns a screen's root when it opens and despawns it when it closes (a stand-in for U3 templates).
fn spawn_screens(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    roots: Query<(Entity, &UiScreenRoot)>,
) {
    if !stack.is_changed() {
        return;
    }
    for (entity, root) in &roots {
        if !stack.0.is_open(root.0.as_str()) {
            commands.entity(entity).despawn();
        }
    }
    let spawned: Vec<String> = roots.iter().map(|(_, r)| r.0.0.clone()).collect();
    for layer in [Layer::Hud, Layer::Menu, Layer::Modal] {
        for open in stack.0.layer(layer) {
            if spawned.contains(&open.id.0) {
                continue;
            }
            match open.id.as_str() {
                "hud" => spawn_hud(&mut commands),
                "menu" => spawn_menu(&mut commands),
                "confirm" => spawn_confirm(&mut commands),
                _ => {}
            }
        }
    }
}

fn spawn_hud(commands: &mut Commands) {
    commands.spawn((
        UiScreenRoot("hud".into()),
        Node {
            position_type: PositionType::Absolute,
            left: px(16),
            bottom: px(16),
            ..default()
        },
        children![(
            HudText,
            Text::new(""),
            TextFont {
                font_size: FontSize::Px(18.0),
                ..default()
            }
        )],
    ));
}

fn button(label: &str) -> impl Bundle {
    (
        Name::new(label.to_owned()),
        Button,
        UiFocusable,
        Node {
            width: px(180),
            height: px(64),
            border: UiRect::all(px(4)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(Color::NONE),
        BackgroundColor(Color::srgb(0.18, 0.22, 0.32)),
        children![Text::new(label.to_owned())],
    )
}

fn spawn_menu(commands: &mut Commands) {
    let root = commands
        .spawn((
            UiScreenRoot("menu".into()),
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .id();
    let grid = commands
        .spawn((
            Node {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::px(3, 180.0),
                row_gap: px(16),
                column_gap: px(16),
                ..default()
            },
            ChildOf(root),
        ))
        .id();
    for (i, label) in MENU.iter().enumerate() {
        let mut entity = commands.spawn((button(label), ChildOf(grid)));
        if i == 0 {
            entity.insert(UiDefaultFocus);
        }
        if *label == "Quit" {
            entity.observe(
                |_: On<Activate>,
                 mut stack: ResMut<ScreenStackRes>,
                 registry: Res<UiRegistryRes>| {
                    stack.0.push(&registry.0, "confirm").unwrap();
                },
            );
        }
    }
}

fn spawn_confirm(commands: &mut Commands) {
    let root = commands
        .spawn((
            UiScreenRoot("confirm".into()),
            GlobalZIndex(10),
            Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();
    let panel = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(16),
                padding: UiRect::all(px(24)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.30, 0.12, 0.12)),
            ChildOf(root),
            children![Text::new("Quit to desktop?")],
        ))
        .id();
    let row = commands
        .spawn((
            Node {
                column_gap: px(16),
                ..default()
            },
            ChildOf(panel),
        ))
        .id();
    commands.spawn((button("Yes"), ChildOf(row))).observe(
        |_: On<Activate>, mut stack: ResMut<ScreenStackRes>| {
            stack.0.close("confirm");
            stack.0.close("menu");
        },
    );
    commands
        .spawn((button("No"), UiDefaultFocus, ChildOf(row)))
        .observe(|_: On<Activate>, mut stack: ResMut<ScreenStackRes>| {
            stack.0.close("confirm");
        });
}

fn highlight_focus(
    focus: Res<InputFocus>,
    mut buttons: Query<(Entity, &mut BorderColor), With<UiFocusable>>,
) {
    if !focus.is_changed() {
        return;
    }
    for (entity, mut border) in &mut buttons {
        *border = BorderColor::all(if focus.get() == Some(entity) {
            Color::srgb(1.0, 0.85, 0.3)
        } else {
            Color::NONE
        });
    }
}

fn hud_text(
    mode: Res<InputMode>,
    keymap: Res<UiKeymap>,
    family: Res<GamepadFamily>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    let device = mode.0;
    let g = |a| {
        glyph(&keymap, a, device, family.0)
            .map(|g| g.label)
            .unwrap_or_default()
    };
    for mut text in &mut text {
        text.0 = format!(
            "{:?} | {} menu  {} confirm  {} back  {}/{} tabs  {} map",
            device,
            g(UiAction::OpenMenu),
            g(UiAction::Confirm),
            g(UiAction::Back),
            g(UiAction::TabPrev),
            g(UiAction::TabNext),
            g(UiAction::OpenMap),
        );
    }
}

// ---- autotest: a synthetic gamepad drives the app ----

#[derive(Resource, Default)]
struct Script {
    shot: Option<String>,
    step: usize,
    wait: u32,
    pad: Option<Entity>,
}

enum Step {
    Wait(u32),
    Press(GamepadButton),
    Release(GamepadButton),
    /// Checks the input target screen and the focused button (by name, `None` = no focus).
    Expect(&'static str, Option<&'static str>),
    ExpectDevice(Device),
    Screenshot,
}

const SCRIPT: &[Step] = &[
    Step::Wait(30),
    Step::Expect("hud", None),
    Step::Press(GamepadButton::Start),
    Step::Wait(3),
    Step::Release(GamepadButton::Start),
    Step::Wait(10),
    Step::ExpectDevice(Device::Gamepad),
    Step::Expect("menu", Some("Inventory")),
    Step::Press(GamepadButton::DPadRight),
    Step::Release(GamepadButton::DPadRight),
    Step::Expect("menu", Some("Magic")),
    Step::Press(GamepadButton::DPadDown),
    Step::Release(GamepadButton::DPadDown),
    Step::Expect("menu", Some("Options")),
    Step::Press(GamepadButton::DPadRight),
    Step::Release(GamepadButton::DPadRight),
    Step::Expect("menu", Some("Quit")),
    Step::Press(GamepadButton::South),
    Step::Release(GamepadButton::South),
    Step::Wait(10),
    Step::Expect("confirm", Some("No")),
    Step::Screenshot,
    // Up from the modal has no neighbour inside it: focus must not leak to the menu below.
    Step::Press(GamepadButton::DPadUp),
    Step::Release(GamepadButton::DPadUp),
    Step::Expect("confirm", Some("No")),
    Step::Press(GamepadButton::DPadLeft),
    Step::Release(GamepadButton::DPadLeft),
    Step::Expect("confirm", Some("Yes")),
    // Back closes the modal and focus returns to the button that opened it.
    Step::Press(GamepadButton::East),
    Step::Release(GamepadButton::East),
    Step::Wait(5),
    Step::Expect("menu", Some("Quit")),
    // Held left repeats: Quit -> Options -> Journal (edge).
    Step::Press(GamepadButton::DPadLeft),
    Step::Wait(60),
    Step::Release(GamepadButton::DPadLeft),
    Step::Expect("menu", Some("Journal")),
    Step::Press(GamepadButton::East),
    Step::Release(GamepadButton::East),
    Step::Wait(5),
    Step::Expect("hud", None),
];

#[allow(clippy::too_many_arguments)]
fn run_script(
    mut commands: Commands,
    mut script: ResMut<Script>,
    mut pads: Query<&mut Gamepad>,
    stack: Res<ScreenStackRes>,
    focus: Res<InputFocus>,
    names: Query<&Name>,
    mode: Res<InputMode>,
    mut exit: MessageWriter<AppExit>,
) {
    let pad = *script
        .pad
        .get_or_insert_with(|| commands.spawn(Gamepad::default()).id());
    if script.wait > 0 {
        script.wait -= 1;
        return;
    }
    let Some(step) = SCRIPT.get(script.step) else {
        info!("autotest: PASS ({} steps)", SCRIPT.len());
        exit.write(AppExit::Success);
        return;
    };
    let index = script.step;
    script.step += 1;
    script.wait = 3;
    let mut set = |button: GamepadButton, down: bool| {
        if let Ok(mut gamepad) = pads.get_mut(pad) {
            if down {
                gamepad.digital_mut().press(button);
            } else {
                gamepad.digital_mut().release(button);
            }
            gamepad
                .analog_mut()
                .set(button, if down { 1.0 } else { 0.0 });
        }
    };
    let mut fail = |what: String| {
        error!("autotest: FAIL at step {index}: {what}");
        exit.write(AppExit::error());
    };
    match step {
        Step::Wait(n) => script.wait = *n,
        Step::Press(b) => set(*b, true),
        Step::Release(b) => set(*b, false),
        Step::Expect(screen, focused) => {
            let target = stack
                .0
                .input_target()
                .map(|s| s.id.0.clone())
                .unwrap_or_default();
            let name = focus
                .get()
                .and_then(|e| names.get(e).ok())
                .map(|n| n.as_str().to_owned());
            if target != *screen || name.as_deref() != *focused {
                fail(format!(
                    "expected {screen}/{focused:?}, got {target}/{name:?}"
                ));
            }
        }
        Step::ExpectDevice(device) => {
            if mode.0 != *device {
                fail(format!("expected device {device:?}, got {:?}", mode.0));
            }
        }
        Step::Screenshot => {
            if let Some(path) = script.shot.clone() {
                commands
                    .spawn(Screenshot::primary_window())
                    .observe(save_to_disk(path));
            }
        }
    }
}
