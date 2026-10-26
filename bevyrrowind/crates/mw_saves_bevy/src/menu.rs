//! Saves, part 2 (ROADMAP 2.5): the framework's save/load screen (`sf_ui_screens::saveload`)
//! over the current world's save folder.
//!
//! - **Open**: [`OpenSaveLoad`] (the pause menu's Save and Load entries). The screen ("saveload",
//!   Menu layer, pauses) goes on top of the pause menu; its Back returns there.
//! - **Listing**: one [`SaveEntryView`] per file of the world's folder (`SlotDir::list`, meta-only
//!   reads): id = file name, order = sequence, title = the place (`sf/meta` `location_label`),
//!   subtitle = in-game date, level, time played and the date saved, reason label for quick and
//!   auto saves, `damaged` when the meta read failed. A fresh run (no world yet) lists every
//!   world's saves in Load mode (ids `<world folder>/<file>`, newest by wall clock), as quickload
//!   takes the newest of any world. Listed again after a save or a delete.
//! - **Names**: `sf/meta` has no save-name field, so a save's title is its place; the generated
//!   name (`SaveLoadModel::new_name`, typed names wait for router text entry) is `OpenMW`'s default,
//!   "Day 24 - 16 Last Seed 7 p.m.", and shows in the subtitle of every save as its in-game date.
//! - **Choices**: New → a manual save in the slot folder; Overwrite → that file
//!   (`SaveTarget::Path`, keeps `.bak`); Load → that file (the load's reset closes the menus, the
//!   clock runs again, as quickload); Delete → the file and its `.bak`, then the list again;
//!   Back → the pause menu. A save while the `SaveGate` is closed (a load, a door fade) is not
//!   queued: a notice says so.
//! - **Place label**: [`SaveDisplay`] is kept current here: an interior's cell name (records)
//!   or "Vvardenfell" outside (until the region or town under the player), and the clock.
//!
//! Ported from `OpenMW` (GPL-3.0-or-later), `apps/openmw/mwgui/savegamedialog.cpp` at
//! 4e718b500ffee6657db90297d7b8a08a57d439bc: the default save name (`SaveGameDialog::onOpen`), the
//! info fields and their order (`onSlotSelected`: level, in-game date and hour with a.m./p.m.,
//! time played, file date `%Y.%m.%d`) and `formatTimeplayed`. Month lengths and the start date
//! (16 Last Seed) are Morrowind's calendar (`OpenMW` `mwworld/datetimemanager.cpp`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_log::prelude::*;
use sf_records::RecordDb;
use sf_save_bevy::sf_save::{SaveMeta, SaveReason, SlotDir, SlotEntry, list_worlds, with_suffix};
use sf_save_bevy::{LoadRequest, SaveConfig, SaveDisplay, SaveDone, SaveGate, SaveRequest, SaveSession, SaveStorage};
use sf_sky_bevy::SkyTime;
use sf_ui::{FluentArgs, FluentValue, Localizer, UiRegistry};
use sf_ui_bevy::{ScreenStackRes, UiLocalizer, UiRegistryRes, UiScreenRoot};
use sf_ui_screens::{SaveEntryView, SaveLoadChoice, SaveLoadEvent, SaveLoadMode, SaveLoadModel, SaveLoadScreen, ScreenSystems};
use sf_world::WorldspaceId;
use sf_world_bevy::{ActiveWorldspace, FadeTransition};

use mw_rpg_bevy::inventory::ItemRecords;

/// The save/load screen's id.
pub const SAVELOAD: &str = "saveload";

const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "saveload"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Template = "saveload" } }
"#;

/// Opens the save/load screen in this mode (the pause menu's Save and Load).
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenSaveLoad(pub SaveLoadMode);

/// The open screen's save ids and their files.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct SaveList(pub BTreeMap<String, PathBuf>);

/// After `SfSavePlugin` and the UI registry (`dialogue::add`); the app and the headless tests.
pub fn add(app: &mut App) {
    app.add_message::<OpenSaveLoad>().init_resource::<SaveList>();
    if let Some(mut registry) = app.world_mut().get_resource_mut::<UiRegistryRes>() {
        registry.0.extend_screens(&screens());
    }
    app.add_systems(
        Update,
        (display, open, host, relist, close_screen).chain().after(ScreenSystems::Input).run_if(resource_exists::<UiRegistryRes>),
    );
}

fn screens() -> UiRegistry {
    let mut db = RecordDb::new();
    sf_ui::register_kinds(&mut db).expect("UI record kinds register");
    let base = db.add_source("bevyrrowind", &[]);
    let outcome = sf_records::apply_record_file(&mut db, base, SCREENS, sf_records::RecordFormat::Toml).expect("saveload screen");
    assert!(outcome.errors.is_empty(), "saveload screen: {:?}", outcome.errors);
    UiRegistry::build(&db).0
}

/// What the listing reads.
#[derive(SystemParam)]
struct Folder<'w> {
    storage: Res<'w, SaveStorage>,
    config: Res<'w, SaveConfig>,
    session: Res<'w, SaveSession>,
    display: Res<'w, SaveDisplay>,
    localizer: Option<Res<'w, UiLocalizer>>,
}

impl Folder<'_> {
    fn strings(&self) -> Option<&Localizer> {
        self.localizer.as_deref().map(|l| &l.0)
    }

    /// The screen's model and its ids' files.
    fn list(&self, mode: SaveLoadMode) -> (SaveLoadModel, SaveList) {
        let fs = &*self.storage.0;
        let l = self.strings();
        let mut files = BTreeMap::new();
        let mut entries = Vec::new();
        if self.session.world_id != 0 {
            for e in SlotDir::world(&self.config.root, self.config.mode, self.session.world_id).list(fs) {
                let id = file_name(&e.path);
                entries.push(entry_view(l, id.clone(), e.sequence, &e));
                files.insert(id, e.path);
            }
        } else if mode == SaveLoadMode::Load {
            for (_, dir) in list_worlds(fs, &self.config.root, self.config.mode) {
                let folder = file_name(dir.dir());
                for e in dir.list(fs) {
                    let id = format!("{folder}/{}", file_name(&e.path));
                    let order = e.meta.as_ref().map_or(0, |m| u64::try_from(m.wall_clock).unwrap_or(0));
                    entries.push(entry_view(l, id.clone(), order, &e));
                    files.insert(id, e.path);
                }
            }
        }
        let model = SaveLoadModel { mode, entries, can_delete: true, new_name: default_name(l, self.display.game_days) };
        (model, SaveList(files))
    }
}

fn file_name(path: &Path) -> String {
    path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
}

/// One listed save.
fn entry_view(l: Option<&Localizer>, id: String, order: u64, e: &SlotEntry) -> SaveEntryView {
    let mut view = SaveEntryView::new(id, order, "");
    view.reason_label = match e.reason {
        SaveReason::Manual => String::new(),
        reason => tr(l, &format!("saves-reason-{}", reason.token()), &[]),
    };
    match &e.meta {
        Ok(meta) => {
            view.title = if meta.location_label.is_empty() {
                tr(l, "saves-untitled", &[("n", FluentValue::from(e.sequence))])
            } else {
                meta.location_label.clone()
            };
            view.subtitle = subtitle(l, meta);
        }
        Err(why) => {
            view.title = tr(l, "saves-damaged", &[]);
            view.subtitle = file_name(&e.path);
            view.damaged = true;
            warn!("saves: {}: {why}", e.path.display());
        }
    }
    view
}

/// `OpenMW`'s info fields: in-game date, level, time played, the date saved.
fn subtitle(l: Option<&Localizer>, meta: &SaveMeta) -> String {
    let mut parts = vec![default_name(l, meta.game_days)];
    if let Some(level) = meta.character_level {
        parts.push(tr(l, "saves-level", &[("level", FluentValue::from(level))]));
    }
    if meta.play_time_ms > 0 {
        parts.push(tr(l, "saves-played", &[("time", FluentValue::from(time_played(l, meta.play_time_ms / 1000)))]));
    }
    parts.push(tr(l, "saves-stamp", &[("date", FluentValue::from(utc_stamp(meta.wall_clock)))]));
    parts.join(" · ")
}

/// `key` with `args`; the key itself without strings (headless).
fn tr(l: Option<&Localizer>, key: &str, args: &[(&str, FluentValue<'static>)]) -> String {
    let Some(l) = l else { return key.to_owned() };
    let mut fluent = FluentArgs::new();
    for (k, v) in args {
        fluent.set(*k, v.clone());
    }
    l.format(key, Some(&fluent)).into_owned()
}

/// `OpenMW` `formatTimeplayed`: hours, minutes, seconds, each only when non-zero.
fn time_played(l: Option<&Localizer>, seconds: u64) -> String {
    if seconds == 0 {
        return tr(l, "saves-duration-second", &[("n", FluentValue::from(0))]);
    }
    let (h, m, s) = (seconds / 3600, (seconds / 60) % 60, seconds % 60);
    [("saves-duration-hour", h), ("saves-duration-minute", m), ("saves-duration-second", s)]
        .into_iter()
        .filter(|(_, n)| *n > 0)
        .map(|(key, n)| tr(l, key, &[("n", FluentValue::from(n))]))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Morrowind's month lengths (Morning Star first).
const MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
/// Day 0 of the clock: 16 Last Seed (month 7), the game's start.
const START: (u32, usize) = (16, 7);

/// The in-game date of `days` (clock days since the start): (days passed from 1, day of month,
/// month index, hour 0-23).
fn calendar(days: f64) -> (u64, u32, usize, u32) {
    let days = days.max(0.0);
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // clamped, game days
    let (whole, hour) = (days.floor() as u64, ((days.fract() * 24.0).floor() as u32).min(23));
    let (mut day, mut month) = START;
    let mut left = whole;
    while left > 0 {
        let step = u64::from(MONTH_DAYS[month] - day + 1).min(left);
        left -= step;
        #[allow(clippy::cast_possible_truncation)] // at most a month's days
        let step = step as u32;
        day += step;
        if day > MONTH_DAYS[month] {
            day = 1;
            month = (month + 1) % 12;
        }
    }
    (whole + 1, day, month, hour)
}

/// `OpenMW`'s default save name: "Day 24 - 16 Last Seed 7 p.m.".
fn default_name(l: Option<&Localizer>, days: f64) -> String {
    let (passed, date, month, hour) = calendar(days);
    let half = if hour >= 12 { "saves-pm" } else { "saves-am" };
    let hour12 = match hour {
        0 => 12,
        13.. => hour - 12,
        h => h,
    };
    tr(
        l,
        "saves-default-name",
        &[
            ("day", FluentValue::from(passed)),
            ("date", FluentValue::from(date)),
            ("month", FluentValue::from(tr(l, &format!("mw-month-{month}"), &[]))),
            ("hour", FluentValue::from(hour12)),
            ("half", FluentValue::from(tr(l, half, &[]))),
        ],
    )
}

/// `%Y.%m.%d %H:%M` in UTC (`OpenMW` shows local time; TODO(saves): the local zone).
fn utc_stamp(unix: i64) -> String {
    let (days, secs) = (unix.div_euclid(86_400), unix.rem_euclid(86_400));
    // Civil from days (Howard Hinnant's `civil_from_days`, public domain).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}.{m:02}.{d:02} {:02}:{:02}", secs / 3600, (secs / 60) % 60)
}

/// [`SaveDisplay`] follows the place and the clock.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn display(
    active: Option<Res<ActiveWorldspace>>,
    records: Option<Res<ItemRecords>>,
    sky: Option<Res<SkyTime>>,
    localizer: Option<Res<UiLocalizer>>,
    mut display: ResMut<SaveDisplay>,
    mut labelled: Local<Option<WorldspaceId>>,
) {
    let here = active.and_then(|a| a.0.clone());
    if *labelled != here {
        let label = here.as_ref().map_or_else(String::new, |ws| place_label(ws, records.as_deref().map(|r| &*r.0), localizer.as_deref().map(|l| &l.0)));
        display.location_label = label;
        *labelled = here;
    }
    if let Some(sky) = sky
        && display.game_days.to_bits() != sky.0.days.to_bits()
    {
        display.game_days = sky.0.days;
    }
}

/// An interior's cell name as the records spell it; the exterior's label (TODO(saves): the
/// region or town under the player, `OpenMW`'s `mPlayerCellName`); else the id.
fn place_label(ws: &WorldspaceId, records: Option<&RecordDb>, l: Option<&Localizer>) -> String {
    if *ws == mw_world::ids::exterior() {
        return tr(l, "saves-place-exterior", &[]);
    }
    records.and_then(|db| mw_world::find_cell(db, ws.as_str())).map_or_else(|| ws.to_string(), |cell| cell.name.clone())
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn open(
    mut commands: Commands,
    mut opens: MessageReader<OpenSaveLoad>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    folder: Folder,
    mut list: ResMut<SaveList>,
) {
    for OpenSaveLoad(mode) in opens.read() {
        if stack.0.is_open(SAVELOAD) {
            continue;
        }
        let (model, files) = folder.list(*mode);
        *list = files;
        commands.spawn((UiScreenRoot(SAVELOAD.into()), SaveLoadScreen::new(model)));
        if let Err(e) = stack.0.push(&registry.0, SAVELOAD) {
            warn!("save/load screen: {e:?}");
        }
    }
}

/// The player's choices, as save and load requests.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn host(
    mut choices: MessageReader<SaveLoadEvent>,
    folder: Folder,
    gate: Res<SaveGate>,
    fade: Option<Res<FadeTransition>>,
    mut list: ResMut<SaveList>,
    mut screens: Query<&mut SaveLoadScreen>,
    mut saves: MessageWriter<SaveRequest>,
    mut loads: MessageWriter<LoadRequest>,
    mut toasts: Option<ResMut<sf_ui_bevy::Toasts>>,
) {
    let l = folder.strings();
    for event in choices.read() {
        let file = |id: &str| list.0.get(id).cloned();
        match &event.choice {
            SaveLoadChoice::New(_) | SaveLoadChoice::Overwrite(_) if gate.is_blocked() => {
                crate::notice(toasts.as_deref_mut(), &tr(l, "saves-cant-save", &[]));
            }
            SaveLoadChoice::New(name) => {
                info!("saves: new save ({name}; titled by its place, sf/meta has no name)");
                saves.write(SaveRequest::slot(SaveReason::Manual));
            }
            SaveLoadChoice::Overwrite(id) => {
                if let Some(path) = file(id) {
                    saves.write(SaveRequest::to_path(path));
                } else {
                    warn!("saves: no file for {id}");
                }
            }
            SaveLoadChoice::Load(_) if fade.as_ref().is_some_and(|f| f.is_active()) => {
                crate::notice(toasts.as_deref_mut(), &tr(l, "saves-cant-load", &[]));
            }
            SaveLoadChoice::Load(id) => {
                if let Some(path) = file(id) {
                    loads.write(LoadRequest::path(path));
                } else {
                    warn!("saves: no file for {id}");
                }
            }
            SaveLoadChoice::Delete(id) => {
                let Some(path) = file(id) else {
                    warn!("saves: no file for {id}");
                    continue;
                };
                let fs = &*folder.storage.0;
                match fs.remove(&path) {
                    Ok(()) => info!("saves: deleted {}", path.display()),
                    Err(e) => crate::notice(toasts.as_deref_mut(), &tr(l, "saves-delete-failed", &[("error", FluentValue::from(e.to_string()))])),
                }
                let bak = with_suffix(&path, "bak");
                if fs.exists(&bak) && fs.remove(&bak).is_err() {
                    warn!("saves: {} not removed", bak.display());
                }
                refill(&folder, &mut list, &mut screens);
            }
            // The screen has closed; the pause menu is under it.
            SaveLoadChoice::Back => {}
        }
    }
}

/// The open screen's list again.
fn refill(folder: &Folder, list: &mut SaveList, screens: &mut Query<&mut SaveLoadScreen>) {
    for mut screen in screens.iter_mut() {
        let (model, files) = folder.list(screen.model.mode);
        *list = files;
        if screen.model != model {
            screen.model = model;
        }
    }
}

/// A finished save (the new file, the `.bak`, a rotation) lists the folder again.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn relist(mut saved: MessageReader<SaveDone>, folder: Folder, mut list: ResMut<SaveList>, mut screens: Query<&mut SaveLoadScreen>) {
    if saved.read().count() > 0 {
        refill(&folder, &mut list, &mut screens);
    }
}

/// The screen's root despawns once off the stack.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn close_screen(mut commands: Commands, stack: Res<ScreenStackRes>, roots: Query<(Entity, &UiScreenRoot)>) {
    for (e, root) in &roots {
        if root.0.as_str() == SAVELOAD && !stack.0.is_open(SAVELOAD) {
            commands.entity(e).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendar_counts_from_16_last_seed() {
        assert_eq!(calendar(0.375), (1, 16, 7, 9), "the start: 9 a.m.");
        assert_eq!(calendar(15.0), (16, 31, 7, 0));
        assert_eq!(calendar(16.5), (17, 1, 8, 12), "into Hearthfire");
        // 16 Last Seed + 365 days = 16 Last Seed again.
        assert_eq!(calendar(365.0), (366, 16, 7, 0));
    }

    #[test]
    fn default_name_and_fields_without_strings() {
        // Keys stand in for text without a localizer.
        assert_eq!(default_name(None, 0.8), "saves-default-name");
        assert_eq!(time_played(None, 3725), "saves-duration-hour saves-duration-minute saves-duration-second");
        assert_eq!(utc_stamp(0), "1970.01.01 00:00");
        assert_eq!(utc_stamp(1_793_017_500), "2026.10.26 12:25");
    }
}
