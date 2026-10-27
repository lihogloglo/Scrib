//! The game's HUD (ROADMAP 2.2): the framework HUD (`sf_ui_screens` [`HudScreen`], ui.md "Stock
//! screens") on the "hud" screen record, driven from the player.
//!
//! - **Root**: one `UiScreenRoot("hud")` + [`HudScreen`], spawned at startup. The record and the
//!   stack entry that keeps it open are `mw_exterior_bevy::pause_menu`'s (the HUD screen is the
//!   input target of play). The root is shown only in player mode with a body, and hidden while
//!   a Menu or Modal screen (pause, options, dialogue, journal, inventory, keybinds, save/load) or
//!   the console is open (ui.md layers: Hud < Menu < Modal).
//! - **Bars**: health, magicka, fatigue from [`PlayerVitals`] (health and the fatigue bar follow the
//!   player's combat health and stamina, `sf_combat_bevy::Combatant`), in Morrowind's order and place
//!   (bottom left, stacked; `OpenMW` `files/data/mygui/openmw_hud.layout` keeps the vanilla
//!   layout). Colours are the Morrowind.ini `[FontColor]` `color_health/magic/fatigue` tokens,
//!   in the theme sheet. Labels are localized (`hud-bar-<id>`), hidden by the Morrowind theme as
//!   vanilla draws none.
//! - **Compass**: the camera's heading (`heading_label`) in the `hud.compass` slot, bottom right
//!   where vanilla draws its minimap. Scaffold until the local map lands.
//! - **Crosshair**: shown whenever the HUD is, first and third person alike: the focus ray goes
//!   through the screen centre in both views (`OpenMW` `World::getFocusObject`, see
//!   [`crate::Focus`]), so the centre dot is where Use acts. Hidden while the lock-on marker is on
//!   screen (`sf_combat_bevy::LockMarkerShown`): the marker is the same dot moved onto the target
//!   (`crate::combat`).
//! - **Use prompt**: from [`crate::Focus`] (the game's `OpenMW` focus ray; the action layer's own
//!   focus, `sf_action::Prompt`, isn't fed by the game yet). The glyph is the Use binding's,
//!   following rebinds and the device; a target with no verb (a creature, a fixed light) shows
//!   its name only, no glyph, as Morrowind's tooltip.

use std::sync::Arc;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_camera::visibility::Visibility;
use bevy_transform::prelude::*;
use sf_camera_bevy::PlayerCamera;
use sf_console_bevy::ConsoleOpen;
use sf_records::{RecordDb, RecordId};
use sf_ui::Layer;
use sf_ui_bevy::{ScreenStackRes, UiLocalizer, UiScreenRoot};
use sf_ui_screens::{HudBar, HudScreen, HudView, ScreenSystems, UsePrompt};

use sf_combat_bevy::{Combatant, LockMarkerShown};

use crate::{Focus, PlayerBody, PlayerMode};

/// The HUD's screen id (its `ui.screen` record is `mw_exterior_bevy::pause_menu`'s).
pub const HUD_SCREEN: &str = "hud";
/// The player's NPC record (Morrowind.esm).
pub(crate) const PLAYER_RECORD: &str = "player";
/// Bar ids (`.sf-hud-bar.<id>` in the theme sheet, `hud-bar-<id>` in the Fluent file), in
/// Morrowind's order, top to bottom.
pub const BAR_IDS: [&str; 3] = ["health", "magicka", "fatigue"];

/// One dynamic stat.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vital {
    pub current: f32,
    pub max: f32,
}

impl Vital {
    fn full(max: f32) -> Self {
        Self { current: max, max }
    }
}

/// Where [`PlayerVitals`] came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VitalsSource {
    /// The `player` NPC record's NPDT health, magicka and fatigue.
    Record,
    /// No record (or an auto-calculated one): [`PlayerVitals::PLACEHOLDER`].
    Placeholder,
}

/// Scaffold `PlayerVitals`: the player's health, magicka and fatigue, static, for the HUD bars.
/// Seeded full from the `player` record; not saved, nothing changes them. TODO(combat 3.5 / rpg
/// stats): the stat system owns these (damage, regeneration, and `OpenMW`'s derivation from
/// attributes at character generation), and this resource goes.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct PlayerVitals {
    pub health: Vital,
    pub magicka: Vital,
    pub fatigue: Vital,
    pub source: VitalsSource,
}

impl PlayerVitals {
    /// Maxima when no record has them (health, magicka, fatigue).
    pub const PLACEHOLDER: [f32; 3] = [100.0, 100.0, 100.0];

    /// From the `player` NPC record (Morrowind.esm's: 35 / 50 / 200), else the placeholder.
    pub fn from_record(db: Option<&RecordDb>) -> Self {
        let stats = db
            .and_then(|db| db.get::<tes3::esp::Npc>(&RecordId::new(PLAYER_RECORD)))
            .and_then(|npc| npc.data.stats.as_ref())
            .filter(|s| s.health > 0);
        match stats {
            Some(s) => Self::new([s.health, s.magicka, s.fatigue].map(f32::from), VitalsSource::Record),
            None => Self::new(Self::PLACEHOLDER, VitalsSource::Placeholder),
        }
    }

    fn new([health, magicka, fatigue]: [f32; 3], source: VitalsSource) -> Self {
        Self { health: Vital::full(health), magicka: Vital::full(magicka), fatigue: Vital::full(fatigue), source }
    }

    /// The bars' values, in [`BAR_IDS`] order.
    pub fn bars(&self) -> [Vital; 3] {
        [self.health, self.magicka, self.fatigue]
    }
}

/// The HUD root, its view-model and the systems that drive them. Called from [`crate::add`];
/// the screens themselves come from `SfScreensPlugin` (`dialogue::add`).
pub fn add(app: &mut App, db: Option<&Arc<RecordDb>>) {
    let vitals = PlayerVitals::from_record(db.map(Arc::as_ref));
    bevy_log::info!("hud: vitals {:?} ({:?})", vitals.bars().map(|v| v.max), vitals.source);
    app.insert_resource(vitals)
        .init_resource::<HudView>()
        .add_systems(Startup, spawn_root)
        .add_systems(Update, sync_vitals.before(drive_hud))
        .add_systems(Update, (drive_hud.after(crate::FocusUpdate), show_hud).before(ScreenSystems::Sync));
}

fn spawn_root(mut commands: Commands) {
    commands.spawn((UiScreenRoot(HUD_SCREEN.into()), HudScreen, Visibility::Hidden));
}

/// Degrees clockwise from north of a forward direction (engine: north is -Z, east +X).
pub fn heading_degrees(forward: Vec3) -> Option<f32> {
    let flat = Vec3::new(forward.x, 0.0, forward.z);
    (flat.length_squared() > 1e-6).then(|| forward.x.atan2(-forward.z).to_degrees().rem_euclid(360.0))
}

/// A localized string, or `fallback` when the Fluent file lacks `key`.
fn localized(localizer: Option<&UiLocalizer>, key: &str, fallback: &str) -> String {
    match localizer {
        Some(l) if l.0.has(key) => l.0.format(key, None).into_owned(),
        _ => fallback.to_owned(),
    }
}

/// The player's combat vitals into [`PlayerVitals`]: health, and stamina on the fatigue bar
/// (combat.md "C1b"), whole numbers (the HUD resyncs only on a change). Magicka stays static.
pub(crate) fn sync_vitals(bodies: Query<&Combatant, With<PlayerBody>>, mut vitals: ResMut<PlayerVitals>) {
    let Some(c) = bodies.iter().next() else { return };
    let v = &c.fighter.vitals;
    let whole = |cur: f32, max: f32| Vital { current: cur.round().clamp(0.0, max), max };
    let mut want = *vitals;
    want.health = whole(v.health.cur, v.health.max);
    want.fatigue = whole(v.stamina.cur, v.stamina.max);
    vitals.set_if_neq(want);
}

/// Player state → [`HudView`] (written only on a change, so the HUD resyncs only then).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn drive_hud(
    mode: Res<PlayerMode>,
    focus: Res<Focus>,
    vitals: Res<PlayerVitals>,
    localizer: Option<Res<UiLocalizer>>,
    lock_shown: Option<Res<LockMarkerShown>>,
    cams: Query<&GlobalTransform, With<PlayerCamera>>,
    mut view: ResMut<HudView>,
) {
    let localizer = localizer.as_deref();
    let bars = BAR_IDS
        .iter()
        .zip(vitals.bars())
        .map(|(id, v)| HudBar {
            id: (*id).to_owned(),
            label: localized(localizer, &format!("hud-bar-{id}"), id),
            current: v.current,
            max: v.max,
        })
        .collect();
    // Whole degrees: the label shows no more, and the HUD resyncs only when it changes.
    let heading = cams.iter().next().and_then(|t| heading_degrees(*t.forward())).map(|d| d.round().rem_euclid(360.0));
    let prompt = focus.target.as_ref().map(|t| match t.verb {
        Some(verb) => UsePrompt {
            verb: localized(localizer, &format!("hud-use-{}", verb.to_lowercase()), verb),
            target: Some(t.name.clone()),
            ready: true,
            action: Some(mw_controls_bevy::Control::Use.id()),
        },
        None => UsePrompt { verb: t.name.clone(), target: None, ready: true, action: None },
    });
    // While the lock marker is on screen it is the cursor: one reticle, not two.
    let locked = lock_shown.is_some_and(|s| s.0);
    let want = HudView { bars, heading, crosshair: mode.body.is_some() && !locked, prompt };
    view.set_if_neq(want);
}

/// The HUD shows in player mode with a body, while no menu, modal or console is open.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn show_hud(
    mode: Res<PlayerMode>,
    stack: Option<Res<ScreenStackRes>>,
    console: Option<Res<ConsoleOpen>>,
    mut roots: Query<(&UiScreenRoot, &mut Visibility), With<HudScreen>>,
) {
    let menu = stack.is_some_and(|s| s.0.top(Layer::Menu).is_some() || s.0.top(Layer::Modal).is_some());
    let console = console.is_some_and(|c| c.0);
    let shown = mode.on && mode.body.is_some() && !menu && !console;
    let want = if shown { Visibility::Inherited } else { Visibility::Hidden };
    for (root, mut visibility) in &mut roots {
        if root.0.as_str() == HUD_SCREEN {
            visibility.set_if_neq(want);
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // exact values copied, not computed
mod tests {
    use bevy::camera::{ComputedCameraValues, RenderTargetInfo, Viewport};
    use bevy::input::InputPlugin;
    use bevy::picking::{InteractionPlugin, PickingPlugin};
    use bevy::prelude::*;
    use bevy::text::TextPlugin;
    use bevy::ui::UiPlugin;
    use bevy_enhanced_input::prelude::EnhancedInputPlugin;
    use sf_ui::UiRegistry;
    use sf_ui_bevy::{SfUiPlugin, UiRegistryRes, UiTextPlugin};
    use sf_ui_screens::SfScreensPlugin;
    use sf_ui_screens::hud::HudParts;

    use super::*;
    use crate::FocusTarget;

    /// The HUD record and one menu, as the game's records declare them.
    const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "hud"
value = { layer = "Hud", implementation = { Template = "hud" } }

[[define]]
kind = "ui.screen"
id = "pause"
value = { layer = "Menu", implementation = { Template = "pause" } }
"#;

    fn registry() -> UiRegistry {
        let mut db = RecordDb::new();
        sf_ui::register_kinds(&mut db).expect("UI record kinds register");
        let base = db.add_source("bevyrrowind", &[]);
        let outcome = sf_records::apply_record_file(&mut db, base, SCREENS, sf_records::RecordFormat::Toml).expect("screens");
        assert!(outcome.errors.is_empty(), "{:?}", outcome.errors);
        UiRegistry::build(&db).0
    }

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            AssetPlugin::default(),
            ImagePlugin::default(),
            bevy::image::TextureAtlasPlugin,
            TextPlugin,
            UiPlugin,
            InputPlugin,
            PickingPlugin,
            InteractionPlugin,
            EnhancedInputPlugin,
            SfUiPlugin,
            UiTextPlugin,
            SfScreensPlugin,
        ))
        .insert_resource(UiRegistryRes(registry()))
        .insert_resource(PlayerMode { on: true, paused: false, body: None, last_ground: None })
        .init_resource::<Focus>();
        mw_controls_bevy::add(&mut app);
        add(&mut app, None);
        app.add_plugins(sf_combat_bevy::LockMarkerPlugin).add_systems(Update, crate::combat::tag_crosshair);
        let target = UVec2::new(1600, 1000);
        app.world_mut().spawn((
            Camera2d,
            Camera {
                computed: ComputedCameraValues {
                    target_info: Some(RenderTargetInfo { physical_size: target, scale_factor: 1.0 }),
                    ..default()
                },
                viewport: Some(Viewport { physical_size: target, ..default() }),
                ..default()
            },
        ));
        // The player: a body, and its camera facing east.
        let body = app.world_mut().spawn_empty().id();
        app.world_mut().resource_mut::<PlayerMode>().body = Some(body);
        app.world_mut().spawn((PlayerCamera::new(body), Transform::default().looking_to(Vec3::X, Vec3::Y)));
        let registry = app.world().resource::<UiRegistryRes>().0.clone();
        app.world_mut().resource_mut::<ScreenStackRes>().0.push(&registry, HUD_SCREEN).expect("hud opens");
        app.finish();
        frames(&mut app, 3);
        app
    }

    fn frames(app: &mut App, n: usize) {
        for _ in 0..n {
            app.update();
        }
    }

    fn hud_root(app: &mut App) -> (Entity, Visibility) {
        let mut q = app.world_mut().query_filtered::<(Entity, &Visibility), (With<HudScreen>, With<HudParts>)>();
        let (e, v) = q.single(app.world()).expect("one built HUD root");
        (e, *v)
    }

    fn text(app: &App, e: Entity) -> String {
        app.world().get::<Text>(e).map(|t| t.0.clone()).unwrap_or_default()
    }

    #[test]
    fn player_mode_shows_bars_heading_crosshair_and_the_use_prompt_and_a_menu_hides_it() {
        let mut app = app();
        let (root, visibility) = hud_root(&mut app);
        assert_eq!(visibility, Visibility::Inherited, "shown in player mode");

        let view = app.world().resource::<HudView>().clone();
        let ids: Vec<&str> = view.bars.iter().map(|b| b.id.as_str()).collect();
        assert_eq!(ids, BAR_IDS);
        let labels: Vec<&str> = view.bars.iter().map(|b| b.label.as_str()).collect();
        assert_eq!(labels, ["Health", "Magicka", "Fatigue"], "localized");
        assert!(view.bars.iter().all(|b| b.max > 0.0 && b.current == b.max), "{:?}", view.bars);
        assert_eq!(view.heading, Some(90.0), "camera faces east");
        assert!(view.crosshair && view.prompt.is_none());
        let parts = app.world().get::<HudParts>(root).expect("parts");
        assert!(BAR_IDS.iter().all(|id| parts.bar(id).is_some()), "a bar entity per id");

        // A door in focus: verb, name and the Use binding's glyph (E by default).
        app.world_mut().resource_mut::<Focus>().target = Some(FocusTarget {
            handle: sf_script::Handle(7),
            record: "door".into(),
            name: "Census and Excise Office".into(),
            verb: Some("Open"),
            distance: 1.0,
        });
        frames(&mut app, 2);
        let parts = app.world().get::<HudParts>(root).expect("parts");
        let (prompt, glyph) = (parts.prompt(), parts.prompt_glyph());
        let shown = |app: &App, e: Entity| app.world().get::<Node>(e).is_some_and(|n| n.display != Display::None);
        assert!(shown(&app, prompt), "prompt shown");
        assert_eq!(text(&app, glyph), "E");
        let words: Vec<String> = app.world().get::<Children>(prompt).expect("prompt children").iter().map(|c| text(&app, c)).collect();
        assert!(words.contains(&"Open Census and Excise Office".to_owned()), "{words:?}");

        // A creature: its name only, no glyph.
        app.world_mut().resource_mut::<Focus>().target.as_mut().expect("focus").verb = None;
        frames(&mut app, 2);
        assert!(shown(&app, prompt) && !shown(&app, glyph), "name-only prompt has no glyph");

        // A menu opens: the HUD hides; it closes: the HUD is back.
        let registry = app.world().resource::<UiRegistryRes>().0.clone();
        app.world_mut().resource_mut::<ScreenStackRes>().0.push(&registry, "pause").expect("menu opens");
        frames(&mut app, 2);
        assert_eq!(hud_root(&mut app).1, Visibility::Hidden, "hidden under a menu");
        app.world_mut().resource_mut::<ScreenStackRes>().0.close("pause");
        frames(&mut app, 2);
        assert_eq!(hud_root(&mut app).1, Visibility::Inherited);

        // The console hides it too; the free camera (no player mode) as well.
        app.world_mut().insert_resource(ConsoleOpen(true));
        frames(&mut app, 1);
        assert_eq!(hud_root(&mut app).1, Visibility::Hidden, "hidden under the console");
        app.world_mut().insert_resource(ConsoleOpen(false));
        app.world_mut().resource_mut::<PlayerMode>().on = false;
        frames(&mut app, 1);
        assert_eq!(hud_root(&mut app).1, Visibility::Hidden, "hidden on the free camera");
    }

    /// One reticle: while the lock marker is on the target the centre crosshair hides, and the
    /// marker wears the crosshair's (themed) look; released, the crosshair is back.
    #[test]
    fn the_lock_marker_replaces_the_crosshair_and_wears_its_look() {
        use bevy::camera::{CameraProjection, PerspectiveProjection};
        use sf_camera_bevy::{LockOn, Lockable};
        use sf_combat_bevy::{LockMarker, LockMarkerShown, LockMarkerStyle};

        let mut app = app();
        let (root, _) = hud_root(&mut app);
        let crosshair = app.world().get::<HudParts>(root).expect("parts").crosshair();
        let shown = |app: &App, e: Entity| app.world().get::<Node>(e).is_some_and(|n| n.display != Display::None);
        let marker = |app: &mut App| {
            let mut q = app.world_mut().query_filtered::<&Visibility, With<LockMarker>>();
            *q.single(app.world()).expect("one marker")
        };
        assert!(shown(&app, crosshair), "no lock: the crosshair");
        assert_eq!(marker(&mut app), Visibility::Hidden);

        // The marker copied the crosshair's look (4 px square, no rim; colour as themed).
        let style = *app.world().resource::<LockMarkerStyle>();
        let fill = app.world().get::<BackgroundColor>(crosshair).expect("crosshair fill").0;
        let node = app.world().get::<Node>(crosshair).expect("crosshair node").clone();
        assert_eq!((Val::Px(style.size), style.fill, style.radius, style.rim), (node.width, fill, node.border_radius, 0.0));
        let mut q = app.world_mut().query_filtered::<(&Node, &BackgroundColor), With<LockMarker>>();
        let (marker_node, marker_fill) = q.single(app.world()).expect("one marker");
        assert_eq!((marker_node.width, marker_fill.0), (node.width, fill), "marker styled");

        // The player camera renders, and the body locks a target 5 m ahead (east, eye level).
        let size = UVec2::new(1600, 1000);
        let clip = PerspectiveProjection { aspect_ratio: 1.6, ..default() }.get_clip_from_view();
        let camera = Camera {
            order: -1,
            computed: ComputedCameraValues {
                clip_from_view: clip,
                target_info: Some(RenderTargetInfo { physical_size: size, scale_factor: 1.0 }),
                ..default()
            },
            ..default()
        };
        let mut cams = app.world_mut().query_filtered::<Entity, With<PlayerCamera>>();
        let cam = cams.single(app.world()).expect("player camera");
        app.world_mut().entity_mut(cam).insert(camera);
        let target = app.world_mut().spawn((Transform::from_xyz(5.0, -1.0, 0.0), Lockable { aim_height: 1.0 })).id();
        let body = app.world().resource::<PlayerMode>().body.expect("body");
        app.world_mut().entity_mut(body).insert(LockOn(target));
        frames(&mut app, 3);
        assert!(app.world().resource::<LockMarkerShown>().0);
        assert_eq!(marker(&mut app), Visibility::Inherited, "locked: the marker");
        assert!(!shown(&app, crosshair), "locked: no centre crosshair");
        assert!(!app.world().resource::<HudView>().crosshair);

        app.world_mut().entity_mut(body).remove::<LockOn>();
        frames(&mut app, 3);
        assert_eq!(marker(&mut app), Visibility::Hidden, "released: no marker");
        assert!(shown(&app, crosshair), "released: the crosshair is back");
    }

    #[test]
    fn heading_is_clockwise_from_north() {
        let deg = |v: Vec3| heading_degrees(v).map(f32::round);
        assert_eq!(deg(Vec3::NEG_Z), Some(0.0));
        assert_eq!(deg(Vec3::X), Some(90.0));
        assert_eq!(deg(Vec3::Z), Some(180.0));
        assert_eq!(deg(Vec3::NEG_X), Some(270.0));
        assert_eq!(heading_degrees(Vec3::Y), None, "straight up has no heading");
    }

    #[test]
    fn vitals_fall_back_to_the_placeholder_without_a_record() {
        let v = PlayerVitals::from_record(Some(&RecordDb::new()));
        assert_eq!(v.source, VitalsSource::Placeholder);
        assert_eq!(v.bars().map(|b| b.max), PlayerVitals::PLACEHOLDER);
    }
}
