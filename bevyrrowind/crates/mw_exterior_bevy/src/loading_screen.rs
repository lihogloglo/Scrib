//! The loading screen while the world builds (menus.md §3 "Intro videos and loading screen",
//! ui-adapter.md "Menus and front end", **Loading screen**): the framework's
//! [`LoadingScreen`] over the whole window from entering [`AppPhase::Loading`] to leaving it
//! (`DespawnOnExit`), on a release start only (dev runs go straight to the world).
//!
//! - **Splash**: a random image under `splash/`, recursive, loose over BSA (the VFS's winner),
//!   as `OpenMW`'s `loadingscreen.cpp` picks one per load (the behaviour, no code). One pick per `Loading`
//!   entry ([`pick_splash`], a pure function of the path list and a seed), loaded through the
//!   asset server as the main menu's backdrop image is. No splash: the plain opaque screen.
//! - **Progress**: the streaming burst's work left. `sf_world_bevy` exposes the pending
//!   requests (`Scheduler::requests`), not a resident/requested pair, so the bar is
//!   `1 - pending / peak pending` over the active worldspace's loads to `Near` or closer (the
//!   loads the burst waits for, `Scheduler::update`), never moving backwards; unknown
//!   (an animated sweep) until the first such request appears (the directory still loading, a
//!   save's load still placing the player).
//! - **Hint**: empty for now (a later slice adds tips as Fluent keys).

use bevy::prelude::*;
use sf_app_bevy::AppPhase;
use sf_app_bevy::bevy_state::prelude::{DespawnOnExit, OnEnter, in_state};
use sf_ui_screens::{LoadingModel, LoadingScreen, ScreenSystems};
use sf_world::{Level, RequestKind};
use sf_world_bevy::{ActiveWorldspace, Streaming};

/// Image extensions the asset server decodes for a splash.
const IMAGE_EXTENSIONS: [&str; 4] = ["tga", "dds", "bmp", "png"];

/// The loading screens' folder (VFS paths are lowercase; compared case-insensitively anyway).
const SPLASH_DIR: &str = "splash/";

/// `open`: the release start (its front end leads into `Loading`), as `main_menu::add`.
pub fn add(app: &mut App, open: bool) {
    if !open {
        return;
    }
    app.add_systems(OnEnter(AppPhase::Loading), open_loading_screen)
        .add_systems(Update, feed_progress.run_if(in_state(AppPhase::Loading)).before(ScreenSystems::Sync));
}

/// splitmix64: one well-mixed draw from `seed` (no RNG crate for one pick per load).
fn mix(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// One image under `splash/` (any depth, any case) from `paths`, chosen by `seed`; none when
/// there is no such image. The candidates are sorted and deduplicated first, so the pick
/// depends only on the set of paths and the seed.
pub fn pick_splash<'a>(paths: impl IntoIterator<Item = &'a str>, seed: u64) -> Option<String> {
    let mut candidates: Vec<String> = paths
        .into_iter()
        .map(|p| p.replace('\\', "/").to_ascii_lowercase())
        .filter(|p| {
            p.strip_prefix(SPLASH_DIR).is_some_and(|rest| {
                !rest.is_empty()
                    && rest.rsplit_once('.').is_some_and(|(name, ext)| {
                        !name.is_empty() && !name.ends_with('/') && IMAGE_EXTENSIONS.contains(&ext)
                    })
            })
        })
        .collect();
    candidates.sort();
    candidates.dedup();
    if candidates.is_empty() {
        return None;
    }
    let len = u64::try_from(candidates.len()).unwrap_or(u64::MAX);
    let index = usize::try_from(mix(seed) % len).unwrap_or(0);
    Some(candidates.swap_remove(index))
}

/// A fresh seed per `Loading` entry: the clock, mixed with a per-run counter.
fn seed(counter: &mut u64) -> u64 {
    *counter = counter.wrapping_add(1);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs().wrapping_mul(1_000_000_000).wrapping_add(u64::from(d.subsec_nanos())));
    nanos ^ mix(*counter)
}

/// The peak count of pending start-area loads this `Loading` has seen (on the screen's root).
#[derive(Component, Debug, Default)]
struct LoadingPeak(usize);

#[allow(clippy::needless_pass_by_value)] // system parameters
fn open_loading_screen(
    mut commands: Commands,
    vfs: Option<Res<mw_assets::SharedVfs>>,
    assets: Option<Res<AssetServer>>,
    mut counter: Local<u64>,
) {
    let splash = vfs.as_deref().and_then(|vfs| pick_splash(vfs.0.paths().map(sf_vfs::VfsPath::as_str), seed(&mut counter)));
    let image = match (splash, assets.as_deref()) {
        (Some(path), Some(assets)) => {
            info!("loading screen: splash {path}");
            Some(assets.load(path))
        }
        (Some(path), None) => {
            warn!("loading screen: no asset server for splash {path}");
            None
        }
        (None, _) => None,
    };
    commands.spawn((
        Name::new("loading screen"),
        LoadingScreen::new(LoadingModel { image, progress: None, hint: String::new() }),
        LoadingPeak::default(),
        DespawnOnExit(AppPhase::Loading),
    ));
}

/// The bar's fraction from the pending count and its peak (see the module docs); `was`: the
/// last fraction shown (the bar never moves backwards).
fn fraction(pending: usize, peak: usize, was: Option<f32>) -> Option<f32> {
    if peak == 0 {
        return was;
    }
    #[allow(clippy::cast_precision_loss)] // cell counts, far below 2^24
    let now = 1.0 - pending as f32 / peak as f32;
    Some(was.map_or(now, |w| w.max(now)))
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn feed_progress(
    streaming: Option<Res<Streaming>>,
    active: Option<Res<ActiveWorldspace>>,
    mut screens: Query<(&mut LoadingScreen, &mut LoadingPeak)>,
) {
    let Some(streaming) = streaming else { return };
    let active = active.as_deref().and_then(|a| a.0.as_ref());
    let pending = streaming
        .scheduler()
        .requests()
        .filter(|r| r.kind == RequestKind::Load && r.target >= Level::Near && active.is_none_or(|a| *a == r.worldspace))
        .count();
    for (mut screen, mut peak) in &mut screens {
        peak.0 = peak.0.max(pending);
        let progress = fraction(pending, peak.0, screen.model.progress);
        if screen.model.progress != progress {
            screen.model.progress = progress;
        }
    }
}

#[cfg(test)]
mod tests {
    use sf_app_bevy::AppPhasePlugin;
    use sf_app_bevy::bevy_state::prelude::{NextState, State};

    use super::*;

    #[test]
    fn splash_pick_keeps_only_splash_images_any_case_any_depth() {
        let paths = [
            "Splash/Splash_Bonelord.tga",
            "splash/sub/deep/Vivec.DDS",
            "splash/readme.txt",
            "textures/splash/not_this.tga",
            "splashy/no.tga",
            "splash/",
            "splash/.tga",
            "meshes/x.nif",
        ];
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..64 {
            seen.insert(pick_splash(paths, seed).expect("two candidates"));
        }
        let want: std::collections::BTreeSet<String> =
            ["splash/splash_bonelord.tga", "splash/sub/deep/vivec.dds"].into_iter().map(String::from).collect();
        assert_eq!(seen, want);
    }

    #[test]
    fn splash_pick_is_a_function_of_the_set_and_seed() {
        let a = ["splash/a.tga", "splash/b.tga", "splash/c.tga"];
        let b = ["splash/c.tga", "SPLASH/A.TGA", "splash/b.tga", "splash/a.tga"];
        for seed in 0..16 {
            assert_eq!(pick_splash(a, seed), pick_splash(b, seed));
        }
    }

    #[test]
    fn splash_pick_none_without_splash_images() {
        assert_eq!(pick_splash([], 7), None);
        assert_eq!(pick_splash(["textures/menu_morrowind.dds", "splash/notes.txt"], 7), None);
    }

    #[test]
    fn loading_fraction_waits_for_work_and_never_goes_back() {
        assert_eq!(fraction(0, 0, None), None);
        assert_eq!(fraction(10, 10, None), Some(0.0));
        assert_eq!(fraction(5, 10, Some(0.0)), Some(0.5));
        // More work found: the peak grows, the bar holds.
        assert_eq!(fraction(15, 20, Some(0.5)), Some(0.5));
        assert_eq!(fraction(0, 20, Some(0.5)), Some(1.0));
    }

    fn screens(app: &mut App) -> usize {
        app.world_mut().query_filtered::<Entity, With<LoadingScreen>>().iter(app.world()).count()
    }

    fn go(app: &mut App, to: AppPhase) {
        app.world_mut().resource_mut::<NextState<AppPhase>>().set(to);
        app.update();
        assert_eq!(*app.world().resource::<State<AppPhase>>().get(), to);
    }

    /// A reduced app (no install, no streaming): the screen is up through `Loading` and gone in
    /// `InGame`, once per entry.
    #[test]
    fn loading_screen_shows_in_loading_and_is_gone_in_game() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AppPhasePlugin { after_boot: AppPhase::Frontend }));
        add(&mut app, true);
        app.update();
        app.update();
        assert_eq!(*app.world().resource::<State<AppPhase>>().get(), AppPhase::Frontend);
        assert_eq!(screens(&mut app), 0);
        for _ in 0..2 {
            go(&mut app, AppPhase::Loading);
            assert_eq!(screens(&mut app), 1);
            app.update();
            assert_eq!(screens(&mut app), 1, "stays while loading");
            go(&mut app, AppPhase::InGame);
            assert_eq!(screens(&mut app), 0);
            go(&mut app, AppPhase::Frontend);
        }
    }
}
