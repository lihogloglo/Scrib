//! The main (title) menu of a release start (ROADMAP 2.2, visual check MM-1): the stock
//! `sf_ui_screens` main menu in the front end, with **no world** behind it (menus.md "App
//! phases", `phase.rs`).
//!
//! - **Open**: on entering `AppPhase::Frontend` when `ExteriorOptions::start_in_main_menu` is set
//!   (running `bevyrrowind` with no subcommand). Dev runs (`view`, `scene`) never open it.
//! - **Leaving**: New game closes the menu and enters `Loading` (the world builds at the new-game
//!   start). Interim until F3 (Continue / Load from the front end): Continue and Load also start
//!   at the new-game start, and once `InGame` Continue's load runs as an in-game load and Load
//!   opens the save/load screen over the game ([`PendingFrontend`]). Should the menu close any
//!   other way in the front end, the world loads too (never a front end without a menu).
//! - **Pause**: the "`main_menu`" record declares `pause = "WhileOpen"`, so the virtual clock is
//!   paused while it is open (`pause_menu.rs` `pause_time`, the pause menu's mechanism) and the
//!   player's controls pause through the Menu-layer seam; the HUD hides under Menu screens.
//! - **Entries**, in Morrowind's order (`OpenMW` `apps/openmw/mwgui/mainmenu.cpp`, commit
//!   675146bd: return/new game/save/load/options/credits/exit): Continue, New game, Load,
//!   Options, Quit. Continue and Load are enabled while a single-player save of any world exists
//!   (checked at open and again when the save/load screen closes, a delete may have emptied it).
//!   Continue loads the newest save of any reason (`mw_saves_bevy::newest_save`, quickload's
//!   search; `OpenMW`'s "return" does the same); Load opens the save/load screen in Load mode
//!   (`mw_saves_bevy::menu`; its Back pops it). Both in game for now (above). New game closes the
//!   menu and play begins; Options opens the options screen on top as the pause menu does
//!   ([`crate::pause_menu::open_options`]); Quit asks (the screen), then exits.
//!   Labels come from the builtin mod's Fluent file (`main-menu-<id>`).
//! - **Backdrop** (menus.md "Backdrop and title", ui-adapter.md "Menus and front end"): the
//!   first of [`MenuBackdropCandidates`] in the VFS, resolved by `sf_ui::resolve_backdrop`: a mod's
//!   `video/menu_background.bik`, vanilla's `[Movies] Options Menu=` video (`mw_menu.bik`), then
//!   `textures/menu_morrowind.dds`, else the plain theme background. A video plays looped and
//!   silent through `sf_video_bevy::VideoPlayer`; one that fails to read or decode falls through to
//!   the next candidate. Fit `cover` (`sf_ui::cover_size`): a clipping layer behind the menu's
//!   parts (`ZIndex(-1)`, a child of the menu's root) holds the media node, sized in percent
//!   of the layer and centred, so the 4:3 art fills a 16:9 window, cropped, never stretched. It
//!   despawns with the menu's root (and on leaving the front end), which stops the decode
//!   thread. No title image: `menu_morrowind` is the backdrop itself (`OpenMW` shows it as the
//!   menu background, `mainmenu.cpp` @ 97722d48), and the video carries the title art.

use bevy::prelude::*;
use mw_saves_bevy::{OpenSaveLoad, SAVELOAD};
use sf_app_bevy::AppPhase;
use sf_app_bevy::bevy_state::prelude::{DespawnOnExit, NextState, OnEnter, in_state};
use sf_records::RecordDb;
use sf_save_bevy::{LoadRequest, SaveConfig, SaveStorage};
use sf_ui::{BackdropCandidate, BackdropKind, UiRegistry, cover_size, resolve_backdrop};
use sf_ui_bevy::{ScreenStackRes, UiLocalizer, UiRegistryRes, UiScreenRoot, UiSettingsStore};
use sf_ui_screens::{MainMenuEntry, MainMenuEvent, MainMenuModel, MainMenuScreen, SaveLoadMode, ScreenSystems};
use sf_vfs::{Vfs, VfsPath};
use sf_video_bevy::{VideoFinished, VideoPlayer, VideoSystems};

/// The main menu's screen id.
pub const MAIN_MENU: &str = "main_menu";

/// Entry ids (also the Fluent keys' suffixes).
pub const CONTINUE: &str = "continue";
pub const NEW_GAME: &str = "new";
pub const LOAD: &str = "load";
pub const OPTIONS: &str = "options";
pub const QUIT: &str = "quit";

/// The game's record of the screen. TODO(ui data): with the other screens, in the builtin
/// mod's data.
const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "main_menu"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Template = "main_menu" } }
"#;

/// Interim (until F3): what the front end's Continue or Load does once the world, built at the
/// new-game start, is in game.
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub enum PendingFrontend {
    /// Continue: load this save (the in-game load path).
    Load(std::path::PathBuf),
    /// Load: open the save/load screen in Load mode over the game.
    OpenLoadScreen,
}

/// After `pause_menu::add` and the app phases (`phase.rs`). `open`: the release start, whose
/// front end shows the menu.
pub fn add(app: &mut App, open: bool) {
    if !open {
        return;
    }
    let Some(mut registry) = app.world_mut().get_resource_mut::<UiRegistryRes>() else { return };
    registry.0.extend_screens(&screens());
    // Continue's request and Load's (the save plugin and `pause_menu` register them too).
    app.add_message::<LoadRequest>().add_message::<OpenSaveLoad>();
    // The backdrop: candidates (the app may insert its own first), a failed video's fall-through
    // after the players run (`VideoPlugin`, added by the app), and the cover fit before UI layout.
    app.init_resource::<MenuBackdropCandidates>().add_message::<VideoFinished>();
    app.add_systems(Update, backdrop_fallthrough.after(VideoSystems))
        .add_systems(PostUpdate, fit_backdrop.before(bevy::ui::UiSystems::Prepare));
    app.add_systems(OnEnter(AppPhase::Frontend), open_main_menu)
        .add_systems(OnEnter(AppPhase::InGame), run_pending)
        .add_systems(
            Update,
            (
                (host_main_menu, refresh_saves).run_if(in_state(AppPhase::Frontend)),
                close_main_menu,
                leave_without_menu.run_if(in_state(AppPhase::Frontend)),
            )
                .chain()
                .after(ScreenSystems::Input)
                .before(crate::pause_menu::pause_time),
        );
}

/// The newest save of any world and reason (Continue's target); none without the save plugin.
fn newest(storage: Option<&SaveStorage>, config: Option<&SaveConfig>) -> Option<std::path::PathBuf> {
    mw_saves_bevy::newest_save(storage?, config?, None)
}

fn screens() -> UiRegistry {
    let mut db = RecordDb::new();
    sf_ui::register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    let outcome = sf_records::apply_record_file(&mut db, base, SCREENS, sf_records::RecordFormat::Toml).expect("main menu screen");
    assert!(outcome.errors.is_empty(), "main menu screen: {:?}", outcome.errors);
    UiRegistry::build(&db).0
}

/// The menu's model: the entries, labels localized (English fallbacks); Continue and Load
/// enabled when `saves` exist.
fn model(localizer: Option<&UiLocalizer>, saves: bool) -> MainMenuModel {
    let label = |id: &str, fallback: &str| {
        let key = format!("main-menu-{id}");
        match localizer {
            Some(l) if l.0.has(&key) => l.0.format(&key, None).into_owned(),
            _ => fallback.to_owned(),
        }
    };
    MainMenuModel::new([
        MainMenuEntry::new(CONTINUE, label(CONTINUE, "Continue")).enabled(saves),
        MainMenuEntry::new(NEW_GAME, label(NEW_GAME, "New Game")),
        MainMenuEntry::new(LOAD, label(LOAD, "Load Game")).enabled(saves),
        MainMenuEntry::new(OPTIONS, label(OPTIONS, "Options")),
        MainMenuEntry::new(QUIT, label(QUIT, "Quit")),
    ])
    .with_quit(QUIT)
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn open_main_menu(
    mut commands: Commands,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    localizer: Option<Res<UiLocalizer>>,
    storage: Option<Res<SaveStorage>>,
    config: Option<Res<SaveConfig>>,
    backdrop: (Res<MenuBackdropCandidates>, Option<Res<mw_assets::SharedVfs>>, Option<Res<AssetServer>>),
) {
    let saves = newest(storage.as_deref(), config.as_deref()).is_some();
    let root = commands.spawn((UiScreenRoot(MAIN_MENU.into()), MainMenuScreen::new(model(localizer.as_deref(), saves)))).id();
    if let Err(e) = stack.0.push(&registry.0, MAIN_MENU) {
        warn!("main menu: {e:?}");
    }
    let (candidates, vfs, assets) = backdrop;
    if let Some(vfs) = vfs {
        spawn_backdrop(&mut commands, root, &candidates.0, &vfs.0, assets.as_deref());
    }
}

/// Adapter hook (menus.md "Backdrop and title"): the menu backdrop's candidates in order, the
/// first whose file is in the VFS wins. `Default`: [`backdrop_candidates`] without an ini.
#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct MenuBackdropCandidates(pub Vec<BackdropCandidate>);

impl Default for MenuBackdropCandidates {
    fn default() -> Self {
        Self(backdrop_candidates(None))
    }
}

/// Vanilla's menu video when `Morrowind.ini` names none.
const MENU_VIDEO: &str = "mw_menu.bik";

/// ui-adapter.md "Menus and front end" order. `options_menu`: `Morrowind.ini`'s
/// `[Movies] Options Menu=` value (none or blank: vanilla's [`MENU_VIDEO`]).
pub fn backdrop_candidates(options_menu: Option<&str>) -> Vec<BackdropCandidate> {
    let menu = options_menu.map(str::trim).filter(|s| !s.is_empty()).unwrap_or(MENU_VIDEO);
    vec![
        BackdropCandidate::video("video/menu_background.bik"),
        BackdropCandidate::video(format!("video/{menu}")),
        BackdropCandidate::image("textures/menu_morrowind.dds"),
    ]
}

/// The backdrop's clipping layer behind the menu's parts; `next`: the candidate after the one
/// shown (where a failed video resumes), `candidates`: the list it resolved from.
#[derive(Component, Debug)]
struct MenuBackdrop {
    candidates: Vec<BackdropCandidate>,
    next: usize,
}

/// The video or image inside the backdrop layer, sized by [`fit_backdrop`].
#[derive(Component, Debug)]
struct BackdropMedia;

fn exists(vfs: &Vfs, path: &str) -> bool {
    VfsPath::new(path).is_ok_and(|p| vfs.contains(&p))
}

/// The layer and its media node under the menu's `root`, then the first candidate that loads.
fn spawn_backdrop(commands: &mut Commands, root: Entity, candidates: &[BackdropCandidate], vfs: &Vfs, assets: Option<&AssetServer>) {
    if resolve_backdrop(candidates, 0, |p| exists(vfs, p)).is_none() {
        return; // the plain theme background
    }
    let layer = commands
        .spawn((
            Name::new("main menu backdrop"),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                overflow: Overflow::clip(),
                ..default()
            },
            // Behind the framework's parts (siblings), whatever their spawn order.
            ZIndex(-1),
            DespawnOnExit(AppPhase::Frontend),
            ChildOf(root),
        ))
        .id();
    let media = commands
        .spawn((
            BackdropMedia,
            Node { position_type: PositionType::Absolute, ..default() },
            ImageNode::default(),
            // Until the media's size is known (no white placeholder flash).
            Visibility::Hidden,
            ChildOf(layer),
        ))
        .id();
    let next = show_candidate(commands, media, candidates, 0, vfs, assets);
    commands.entity(layer).insert(MenuBackdrop { candidates: candidates.to_vec(), next });
}

/// Puts the first candidate at or after `from` that loads on `media`: a looped silent
/// [`VideoPlayer`] or an [`ImageNode`]. Returns the index after it (`len`: none left; the layer
/// stays empty, the theme background shows).
fn show_candidate(commands: &mut Commands, media: Entity, candidates: &[BackdropCandidate], mut from: usize, vfs: &Vfs, assets: Option<&AssetServer>) -> usize {
    while let Some(i) = resolve_backdrop(candidates, from, |p| exists(vfs, p)) {
        let candidate = &candidates[i];
        from = i + 1;
        match candidate.kind {
            BackdropKind::Video => match VfsPath::new(&candidate.path).map_err(|e| e.to_string()).and_then(|p| vfs.read(&p).map_err(|e| e.to_string())) {
                Ok(bytes) => {
                    info!("main menu: backdrop video {}", candidate.path);
                    commands.entity(media).insert(VideoPlayer::new(bytes, true));
                    return from;
                }
                Err(e) => warn!("main menu: backdrop video {}: {e}; trying the next", candidate.path),
            },
            BackdropKind::Image => {
                let Some(assets) = assets else {
                    warn!("main menu: no asset server for backdrop image {}", candidate.path);
                    continue;
                };
                info!("main menu: backdrop image {}", candidate.path);
                commands.entity(media).insert(ImageNode::new(assets.load(candidate.path.clone())));
                return from;
            }
        }
    }
    candidates.len()
}

/// A backdrop video that failed to open or decode (a looped player finishes only on an error):
/// drop its player and show the next candidate.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn backdrop_fallthrough(
    mut commands: Commands,
    mut finished: MessageReader<VideoFinished>,
    media: Query<&ChildOf, With<BackdropMedia>>,
    mut layers: Query<&mut MenuBackdrop>,
    vfs: Option<Res<mw_assets::SharedVfs>>,
    assets: Option<Res<AssetServer>>,
) {
    for done in finished.read() {
        let Ok(parent) = media.get(done.entity) else { continue };
        let Ok(mut layer) = layers.get_mut(parent.parent()) else { continue };
        warn!("main menu: backdrop video {} failed; trying the next", layer.candidates.get(layer.next.wrapping_sub(1)).map_or("?", |c| c.path.as_str()));
        commands.entity(done.entity).remove::<VideoPlayer>().insert((ImageNode::default(), Visibility::Hidden));
        let Some(vfs) = vfs.as_deref() else { continue };
        let candidates = layer.candidates.clone();
        layer.next = show_candidate(&mut commands, done.entity, &candidates, layer.next, &vfs.0, assets.as_deref());
    }
}

/// The backdrop media node's parts the fit reads and writes.
type MediaParts = (&'static ChildOf, &'static mut Node, &'static mut Visibility, &'static ImageNode, Option<&'static VideoPlayer>);

/// The `cover` fit: the media node in percent of its layer, centred (the layer clips the rest).
/// Hidden until the media's size is known (a video once open, an image once loaded).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn fit_backdrop(
    images: Res<Assets<Image>>,
    layers: Query<&ComputedNode, With<MenuBackdrop>>,
    mut media: Query<MediaParts, With<BackdropMedia>>,
) {
    for (parent, mut node, mut visibility, image, video) in &mut media {
        let open = video.is_none_or(|v| v.image().is_some());
        let size = open.then(|| images.get(&image.image)).flatten().map(Image::size_f32);
        let container = layers.get(parent.parent()).map(ComputedNode::size).unwrap_or_default();
        let shown = match size {
            Some(size) if container.x > 0.0 && container.y > 0.0 && size.x > 0.0 && size.y > 0.0 => {
                let (w, h) = cover_size((container.x, container.y), (size.x, size.y));
                let (w, h) = (w / container.x * 100.0, h / container.y * 100.0);
                let fit = (Val::Percent(w), Val::Percent(h), Val::Percent((100.0 - w) / 2.0), Val::Percent((100.0 - h) / 2.0));
                if (node.width, node.height, node.left, node.top) != fit {
                    (node.width, node.height, node.left, node.top) = fit;
                }
                Visibility::Inherited
            }
            _ => Visibility::Hidden,
        };
        visibility.set_if_neq(shown);
    }
}

/// The menu's choices (the screen never closes itself). Quit has asked already.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn host_main_menu(
    mut commands: Commands,
    mut choices: MessageReader<MainMenuEvent>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    store: Option<Res<UiSettingsStore>>,
    localizer: Option<Res<UiLocalizer>>,
    saves: (Option<Res<SaveStorage>>, Option<Res<SaveConfig>>),
    mut phase: ResMut<NextState<AppPhase>>,
    mut exit: MessageWriter<AppExit>,
) {
    for event in choices.read() {
        match event.id.as_str() {
            CONTINUE => {
                if let Some(path) = newest(saves.0.as_deref(), saves.1.as_deref()) {
                    info!("main menu: continue from {} (loads once in game)", path.display());
                    commands.insert_resource(PendingFrontend::Load(path));
                    stack.0.close(MAIN_MENU);
                    phase.set(AppPhase::Loading);
                } else {
                    warn!("main menu: no save to continue");
                }
            }
            NEW_GAME => {
                info!("main menu: new game");
                stack.0.close(MAIN_MENU);
                phase.set(AppPhase::Loading);
            }
            LOAD => {
                info!("main menu: load (the save screen opens once in game)");
                commands.insert_resource(PendingFrontend::OpenLoadScreen);
                stack.0.close(MAIN_MENU);
                phase.set(AppPhase::Loading);
            }
            OPTIONS => crate::pause_menu::open_options(&mut commands, &mut stack, &registry, store.as_deref(), localizer.as_deref()),
            QUIT => {
                info!("main menu: quit");
                exit.write(AppExit::Success);
            }
            other => warn!("main menu: unknown entry {other}"),
        }
    }
}

/// Continue and Load follow the save folder when the save/load screen closes (a delete may
/// have emptied it).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn refresh_saves(
    stack: Res<ScreenStackRes>,
    storage: Option<Res<SaveStorage>>,
    config: Option<Res<SaveConfig>>,
    mut menus: Query<&mut MainMenuScreen>,
    mut was_open: Local<bool>,
) {
    let open = stack.0.is_open(SAVELOAD);
    let closed = *was_open && !open;
    *was_open = open;
    if !closed || menus.is_empty() {
        return;
    }
    let saves = newest(storage.as_deref(), config.as_deref()).is_some();
    for mut menu in &mut menus {
        if menu.model.is_enabled(CONTINUE) != saves || menu.model.is_enabled(LOAD) != saves {
            menu.model.set_enabled(CONTINUE, saves).set_enabled(LOAD, saves);
        }
    }
}

/// The front end never stands without its menu: closed some other way (a screen clearing the
/// Menu layer), the world loads as for New game.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn leave_without_menu(stack: Res<ScreenStackRes>, mut phase: ResMut<NextState<AppPhase>>) {
    if !stack.0.is_open(MAIN_MENU) && matches!(*phase, NextState::Unchanged) {
        warn!("main menu: closed in the front end; loading the world");
        phase.set(AppPhase::Loading);
    }
}

/// Interim (F3 replaces it): the front end's Continue or Load, once in game.
fn run_pending(
    mut commands: Commands,
    pending: Option<Res<PendingFrontend>>,
    mut loads: MessageWriter<LoadRequest>,
    mut open_saves: MessageWriter<OpenSaveLoad>,
) {
    let Some(pending) = pending else { return };
    match &*pending {
        PendingFrontend::Load(path) => {
            info!("main menu: continue, loading {}", path.display());
            loads.write(LoadRequest::path(path.clone()));
        }
        PendingFrontend::OpenLoadScreen => {
            open_saves.write(OpenSaveLoad(SaveLoadMode::Load));
        }
    }
    commands.remove_resource::<PendingFrontend>();
}

/// The menu's root despawns once off the stack.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn close_main_menu(mut commands: Commands, stack: Res<ScreenStackRes>, roots: Query<(Entity, &UiScreenRoot), With<MainMenuScreen>>) {
    if stack.0.is_open(MAIN_MENU) {
        return;
    }
    for (e, root) in &roots {
        if root.0.as_str() == MAIN_MENU {
            commands.entity(e).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    //! Headless, on the pause menu's test app (`pause_menu.rs` tests).
    use std::path::PathBuf;

    use bevy::ecs::message::Messages;
    use sf_app_bevy::AppPhasePlugin;
    use sf_app_bevy::bevy_state::prelude::State;
    use sf_save_bevy::sf_save::SaveReason;
    use sf_save_bevy::{LoadDone, SaveDone, SaveRequest, SfSavePlugin};
    use sf_ui_screens::SaveLoadScreen;

    use super::*;
    use crate::pause_menu::tests::{app_with, frames, open, paused, roots, tap};
    use crate::pause_menu::{HUD, OPTIONS as OPTIONS_SCREEN, PAUSE};

    /// The files of the finished loads.
    #[derive(Resource, Default)]
    struct Loads(Vec<PathBuf>);

    fn record_loads(mut done: MessageReader<LoadDone>, mut loads: ResMut<Loads>) {
        loads.0.extend(done.read().filter_map(|d| d.0.as_ref().ok().map(|r| r.path.clone())));
    }

    fn release() -> App {
        app_with(|app| {
            app.add_plugins(AppPhasePlugin::default());
            add(app, true);
        })
    }

    fn phase(app: &App) -> AppPhase {
        *app.world().resource::<State<AppPhase>>().get()
    }

    /// The world is "in" (the streaming signal, which these apps don't run).
    fn in_game(app: &mut App) {
        app.world_mut().resource_mut::<NextState<AppPhase>>().set(AppPhase::InGame);
        frames(app, 3);
    }

    fn menu(app: &mut App) -> MainMenuScreen {
        let mut q = app.world_mut().query::<&MainMenuScreen>();
        q.single(app.world()).expect("one main menu").clone()
    }

    #[test]
    fn release_start_opens_the_menu_paused_and_new_game_plays() {
        let mut app = release();
        assert!(open(&app, MAIN_MENU) && open(&app, HUD), "the menu over the HUD screen");
        assert_eq!(roots(&mut app, MAIN_MENU), 1);
        assert!(paused(&app), "the game is paused under the menu");
        let screen = menu(&mut app);
        assert_eq!(screen.model.ids(), [CONTINUE, NEW_GAME, LOAD, OPTIONS, QUIT]);
        assert!(!screen.model.is_enabled(CONTINUE) && !screen.model.is_enabled(LOAD), "no save plugin, no saves");
        assert_eq!(screen.model.entry(NEW_GAME).map(|e| e.label.as_str()), Some("New Game"), "the Fluent string");

        // Esc is the menu's Back: nothing at the top level, and no pause menu over it.
        tap(&mut app, KeyCode::Escape);
        assert!(open(&app, MAIN_MENU) && !open(&app, PAUSE) && paused(&app));

        // Focus starts on New game (Continue is disabled); confirm.
        tap(&mut app, KeyCode::Enter);
        assert!(!open(&app, MAIN_MENU), "New game closes the menu");
        assert_eq!(roots(&mut app, MAIN_MENU), 0);
        assert!(!paused(&app), "play begins");
        assert_eq!(phase(&app), AppPhase::Loading, "New game builds the world");
        assert!(app.should_exit().is_none());

        // Play: Esc is the pause menu now.
        tap(&mut app, KeyCode::Escape);
        assert!(open(&app, PAUSE) && paused(&app));
    }

    #[test]
    fn options_open_on_top_and_quit_exits_after_yes() {
        let mut app = release();
        // New game -> (Load, disabled, skipped) -> Options.
        tap(&mut app, KeyCode::ArrowDown);
        tap(&mut app, KeyCode::Enter);
        assert!(open(&app, OPTIONS_SCREEN) && open(&app, MAIN_MENU), "options over the menu");
        assert_eq!(roots(&mut app, OPTIONS_SCREEN), 1);
        tap(&mut app, KeyCode::Escape);
        assert!(!open(&app, OPTIONS_SCREEN) && open(&app, MAIN_MENU) && paused(&app));
        assert_eq!(roots(&mut app, OPTIONS_SCREEN), 0);

        // Options -> Quit; it asks (No focused); left to Yes; confirm.
        tap(&mut app, KeyCode::ArrowDown);
        tap(&mut app, KeyCode::Enter);
        assert!(app.should_exit().is_none(), "Quit asks first");
        tap(&mut app, KeyCode::ArrowLeft);
        tap(&mut app, KeyCode::Enter);
        assert_eq!(app.should_exit(), Some(AppExit::Success));
    }

    /// A release start with the save plugin on `root` (the save/load screen hosted).
    fn release_with_saves(root: &std::path::Path) -> App {
        let config = SaveConfig { root: root.to_path_buf(), background: false, ..default() };
        app_with(|app| {
            app.add_plugins(SfSavePlugin { config })
                .insert_resource(sf_world_bevy::WorldOverlay::default())
                .insert_resource(sf_sky_bevy::SkyTime::default());
            mw_saves_bevy::register(app);
            mw_saves_bevy::menu::add(app);
            app.add_plugins(AppPhasePlugin::default());
            add(app, true);
            app.init_resource::<Loads>().add_systems(Update, record_loads);
        })
    }

    /// A quicksave, then a manual save (the newest: Continue takes any reason, quickload only
    /// quicksaves), written by an earlier run into `root`. Returns (quick, manual) files.
    fn earlier_run(root: &std::path::Path) -> (PathBuf, PathBuf) {
        let mut app = App::new();
        app.add_plugins(SfSavePlugin { config: SaveConfig { root: root.to_path_buf(), background: false, ..default() } })
            .insert_resource(sf_world_bevy::WorldOverlay::default())
            .insert_resource(sf_sky_bevy::SkyTime::default());
        mw_saves_bevy::register(&mut app);
        let mut save = |request: SaveRequest| {
            app.world_mut().write_message(request);
            app.update();
            let done: Vec<_> = app.world_mut().resource_mut::<Messages<SaveDone>>().drain().collect();
            done[0].0.as_ref().expect("saved").path.clone()
        };
        let quick = save(SaveRequest::quick());
        let manual = save(SaveRequest::slot(SaveReason::Manual));
        (quick, manual)
    }

    fn temp_root(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("bw-main-menu-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        root
    }

    fn choose(app: &mut App, id: &str) {
        let mut q = app.world_mut().query_filtered::<Entity, With<MainMenuScreen>>();
        let screen = q.single(app.world()).expect("one main menu");
        app.world_mut().write_message(MainMenuEvent { screen, id: id.into() });
    }

    #[test]
    fn no_saves_leave_continue_and_load_disabled() {
        let root = temp_root("empty");
        let mut app = release_with_saves(&root);
        let screen = menu(&mut app);
        assert!(!screen.model.is_enabled(CONTINUE) && !screen.model.is_enabled(LOAD));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn continue_loads_the_newest_save_and_plays() {
        let root = temp_root("continue");
        let (quick, manual) = earlier_run(&root);
        assert!(quick.is_file() && manual.is_file());
        let mut app = release_with_saves(&root);
        assert_eq!(phase(&app), AppPhase::Frontend);
        let (storage, config) = (app.world().resource::<SaveStorage>(), app.world().resource::<SaveConfig>());
        assert_eq!(mw_saves_bevy::newest_save(storage, config, Some(SaveReason::Quick)), Some(quick), "quickload's target");
        let screen = menu(&mut app);
        assert!(screen.model.is_enabled(CONTINUE) && screen.model.is_enabled(LOAD));
        assert!(open(&app, MAIN_MENU) && paused(&app));

        // Focus starts on Continue now; confirm: the world builds at the new-game start (interim,
        // F3), then the newest save of any reason loads in game.
        tap(&mut app, KeyCode::Enter);
        assert_eq!(phase(&app), AppPhase::Loading);
        assert_eq!(app.world().get_resource::<PendingFrontend>(), Some(&PendingFrontend::Load(manual.clone())));
        assert!(app.world().resource::<Loads>().0.is_empty(), "nothing loads into a missing world");
        assert!(!open(&app, MAIN_MENU) && !paused(&app), "the menu closes");
        assert_eq!(roots(&mut app, MAIN_MENU), 0);
        in_game(&mut app);
        let loads = app.world().resource::<Loads>().0.clone();
        assert_eq!(loads, [manual], "the manual save, newer than the quicksave");
        assert!(app.world().get_resource::<PendingFrontend>().is_none() && !paused(&app), "play runs");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn load_builds_the_world_then_opens_the_save_screen() {
        let root = temp_root("load");
        let _ = earlier_run(&root);
        let mut app = release_with_saves(&root);

        // Interim (F3): the world builds at the new-game start; the save screen waits for it.
        choose(&mut app, LOAD);
        frames(&mut app, 3);
        assert_eq!(phase(&app), AppPhase::Loading);
        assert!(!open(&app, MAIN_MENU) && !open(&app, SAVELOAD), "no save screen over a missing world");
        assert_eq!(app.world().get_resource::<PendingFrontend>(), Some(&PendingFrontend::OpenLoadScreen));

        in_game(&mut app);
        assert!(open(&app, SAVELOAD) && paused(&app), "the save screen over the game");
        let mut q = app.world_mut().query::<&SaveLoadScreen>();
        let model = q.single(app.world()).expect("save/load screen").model.clone();
        assert_eq!(model.mode, SaveLoadMode::Load);
        assert_eq!(model.entries.len(), 2, "a fresh run lists every world's saves");

        // Back: the new game plays, no main menu, no pause menu.
        tap(&mut app, KeyCode::Escape);
        assert!(!open(&app, SAVELOAD) && !open(&app, MAIN_MENU) && !open(&app, PAUSE) && !paused(&app));
        assert!(app.world().resource::<Loads>().0.is_empty());
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Something the world setup spawns (the reduced app has no exterior).
    #[derive(Component)]
    struct WorldProbe;

    #[derive(Resource, Default)]
    struct DriveRuns(u32);

    /// F1: the release start sits in the front end with no world (no setup, no streaming
    /// drive) behind its UI camera; New game builds the world and drops the camera. The
    /// reduced app (no Morrowind install): the phase wiring of `phase.rs` on the menu's test app.
    #[test]
    fn release_front_end_has_no_world_until_new_game() {
        let mut app = app_with(|app| {
            app.insert_resource(sf_world_bevy::Streaming::new(sf_world_bevy::StreamingConfig::default()))
                .init_resource::<sf_world_bevy::ActiveWorldspace>()
                .init_resource::<DriveRuns>()
                .add_systems(OnEnter(sf_app_bevy::WorldAlive), |mut commands: Commands| {
                    commands.spawn(WorldProbe);
                })
                .add_systems(
                    Update,
                    (|mut runs: ResMut<DriveRuns>| runs.0 += 1).in_set(sf_world_bevy::StreamingSystems::Drive),
                );
            crate::phase::add(app, true);
            add(app, true);
        });
        let world = |app: &mut App| app.world_mut().query::<&WorldProbe>().iter(app.world()).count();
        let cameras = |app: &mut App| app.world_mut().query::<&crate::phase::FrontendCamera>().iter(app.world()).count();
        frames(&mut app, 10);
        assert_eq!(phase(&app), AppPhase::Frontend);
        assert!(open(&app, MAIN_MENU));
        assert_eq!(world(&mut app), 0, "no world setup ran");
        assert_eq!(app.world().resource::<DriveRuns>().0, 0, "streaming never ran");
        assert_eq!(cameras(&mut app), 1, "the front end's UI camera");

        choose(&mut app, NEW_GAME);
        frames(&mut app, 3);
        assert!(app.world().get_resource::<State<sf_app_bevy::WorldAlive>>().is_some());
        assert_eq!(world(&mut app), 1, "the world set up once");
        assert!(app.world().resource::<DriveRuns>().0 > 0, "streaming runs");
        assert_eq!(cameras(&mut app), 0, "the front end's camera is gone");
        assert!(!open(&app, MAIN_MENU));
        // No worldspace active: no directory to wait for; the burst it arms never ends without
        // the streaming plugin, so the reduced app stays in Loading.
        assert_eq!(phase(&app), AppPhase::Loading);
        assert!(app.world().resource::<sf_world_bevy::Streaming>().in_burst(), "the start burst is armed");
    }

    /// A release start over an in-memory VFS of `files`; `video`: with the video players.
    fn release_with_vfs(files: &[(&str, &[u8])], video: bool) -> App {
        let source = files.iter().fold(sf_vfs::MemSource::new("test"), |s, (path, bytes)| s.with(path, bytes.to_vec()));
        let vfs = std::sync::Arc::new(Vfs::builder().mount(source).build());
        app_with(|app| {
            app.insert_resource(mw_assets::SharedVfs(vfs));
            if video {
                app.add_plugins(sf_video_bevy::VideoPlugin);
            }
            app.add_plugins(AppPhasePlugin::default());
            add(app, true);
        })
    }

    /// The backdrop media node: whether it holds a video player, and its image's asset path.
    fn backdrop(app: &mut App) -> Option<(Entity, bool, Option<String>)> {
        let mut q = app.world_mut().query_filtered::<(Entity, Has<VideoPlayer>, &ImageNode), With<BackdropMedia>>();
        let found: Vec<_> = q.iter(app.world()).map(|(e, video, image)| (e, video, image.image.path().map(|p| p.path().to_string_lossy().replace('\\', "/")))).collect();
        assert!(found.len() <= 1, "one backdrop at most");
        found.into_iter().next()
    }

    const DDS: &str = "textures/menu_morrowind.dds";

    #[test]
    fn backdrop_candidates_follow_the_ini() {
        let paths = |c: Vec<BackdropCandidate>| c.into_iter().map(|c| (c.path, c.kind)).collect::<Vec<_>>();
        let vanilla = [
            ("video/menu_background.bik".to_owned(), BackdropKind::Video),
            ("video/mw_menu.bik".to_owned(), BackdropKind::Video),
            (DDS.to_owned(), BackdropKind::Image),
        ];
        assert_eq!(paths(backdrop_candidates(None)), vanilla);
        assert_eq!(paths(backdrop_candidates(Some("  "))), vanilla, "blank: vanilla's video");
        assert_eq!(paths(backdrop_candidates(Some(" my_menu.bik ")))[1].0, "video/my_menu.bik");
        assert_eq!(MenuBackdropCandidates::default().0, backdrop_candidates(None));
    }

    #[test]
    fn the_menu_video_plays_behind_the_menu_and_stops_with_it() {
        // No players: the video stays pending (no decode in a headless test).
        let mut app = release_with_vfs(&[("video/mw_menu.bik", b"BIKi"), (DDS, b"dds")], false);
        let (_, video, _) = backdrop(&mut app).expect("a backdrop");
        assert!(video, "vanilla's menu video wins over the image");
        let screen = menu(&mut app);
        assert!(screen.model.title_image.is_none() && screen.model.background.is_none(), "the art is in the video");

        choose(&mut app, NEW_GAME);
        frames(&mut app, 3);
        assert!(backdrop(&mut app).is_none(), "the player despawns with the menu");
    }

    #[test]
    fn without_a_video_the_title_image_covers_the_window() {
        let mut app = release_with_vfs(&[(DDS, b"dds")], false);
        let (media, video, path) = backdrop(&mut app).expect("a backdrop");
        assert!(!video);
        assert_eq!(path.as_deref(), Some(DDS));

        // A loaded 640x480 image in the test window (1600x1000): full width, cropped top and bottom.
        let image = app.world_mut().resource_mut::<Assets<Image>>().add(Image::new_fill(
            bevy::render::render_resource::Extent3d { width: 640, height: 480, depth_or_array_layers: 1 },
            bevy::render::render_resource::TextureDimension::D2,
            &[0, 0, 0, 255],
            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
            bevy::asset::RenderAssetUsages::default(),
        ));
        app.world_mut().entity_mut(media).insert(ImageNode::new(image));
        frames(&mut app, 3);
        let node = app.world().get::<Node>(media).expect("media node");
        let percent = |v: Val| match v {
            Val::Percent(p) => p,
            other => panic!("not a percentage: {other:?}"),
        };
        let fit = [node.width, node.height, node.left, node.top].map(percent);
        for (got, want) in fit.into_iter().zip([100.0, 120.0, 0.0, -10.0]) {
            assert!((got - want).abs() < 1e-3, "cover fit {fit:?}");
        }
        assert_eq!(app.world().get::<Visibility>(media), Some(&Visibility::Inherited));
    }

    #[test]
    fn a_video_that_fails_to_decode_falls_through_to_the_image() {
        let mut app = release_with_vfs(&[("video/menu_background.bik", b"not a video"), (DDS, b"dds")], true);
        frames(&mut app, 3);
        let (_, video, path) = backdrop(&mut app).expect("a backdrop");
        assert!(!video, "the failed player is dropped");
        assert_eq!(path.as_deref(), Some(DDS));
    }

    #[test]
    fn no_candidate_leaves_the_theme_background() {
        let mut app = release_with_vfs(&[], false);
        assert!(backdrop(&mut app).is_none());
        assert!(open(&app, MAIN_MENU));
    }

    #[test]
    fn dev_runs_open_no_main_menu() {
        let mut app = app_with(|app| add(app, false));
        assert!(!open(&app, MAIN_MENU) && !paused(&app));
        assert_eq!(roots(&mut app, MAIN_MENU), 0);
    }
}
