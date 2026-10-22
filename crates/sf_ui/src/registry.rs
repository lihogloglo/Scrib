//! The UI registry: UI records resolved once per mod set (ui.md "Interfaces").

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use indexmap::IndexMap;
use sf_records::{Op, RecordDb, SourceId};

use crate::ids::{ColumnId, ListId, ModId, PresetId, ScreenId, SettingKey, SlotId, ThemeId};
use crate::records::{
    ColumnDef, PresetDef, ScreenDef, SettingDef, SettingKind, SlotContribution, ThemeDef,
};

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
}

impl UiConflict {
    pub fn severity(&self) -> Severity {
        match self {
            UiConflict::ScreenReplaced { .. } => Severity::Info,
            UiConflict::UnknownSlot { .. } | UiConflict::UnknownPreset { .. } => Severity::Warning,
            UiConflict::InvalidSetting { .. } => Severity::Error,
        }
    }

    /// The mod the finding is about.
    pub fn owner(&self) -> &ModId {
        match self {
            UiConflict::ScreenReplaced { by: owner, .. }
            | UiConflict::UnknownSlot { owner, .. }
            | UiConflict::UnknownPreset { owner, .. }
            | UiConflict::InvalidSetting { owner, .. } => owner,
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

    pub fn screen(&self, id: &str) -> Option<&Def<ScreenId, ScreenDef>> {
        self.screens.get(id)
    }

    pub fn screens(&self) -> impl Iterator<Item = &Def<ScreenId, ScreenDef>> {
        self.screens.values()
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
        SettingKind::Bool { .. } => Ok(()),
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
        } => {
            if !(min.is_finite() && max.is_finite() && default.is_finite()) || min > max {
                Err(format!("bad range {min}..={max}"))
            } else if !(min..=max).contains(&default) {
                Err(format!("default {default} outside {min}..={max}"))
            } else if step.is_some_and(|s| s.is_nan() || s <= 0.0) {
                Err("step not positive".into())
            } else {
                Ok(())
            }
        }
        SettingKind::Choice { default, options } => {
            if options.contains(default) {
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
