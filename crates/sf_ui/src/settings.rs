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

use std::collections::BTreeMap;

use indexmap::IndexMap;

use crate::ids::{ModId, SettingKey};
use crate::records::{SettingDef, SettingKind};
use crate::registry::{FRAMEWORK_MOD, UiRegistry};

/// Volume buses on the framework's audio settings page, in menu order
/// (the names `sf_audio::Bus::USER` uses).
pub const AUDIO_BUSES: [&str; 6] = ["master", "music", "sfx", "voice", "ambience", "ui"];

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
            label,
            description: None,
            page: Some("audio".into()),
            group: Some("volume".into()),
            kind,
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

/// A setting value.
#[derive(Clone, Debug, PartialEq)]
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

struct Slot {
    owner: ModId,
    /// Key inside the owner's profile table.
    local: String,
    kind: SettingKind,
    value: SettingValue,
}

/// Current values of every registered setting.
pub struct SettingsStore {
    slots: IndexMap<SettingKey, Slot>,
    /// Profile entries with no registered setting, kept for saving.
    unknown: BTreeMap<String, toml::Table>,
}

impl SettingsStore {
    /// Every registered setting at its default.
    pub fn new(registry: &UiRegistry) -> Self {
        let slots = registry
            .settings()
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
                    value: default_of(&d.def.kind),
                };
                (d.id.clone(), slot)
            })
            .collect();
        Self {
            slots,
            unknown: BTreeMap::new(),
        }
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
        let slot = self
            .slots
            .get_mut(key)
            .ok_or_else(|| SettingError::Unknown(key.to_owned()))?;
        slot.value = coerce(key, &slot.kind, value)?;
        Ok(&slot.value)
    }

    /// Resets one setting to its default.
    pub fn reset(&mut self, key: &str) -> bool {
        self.slots
            .get_mut(key)
            .map(|s| s.value = default_of(&s.kind))
            .is_some()
    }

    /// Reads values from profile tables (keyed by mod id). Invalid values keep
    /// the default and are returned as errors; unknown entries are kept for saving.
    pub fn load_profile(&mut self, tables: &BTreeMap<String, toml::Table>) -> Vec<SettingError> {
        let mut errors = Vec::new();
        self.unknown.clear();
        for (mod_id, table) in tables {
            for (local, raw) in table {
                let found = self
                    .slots
                    .iter_mut()
                    .find(|(_, s)| s.owner.as_str() == mod_id && &s.local == local);
                let Some((key, slot)) = found else {
                    self.unknown
                        .entry(mod_id.clone())
                        .or_default()
                        .insert(local.clone(), raw.clone());
                    continue;
                };
                let result = match from_toml(raw) {
                    Some(v) => coerce(key.as_str(), &slot.kind, v),
                    None => Err(SettingError::WrongType {
                        key: key.0.clone(),
                        expected: expected(&slot.kind),
                    }),
                };
                match result {
                    Ok(v) => slot.value = v,
                    Err(e) => {
                        errors.push(e);
                        slot.value = default_of(&slot.kind);
                    }
                }
            }
        }
        errors
    }

    /// Profile tables to save: values that differ from their default, plus
    /// passed-through unknown entries.
    pub fn to_profile(&self) -> BTreeMap<String, toml::Table> {
        let mut out = self.unknown.clone();
        for slot in self
            .slots
            .values()
            .filter(|s| s.value != default_of(&s.kind))
        {
            out.entry(slot.owner.0.clone())
                .or_default()
                .insert(slot.local.clone(), to_toml(&slot.value));
        }
        out
    }
}

fn default_of(kind: &SettingKind) -> SettingValue {
    match kind {
        SettingKind::Bool { default } => SettingValue::Bool(*default),
        SettingKind::Int { default, .. } => SettingValue::Int(*default),
        SettingKind::Float { default, .. } => SettingValue::Float(*default),
        SettingKind::Choice { default, .. } | SettingKind::Text { default, .. } => {
            SettingValue::Text(default.clone())
        }
    }
}

fn expected(kind: &SettingKind) -> &'static str {
    match kind {
        SettingKind::Bool { .. } => "a bool",
        SettingKind::Int { .. } => "an integer",
        SettingKind::Float { .. } => "a number",
        SettingKind::Choice { .. } => "an option id",
        SettingKind::Text { .. } => "text",
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn coerce(
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
