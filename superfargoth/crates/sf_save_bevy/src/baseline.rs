//! The new-game baseline (`docs/specs/menus.md` "### 1. App phases": Main menu from the pause
//! menu leaves `InGame` for `Frontend`, and New game from there must start a fresh playthrough,
//! not continue the last one).
//!
//! Game-state resources (quests, stats, clock, globals, effects, purses) are built from content
//! at app build, so before any world exists (the front end) they hold the pristine new-game
//! state. [`capture_baseline`] snapshots every save provider then, encoded at its current
//! version with its flags, exactly as a save would; [`apply_baseline`] later decodes that
//! snapshot as a load does and runs the load's apply path (checks, reset hooks, every provider
//! in apply order: a full replace), then starts a new [`SaveSession`]. One path restores state,
//! so a section that loads correctly also resets correctly (save.md "Loading" step 4).

use bevy_ecs::prelude::*;
use sf_save::{SectionFlags, SectionKey};

use crate::{Autosave, LOAD_FAILED, LoadError, SaveGate, SaveSection, SaveSections, SaveSession, apply_with};

/// One provider's section in the baseline, as a save file would hold it.
#[derive(Clone, Debug)]
struct Entry {
    key: SectionKey,
    version: u32,
    flags: SectionFlags,
    bytes: Vec<u8>,
}

/// Every provider's snapshot taken before any world existed ([`capture_baseline`]).
#[derive(Resource, Clone, Debug, Default)]
pub struct NewGameBaseline {
    entries: Vec<Entry>,
    /// Providers whose snapshot failed at capture (family, message): they get no value on apply,
    /// so they reset to their empty state ([`SaveSection::value`] applies `T::default()`).
    failed: Vec<(String, String)>,
}

impl NewGameBaseline {
    /// The captured sections: key, schema version, flags and encoded size.
    pub fn sections(&self) -> impl Iterator<Item = (&SectionKey, u32, SectionFlags, usize)> {
        self.entries.iter().map(|e| (&e.key, e.version, e.flags, e.bytes.len()))
    }

    /// Providers whose snapshot failed at capture (family, message).
    pub fn failed(&self) -> &[(String, String)] {
        &self.failed
    }
}

/// Snapshots every registered provider into [`NewGameBaseline`] (exclusive). Call it once the
/// content has loaded and before any world builds (the first `Frontend` entry); later calls
/// keep the first capture. Without the save plugin's [`SaveSections`] there is nothing to do.
pub fn capture_baseline(world: &mut World) {
    if world.contains_resource::<NewGameBaseline>() {
        return;
    }
    let Some(providers) = world.get_resource::<SaveSections>() else { return };
    let mut baseline = NewGameBaseline::default();
    for s in &providers.sections {
        match s.io.snapshot(world) {
            Ok(list) => baseline.entries.extend(list.into_iter().map(|(key, encode)| Entry {
                key,
                version: s.spec.current(),
                flags: s.flags,
                bytes: encode(),
            })),
            Err(message) => {
                log::warn!("sf_save_bevy: new-game baseline: section {} failed its snapshot: {message}", s.spec.family());
                baseline.failed.push((s.spec.family().to_string(), message));
            }
        }
    }
    log::info!("sf_save_bevy: new-game baseline captured ({} sections)", baseline.entries.len());
    world.insert_resource(baseline);
}

/// Restores the [`NewGameBaseline`] through the load path, then starts a new playthrough.
///
/// Decodes each section as a load does (migrated only if a provider's version changed since the
/// capture, which can't happen in one run), runs the checks, reset hooks and every provider's
/// apply, and clears the dormant sections (a new game carries no missing mod's data). Then the
/// session is [`SaveSession::default`]: `world_id = 0`, so the first save names the world
/// (`drive`'s rule), like a new game in a fresh process, with no parent save, `last_path = None`
/// (quicksave and Continue don't target the old playthrough's file), counters and play time at
/// zero; [`Autosave`] restarts its timer.
///
/// Errors: [`LoadError::NotFound`] when no baseline was captured (nothing changed); a check,
/// decode or migrate refusal changes nothing; [`LoadError::Apply`] leaves a partly applied world
/// and closes the [`SaveGate`] ([`LOAD_FAILED`]) as a failed load does. Success opens it.
pub fn apply_baseline(world: &mut World) -> Result<(), LoadError> {
    let Some(baseline) = world.get_resource::<NewGameBaseline>().cloned() else { return Err(LoadError::NotFound) };
    let providers = world.remove_resource::<SaveSections>().unwrap_or_default();
    let result = decode(&providers, baseline).and_then(|values| apply_with(world, &providers, values, Vec::new()));
    world.insert_resource(providers);
    if let Err(e) = result {
        if e.partly_loaded() {
            world.resource_mut::<SaveGate>().block(LOAD_FAILED);
        }
        return Err(e);
    }
    if let Some(mut gate) = world.get_resource_mut::<SaveGate>() {
        gate.unblock(LOAD_FAILED);
    }
    world.insert_resource(SaveSession::default());
    if let Some(mut auto) = world.get_resource_mut::<Autosave>() {
        auto.elapsed = std::time::Duration::ZERO;
    }
    Ok(())
}

/// The load's decode step (`LoadJob::load`) over the baseline's entries.
fn decode(providers: &SaveSections, baseline: NewGameBaseline) -> Result<Vec<(usize, SectionKey, crate::Decoded)>, LoadError> {
    let mut values = Vec::with_capacity(baseline.entries.len());
    for e in baseline.entries {
        let Some(spec) = providers.registry.spec(&e.key) else {
            log::warn!("sf_save_bevy: new-game baseline: no provider for {} any more; skipped", e.key);
            continue;
        };
        let (raw, _) = providers.registry.migrate(&e.key, e.version, e.bytes)?;
        let index = providers.sections.iter().position(|s: &SaveSection| s.spec.family() == spec.family()).expect("every registered spec has a provider");
        let value = providers.sections[index].io.decode(&raw).map_err(|message| LoadError::Decode { key: e.key.clone(), message })?;
        values.push((index, e.key, value));
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::Duration;

    use bevy_app::App;

    use super::*;
    use crate::{SaveAppExt, SaveConfig, SfSavePlugin, read, write};

    #[derive(Resource, Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Quests(Vec<String>);

    fn key() -> SectionKey {
        SectionKey::new("test", "baseline").unwrap()
    }

    #[test]
    fn new_game_restores_the_baseline_and_starts_a_new_session() {
        let mut app = App::new();
        let config = SaveConfig { root: PathBuf::from("unused"), background: false, ..SaveConfig::default() };
        app.add_plugins(SfSavePlugin { config })
            .insert_resource(Quests(vec!["pristine".into()]))
            .add_save_section(SaveSection::value(key(), 1, read(|q: &Quests| q.clone()), write(|q: &mut Quests, v| *q = v)));
        let world = app.world_mut();
        assert_eq!(apply_baseline(world), Err(LoadError::NotFound), "nothing captured yet");

        capture_baseline(world);
        assert_eq!(world.resource::<NewGameBaseline>().sections().count(), 1);
        // The old playthrough: state moved on, a saved world, a file to quicksave over.
        world.resource_mut::<Quests>().0.push("finished".into());
        *world.resource_mut::<SaveSession>() = SaveSession {
            world_id: 42,
            last_save: Some(7),
            sequence: 3,
            play_time: Duration::from_secs(600),
            sim_tick: 36_000,
            last_path: Some(PathBuf::from("saves/old/0003.sfsave")),
        };
        world.resource_mut::<Autosave>().elapsed = Duration::from_secs(90);
        // A second capture keeps the first (the front end is entered again after a game).
        capture_baseline(world);

        apply_baseline(world).unwrap();
        assert_eq!(*world.resource::<Quests>(), Quests(vec!["pristine".into()]));
        let session = world.resource::<SaveSession>();
        assert_eq!(session.last_path, None);
        assert_ne!(session.world_id, 42);
        assert_eq!(*session, SaveSession::default());
        assert_eq!(world.resource::<Autosave>().elapsed, Duration::ZERO);
    }
}
