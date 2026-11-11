//! Settings registry values (ui.md "Settings registry", the MCM equivalent).
//!
//! `ui.setting` records give types, defaults and ranges; the profile stores
//! values per mod (`[settings.<mod>]` tables, `sf_mods::Profile::settings`).
//! The store keeps values for every registered setting, and passes through
//! profile entries it doesn't know (a mod disabled for now keeps its values).
//!
//! The framework's own settings are built in (owner `sf`, stored under
//! `[settings.sf]`): the audio page's per-bus volume sliders and mutes
//! ([`AUDIO_BUSES`], audio.md "Volume mixer").
//!
//! **Scope** (menus.md "Settings v2"): `Machine` settings (display, graphics)
//! are not in the profile but in a per-machine TOML file ([`SettingsStore::load_machine`],
//! [`SettingsStore::save_machine`]; `[<owner>] key = value`, the profile's
//! `[settings.<owner>]` tables at top level). `Save` settings stay in the
//! profile until saves carry settings.
//!
//! **Change log**: every write that changes a value ([`SettingsStore::set`],
//! `reset`, the loads, presets) is remembered per key with the value it had
//! before; [`SettingsStore::take_changes`] drains it as [`SettingChange`]s
//! (`sf_ui_bevy` turns them into `SettingChanged` messages once per frame).
//! A key changed and changed back before the drain reports nothing.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use indexmap::IndexMap;

use crate::ids::{ModId, SettingKey};
use crate::records::{SettingDef, SettingKind, SettingScope};
use crate::registry::{FRAMEWORK_MOD, UiRegistry};

/// Volume buses on the framework's audio settings page, in menu order
/// (the names `sf_audio::Bus::USER` uses).
pub const AUDIO_BUSES: [&str; 7] = ["master", "music", "sfx", "voice", "ambience", "ui", "movie"];

/// Key of a bus's volume slider (`sf.audio_music_volume`): a float 0..1, the
/// slider position (the audio side maps it to dB).
pub fn audio_volume_key(bus: &str) -> String {
    format!("{FRAMEWORK_MOD}.audio_{bus}_volume")
}

/// Key of a bus's mute toggle (`sf.audio_music_muted`).
pub fn audio_muted_key(bus: &str) -> String {
    format!("{FRAMEWORK_MOD}.audio_{bus}_muted")
}

/// Step of the volume sliders: 20 steps, a few presses on a gamepad.
pub const AUDIO_VOLUME_STEP: f64 = 0.05;

impl SettingDef {
    /// The framework's built-in settings, in page order: a volume slider and a
    /// mute per [`AUDIO_BUSES`] entry (page `audio`, group `volume`; labels
    /// `set-audio-<bus>-volume` / `-muted`, which the content's strings give).
    /// A record with the same id replaces the definition (range, default),
    /// and the value stays in `[settings.sf]`.
    pub fn builtins() -> Vec<(SettingKey, SettingDef)> {
        let def = |label: String, kind| SettingDef {
            page: Some("audio".into()),
            group: Some("volume".into()),
            ..SettingDef::new(label, kind)
        };
        AUDIO_BUSES
            .iter()
            .flat_map(|bus| {
                [
                    (
                        SettingKey::new(audio_volume_key(bus)),
                        def(
                            format!("set-audio-{bus}-volume"),
                            SettingKind::Float {
                                default: 1.0,
                                min: 0.0,
                                max: 1.0,
                                step: Some(AUDIO_VOLUME_STEP),
                                log_scale: false,
                            },
                        ),
                    ),
                    (
                        SettingKey::new(audio_muted_key(bus)),
                        def(
                            format!("set-audio-{bus}-muted"),
                            SettingKind::Bool { default: false },
                        ),
                    ),
                ]
            })
            .collect()
    }
}

/// A setting value (untagged in records: `true`, `3`, `0.5`, `"dlss"`).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SettingValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    /// `Choice` option ids and `Text` values.
    Text(String),
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum SettingError {
    #[error("unknown setting `{0}`")]
    Unknown(String),
    #[error("setting `{key}` expects {expected}")]
    WrongType { key: String, expected: &'static str },
    #[error("setting `{key}`: `{value}` is not an option")]
    NotAnOption { key: String, value: String },
}

/// One effective change, drained by [`SettingsStore::take_changes`].
#[derive(Debug, Clone, PartialEq)]
pub struct SettingChange {
    pub key: SettingKey,
    /// The value before the first write since the last drain. Always `Some`
    /// today (every registered setting has a value); `None` is kept for a
    /// setting that had none.
    pub old: Option<SettingValue>,
    pub new: SettingValue,
}

/// Why the machine settings file could not be read or written. On a read
/// error every `Machine` setting keeps its default.
#[derive(Debug, thiserror::Error)]
pub enum MachineFileError {
    #[error("machine settings `{}`: {source}", path.display())]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("machine settings `{}`: {message}", path.display())]
    Parse { path: PathBuf, message: String },
}

struct Slot {
    owner: ModId,
    /// Key inside the owner's profile table.
    local: String,
    kind: SettingKind,
    scope: SettingScope,
    value: SettingValue,
}

impl Slot {
    /// In the machine file rather than the profile.
    fn machine(&self) -> bool {
        self.scope == SettingScope::Machine
    }
}

/// Current values of every registered setting.
pub struct SettingsStore {
    slots: IndexMap<SettingKey, Slot>,
    /// Profile entries with no registered setting, kept for saving.
    unknown: BTreeMap<String, toml::Table>,
    /// Machine file entries with no registered setting, kept for saving.
    machine_unknown: BTreeMap<String, toml::Table>,
    /// Keys written since the last [`Self::take_changes`], with their value
    /// before the first of those writes.
    changed: IndexMap<SettingKey, SettingValue>,
}

impl SettingsStore {
    /// Every registered setting at its default (actions have no value).
    pub fn new(registry: &UiRegistry) -> Self {
        let slots = registry
            .settings()
            .filter(|d| d.def.kind != SettingKind::Action)
            .map(|d| {
                let local =
                    d.id.as_str()
                        .strip_prefix(&format!("{}.", d.owner))
                        .unwrap_or(d.id.as_str())
                        .to_owned();
                let slot = Slot {
                    owner: d.owner.clone(),
                    local,
                    kind: d.def.kind.clone(),
                    scope: d.def.scope,
                    value: default_of(&d.def.kind),
                };
                (d.id.clone(), slot)
            })
            .collect();
        Self {
            slots,
            unknown: BTreeMap::new(),
            machine_unknown: BTreeMap::new(),
            changed: IndexMap::new(),
        }
    }

    /// Whether a value changed since the last [`Self::take_changes`] (it may
    /// still drain to nothing if it was changed back).
    pub fn has_changes(&self) -> bool {
        !self.changed.is_empty()
    }

    /// Drains the change log: one entry per key whose value now differs from
    /// the value it had at the previous drain, in first-write order.
    pub fn take_changes(&mut self) -> Vec<SettingChange> {
        let changed = std::mem::take(&mut self.changed);
        changed
            .into_iter()
            .filter_map(|(key, old)| {
                let new = self.slots.get(&key)?.value.clone();
                (new != old).then_some(SettingChange {
                    key,
                    old: Some(old),
                    new,
                })
            })
            .collect()
    }

    pub fn get(&self, key: &str) -> Option<&SettingValue> {
        self.slots.get(key).map(|s| &s.value)
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.get(key)? {
            SettingValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        match self.get(key)? {
            SettingValue::Float(x) => Some(*x),
            #[allow(clippy::cast_precision_loss)]
            SettingValue::Int(n) => Some(*n as f64),
            _ => None,
        }
    }

    /// Sets a value. Numbers are clamped to the range and snapped to the step;
    /// an int setting accepts a float (rounded) and a float setting an int.
    /// Returns the value actually stored.
    pub fn set(&mut self, key: &str, value: SettingValue) -> Result<&SettingValue, SettingError> {
        let (key, slot) = self
            .slots
            .get_full_mut(key)
            .map(|(_, k, s)| (k, s))
            .ok_or_else(|| SettingError::Unknown(key.to_owned()))?;
        let value = coerce(key.as_str(), &slot.kind, value)?;
        write(&mut self.changed, key, slot, value);
        Ok(&slot.value)
    }

    /// Resets one setting to its default.
    pub fn reset(&mut self, key: &str) -> bool {
        let Some((key, slot)) = self.slots.get_full_mut(key).map(|(_, k, s)| (k, s)) else {
            return false;
        };
        let value = default_of(&slot.kind);
        write(&mut self.changed, key, slot, value);
        true
    }

    /// Reads values from profile tables (keyed by mod id). Invalid values keep
    /// the default and are returned as errors; unknown entries are kept for saving.
    /// Entries for `Machine` settings are ignored (they live in the machine file).
    pub fn load_profile(&mut self, tables: &BTreeMap<String, toml::Table>) -> Vec<SettingError> {
        self.load_tables(tables, false)
    }

    /// Reads tables into the slots of one storage (`machine`: the machine
    /// file, else the profile). Entries for the other storage's settings are
    /// ignored; entries for no setting are kept for that storage's save.
    fn load_tables(
        &mut self,
        tables: &BTreeMap<String, toml::Table>,
        machine: bool,
    ) -> Vec<SettingError> {
        let mut errors = Vec::new();
        let mut unknown = BTreeMap::<String, toml::Table>::new();
        for (mod_id, table) in tables {
            for (local, raw) in table {
                let found = self
                    .slots
                    .iter_mut()
                    .find(|(_, s)| s.owner.as_str() == mod_id && &s.local == local);
                let Some((key, slot)) = found else {
                    unknown
                        .entry(mod_id.clone())
                        .or_default()
                        .insert(local.clone(), raw.clone());
                    continue;
                };
                if slot.machine() != machine {
                    continue;
                }
                let result = match from_toml(raw) {
                    Some(v) => coerce(key.as_str(), &slot.kind, v),
                    None => Err(SettingError::WrongType {
                        key: key.0.clone(),
                        expected: expected(&slot.kind),
                    }),
                };
                let value = result.unwrap_or_else(|e| {
                    errors.push(e);
                    default_of(&slot.kind)
                });
                write(&mut self.changed, key, slot, value);
            }
        }
        if machine {
            self.machine_unknown = unknown;
        } else {
            self.unknown = unknown;
        }
        errors
    }

    /// Reads the machine settings file at `path` (`Machine` settings only).
    /// A missing file is a first run: nothing changes, no error. A file that
    /// can't be read or parsed resets every `Machine` setting to its default
    /// and returns the error; invalid values inside a good file keep the
    /// default and come back as [`SettingError`]s, like the profile's.
    pub fn load_machine(&mut self, path: &Path) -> Result<Vec<SettingError>, MachineFileError> {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(source) => {
                self.reset_machine();
                return Err(MachineFileError::Io {
                    path: path.to_owned(),
                    source,
                });
            }
        };
        let tables = match toml::from_str::<toml::Table>(&text) {
            Ok(root) => machine_tables(root).map_err(|key| {
                format!("`{key}` is not a table of settings ([<owner>] key = value)")
            }),
            Err(e) => Err(e.to_string()),
        };
        match tables {
            Ok(tables) => Ok(self.load_tables(&tables, true)),
            Err(message) => {
                self.reset_machine();
                Err(MachineFileError::Parse {
                    path: path.to_owned(),
                    message,
                })
            }
        }
    }

    /// Every `Machine` setting back to its default, unknown machine entries dropped.
    fn reset_machine(&mut self) {
        self.machine_unknown.clear();
        for (key, slot) in self.slots.iter_mut().filter(|(_, s)| s.machine()) {
            let value = default_of(&slot.kind);
            write(&mut self.changed, key, slot, value);
        }
    }

    /// The machine file's tables: `Machine` values that differ from their
    /// default, plus passed-through unknown machine entries.
    pub fn to_machine(&self) -> toml::Table {
        self.tables(true)
            .into_iter()
            .filter(|(_, t)| !t.is_empty())
            .map(|(owner, t)| (owner, toml::Value::Table(t)))
            .collect()
    }

    /// Writes [`Self::to_machine`] to `path` (parent folders created, written
    /// to a temporary file then renamed over the old one, so a crash mid-save
    /// keeps the previous file). With nothing to store and no file yet, writes
    /// nothing. Returns whether it wrote.
    pub fn save_machine(&self, path: &Path) -> Result<bool, MachineFileError> {
        let io = |source| MachineFileError::Io {
            path: path.to_owned(),
            source,
        };
        let table = self.to_machine();
        if table.is_empty() && !path.exists() {
            return Ok(false);
        }
        if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(dir).map_err(io)?;
        }
        let mut temp = path.as_os_str().to_owned();
        temp.push(".new");
        let temp = PathBuf::from(temp);
        std::fs::write(&temp, table.to_string()).map_err(io)?;
        std::fs::rename(&temp, path).map_err(io)?;
        Ok(true)
    }

    /// Profile tables to save: values that differ from their default, plus
    /// passed-through unknown entries.
    /// `Machine` settings are not included (see [`Self::to_machine`]).
    pub fn to_profile(&self) -> BTreeMap<String, toml::Table> {
        self.tables(false)
    }

    /// Non-default values of one storage's settings plus its unknown entries.
    fn tables(&self, machine: bool) -> BTreeMap<String, toml::Table> {
        let mut out = if machine {
            self.machine_unknown.clone()
        } else {
            self.unknown.clone()
        };
        for slot in self
            .slots
            .values()
            .filter(|s| s.machine() == machine && s.value != default_of(&s.kind))
        {
            out.entry(slot.owner.0.clone())
                .or_default()
                .insert(slot.local.clone(), to_toml(&slot.value));
        }
        out
    }
}

/// Stores `value` in `slot`, logging the old value on the first change since the last drain.
fn write(
    changed: &mut IndexMap<SettingKey, SettingValue>,
    key: &SettingKey,
    slot: &mut Slot,
    value: SettingValue,
) {
    if slot.value == value {
        return;
    }
    let old = std::mem::replace(&mut slot.value, value);
    changed.entry(key.clone()).or_insert(old);
}

/// The machine file's top-level tables, or the first key that isn't a table.
fn machine_tables(root: toml::Table) -> Result<BTreeMap<String, toml::Table>, String> {
    root.into_iter()
        .map(|(owner, value)| match value {
            toml::Value::Table(t) => Ok((owner, t)),
            _ => Err(owner),
        })
        .collect()
}

fn default_of(kind: &SettingKind) -> SettingValue {
    match kind {
        SettingKind::Bool { default } => SettingValue::Bool(*default),
        SettingKind::Int { default, .. } => SettingValue::Int(*default),
        SettingKind::Float { default, .. } => SettingValue::Float(*default),
        SettingKind::Choice { default, .. } | SettingKind::Text { default, .. } => {
            SettingValue::Text(default.clone())
        }
        // Never stored: `SettingsStore::new` gives actions no slot.
        SettingKind::Action => SettingValue::Bool(false),
    }
}

fn expected(kind: &SettingKind) -> &'static str {
    match kind {
        SettingKind::Bool { .. } => "a bool",
        SettingKind::Int { .. } => "an integer",
        SettingKind::Float { .. } => "a number",
        SettingKind::Choice { .. } => "an option id",
        SettingKind::Text { .. } => "text",
        SettingKind::Action => "no value (an action)",
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub(crate) fn coerce(
    key: &str,
    kind: &SettingKind,
    value: SettingValue,
) -> Result<SettingValue, SettingError> {
    let wrong = || SettingError::WrongType {
        key: key.to_owned(),
        expected: expected(kind),
    };
    match (kind, value) {
        (SettingKind::Bool { .. }, SettingValue::Bool(b)) => Ok(SettingValue::Bool(b)),
        (
            SettingKind::Int { min, max, step, .. },
            v @ (SettingValue::Int(_) | SettingValue::Float(_)),
        ) => {
            let n = match v {
                SettingValue::Int(n) => n,
                SettingValue::Float(x) if x.is_finite() => x.round() as i64,
                _ => return Err(wrong()),
            };
            let snapped = min + ((n.clamp(*min, *max) - min + step / 2) / step) * step;
            Ok(SettingValue::Int(snapped.min(*max)))
        }
        (
            SettingKind::Float { min, max, step, .. },
            v @ (SettingValue::Int(_) | SettingValue::Float(_)),
        ) => {
            let x = match v {
                SettingValue::Int(n) => n as f64,
                SettingValue::Float(x) if x.is_finite() => x,
                _ => return Err(wrong()),
            };
            let mut x = x.clamp(*min, *max);
            if let Some(step) = step {
                let k = ((x - min) / step).round();
                // A step like 0.05 is 1/20: dividing by 20 lands on 0.6 where
                // multiplying by 0.05 gives 0.6000000000000001 (saved as such).
                let per_unit = (1.0 / step).round();
                let offset = if per_unit >= 1.0 && (1.0 / step - per_unit).abs() < 1e-9 {
                    k / per_unit
                } else {
                    k * step
                };
                x = (min + offset).min(*max);
            }
            Ok(SettingValue::Float(x))
        }
        (SettingKind::Choice { options, .. }, SettingValue::Text(s)) => {
            if options.contains(&s) {
                Ok(SettingValue::Text(s))
            } else {
                Err(SettingError::NotAnOption {
                    key: key.to_owned(),
                    value: s,
                })
            }
        }
        (SettingKind::Text { max_len, .. }, SettingValue::Text(s)) => {
            Ok(SettingValue::Text(match max_len {
                Some(max) => s.chars().take(*max).collect(),
                None => s,
            }))
        }
        _ => Err(wrong()),
    }
}

fn from_toml(value: &toml::Value) -> Option<SettingValue> {
    match value {
        toml::Value::Boolean(b) => Some(SettingValue::Bool(*b)),
        toml::Value::Integer(n) => Some(SettingValue::Int(*n)),
        toml::Value::Float(x) => Some(SettingValue::Float(*x)),
        toml::Value::String(s) => Some(SettingValue::Text(s.clone())),
        _ => None,
    }
}

fn to_toml(value: &SettingValue) -> toml::Value {
    match value {
        SettingValue::Bool(b) => toml::Value::Boolean(*b),
        SettingValue::Int(n) => toml::Value::Integer(*n),
        SettingValue::Float(x) => toml::Value::Float(*x),
        SettingValue::Text(s) => toml::Value::String(s.clone()),
    }
}
