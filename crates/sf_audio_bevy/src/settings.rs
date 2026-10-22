//! The settings bridge (`settings` feature): the framework's built-in volume settings
//! (`sf.audio_<bus>_volume`, `sf.audio_<bus>_muted`, `sf_ui`'s audio page) become
//! [`AudioVolumes`] whenever the settings store changes. The store itself is saved to the player
//! profile by `sf_ui_bevy`, so a slider moved in the options menu is heard within the frame and
//! kept across launches. Behind a feature so the audio plugin builds and tests without the UI
//! stack; the app turns it on and adds [`SettingsAudioPlugin`] next to `SfUiPlugin`.

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use sf_audio::{Bus, VolumeSettings};
use sf_ui::{SettingsStore, audio_muted_key, audio_volume_key};
use sf_ui_bevy::UiSettingsStore;

use crate::{AudioSystems, AudioVolumes};

/// Feeds the volume settings to the audio plugin (add both).
pub struct SettingsAudioPlugin;

impl Plugin for SettingsAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioVolumes>().add_systems(Update, volumes_from_settings.before(AudioSystems));
    }
}

/// The volume settings in `store` (a missing or mistyped entry keeps the full, unmuted default).
pub fn volumes_from_store(store: &SettingsStore) -> VolumeSettings {
    let mut out = VolumeSettings::default();
    for bus in Bus::USER {
        if let Some(level) = store.get_f64(&audio_volume_key(bus.name())) {
            #[expect(clippy::cast_possible_truncation, reason = "a 0..1 slider position")]
            out.set_level(bus, level as f32);
        }
        if let Some(muted) = store.get_bool(&audio_muted_key(bus.name())) {
            out.set_muted(bus, muted);
        }
    }
    out
}

/// Copies the store's volumes into [`AudioVolumes`] when the store changed (or appeared).
pub fn volumes_from_settings(store: Option<Res<UiSettingsStore>>, mut volumes: ResMut<AudioVolumes>) {
    let Some(store) = store.filter(DetectChanges::is_changed) else { return };
    let next = volumes_from_store(&store.0);
    if volumes.0 != next {
        volumes.0 = next;
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::float_cmp, reason = "gains of exact slider positions")]

    use std::time::Duration;

    use bevy_app::App;
    use bevy_time::{TimePlugin, TimeUpdateStrategy};
    use sf_audio::{SoundBank, slider_to_gain};
    use sf_records::RecordDb;
    use sf_ui::{AUDIO_BUSES, SettingValue, UiRegistry};

    use super::*;
    use crate::{AudioMixer, SfAudioPlugin};

    #[test]
    fn every_menu_bus_has_its_settings() {
        let names: Vec<&str> = Bus::USER.iter().map(|b| b.name()).collect();
        assert_eq!(names, AUDIO_BUSES, "sf_ui's audio page lists sf_audio's menu buses");
        let (registry, _) = UiRegistry::build(&RecordDb::new());
        for bus in Bus::USER {
            assert!(registry.setting(&audio_volume_key(bus.name())).is_some(), "{bus:?}");
            assert!(registry.setting(&audio_muted_key(bus.name())).is_some(), "{bus:?}");
        }
    }

    #[test]
    fn settings_drive_the_mixer_within_the_frame() {
        let mut app = App::new();
        app.add_plugins(TimePlugin)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(16)))
            .add_plugins((SfAudioPlugin { bank: SoundBank::default(), playback: false }, SettingsAudioPlugin));
        let (registry, _) = UiRegistry::build(&RecordDb::new());
        app.insert_resource(UiSettingsStore(SettingsStore::new(&registry)));
        app.update();
        let gain = |app: &App, bus| app.world().resource::<AudioMixer>().0.bus_gain(bus);
        assert_eq!(gain(&app, Bus::Music), 1.0);

        let set = |app: &mut App, key: String, value| {
            app.world_mut().resource_mut::<UiSettingsStore>().0.set(&key, value).unwrap();
        };
        set(&mut app, audio_volume_key("music"), SettingValue::Float(0.5));
        set(&mut app, audio_muted_key("sfx"), SettingValue::Bool(true));
        set(&mut app, audio_volume_key("ambience"), SettingValue::Float(0.75));
        app.update();
        assert_eq!(gain(&app, Bus::Music), slider_to_gain(0.5));
        assert_eq!(gain(&app, Bus::Sfx), 0.0);
        assert_eq!(gain(&app, Bus::Weather), slider_to_gain(0.75), "the weather follows the ambience slider");
        assert_eq!(gain(&app, Bus::Ui), 1.0);

        set(&mut app, audio_muted_key("sfx"), SettingValue::Bool(false));
        set(&mut app, audio_volume_key("master"), SettingValue::Float(0.0));
        app.update();
        assert!(Bus::ALL.iter().all(|b| gain(&app, *b) == 0.0), "master at the bottom is silence");
        assert_eq!(app.world().resource::<AudioVolumes>().0.get(Bus::Music).level, 0.5, "levels kept");
    }
}
