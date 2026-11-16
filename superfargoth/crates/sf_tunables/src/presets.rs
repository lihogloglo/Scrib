//! Presets as data (spec `docs/specs/tunables.md` "Presets are bundles of tunables", slice T5b).
//!
//! - **Record kinds** (types here; the host's mod adapter registers them, so the core needs no
//!   record store): [`TUNABLE_KIND`] (`sf.tunable`, [`TunableRecord`]): a mod's value for one
//!   tunable (the record id is its name), written at the `Mod` layer ([`apply_mod_values`]);
//!   [`PRESET_KIND`] (`sf.tunable_preset`, [`PresetRecord`]): one driver's levels, tunable name to
//!   value, each value with its source.
//! - **Drivers** are choice tunables: the overall [`GRAPHICS_PRESET`] (`sf.graphics_preset`), whose
//!   levels set the group drivers, and one per UE-style scalability group ([`DRIVERS`], `sg.*`).
//!   Levels [`LEVELS`] plus [`CUSTOM`], which a driver reads when its members match no level.
//! - [`MergedPresets`]: the records merged per driver in load order (a later record's value wins
//!   for a key, as `ui.setting_preset`), records whose `when` the host's capabilities don't meet
//!   left out. [`PresetBook`]: the merge checked against the registered tunables; it writes a level
//!   at the `Preset` layer ([`PresetBook::apply`]), derives each driver's level from its members'
//!   effective values ([`PresetBook::level_of`]) and keeps the driver showing it ([`PresetBook::sync`]).
//!
//! Why not `ui.setting_preset` (`sf_ui`): its values target the menu's settings store and are
//! checked against `ui.setting` rows (a key that isn't a setting is dropped), while most members
//! here are `Dev` tunables with no menu row; and it carries no per-value provenance. The shape
//! (driver, levels, merge by driver) is the same, so a Graphics page can map onto it later.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::{
    AnyDef, ArgKind, Def, Group, Lifetime, Scope, SetBy, SetOutcome, Source, TunableId, Tunables,
    Value, def,
};

/// `sf.tunable`: a mod's value for one tunable ([`TunableRecord`]).
pub const TUNABLE_KIND: &str = "sf.tunable";
/// `sf.tunable_preset`: one driver's levels ([`PresetRecord`]).
pub const PRESET_KIND: &str = "sf.tunable_preset";
/// The level a driver reads when its members match none.
pub const CUSTOM: &str = "custom";
/// The levels, weakest first (UE scalability 0-4, with a floor under Low).
pub const LEVELS: [&str; 5] = ["extremely_low", "low", "medium", "high", "ultra"];
/// A driver's options: the levels and [`CUSTOM`].
const DRIVER_CHOICES: &[&str] = &["extremely_low", "low", "medium", "high", "ultra", "custom"];

const DRIVER_SOURCE: Source = Source::Industry(
    "UE scalability groups (sg.*Quality, levels 0-4; the overall level reads Custom once a group differs)",
);

/// The overall graphics preset: its levels set every group driver.
pub const GRAPHICS_PRESET: Def<&'static str> = def!(choice "sf.graphics_preset" = "custom")
    .choices(DRIVER_CHOICES)
    .help("Overall graphics preset: sets every sg.* group driver; reads custom when a group differs")
    .source(DRIVER_SOURCE)
    .scope(Scope::Player);
/// View distance: LOD bias, size culling, far-field handoff.
pub const SG_VIEW_DISTANCE: Def<&'static str> = def!(choice "sg.view_distance" = "custom")
    .choices(DRIVER_CHOICES)
    .help("View distance group: LOD bias, size culling, far-field handoff")
    .source(DRIVER_SOURCE)
    .group(Group::ViewDistance)
    .scope(Scope::Player);
/// Shadows: sun cascades, lamp shadows, contact shadows, filter.
pub const SG_SHADOWS: Def<&'static str> = def!(choice "sg.shadows" = "custom")
    .choices(DRIVER_CHOICES)
    .help("Shadows group: sun cascades and distance, PCSS, filter, lamp shadows, contact shadows")
    .source(DRIVER_SOURCE)
    .group(Group::Shadows)
    .scope(Scope::Player);
/// Lighting: ambient occlusion, global illumination.
pub const SG_LIGHTING: Def<&'static str> = def!(choice "sg.lighting" = "custom")
    .choices(DRIVER_CHOICES)
    .help("Lighting group: ambient occlusion (GTAO), probe GI")
    .source(DRIVER_SOURCE)
    .group(Group::Lighting)
    .scope(Scope::Player);
/// Post: bloom, temporal resolve and upscaling.
pub const SG_POST: Def<&'static str> = def!(choice "sg.post" = "custom")
    .choices(DRIVER_CHOICES)
    .help("Post-processing group: bloom, temporal resolve and upscaling")
    .source(DRIVER_SOURCE)
    .group(Group::Post)
    .scope(Scope::Player);
/// Textures: load cap, parallax.
pub const SG_TEXTURES: Def<&'static str> = def!(choice "sg.textures" = "custom")
    .choices(DRIVER_CHOICES)
    .help("Textures group: load size cap, parallax")
    .source(DRIVER_SOURCE)
    .group(Group::Textures)
    .scope(Scope::Player);
/// Effects: surface weather and other effect budgets.
pub const SG_EFFECTS: Def<&'static str> = def!(choice "sg.effects" = "custom")
    .choices(DRIVER_CHOICES)
    .help("Effects group: surface weather (puddles, shelter) and effect budgets")
    .source(DRIVER_SOURCE)
    .group(Group::Effects)
    .scope(Scope::Player);
/// Water.
pub const SG_WATER: Def<&'static str> = def!(choice "sg.water" = "custom")
    .choices(DRIVER_CHOICES)
    .help("Water group: the water's quality tier")
    .source(DRIVER_SOURCE)
    .group(Group::Water)
    .scope(Scope::Player);
/// Sky: clouds.
pub const SG_SKY: Def<&'static str> = def!(choice "sg.sky" = "custom")
    .choices(DRIVER_CHOICES)
    .help("Sky group: cloud sampling")
    .source(DRIVER_SOURCE)
    .group(Group::Sky)
    .scope(Scope::Player);

/// The drivers, overall first.
pub const DRIVERS: [&Def<&'static str>; 9] = [
    &GRAPHICS_PRESET,
    &SG_VIEW_DISTANCE,
    &SG_SHADOWS,
    &SG_LIGHTING,
    &SG_POST,
    &SG_TEXTURES,
    &SG_EFFECTS,
    &SG_WATER,
    &SG_SKY,
];

/// This module's tunables (the drivers).
pub const TUNABLES: &[&dyn AnyDef] = &[
    &GRAPHICS_PRESET,
    &SG_VIEW_DISTANCE,
    &SG_SHADOWS,
    &SG_LIGHTING,
    &SG_POST,
    &SG_TEXTURES,
    &SG_EFFECTS,
    &SG_WATER,
    &SG_SKY,
];

/// A value in a record, untagged: `true`, `3`, `0.5`, `"auto"`, `"35,45"` (pairs as text). Parsed by
/// the tunable's kind, like a console line.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
}

impl RecordValue {
    /// As typed in the console (`on`/`off` for bools).
    pub fn text(&self) -> String {
        match self {
            RecordValue::Bool(b) => if *b { "on" } else { "off" }.to_owned(),
            RecordValue::Int(i) => i.to_string(),
            RecordValue::Float(x) => x.to_string(),
            RecordValue::Text(s) => s.clone(),
        }
    }
}

/// `sf.tunable`: a mod's value for the tunable named by the record id, at the `Mod` layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TunableRecord {
    pub value: RecordValue,
    /// Why this value (`industry: ...`, `measured: <ledger line>`, `owner: <date>`, `guess`).
    #[serde(default)]
    pub source: String,
}

/// `sf.tunable_preset`: a driver's levels. Records naming the same driver merge in load order, a
/// later record's value winning for a key: a mod adds or changes keys of a level with its own record.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PresetRecord {
    /// The driver tunable (`sg.shadows`, `sf.graphics_preset`).
    pub driver: String,
    /// A host capability the record needs (`dlss`), or lacks (`!dlss`); `None`: always.
    #[serde(default)]
    pub when: Option<String>,
    /// The source of every value without its own in `sources`.
    #[serde(default)]
    pub source: String,
    /// Level id to the values it writes (tunable name to value). A value naming another driver
    /// sets that driver's level.
    #[serde(default)]
    pub levels: BTreeMap<String, BTreeMap<String, RecordValue>>,
    /// Level id to tunable name to the source of that one value.
    #[serde(default)]
    pub sources: BTreeMap<String, BTreeMap<String, String>>,
}

impl PresetRecord {
    fn applies(&self, capabilities: &[&str]) -> bool {
        match self.when.as_deref().map(str::trim) {
            None | Some("") => true,
            Some(w) => match w.strip_prefix('!') {
                Some(lacks) => !capabilities.contains(&lacks.trim()),
                None => capabilities.contains(&w),
            },
        }
    }
}

/// Writes `sf.tunable` records at the `Mod` layer, in order (`(tunable name, record, mod)`; a later
/// one wins). Returns the refused ones (unknown name, a value the kind refuses).
pub fn apply_mod_values<'a>(
    store: &mut Tunables,
    records: impl IntoIterator<Item = (&'a str, &'a TunableRecord, &'a str)>,
) -> Vec<String> {
    let mut refused = Vec::new();
    for (name, record, owner) in records {
        let Some(id) = store.id(name) else {
            refused.push(format!("{TUNABLE_KIND} {name} (mod {owner}): no such tunable"));
            continue;
        };
        let note = format!("mod {owner}: {}", or_guess(&record.source));
        let result = store
            .var(id)
            .kind()
            .parse(&record.value.text())
            .and_then(|v| store.set_noted(id, v, SetBy::Mod, Some(note)));
        if let Err(e) = result {
            refused.push(format!("{TUNABLE_KIND} {name} (mod {owner}): {e}"));
        }
    }
    refused
}

fn or_guess(source: &str) -> &str {
    if source.trim().is_empty() { "guess" } else { source }
}

/// One merged value: the value, its source and the mod that wrote it.
#[derive(Clone, Debug, PartialEq)]
pub struct MergedValue {
    pub value: RecordValue,
    pub source: String,
    pub owner: String,
}

/// One driver's merged levels.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MergedDriver {
    pub driver: String,
    pub levels: BTreeMap<String, BTreeMap<String, MergedValue>>,
}

/// `sf.tunable_preset` records merged per driver, in first-seen order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MergedPresets {
    pub drivers: Vec<MergedDriver>,
}

impl MergedPresets {
    /// Merges `(record, mod)` pairs in load order; a record whose `when` the `capabilities` don't
    /// meet is left out.
    pub fn merge<'a>(
        records: impl IntoIterator<Item = (&'a PresetRecord, &'a str)>,
        capabilities: &[&str],
    ) -> Self {
        let mut out = Self::default();
        for (record, owner) in records {
            if !record.applies(capabilities) {
                continue;
            }
            let i = if let Some(i) = out.drivers.iter().position(|d| d.driver == record.driver) {
                i
            } else {
                out.drivers.push(MergedDriver { driver: record.driver.clone(), levels: BTreeMap::new() });
                out.drivers.len() - 1
            };
            for (level, values) in &record.levels {
                let merged = out.drivers[i].levels.entry(level.clone()).or_default();
                for (key, value) in values {
                    let source = record
                        .sources
                        .get(level)
                        .and_then(|s| s.get(key))
                        .unwrap_or(&record.source);
                    merged.insert(
                        key.clone(),
                        MergedValue {
                            value: value.clone(),
                            source: or_guess(source).to_owned(),
                            owner: owner.to_owned(),
                        },
                    );
                }
            }
        }
        out
    }

    pub fn driver(&self, name: &str) -> Option<&MergedDriver> {
        self.drivers.iter().find(|d| d.driver == name)
    }

    /// Every non-driver value `driver`'s `level` writes, through the drivers it sets (no store
    /// needed: the shipped table, the index's per-level columns, startup values).
    pub fn flatten(&self, driver: &str, level: &str) -> BTreeMap<String, MergedValue> {
        let mut out = BTreeMap::new();
        self.flatten_into(driver, level, &mut out, 0);
        out
    }

    fn flatten_into(
        &self,
        driver: &str,
        level: &str,
        out: &mut BTreeMap<String, MergedValue>,
        depth: usize,
    ) {
        let Some(values) = self.driver(driver).and_then(|d| d.levels.get(level)) else {
            return;
        };
        for (key, value) in values {
            if self.driver(key).is_some() && depth < MAX_DEPTH {
                self.flatten_into(key, &value.value.text(), out, depth + 1);
            } else {
                out.insert(key.clone(), value.clone());
            }
        }
    }
}

/// How deep drivers may nest (overall → group is 1).
const MAX_DEPTH: usize = 4; // tunables: const (a recursion guard against driver cycles, not a trade-off)

/// One value a level writes.
#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    pub id: TunableId,
    pub name: String,
    /// Parsed by the member's kind.
    pub value: Value,
    pub source: String,
    pub owner: String,
    /// The member is itself a driver: the value is its level.
    pub driver: bool,
}

/// One level of a driver.
#[derive(Clone, Debug, PartialEq)]
pub struct Level {
    pub name: String,
    pub values: Vec<Member>,
}

/// A driver: its tunable and its levels, in the driver's option order.
#[derive(Clone, Debug, PartialEq)]
pub struct Driver {
    pub name: String,
    pub id: TunableId,
    pub levels: Vec<Level>,
    /// Every member any level writes.
    pub members: Vec<TunableId>,
}

impl Driver {
    pub fn level(&self, name: &str) -> Option<&Level> {
        self.levels.iter().find(|l| l.name == name)
    }

    /// The level after `current` in option order, wrapping (from [`CUSTOM`] or an unknown level:
    /// the first).
    pub fn next_level(&self, current: &str) -> Option<&str> {
        let i = self.levels.iter().position(|l| l.name == current);
        let next = i.map_or(0, |i| (i + 1) % self.levels.len());
        self.levels.get(next).map(|l| l.name.as_str())
    }
}

/// What applying a level did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Applied {
    /// Members whose effective value changed.
    pub changed: Vec<TunableId>,
    /// Members whose new `Preset` value sits under a stronger layer (still in effect).
    pub outranked: Vec<(TunableId, SetBy)>,
}

impl Applied {
    /// Changed members whose lifetime isn't live: `(name, lifetime)` (the console says when they apply).
    pub fn deferred(&self, store: &Tunables) -> Vec<(String, Lifetime)> {
        self.changed
            .iter()
            .map(|id| store.var(*id))
            .filter(|t| matches!(t.meta().lifetime, Lifetime::Respawn | Lifetime::Restart | Lifetime::Bake))
            .map(|t| (t.name().to_owned(), t.meta().lifetime))
            .collect()
    }

    /// One line for the console and the log: counts, deferred members by lifetime, outranked ones.
    pub fn summary(&self, store: &Tunables) -> String {
        let mut out = format!("{} value(s) changed", self.changed.len());
        let deferred = self.deferred(store);
        for (lifetime, label) in [
            (Lifetime::Restart, "applies on restart"),
            (Lifetime::Respawn, "applies to new spawns"),
            (Lifetime::Bake, "needs a re-bake"),
        ] {
            let names: Vec<&str> =
                deferred.iter().filter(|(_, l)| *l == lifetime).map(|(n, _)| n.as_str()).collect();
            if !names.is_empty() {
                let _ = write!(out, "; {label}: {}", names.join(", "));
            }
        }
        if !self.outranked.is_empty() {
            let names: Vec<String> = self
                .outranked
                .iter()
                .map(|(id, by)| format!("{} ({})", store.var(*id).name(), by.describe()))
                .collect();
            let _ = write!(out, "; kept above the preset: {}", names.join(", "));
        }
        out
    }
}

/// The merged presets checked against the registered tunables (spec "Presets are bundles of
/// tunables"). Drivers that set other drivers come first.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PresetBook {
    pub drivers: Vec<Driver>,
}

impl PresetBook {
    /// Checks `merged` against `store`: a driver must be a registered choice tunable offering
    /// [`CUSTOM`]; a level one of its options; a member registered, its value accepted by its kind;
    /// a non-driver member in one driver only. What fails is left out and reported.
    pub fn build(store: &Tunables, merged: &MergedPresets) -> (Self, Vec<String>) {
        let mut issues = Vec::new();
        let mut drivers = Vec::new();
        let mut owner_of: BTreeMap<TunableId, String> = BTreeMap::new();
        let options_of = |name: &str| match store.id(name).map(|id| (id, store.var(id).kind())) {
            Some((id, ArgKind::Choice(options))) if options.iter().any(|o| o == CUSTOM) => Some((id, options.clone())),
            _ => None,
        };
        for m in &merged.drivers {
            let Some((id, options)) = options_of(&m.driver) else {
                issues.push(format!("preset driver {}: not a registered choice tunable with `{CUSTOM}`", m.driver));
                continue;
            };
            for level in m.levels.keys() {
                if level == CUSTOM || !options.contains(level) {
                    issues.push(format!("preset driver {}: level {level} is not one of its options", m.driver));
                }
            }
            let mut driver = Driver { name: m.driver.clone(), id, levels: Vec::new(), members: Vec::new() };
            for option in options.iter().filter(|o| *o != CUSTOM) {
                let Some(values) = m.levels.get(option) else { continue };
                let mut level = Level { name: option.clone(), values: Vec::new() };
                for (key, v) in values {
                    let Some(member) = store.id(key) else {
                        issues.push(format!("preset {} {option}: no tunable {key} (mod {})", m.driver, v.owner));
                        continue;
                    };
                    let value = match store.var(member).kind().parse(&v.value.text()) {
                        Ok(value) => value,
                        Err(e) => {
                            issues.push(format!("preset {} {option}: {key}: {e} (mod {})", m.driver, v.owner));
                            continue;
                        }
                    };
                    let is_driver = merged.driver(key).is_some() && options_of(key).is_some();
                    if !is_driver {
                        match owner_of.get(&member) {
                            Some(other) if *other != m.driver => issues.push(format!(
                                "preset {}: {key} is already a member of {other} (a value belongs to one group)",
                                m.driver
                            )),
                            _ => {
                                owner_of.insert(member, m.driver.clone());
                            }
                        }
                    }
                    if !driver.members.contains(&member) {
                        driver.members.push(member);
                    }
                    level.values.push(Member {
                        id: member,
                        name: key.clone(),
                        value,
                        source: v.source.clone(),
                        owner: v.owner.clone(),
                        driver: is_driver,
                    });
                }
                driver.levels.push(level);
            }
            drivers.push(driver);
        }
        let mut book = Self { drivers };
        let depths: Vec<usize> = book.drivers.iter().map(|d| book.depth(&d.name, 0)).collect();
        let mut order: Vec<usize> = (0..book.drivers.len()).collect();
        order.sort_by_key(|&i| std::cmp::Reverse(depths[i]));
        book.drivers = order.into_iter().map(|i| book.drivers[i].clone()).collect();
        (book, issues)
    }

    /// How many driver levels sit under `name` (0: only plain members).
    fn depth(&self, name: &str, at: usize) -> usize {
        if at >= MAX_DEPTH {
            return at;
        }
        self.driver(name).map_or(0, |d| {
            d.levels
                .iter()
                .flat_map(|l| &l.values)
                .filter(|m| m.driver)
                .map(|m| 1 + self.depth(&m.name, at + 1))
                .max()
                .unwrap_or(0)
        })
    }

    pub fn driver(&self, name: &str) -> Option<&Driver> {
        self.drivers.iter().find(|d| d.name == name)
    }

    pub fn is_driver(&self, id: TunableId) -> bool {
        self.drivers.iter().any(|d| d.id == id)
    }

    /// Writes `driver`'s `level` at the `Preset` layer: its values (a driver member sets that
    /// driver's level), clearing the `Preset` layer of members the level doesn't write. Stronger
    /// layers stay in effect ([`Applied::outranked`]). [`CUSTOM`] writes nothing. Call
    /// [`Self::sync`] after.
    pub fn apply(&self, store: &mut Tunables, driver: &str, level: &str) -> Result<Applied, String> {
        let mut applied = Applied::default();
        self.apply_into(store, driver, level, &mut applied, 0)?;
        Ok(applied)
    }

    fn apply_into(
        &self,
        store: &mut Tunables,
        driver: &str,
        level: &str,
        applied: &mut Applied,
        depth: usize,
    ) -> Result<(), String> {
        let d = self.driver(driver).ok_or_else(|| format!("{driver} is not a preset driver"))?;
        if level == CUSTOM {
            return Ok(());
        }
        let l = d.level(level).ok_or_else(|| {
            let names: Vec<&str> = d.levels.iter().map(|l| l.name.as_str()).collect();
            format!("{driver} has no level {level} (levels: {})", names.join(", "))
        })?;
        for &member in &d.members {
            if !l.values.iter().any(|m| m.id == member) && !self.is_driver(member) && store.clear(member, SetBy::Preset) {
                applied.changed.push(member);
            }
        }
        for m in &l.values {
            if m.driver {
                if depth < MAX_DEPTH {
                    let text = m.value.as_str().unwrap_or_default().to_owned();
                    self.apply_into(store, &m.name, &text, applied, depth + 1)?;
                }
                continue;
            }
            let note = format!("{driver} {level}; {}", m.source);
            match store.set_noted(m.id, m.value.clone(), SetBy::Preset, Some(note))? {
                SetOutcome::Changed => applied.changed.push(m.id),
                SetOutcome::Outranked(by) => applied.outranked.push((m.id, by)),
                SetOutcome::Unchanged => {}
            }
        }
        // The driver shows the level it was asked for while its values hold ([`Self::level_of`]).
        store.set_noted(d.id, Value::Str(level.to_owned()), SetBy::Preset, Some("level requested".to_owned()))?;
        Ok(())
    }

    /// The level `driver` shows: the one it shows now (its `Preset` layer) while that level's
    /// values are all in effect (two levels may hold the same values: Low and Extremely low water),
    /// else the first level, in option order, whose values are; `None`: custom.
    pub fn level_of(&self, store: &Tunables, driver: &str) -> Option<&str> {
        let d = self.driver(driver)?;
        let shown = store.var(d.id).layer(SetBy::Preset).and_then(Value::as_str);
        if let Some(l) = shown.and_then(|s| d.level(s)).filter(|l| self.matches(store, l, 0)) {
            return Some(l.name.as_str());
        }
        d.levels.iter().find(|l| self.matches(store, l, 0)).map(|l| l.name.as_str())
    }

    /// Whether every value of `driver`'s `level` is in effect (a driver member: its level's values).
    pub fn is_in_effect(&self, store: &Tunables, driver: &str, level: &str) -> bool {
        self.driver(driver).and_then(|d| d.level(level).map(|l| (d, l))).is_some_and(|(_, l)| self.matches(store, l, 0))
    }

    fn matches(&self, store: &Tunables, l: &Level, depth: usize) -> bool {
        !l.values.is_empty()
            && l.values.iter().all(|m| {
                if m.driver {
                    depth < MAX_DEPTH
                        && self.driver(&m.name).is_some_and(|md| {
                            md.level(m.value.as_str().unwrap_or_default())
                                .is_some_and(|ml| self.matches(store, ml, depth + 1))
                        })
                } else {
                    values_match(store.get(m.id), &m.value)
                }
            })
    }

    /// Sets each driver's `Preset` layer to its derived level (or [`CUSTOM`]). Returns the drivers
    /// whose shown level changed.
    pub fn sync(&self, store: &mut Tunables) -> Vec<TunableId> {
        let mut changed = Vec::new();
        for d in &self.drivers {
            let shown = Value::Str(self.level_of(store, &d.name).unwrap_or(CUSTOM).to_owned());
            if store.var(d.id).layer(SetBy::Preset) == Some(&shown) {
                continue;
            }
            let outcome = store.set_noted(d.id, shown, SetBy::Preset, Some("derived from its members".to_owned()));
            if outcome.is_ok_and(SetOutcome::changed) {
                changed.push(d.id);
            }
        }
        changed
    }

    /// Whether any driver's shown level differs from its derived one ([`Self::sync`] would write).
    pub fn out_of_sync(&self, store: &Tunables) -> bool {
        self.drivers.iter().any(|d| {
            let shown = Value::Str(self.level_of(store, &d.name).unwrap_or(CUSTOM).to_owned());
            store.var(d.id).layer(SetBy::Preset) != Some(&shown)
        })
    }

    /// Level requests: drivers set above the `Preset` layer (console, BRP, `--set`, a key), overall
    /// drivers first: `(driver, level, layer)`. [`Self::take_requests`] consumes them.
    pub fn requests(&self, store: &Tunables) -> Vec<(String, String, SetBy)> {
        self.drivers
            .iter()
            .filter_map(|d| {
                let var = store.var(d.id);
                (var.set_by() > SetBy::Preset)
                    .then(|| (d.name.clone(), var.value().as_str().unwrap_or(CUSTOM).to_owned(), var.set_by()))
            })
            .collect()
    }

    /// Applies every request ([`Self::requests`]) and clears the layers it came on, so the driver
    /// shows its derived level again; then [`Self::sync`]. Returns each request with what it did.
    pub fn take_requests(&self, store: &mut Tunables) -> Vec<(String, String, Result<Applied, String>)> {
        let requests = self.requests(store);
        let mut out = Vec::new();
        for (driver, level, _) in requests {
            let id = self.driver(&driver).expect("a request names a driver").id;
            for layer in SetBy::ALL.into_iter().filter(|l| *l > SetBy::Preset) {
                store.clear(id, layer);
            }
            let result = self.apply(store, &driver, &level);
            out.push((driver, level, result));
        }
        self.sync(store);
        out
    }
}

/// Equal values; floats within a relative 1e-6 (an `f32` round trip).
pub fn values_match(a: &Value, b: &Value) -> bool {
    let close = |x: f64, y: f64| (x - y).abs() <= 1e-6 * x.abs().max(y.abs()).max(1.0);
    match (a, b) {
        (Value::Float(x), Value::Float(y)) => close(*x, *y),
        (Value::Float2(x), Value::Float2(y)) => x.iter().zip(y).all(|(x, y)| close(*x, *y)),
        (Value::Float3(x), Value::Float3(y)) => x.iter().zip(y).all(|(x, y)| close(*x, *y)),
        _ => a == b,
    }
}
