//! Headless: the composed sheet really cascades in load order through flair, CSS errors land on
//! the mod that ships them, and root attributes follow the input device.

use std::path::Path;

use bevy::asset::io::AssetSourceBuilder;
use bevy::asset::io::memory::{Dir, MemoryAssetReader};
use bevy::image::{ImagePlugin, TextureAtlasPlugin};
use bevy::input::InputPlugin;
use bevy::input_focus::{InputFocus, InputFocusVisible};
use bevy::picking::{InteractionPlugin, PickingPlugin, PickingSettings};
use bevy::prelude::*;
use bevy::render::texture::ManualTextureViews;
use bevy::text::TextPlugin;
use bevy::ui::UiPlugin;
use sf_ui_bevy::{Device, InputMode, UiScreenRoot};

use crate::{LayerKind, UiCssPlugin, UiCssReport, UiCssSheets, UiCssSourcePlugin};

fn app(content: &Dir) -> App {
    let dir = content.clone();
    let mut app = App::new();
    app.add_plugins(UiCssSourcePlugin::new(AssetSourceBuilder::new(move || {
        Box::new(MemoryAssetReader { root: dir.clone() })
    })))
    .insert_resource(PickingSettings {
        is_enabled: false,
        ..default()
    })
    .add_plugins((
        TaskPoolPlugin {
            task_pool_options: TaskPoolOptions::with_num_threads(1),
        },
        bevy::time::TimePlugin,
        AssetPlugin::default(),
        WindowPlugin::default(),
        ImagePlugin::default(),
        TextureAtlasPlugin,
        TextPlugin,
        (InputPlugin, PickingPlugin, InteractionPlugin, UiPlugin),
        UiCssPlugin,
    ))
    .init_resource::<ManualTextureViews>()
    .init_resource::<InputFocus>()
    .init_resource::<InputFocusVisible>();
    app.finish();
    app
}

fn spawn_screen(app: &mut App) -> Entity {
    app.world_mut()
        .spawn((
            UiScreenRoot("menu".into()),
            Node::default(),
            children![(Name::new("target"), Node::default())],
        ))
        .id()
}

fn target_left(app: &mut App) -> Val {
    let mut q = app.world_mut().query::<(&Name, &Node)>();
    q.iter(app.world())
        .find(|(n, _)| n.as_str() == "target")
        .map_or(Val::Auto, |(_, node)| node.left)
}

/// Updates until `done` or a generous frame budget (loads run on the IO pool).
fn run_until(app: &mut App, mut done: impl FnMut(&mut App) -> bool) -> bool {
    for _ in 0..400 {
        app.update();
        if done(app) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    false
}

fn mods(list: &[(&str, &str)]) -> UiCssSheets {
    UiCssSheets {
        mods: list
            .iter()
            .map(|(id, path)| ((*id).to_owned(), (*path).to_owned()))
            .collect(),
        ..default()
    }
}

#[test]
fn later_mod_wins_and_reordering_restyles() {
    let content = Dir::new(Path::new("").to_path_buf());
    content.insert_asset_text(Path::new("ui/a/style.css"), "#target { left: 10px; }");
    // Higher specificity in an earlier layer still loses to a later layer (Cascade 5).
    content.insert_asset_text(Path::new("ui/b/style.css"), "* > #target { left: 20px; }");
    let mut app = app(&content);
    app.insert_resource(mods(&[("b", "ui/b/style.css"), ("a", "ui/a/style.css")]));
    spawn_screen(&mut app);
    assert!(
        run_until(&mut app, |app| target_left(app) == Val::Px(10.0)),
        "got {:?}",
        target_left(&mut app)
    );

    // New load order: `b` last, so its rule wins after the live recomposition.
    app.insert_resource(mods(&[("a", "ui/a/style.css"), ("b", "ui/b/style.css")]));
    assert!(
        run_until(&mut app, |app| target_left(app) == Val::Px(20.0)),
        "got {:?}",
        target_left(&mut app)
    );
}

#[test]
fn css_errors_are_attributed_to_the_mod() {
    let content = Dir::new(Path::new("").to_path_buf());
    content.insert_asset_text(Path::new("ui/good/style.css"), "#target { left: 10px; }");
    // One broken rule; its valid rule still applies (the root imports leniently).
    content.insert_asset_text(
        Path::new("ui/broken/style.css"),
        "#target { top: 5px; }\n#target { left: banana; }",
    );
    let mut app = app(&content);
    app.insert_resource(mods(&[
        ("good", "ui/good/style.css"),
        ("broken", "ui/broken/style.css"),
    ]));
    spawn_screen(&mut app);
    let reported = run_until(&mut app, |app| {
        !app.world().resource::<UiCssReport>().issues.is_empty()
    });
    let report = app.world().resource::<UiCssReport>().clone();
    assert!(reported, "no issue reported");
    assert_eq!(report.issues.len(), 1, "{report:?}");
    let issue = &report.issues[0];
    assert_eq!(
        (issue.kind, issue.mod_id.as_deref(), issue.path.as_str()),
        (LayerKind::Mod, Some("broken"), "ui/broken/style.css")
    );
    assert!(issue.message.contains("banana"), "{}", issue.message);
    assert_eq!(report.for_mod("good").count(), 0);
    assert!(run_until(&mut app, |app| {
        let mut q = app.world_mut().query::<(&Name, &Node)>();
        q.iter(app.world()).any(|(n, node)| {
            n.as_str() == "target" && node.top == Val::Px(5.0) && node.left == Val::Px(10.0)
        })
    }));
}

#[test]
fn input_attribute_follows_the_device() {
    let content = Dir::new(Path::new("").to_path_buf());
    content.insert_asset_text(
        Path::new("ui/a/style.css"),
        "#target { left: 1px; }\n[input=gamepad] #target { left: 2px; }\n.preset-pc #target { top: 3px; }",
    );
    let mut app = app(&content);
    let mut db = sf_records::RecordDb::new();
    sf_ui::register_kinds(&mut db).unwrap();
    app.insert_resource(sf_ui_bevy::UiRegistryRes(sf_ui::UiRegistry::build(&db).0));
    app.insert_resource(mods(&[("a", "ui/a/style.css")]));
    spawn_screen(&mut app);
    assert!(
        run_until(&mut app, |app| target_left(app) == Val::Px(1.0)),
        "got {:?}",
        target_left(&mut app)
    );
    app.insert_resource(InputMode(Device::Gamepad));
    assert!(
        run_until(&mut app, |app| target_left(app) == Val::Px(2.0)),
        "got {:?}",
        target_left(&mut app)
    );
    let mut q = app.world_mut().query::<(&Name, &Node)>();
    let top = q
        .iter(app.world())
        .find(|(n, _)| n.as_str() == "target")
        .map(|(_, node)| node.top);
    assert_eq!(
        top,
        Some(Val::Px(3.0)),
        "root class from the built-in pc_dense preset"
    );
}
