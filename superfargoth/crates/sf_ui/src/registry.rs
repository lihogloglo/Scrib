//! The UI registry: UI records resolved once per mod set (ui.md "Interfaces").

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use indexmap::IndexMap;
use sf_records::{Op, RecordDb, SourceId, paths_overlap};

use crate::ids::{ColumnId, ListId, ModId, PresetId, ScreenId, SettingKey, SlotId, ThemeId};
use crate::records::{
    ColumnDef, PresetDef, ScreenDef, SettingDef, SettingKind, SettingPresetDef, SlotContribution,
    ThemeDef,
};
use crate::presets::CUSTOM_LEVEL;
use crate::settings::{SettingValue, coerce};

/// Mod id of the built-in definitions (presets and settings the framework ships).
pub const FRAMEWORK_MOD: &str = "sf";

/// A resolved record: id, owning mod, load position, and value.
#[derive(Clone, Debug, PartialEq)]
pub struct Def<I, T> {
    pub id: I,
    /// The mod whose define the current value is based on.
    pub owner: ModId,
    /// Load position of the owner (`None` for built-ins, which load first).
    pub source: Option<SourceId>,
    pub def: T,
}

/// How serious a [`UiConflict`] is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Expected moddability (a screen replaced), listed in the mod manager.
    Info,
    /// Content that does nothing (a contribution to a slot no screen offers).
    Warning,
    /// Content that was dropped.
    Error,
}

/// One finding of [`UiRegistry::build`], attributed to a mod.
#[derive(Clone, Debug, PartialEq)]
pub enum UiConflict {
    /// Several mods defined the screen; the last one wins.
    ScreenReplaced {
        screen: ScreenId,
        by: ModId,
        replaced: Vec<ModId>,
    },
    /// A slot contribution names a slot no screen offers.
    UnknownSlot {
        contribution: String,
        slot: SlotId,
        owner: ModId,
    },
    /// A screen or theme lists a preset that doesn't exist.
    UnknownPreset {
        record: String,
        preset: PresetId,
        owner: ModId,
    },
    /// A setting's default or range is invalid; the setting was dropped.
    InvalidSetting {
        key: SettingKey,
        owner: ModId,
        reason: String,
    },
    /// A `ui.setting_preset` is invalid: with `level` and `key`, that one
    /// value was dropped (owner: the mod that wrote it); without, the whole
    /// preset was (a missing or non-Choice driver).
    InvalidSettingPreset {
        preset: String,
        level: Option<String>,
        key: Option<SettingKey>,
        owner: ModId,
        reason: String,
    },
}

impl UiConflict {
    pub fn severity(&self) -> Severity {
        match self {
            UiConflict::ScreenReplaced { .. } => Severity::Info,
            UiConflict::UnknownSlot { .. } | UiConflict::UnknownPreset { .. } => Severity::Warning,
            UiConflict::InvalidSetting { .. } | UiConflict::InvalidSettingPreset { .. } => {
                Severity::Error
            }
        }
    }

    /// The mod the finding is about.
    pub fn owner(&self) -> &ModId {
        match self {
            UiConflict::ScreenReplaced { by: owner, .. }
            | UiConflict::UnknownSlot { owner, .. }
            | UiConflict::UnknownPreset { owner, .. }
            | UiConflict::InvalidSetting { owner, .. }
            | UiConflict::InvalidSettingPreset { owner, .. } => owner,
        }
    }
}

impl fmt::Display for UiConflict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiConflict::ScreenReplaced {
                screen,
                by,
                replaced,
            } => {
                let replaced: Vec<&str> = replaced.iter().map(ModId::as_str).collect();
                write!(
                    f,
                    "screen `{screen}` from {} replaced by {by}",
                    replaced.join(", ")
                )
            }
            UiConflict::UnknownSlot {
                contribution,
                slot,
                owner,
            } => {
                write!(
                    f,
                    "{owner}: slot contribution `{contribution}` targets unknown slot `{slot}`"
                )
            }
            UiConflict::UnknownPreset {
                record,
                preset,
                owner,
            } => {
                write!(f, "{owner}: `{record}` lists unknown preset `{preset}`")
            }
            UiConflict::InvalidSetting { key, owner, reason } => {
                write!(f, "{owner}: setting `{key}` dropped: {reason}")
            }
            UiConflict::InvalidSettingPreset {
                preset,
                level,
                key,
                owner,
                reason,
            } => match (level, key) {
                (Some(level), Some(key)) => write!(
                    f,
                    "{owner}: setting preset `{preset}` level `{level}`: `{key}` dropped: {reason}"
                ),
                _ => write!(f, "{owner}: setting preset `{preset}` dropped: {reason}"),
            },
        }
    }
}

/// Screens, slots, columns, themes, presets and settings of one mod set.
#[derive(Clone, Debug, Default)]
pub struct UiRegistry {
    screens: IndexMap<ScreenId, Def<ScreenId, ScreenDef>>,
    slots: BTreeMap<SlotId, Vec<Def<String, SlotContribution>>>,
    columns: BTreeMap<ListId, Vec<Def<ColumnId, ColumnDef>>>,
    themes: IndexMap<ThemeId, Def<ThemeId, ThemeDef>>,
    presets: IndexMap<PresetId, Def<PresetId, PresetDef>>,
    settings: IndexMap<SettingKey, Def<SettingKey, SettingDef>>,
    setting_presets: IndexMap<String, Def<String, SettingPresetDef>>,
}

impl UiRegistry {
    /// Builds the registry from the record database after every mod loaded.
    /// Record sources are mods in load order (the mod pipeline adds one per mod).
    /// Kinds that were never registered are treated as empty.
    pub fn build(db: &RecordDb) -> (Self, Vec<UiConflict>) {
        let mut conflicts = Vec::new();
        let mut reg = UiRegistry::default();

        for (id, def) in PresetDef::builtins() {
            let id = PresetId::from(id);
            reg.presets.insert(
                id.clone(),
                Def {
                    id,
                    owner: ModId::from(FRAMEWORK_MOD),
                    source: None,
                    def,
                },
            );
        }
        for d in collect::<PresetDef, _>(db, PresetId::new) {
            reg.presets.insert(d.id.clone(), d);
        }

        for d in collect::<ScreenDef, _>(db, ScreenId::new) {
            let defines = define_sources(db, &d.id.0);
            if defines.len() > 1 {
                conflicts.push(UiConflict::ScreenReplaced {
                    screen: d.id.clone(),
                    by: d.owner.clone(),
                    replaced: defines[..defines.len() - 1]
                        .iter()
                        .map(|s| mod_of(db, *s))
                        .collect(),
                });
            }
            for preset in d
                .def
                .presets
                .iter()
                .filter(|p| !reg.presets.contains_key(*p))
            {
                conflicts.push(UiConflict::UnknownPreset {
                    record: d.id.0.clone(),
                    preset: preset.clone(),
                    owner: d.owner.clone(),
                });
            }
            reg.screens.insert(d.id.clone(), d);
        }

        for d in collect::<ThemeDef, _>(db, ThemeId::new) {
            for preset in d
                .def
                .presets
                .iter()
                .filter(|p| !reg.presets.contains_key(*p))
            {
                conflicts.push(UiConflict::UnknownPreset {
                    record: d.id.0.clone(),
                    preset: preset.clone(),
                    owner: d.owner.clone(),
                });
            }
            reg.themes.insert(d.id.clone(), d);
        }

        let offered: BTreeSet<&SlotId> = reg.screens.values().flat_map(|s| &s.def.slots).collect();
        let mut slots: BTreeMap<SlotId, Vec<_>> = BTreeMap::new();
        for d in collect::<SlotContribution, _>(db, String::from) {
            if !offered.contains(&d.def.slot) {
                conflicts.push(UiConflict::UnknownSlot {
                    contribution: d.id.clone(),
                    slot: d.def.slot.clone(),
                    owner: d.owner.clone(),
                });
            }
            slots.entry(d.def.slot.clone()).or_default().push(d);
        }
        for list in slots.values_mut() {
            list.sort_by_key(|d| (d.def.priority, d.source));
        }
        reg.slots = slots;

        for d in collect::<ColumnDef, _>(db, ColumnId::new) {
            reg.columns.entry(d.def.list.clone()).or_default().push(d);
        }
        for list in reg.columns.values_mut() {
            list.sort_by_key(|d| (d.def.priority, d.source));
        }

        reg.add_settings(db, &mut conflicts);
        reg.add_setting_presets(db, &mut conflicts);

        (reg, conflicts)
    }

    /// Built-in settings, then `ui.setting` records (invalid ones dropped
    /// with an error; a record with a built-in's id retunes it).
    fn add_settings(&mut self, db: &RecordDb, conflicts: &mut Vec<UiConflict>) {
        for (id, def) in SettingDef::builtins() {
            self.settings.insert(
                id.clone(),
                Def {
                    id,
                    owner: ModId::from(FRAMEWORK_MOD),
                    source: None,
                    def,
                },
            );
        }
        for mut d in collect::<SettingDef, _>(db, SettingKey::new) {
            match validate_setting(&d.def.kind) {
                Ok(()) => {
                    // A mod retuning a built-in setting doesn't move its value out
                    // of `[settings.sf]`; `source` still names the mod.
                    if self.settings.get(&d.id).is_some_and(|b| b.source.is_none()) {
                        d.owner = ModId::from(FRAMEWORK_MOD);
                    }
                    self.settings.insert(d.id.clone(), d);
                }
                Err(reason) => conflicts.push(UiConflict::InvalidSetting {
                    key: d.id,
                    owner: d.owner,
                    reason,
                }),
            }
        }
    }

    /// `ui.setting_preset` records, checked against the settings: a missing or
    /// non-Choice driver (or one not offering every level and `custom`) drops
    /// the record; an unknown key or a value that doesn't fit drops that value,
    /// attributed to the mod that wrote it. Kept values are stored as the
    /// store would hold them (an int written for a float setting becomes a float).
    ///
    /// Records sharing a driver merge into the first one, in load order, a
    /// later record's value winning for a key: that is how a mod adds keys to a
    /// level (field-path patches can't, as setting keys contain dots).
    fn add_setting_presets(&mut self, db: &RecordDb, conflicts: &mut Vec<UiConflict>) {
        let mut records = collect::<SettingPresetDef, _>(db, String::from);
        records.sort_by_key(|d| d.source);
        for mut d in records {
            let preset_error = |owner: ModId, reason: String| UiConflict::InvalidSettingPreset {
                preset: d.id.clone(),
                level: None,
                key: None,
                owner,
                reason,
            };
            let driver = d.def.driver.clone();
            let options = match self.settings.get(driver.as_str()).map(|s| &s.def.kind) {
                None => {
                    conflicts.push(preset_error(
                        d.owner.clone(),
                        format!("driver `{driver}` is not a setting"),
                    ));
                    continue;
                }
                Some(SettingKind::Choice { options, .. }) => options,
                Some(_) => {
                    conflicts.push(preset_error(
                        d.owner.clone(),
                        format!("driver `{driver}` is not a Choice setting"),
                    ));
                    continue;
                }
            };
            if d.def.levels.contains_key(CUSTOM_LEVEL) {
                let owner = writer_of(db, &d, &format!("levels.{CUSTOM_LEVEL}"));
                conflicts.push(preset_error(
                    owner,
                    format!("level id `{CUSTOM_LEVEL}` is reserved"),
                ));
                continue;
            }
            let missing = std::iter::once(CUSTOM_LEVEL)
                .chain(d.def.levels.keys().map(String::as_str))
                .find(|o| !options.iter().any(|x| x == o));
            if let Some(missing) = missing {
                let owner = if missing == CUSTOM_LEVEL {
                    d.owner.clone()
                } else {
                    writer_of(db, &d, &format!("levels.{missing}"))
                };
                conflicts.push(preset_error(
                    owner,
                    format!("driver `{driver}` doesn't offer `{missing}`"),
                ));
                continue;
            }

            let mut levels = std::mem::take(&mut d.def.levels);
            for (level, values) in &mut levels {
                let checked = std::mem::take(values);
                for (key, value) in checked {
                    match self.preset_value(&driver, &key, value) {
                        Ok(value) => {
                            values.insert(key, value);
                        }
                        Err(reason) => conflicts.push(UiConflict::InvalidSettingPreset {
                            preset: d.id.clone(),
                            level: Some(level.clone()),
                            owner: writer_of(db, &d, &format!("levels.{level}.{key}")),
                            key: Some(SettingKey::new(key)),
                            reason,
                        }),
                    }
                }
            }
            d.def.levels = levels;
            let base = self
                .setting_presets
                .values_mut()
                .find(|p| p.def.driver == d.def.driver);
            match base {
                Some(base) => {
                    for (level, values) in d.def.levels {
                        base.def.levels.entry(level).or_default().extend(values);
                    }
                }
                None => {
                    self.setting_presets.insert(d.id.clone(), d);
                }
            }
        }
    }

    /// A preset level's value for `key`, as the store holds it, or why it
    /// doesn't fit the setting.
    fn preset_value(
        &self,
        driver: &str,
        key: &str,
        value: SettingValue,
    ) -> Result<SettingValue, String> {
        if key == driver {
            return Err("the driver can't be a level member".into());
        }
        let Some(setting) = self.settings.get(key) else {
            return Err("unknown setting".into());
        };
        let stored = coerce(key, &setting.def.kind, value.clone()).map_err(|e| e.to_string())?;
        let same = match (&value, &stored) {
            #[allow(clippy::cast_precision_loss)]
            (SettingValue::Int(n), SettingValue::Float(x))
            | (SettingValue::Float(x), SettingValue::Int(n)) => (*n as f64 - x).abs() == 0.0,
            (a, b) => a == b,
        };
        if same {
            Ok(stored)
        } else {
            Err(format!(
                "{} is out of range or off step (the setting would hold {})",
                show(&value),
                show(&stored)
            ))
        }
    }

    pub fn screen(&self, id: &str) -> Option<&Def<ScreenId, ScreenDef>> {
        self.screens.get(id)
    }

    pub fn screens(&self) -> impl Iterator<Item = &Def<ScreenId, ScreenDef>> {
        self.screens.values()
    }

    /// Adds `other`'s screens as a later source (same id: `other` wins). For hosts that
    /// assemble screens from several record sets before content-driven screens land.
    pub fn extend_screens(&mut self, other: &UiRegistry) {
        for (id, def) in &other.screens {
            self.screens.insert(id.clone(), def.clone());
        }
    }

    /// Contributions to a slot, ordered by priority, then load order.
    pub fn slot(&self, id: &str) -> &[Def<String, SlotContribution>] {
        self.slots.get(id).map_or(&[], Vec::as_slice)
    }

    /// Columns of a list, ordered by priority, then load order.
    pub fn columns(&self, list: &str) -> &[Def<ColumnId, ColumnDef>] {
        self.columns.get(list).map_or(&[], Vec::as_slice)
    }

    pub fn theme(&self, id: &str) -> Option<&Def<ThemeId, ThemeDef>> {
        self.themes.get(id)
    }

    pub fn themes(&self) -> impl Iterator<Item = &Def<ThemeId, ThemeDef>> {
        self.themes.values()
    }

    pub fn preset(&self, id: &str) -> Option<&Def<PresetId, PresetDef>> {
        self.presets.get(id)
    }

    pub fn presets(&self) -> impl Iterator<Item = &Def<PresetId, PresetDef>> {
        self.presets.values()
    }

    pub fn setting(&self, key: &str) -> Option<&Def<SettingKey, SettingDef>> {
        self.settings.get(key)
    }

    /// Settings in first-definition order.
    pub fn settings(&self) -> impl Iterator<Item = &Def<SettingKey, SettingDef>> {
        self.settings.values()
    }

    /// A valid setting preset by the id of its first record (the one later
    /// records sharing its driver merged into).
    pub fn setting_preset(&self, id: &str) -> Option<&Def<String, SettingPresetDef>> {
        self.setting_presets.get(id)
    }

    /// The setting preset a driver setting shows.
    pub fn setting_preset_for_driver(
        &self,
        driver: &str,
    ) -> Option<&Def<String, SettingPresetDef>> {
        self.setting_presets.values().find(|p| p.def.driver == driver)
    }

    /// Valid setting presets in load order (each merged with the later
    /// records sharing its driver).
    pub fn setting_presets(&self) -> impl Iterator<Item = &Def<String, SettingPresetDef>> {
        self.setting_presets.values()
    }
}

/// The mod that last wrote `path` of a record (the latest patch since the
/// current define whose fields cover it), else the mod that defined it.
fn writer_of(db: &RecordDb, d: &Def<String, SettingPresetDef>, path: &str) -> ModId {
    db.entry::<SettingPresetDef>(&d.id.as_str().into())
        .and_then(|entry| {
            entry
                .history
                .iter()
                .rev()
                .take_while(|t| t.op != Op::Define)
                .find(|t| {
                    t.fields
                        .as_ref()
                        .is_some_and(|f| f.iter().any(|p| paths_overlap(p, path)))
                })
        })
        .map_or_else(|| d.owner.clone(), |t| mod_of(db, t.source))
}

fn show(value: &SettingValue) -> String {
    match value {
        SettingValue::Bool(b) => b.to_string(),
        SettingValue::Int(n) => n.to_string(),
        SettingValue::Float(x) => x.to_string(),
        SettingValue::Text(s) => format!("`{s}`"),
    }
}

/// Live records of a kind as [`Def`]s, in first-definition order.
fn collect<T, I>(db: &RecordDb, make_id: impl Fn(String) -> I) -> Vec<Def<I, T>>
where
    T: Clone + Send + Sync + 'static,
{
    db.iter::<T>()
        .map(|(id, value)| {
            let entry = db.entry::<T>(id).expect("iterated records have entries");
            let source = entry
                .defined_by()
                .unwrap_or_else(|| entry.last_touched_by());
            Def {
                id: make_id(id.as_str().to_owned()),
                owner: mod_of(db, source),
                source: Some(source),
                def: value.clone(),
            }
        })
        .collect()
}

/// Sources that defined a screen record, oldest first.
fn define_sources(db: &RecordDb, id: &str) -> Vec<SourceId> {
    let Some(entry) = db.entry::<ScreenDef>(&id.into()) else {
        return Vec::new();
    };
    let mut out: Vec<SourceId> = Vec::new();
    for touch in entry.history.iter().filter(|t| t.op == Op::Define) {
        if !out.contains(&touch.source) {
            out.push(touch.source);
        }
    }
    out
}

fn mod_of(db: &RecordDb, source: SourceId) -> ModId {
    ModId::new(db.source(source).name.clone())
}

fn validate_setting(kind: &SettingKind) -> Result<(), String> {
    match kind {
        SettingKind::Bool { .. } | SettingKind::Action => Ok(()),
        SettingKind::Int {
            default,
            min,
            max,
            step,
        } => {
            if min > max {
                Err(format!("min {min} > max {max}"))
            } else if !(min..=max).contains(&default) {
                Err(format!("default {default} outside {min}..={max}"))
            } else if *step <= 0 {
                Err(format!("step {step} not positive"))
            } else {
                Ok(())
            }
        }
        SettingKind::Float {
            default,
            min,
            max,
            step,
            log_scale,
        } => {
            if !(min.is_finite() && max.is_finite() && default.is_finite()) || min > max {
                Err(format!("bad range {min}..={max}"))
            } else if !(min..=max).contains(&default) {
                Err(format!("default {default} outside {min}..={max}"))
            } else if step.is_some_and(|s| s.is_nan() || s <= 0.0) {
                Err("step not positive".into())
            } else if *log_scale && *min <= 0.0 {
                Err(format!("log scale needs min > 0, got {min}"))
            } else {
                Ok(())
            }
        }
        SettingKind::Choice {
            default,
            options,
            options_from,
        } => {
            // A provider's options are only known at run time.
            if options_from.is_some() || options.contains(default) {
                Ok(())
            } else {
                Err(format!("default `{default}` not among the options"))
            }
        }
        SettingKind::Text { default, max_len } => match max_len {
            Some(max) if default.chars().count() > *max => {
                Err(format!("default longer than {max}"))
            }
            _ => Ok(()),
        },
    }
}
