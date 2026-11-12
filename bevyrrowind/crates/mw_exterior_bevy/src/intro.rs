//! The intro videos of a release start (menus.md "Intro videos and loading screen", owner answer
//! 2: first launch only, skippable): Bethesda's and Morrowind's logos, in `AppPhase::Boot`, before
//! the main menu.
//!
//! - **List** (adapter hook [`IntroVideos`], ui-adapter.md "Menus and front end"):
//!   `Morrowind.ini`'s `[Movies] Company Logo=` then `Morrowind Logo=` under `video/`, vanilla's
//!   `bethesda logo.bik` and `mw_logo.bik` when the ini names none ([`intro_videos`]). A file not
//!   in the VFS is skipped silently (an install without them, a total conversion); one that fails
//!   to read or decode ends with a `warn!` and the next plays.
//! - **When**: the framework setting `sf.intro_videos` (`sf_ui::IntroVideos`): `first_launch`
//!   (default) plays them until they have played once, `always` every start, `off` never. Dev runs
//!   (`view`, `scene`) never play them.
//! - **First launch** ([`IntroSeen`]): a marker file `<user dir>/intro_seen`, written when the
//!   sequence ends after playing at least one video (finished or skipped). A file of its own, not
//!   a hidden setting: deleting it shows the logos again, and no options page lists it.
//! - **Playback**: while it plays, the boot hold [`INTRO_HOLD`] (`sf_app_bevy::BootHolds`) keeps
//!   `Boot`; a black full-window node over its own UI camera (Boot has none) shows each video
//!   letterboxed (`sf_ui::contain_size`), one [`VideoPlayer`] at a time, skippable (Confirm, Back,
//!   Escape, a click: `sf_video_bevy`'s skip input, which consumes the press). Sound plays on
//!   the movie bus. After the last one the node and camera despawn and the hold is released; the
//!   front end opens on the next update.

use std::collections::VecDeque;
use std::path::PathBuf;

use bevy::prelude::*;
use sf_app_bevy::BootHolds;
use sf_ui_bevy::UiSettingsStore;
use sf_vfs::{Vfs, VfsPath};
use sf_video_bevy::{VideoFinished, VideoPlayer, VideoSystems};

use crate::main_menu::MenuMovies;

/// The boot hold's name while the intro plays.
pub const INTRO_HOLD: &str = "intro_videos";

/// Vanilla's `[Movies] Company Logo=` and `Morrowind Logo=`.
const COMPANY_LOGO: &str = "bethesda logo.bik";
const MORROWIND_LOGO: &str = "mw_logo.bik";

/// The first-launch marker's file name in the user data folder.
pub const SEEN_FILE: &str = "intro_seen";

/// Adapter hook (menus.md §3): the intro videos' VFS paths, in play order.
#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct IntroVideos(pub Vec<String>);

/// The intro list from `Morrowind.ini`'s `[Movies]` names (none or blank: vanilla's).
pub fn intro_videos(movies: &MenuMovies) -> Vec<String> {
    let name = |ini: &Option<String>, vanilla: &'static str| {
        let name = ini.as_deref().map(str::trim).filter(|s| !s.is_empty()).unwrap_or(vanilla);
        format!("video/{name}")
    };
    vec![name(&movies.company_logo, COMPANY_LOGO), name(&movies.morrowind_logo, MORROWIND_LOGO)]
}

/// Where the first-launch marker lives (`None`: no user folder; `first_launch` then plays every
/// start, nothing can remember it).
#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct IntroSeen(pub Option<PathBuf>);

impl IntroSeen {
    fn is_seen(&self) -> bool {
        self.0.as_deref().is_some_and(std::path::Path::exists)
    }

    fn record(&self) {
        let Some(path) = self.0.as_deref() else { return };
        let write = || -> std::io::Result<()> {
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)?;
            }
            std::fs::write(path, "The intro videos have played once (sf.intro_videos = first_launch). Delete to see them again.\n")
        };
        if let Err(e) = write() {
            warn!("intro: can't record the first launch in {}: {e}", path.display());
        }
    }
}

/// The intro in progress: what is left to play, its node, camera and current player.
#[derive(Resource, Debug)]
struct IntroSequence {
    queue: VecDeque<String>,
    root: Entity,
    camera: Entity,
    media: Option<Entity>,
    played: bool,
}

/// The intro's full-window black node.
#[derive(Component, Debug)]
struct IntroRoot;

/// The intro's UI camera.
#[derive(Component, Debug)]
struct IntroCamera;

/// The node of the playing video; its VFS path.
#[derive(Component, Debug)]
struct IntroMedia(String);

/// The release start's intro (after the app phases and the video players, `exterior.rs`).
/// `release`: the release start; dev runs never play intros.
pub fn add(app: &mut App, release: bool, movies: &MenuMovies) {
    if !release {
        return;
    }
    let seen = mw_boot_bevy::mods::user_data().map(|dir| dir.join(SEEN_FILE));
    add_with(app, IntroVideos(intro_videos(movies)), IntroSeen(seen));
}

fn add_with(app: &mut App, videos: IntroVideos, seen: IntroSeen) {
    app.insert_resource(videos).insert_resource(seen).add_message::<VideoFinished>();
    app.add_systems(Startup, start_intro)
        .add_systems(Update, advance_intro.after(VideoSystems).run_if(resource_exists::<IntroSequence>))
        .add_systems(PostUpdate, fit_intro.before(bevy::ui::UiSystems::Prepare).run_if(resource_exists::<IntroSequence>));
}

fn exists(vfs: &Vfs, path: &str) -> bool {
    VfsPath::new(path).is_ok_and(|p| vfs.contains(&p))
}

/// Startup: whether the intro plays (setting, first-launch marker, files present); if so, holds
/// `Boot` and spawns the black node and its camera (the first video starts in `Update`).
#[allow(clippy::needless_pass_by_value)] // system parameters
fn start_intro(
    mut commands: Commands,
    videos: Res<IntroVideos>,
    seen: Res<IntroSeen>,
    store: Option<Res<UiSettingsStore>>,
    vfs: Option<Res<mw_assets::SharedVfs>>,
    holds: Option<ResMut<BootHolds>>,
) {
    let mode = store.map(|s| sf_ui::IntroVideos::from_store(&s.0)).unwrap_or_default();
    match mode {
        sf_ui::IntroVideos::Off => return,
        sf_ui::IntroVideos::FirstLaunch if seen.is_seen() => {
            info!("intro: already seen (sf.intro_videos = first_launch)");
            return;
        }
        _ => {}
    }
    let (Some(vfs), Some(mut holds)) = (vfs, holds) else { return };
    let queue: VecDeque<String> = videos.0.iter().filter(|p| exists(&vfs.0, p)).cloned().collect();
    if queue.is_empty() {
        return;
    }
    holds.hold(INTRO_HOLD);
    let camera = commands
        .spawn((IntroCamera, Camera2d, Camera { order: 1, clear_color: ClearColorConfig::Custom(Color::BLACK), ..default() }))
        .id();
    let root = commands
        .spawn((
            IntroRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            GlobalZIndex(i32::MAX - 1),
            UiTargetCamera(camera),
        ))
        .id();
    commands.insert_resource(IntroSequence { queue, root, camera, media: None, played: false });
}

/// Starts the next video once the current one has ended (finished, skipped or failed); after the
/// last, despawns the intro, records the first launch and releases the boot hold.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn advance_intro(
    mut commands: Commands,
    mut sequence: ResMut<IntroSequence>,
    mut finished: MessageReader<VideoFinished>,
    playing: Query<&IntroMedia>,
    vfs: Option<Res<mw_assets::SharedVfs>>,
    seen: Res<IntroSeen>,
    mut holds: ResMut<BootHolds>,
) {
    let current = sequence.media;
    // Every message read (no short-circuit), so none is seen twice.
    let mut ended = false;
    for f in finished.read() {
        ended |= Some(f.entity) == current;
    }
    if current.is_some() && !ended {
        return;
    }
    if let Some(media) = sequence.media.take() {
        if let Ok(IntroMedia(path)) = playing.get(media) {
            info!("intro: {path} ended");
        }
        commands.entity(media).despawn();
    }
    while let Some(path) = sequence.queue.pop_front() {
        let bytes = vfs
            .as_deref()
            .ok_or_else(|| "no VFS".to_owned())
            .and_then(|vfs| VfsPath::new(&path).map_err(|e| e.to_string()).and_then(|p| vfs.0.read(&p).map_err(|e| e.to_string())));
        match bytes {
            Ok(bytes) => {
                info!("intro: playing {path}");
                let media = commands
                    .spawn((
                        IntroMedia(path),
                        Node::default(),
                        // Until the video's size is known (no white placeholder flash).
                        Visibility::Hidden,
                        VideoPlayer::new(bytes, false).skippable(true),
                        ChildOf(sequence.root),
                    ))
                    .id();
                sequence.media = Some(media);
                sequence.played = true;
                return;
            }
            Err(e) => warn!("intro: {path}: {e}; skipping it"),
        }
    }
    info!("intro: done");
    commands.entity(sequence.root).despawn();
    commands.entity(sequence.camera).despawn();
    commands.remove_resource::<IntroSequence>();
    if sequence.played {
        seen.record();
    }
    holds.release(INTRO_HOLD);
}

/// The `contain` fit: the video whole, centred over black, in percent of the window's node.
/// Hidden until the video is open and its size known.
fn fit_intro(images: Res<Assets<Image>>, roots: Query<&ComputedNode, With<IntroRoot>>, mut media: Query<(&ChildOf, &mut Node, &mut Visibility, &ImageNode, &VideoPlayer), With<IntroMedia>>) {
    for (parent, mut node, mut visibility, image, video) in &mut media {
        let size = video.image().and_then(|_| images.get(&image.image)).map(Image::size_f32);
        let container = roots.get(parent.parent()).map(ComputedNode::size).unwrap_or_default();
        let shown = match size {
            Some(size) if container.x > 0.0 && container.y > 0.0 && size.x > 0.0 && size.y > 0.0 => {
                let (w, h) = sf_ui::contain_size((container.x, container.y), (size.x, size.y));
                let fit = (Val::Percent(w / container.x * 100.0), Val::Percent(h / container.y * 100.0));
                if (node.width, node.height) != fit {
                    (node.width, node.height) = fit;
                }
                Visibility::Inherited
            }
            _ => Visibility::Hidden,
        };
        visibility.set_if_neq(shown);
    }
}

#[cfg(test)]
mod tests {
    //! Headless, on the pause menu's test app (`pause_menu.rs` tests). No video players: a
    //! video's end is a written `VideoFinished` (no decode in a headless test).
    use sf_app_bevy::bevy_state::prelude::State;
    use sf_app_bevy::{AppPhase, AppPhasePlugin};
    use sf_ui::{INTRO_VIDEOS_KEY, SettingValue};

    use super::*;
    use crate::pause_menu::tests::{app_with, frames};

    const LOGO: &str = "video/bethesda logo.bik";
    const TITLE: &str = "video/mw_logo.bik";

    /// A fresh first-launch marker path (no file yet).
    fn seen_path(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("bw-intro-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir.join(SEEN_FILE)
    }

    /// A release boot over a VFS of `files`; `mode`: `sf.intro_videos` (`None`: the default).
    fn intro_app(files: &[&str], mode: Option<&str>, seen: &std::path::Path) -> App {
        let source = files.iter().fold(sf_vfs::MemSource::new("test"), |s, path| s.with(path, b"BIKi".to_vec()));
        let vfs = std::sync::Arc::new(Vfs::builder().mount(source).build());
        app_with(|app| {
            app.insert_resource(mw_assets::SharedVfs(vfs));
            app.add_plugins(AppPhasePlugin::default());
            if let Some(mode) = mode {
                let mut store = app.world_mut().resource_mut::<UiSettingsStore>();
                store.0.set(INTRO_VIDEOS_KEY, SettingValue::Text(mode.into())).expect("an intro option");
            }
            add_with(app, IntroVideos(intro_videos(&MenuMovies::default())), IntroSeen(Some(seen.to_owned())));
        })
    }

    fn phase(app: &App) -> AppPhase {
        *app.world().resource::<State<AppPhase>>().get()
    }

    /// The playing video's node and path (at most one).
    fn playing(app: &mut App) -> Option<(Entity, String)> {
        let mut q = app.world_mut().query_filtered::<(Entity, &IntroMedia), With<VideoPlayer>>();
        let found: Vec<_> = q.iter(app.world()).map(|(e, m)| (e, m.0.clone())).collect();
        assert!(found.len() <= 1, "one intro video at a time");
        found.into_iter().next()
    }

    fn intro_parts(app: &mut App) -> (usize, usize) {
        let roots = app.world_mut().query::<&IntroRoot>().iter(app.world()).count();
        let cameras = app.world_mut().query::<&IntroCamera>().iter(app.world()).count();
        (roots, cameras)
    }

    fn held(app: &App) -> bool {
        app.world().resource::<BootHolds>().is_held(INTRO_HOLD)
    }

    fn end(app: &mut App, entity: Entity) {
        app.world_mut().write_message(VideoFinished { entity });
        app.update();
    }

    #[test]
    fn the_intro_list_follows_the_ini() {
        assert_eq!(intro_videos(&MenuMovies::default()), [LOGO, TITLE]);
        let movies = MenuMovies { company_logo: Some(" studio.bik ".into()), morrowind_logo: Some("  ".into()), ..MenuMovies::default() };
        assert_eq!(intro_videos(&movies), ["video/studio.bik", TITLE]);
    }

    #[test]
    fn always_plays_both_videos_in_boot_then_the_front_end() {
        let seen = seen_path("always");
        let mut app = intro_app(&[LOGO, TITLE], Some("always"), &seen);
        assert_eq!(phase(&app), AppPhase::Boot);
        assert!(held(&app));
        assert_eq!(intro_parts(&mut app), (1, 1));
        let (first, path) = playing(&mut app).expect("the first video plays");
        assert_eq!(path, LOGO);
        assert!(app.world().get::<VideoPlayer>(first).unwrap().is_skippable());
        frames(&mut app, 5);
        assert_eq!(phase(&app), AppPhase::Boot, "held while it plays");

        end(&mut app, first);
        let (second, path) = playing(&mut app).expect("the second video plays");
        assert_eq!(path, TITLE);
        assert!(app.world().get_entity(first).is_err(), "the first player despawned");
        frames(&mut app, 2);
        assert_eq!(phase(&app), AppPhase::Boot);

        end(&mut app, second);
        assert_eq!(playing(&mut app), None);
        assert_eq!(intro_parts(&mut app), (0, 0), "node and camera gone");
        assert!(!held(&app));
        app.update();
        assert_eq!(phase(&app), AppPhase::Frontend);
        assert!(seen.exists(), "the first launch is recorded");
        let _ = std::fs::remove_dir_all(seen.parent().unwrap());
    }

    #[test]
    fn a_missing_video_is_skipped() {
        let seen = seen_path("missing");
        let mut app = intro_app(&[TITLE], Some("always"), &seen);
        let (only, path) = playing(&mut app).expect("the present video plays");
        assert_eq!(path, TITLE);
        end(&mut app, only);
        app.update();
        assert_eq!(phase(&app), AppPhase::Frontend);

        let mut app = intro_app(&[], Some("always"), &seen_path("none"));
        assert_eq!(phase(&app), AppPhase::Frontend, "no video: no hold");
        assert_eq!((playing(&mut app), intro_parts(&mut app)), (None, (0, 0)));
        let _ = std::fs::remove_dir_all(seen.parent().unwrap());
    }

    #[test]
    fn off_goes_straight_to_the_front_end() {
        let seen = seen_path("off");
        let mut app = intro_app(&[LOGO, TITLE], Some("off"), &seen);
        assert_eq!(phase(&app), AppPhase::Frontend);
        assert_eq!((playing(&mut app), intro_parts(&mut app)), (None, (0, 0)));
        assert!(!seen.exists());
    }

    #[test]
    fn first_launch_plays_once_then_never() {
        let seen = seen_path("first");
        let mut app = intro_app(&[LOGO, TITLE], None, &seen);
        assert_eq!(phase(&app), AppPhase::Boot);
        assert!(!seen.exists(), "recorded only at the end");
        for _ in 0..2 {
            let (media, _) = playing(&mut app).expect("a video plays");
            end(&mut app, media);
        }
        app.update();
        assert_eq!(phase(&app), AppPhase::Frontend);
        assert!(seen.exists());

        let mut app = intro_app(&[LOGO, TITLE], None, &seen);
        assert_eq!(phase(&app), AppPhase::Frontend, "seen: no intro");
        assert_eq!((playing(&mut app), intro_parts(&mut app)), (None, (0, 0)));
        let _ = std::fs::remove_dir_all(seen.parent().unwrap());
    }
}
