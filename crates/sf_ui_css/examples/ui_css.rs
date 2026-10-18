//! A menu styled by three mod sheets in load order (ROADMAP M5 U2 Verify).
//!
//! `cargo run -p sf_ui_css --example ui_css` opens a window. The mods are `base_ui` (look and
//! tokens), `bright_buttons` (loads later, so its button colour wins) and `broken_mod` (one
//! invalid value, reported against it in the HUD). Edit
//! `crates/sf_ui_css/examples/assets/ui/bright_buttons/style.css` and save: the menu restyles.
//! Arrows/d-pad move focus; using a gamepad switches the root `input` attribute and the focus ring.
//!
//! `--autotest [dir]` works on a temporary copy of the assets: it waits for the styled menu and the
//! error report, saves `dir/ui_css_before.png`, rewrites the button colour on disk, waits for the
//! hot reload to apply it, saves `dir/ui_css_after.png` and exits non-zero on a failed check.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use bevy::asset::io::AssetSourceBuilder;
use bevy::color::Srgba;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::ui_widgets::Button;
use bevy_enhanced_input::prelude::EnhancedInputPlugin;
use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_ui::{UiRegistry, register_kinds};
use sf_ui_bevy::{
    InputMode, ScreenStackRes, SfUiPlugin, UiDefaultFocus, UiFocusable, UiRegistryRes, UiScreenRoot,
};
use sf_ui_css::{UiClass, UiCssPlugin, UiCssReport, UiCssSheets, UiCssSourcePlugin};

const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "hud"
value = { layer = "Hud", implementation = { Template = "hud" } }

[[define]]
kind = "ui.screen"
id = "menu"
value = { layer = "Menu", implementation = { Template = "menu" } }
"#;

const MENU: [&str; 6] = ["Inventory", "Magic", "Map", "Journal", "Options", "Quit"];
const BUTTONS_CSS: &str = "ui/bright_buttons/style.css";
const GREEN: &str = "2e7d32";
const RED: &str = "c62828";

fn main() -> AppExit {
    let args: Vec<String> = std::env::args().collect();
    let autotest = args
        .iter()
        .position(|a| a == "--autotest")
        .map(|i| args.get(i + 1).map(PathBuf::from));
    let assets = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/assets");
    let (content, temp) = if autotest.is_some() {
        let temp = tempfile::tempdir().expect("temp dir");
        copy_dir(&assets, temp.path()).expect("copy assets");
        (temp.path().to_path_buf(), Some(temp))
    } else {
        (assets, None)
    };

    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let source = db.add_source("example", &[]);
    apply_record_file(&mut db, source, SCREENS, RecordFormat::Toml).unwrap();
    let (registry, _) = UiRegistry::build(&db);
    let mut stack = ScreenStackRes::default();
    stack.0.push(&registry, "hud").unwrap();
    stack.0.push(&registry, "menu").unwrap();

    let mut app = App::new();
    app.add_plugins(UiCssSourcePlugin::new(
        AssetSourceBuilder::platform_default(content.to_str().expect("utf-8 path"), None),
    ))
    .add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "sf_ui_css: ui_css".into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            }),
        EnhancedInputPlugin,
        SfUiPlugin,
        UiCssPlugin,
    ))
    .insert_resource(UiRegistryRes(registry))
    .insert_resource(stack)
    // In a game: `sf_ui_css::mods_from_profile(&loaded)`, i.e. each mod's `[ui] styles` in load order.
    .insert_resource(UiCssSheets {
        mods: [
            ("base_ui", "ui/base_ui/style.css"),
            ("bright_buttons", BUTTONS_CSS),
            ("broken_mod", "ui/broken_mod/style.css"),
        ]
        .map(|(id, path)| (id.to_owned(), path.to_owned()))
        .into(),
        ..default()
    })
    .add_systems(Startup, spawn)
    .add_systems(Update, hud_text);
    if let Some(shots) = autotest {
        app.insert_resource(Autotest {
            content,
            shots,
            _temp: temp,
            ..default()
        })
        .add_systems(Update, run_autotest);
    }
    app.run()
}

fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

#[derive(Component)]
struct HudText;

/// Layout and colours come from the sheets; the code only builds the tree and names classes.
fn spawn(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        UiScreenRoot("hud".into()),
        UiClass::new("hud"),
        Node::default(),
        children![(HudText, Text::new(""))],
    ));
    commands
        .spawn((
            UiScreenRoot("menu".into()),
            UiClass::new("menu"),
            Node::default(),
        ))
        .with_children(|menu| {
            menu.spawn((UiClass::new("panel"), Node::default()))
                .with_children(|panel| {
                    for (i, label) in MENU.iter().enumerate() {
                        let mut button = panel.spawn((
                            Name::new(*label),
                            Button,
                            UiFocusable,
                            UiClass::new("menu-button"),
                            Node::default(),
                            children![Text::new(*label)],
                        ));
                        if i == 0 {
                            button.insert(UiDefaultFocus);
                        }
                    }
                });
        });
}

fn hud_text(
    mode: Res<InputMode>,
    report: Res<UiCssReport>,
    mut text: Query<&mut Text, With<HudText>>,
) {
    if !mode.is_changed() && !report.is_changed() {
        return;
    }
    let issues: Vec<String> = report
        .issues
        .iter()
        .map(|i| format!("{} ({})", i.mod_id.as_deref().unwrap_or("-"), i.path))
        .collect();
    for mut text in &mut text {
        text.0 = format!(
            "input={:?} | CSS issues: {} | edit {BUTTONS_CSS} and save to restyle",
            mode.0,
            if issues.is_empty() {
                "none".into()
            } else {
                issues.join(", ")
            }
        );
    }
}

// ---- autotest: hot reload on a temporary copy ----

#[derive(Resource, Default)]
struct Autotest {
    content: PathBuf,
    shots: Option<PathBuf>,
    _temp: Option<tempfile::TempDir>,
    phase: u32,
    frames: u32,
    since: Option<Instant>,
}

fn is_color(color: Color, hex: &str) -> bool {
    let (a, b) = (color.to_srgba(), Srgba::hex(hex).unwrap());
    (a.red - b.red).abs() < 0.01
        && (a.green - b.green).abs() < 0.01
        && (a.blue - b.blue).abs() < 0.01
}

fn run_autotest(
    mut commands: Commands,
    mut test: ResMut<Autotest>,
    report: Res<UiCssReport>,
    buttons: Query<(&Name, &BackgroundColor)>,
    mut exit: MessageWriter<AppExit>,
) {
    test.frames += 1;
    let color = buttons
        .iter()
        .find(|(n, _)| n.as_str() == "Magic")
        .map(|(_, c)| c.0);
    let mut fail = |what: &str| {
        error!(
            "autotest: FAIL: {what} (button colour {color:?}, report {:?})",
            report.issues
        );
        exit.write(AppExit::error());
    };
    let shot = |commands: &mut Commands, test: &Autotest, name: &str| {
        if let Some(dir) = &test.shots {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(dir.join(name)));
        }
    };
    match test.phase {
        // Styled from the sheets (later mod's colour) and the broken mod reported.
        0 => {
            let broken = report.for_mod("broken_mod").count() == 1 && report.issues.len() == 1;
            if test.frames > 30 && broken && color.is_some_and(|c| is_color(c, GREEN)) {
                shot(&mut commands, &test, "ui_css_before.png");
                test.phase = 1;
                test.frames = 0;
            } else if test.frames > 1200 {
                fail("sheets not applied or error not attributed to broken_mod");
            }
        }
        // Edit the mod's sheet on disk, as a modder would.
        1 if test.frames > 10 => {
            let path = test.content.join(BUTTONS_CSS);
            let css = std::fs::read_to_string(&path).unwrap().replace(GREEN, RED);
            std::fs::write(&path, css).unwrap();
            info!("autotest: rewrote {}", path.display());
            test.since = Some(Instant::now());
            test.phase = 2;
        }
        2 => {
            if color.is_some_and(|c| is_color(c, RED)) {
                info!(
                    "autotest: hot reload applied after {:?}",
                    test.since.unwrap().elapsed()
                );
                shot(&mut commands, &test, "ui_css_after.png");
                test.phase = 3;
                test.frames = 0;
            } else if test.since.unwrap().elapsed() > Duration::from_secs(20) {
                fail("edited sheet not applied within 20 s");
            }
        }
        3 if test.frames > 10 => {
            info!("autotest: PASS");
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
