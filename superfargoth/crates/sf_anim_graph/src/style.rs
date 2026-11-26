//! Motion style (AN14a): the per-character motion profile and its smoothing,
//! the class table (parse, class lookup, slot resolve) and the pure class
//! chooser. Bevy-free; the framework knows fields, never a stat or a race.
//!
//! Spec: `docs/specs/animation-style.md` "Profile", "Class table", "Chooser".
//!
//! Every public type serialises in a postcard-safe shape, so a cooked table
//! can ship later. Only [`StyleTable::parse`] (RON) needs the `bake` feature.

use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
pub use sf_anim::rig::Side;
use sf_tunables::{Def, Lifetime, Net, Scope, Source, def};

// ---------------------------------------------------------------- tunables

/// Default of [`SmoothRates::injury_rise_s`].
pub const SMOOTH_INJURY_RISE_S: Def<f32> = def!(float "anim.style.smooth.injury_rise_s" = 0.5)
    .range(0.05, 10.0)
    .help("Seconds (time constant) for the smoothed injury to rise toward its target")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// Default of [`SmoothRates::injury_fall_s`].
pub const SMOOTH_INJURY_FALL_S: Def<f32> = def!(float "anim.style.smooth.injury_fall_s" = 20.0)
    .range(1.0, 600.0)
    .help("Seconds (time constant) for the smoothed injury to fall toward its target (a limp fades out)")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// Default of [`SmoothRates::fatigue_s`].
pub const SMOOTH_FATIGUE_S: Def<f32> = def!(float "anim.style.smooth.fatigue_s" = 3.0)
    .range(0.1, 600.0)
    .help("Seconds (time constant) for the smoothed fatigue, both directions")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// Default of [`SmoothRates::load_s`].
pub const SMOOTH_LOAD_S: Def<f32> = def!(float "anim.style.smooth.load_s" = 1.0)
    .range(0.1, 600.0)
    .help("Seconds (time constant) for the smoothed carried load, both directions")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// Default of [`SmoothRates::mood_s`].
pub const SMOOTH_MOOD_S: Def<f32> = def!(float "anim.style.smooth.mood_s" = 2.0)
    .range(0.1, 600.0)
    .help("Seconds (time constant) for the smoothed mood, both directions")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// Default of [`SmoothRates::intox_rise_s`].
pub const SMOOTH_INTOX_RISE_S: Def<f32> = def!(float "anim.style.smooth.intox_rise_s" = 5.0)
    .range(0.1, 600.0)
    .help("Seconds (time constant) for the smoothed intoxication to rise toward its target")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// Default of [`SmoothRates::intox_fall_s`].
pub const SMOOTH_INTOX_FALL_S: Def<f32> = def!(float "anim.style.smooth.intox_fall_s" = 60.0)
    .range(0.1, 600.0)
    .help("Seconds (time constant) for the smoothed intoxication to fall toward its target")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// Default hold of the chooser, in seconds (the caller turns it into
/// [`ChooseParams::hold_ticks`] at its tick rate).
pub const CHOOSE_HOLD_S: Def<f32> = def!(float "anim.style.choose.hold_s" = 2.0)
    .range(0.0, 30.0)
    .help("Seconds a chosen style class is held before the chooser may switch to another")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// Default of [`ChooseParams::hysteresis`].
pub const CHOOSE_HYSTERESIS: Def<f32> = def!(float "anim.style.choose.hysteresis" = 0.1)
    .range(0.0, 0.5)
    .help("How far below its enter threshold an active chooser row still holds when the row gives no exit")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

// ----------------------------------------------------------------- names

/// Stable text id of a style class (saves, mods).
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassName(pub Arc<str>);

impl ClassName {
    pub fn new(name: &str) -> Self {
        Self(Arc::from(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ClassName {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl fmt::Debug for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &*self.0)
    }
}

impl fmt::Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for ClassName {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ClassName {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d).map(|s| Self(Arc::from(s)))
    }
}

/// Index into the loaded table's classes (sorted by name, so equal on every peer).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ClassId(pub u16);

// --------------------------------------------------------------- profile

/// Per-individual motion profile (host gameplay state). `age` and `style_id`
/// are saved; the other fields are smoothed from [`MotionTargets`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionProfile {
    /// Identity class; default "neutral".
    pub style_id: ClassName,
    /// 0 young adult .. 1 very old; not smoothed.
    #[serde(default)]
    pub age: f32,
    /// 0..1.
    #[serde(default)]
    pub injury: f32,
    /// The injured leg; `Center` = none.
    #[serde(default)]
    pub injury_side: Side,
    /// 0..1.
    #[serde(default)]
    pub fatigue: f32,
    /// 0..1 (1 = at capacity).
    #[serde(default)]
    pub load: f32,
    /// 0..1.
    #[serde(default)]
    pub intoxication: f32,
    /// -1 dejected/afraid .. +1 proud.
    #[serde(default)]
    pub mood: f32,
}

impl Default for MotionProfile {
    fn default() -> Self {
        let style_id = ClassName::new("neutral");
        // tunables: const (state at rest, not a setting: no profile = neutral at zeros, today's look)
        Self { style_id, age: 0.0, injury: 0.0, injury_side: Side::Center, fatigue: 0.0, load: 0.0, intoxication: 0.0, mood: 0.0 }
    }
}

/// What the game adapter wants the smoothed fields to reach.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MotionTargets {
    pub injury: f32,
    pub injury_side: Side,
    pub fatigue: f32,
    pub load: f32,
    pub intoxication: f32,
    pub mood: f32,
}

/// Time constants (seconds) of [`MotionProfile::step`]; `<= 0` snaps.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmoothRates {
    pub injury_rise_s: f32,
    pub injury_fall_s: f32,
    pub fatigue_s: f32,
    pub load_s: f32,
    pub mood_s: f32,
    pub intox_rise_s: f32,
    pub intox_fall_s: f32,
}

impl Default for SmoothRates {
    fn default() -> Self {
        Self {
            injury_rise_s: SMOOTH_INJURY_RISE_S.default_value(),
            injury_fall_s: SMOOTH_INJURY_FALL_S.default_value(),
            fatigue_s: SMOOTH_FATIGUE_S.default_value(),
            load_s: SMOOTH_LOAD_S.default_value(),
            mood_s: SMOOTH_MOOD_S.default_value(),
            intox_rise_s: SMOOTH_INTOX_RISE_S.default_value(),
            intox_fall_s: SMOOTH_INTOX_FALL_S.default_value(),
        }
    }
}

fn unit(x: f32) -> f32 {
    if x.is_nan() { 0.0 } else { x.clamp(0.0, 1.0) }
}

fn signed_unit(x: f32) -> f32 {
    if x.is_nan() { 0.0 } else { x.clamp(-1.0, 1.0) }
}

/// One exponential approach of `x` toward `t` over `dt` with time constant `tau`.
fn approach(x: f32, t: f32, dt: f32, tau: f32) -> f32 {
    if tau <= 0.0 {
        t
    } else {
        x + (t - x) * (1.0 - (-dt / tau).exp())
    }
}

/// `rise` when the target is above the value, `fall` otherwise.
fn tau(x: f32, t: f32, rise: f32, fall: f32) -> f32 {
    if t > x { rise } else { fall }
}

impl MotionProfile {
    /// One host tick: `x += (t - x)(1 - e^(-dt/tau))`, tau = the field's rise
    /// or fall time. `injury_side` follows the target while the target injury
    /// is above 0 and keeps its last side while the injury decays.
    pub fn step(&mut self, t: &MotionTargets, dt: f32, rates: &SmoothRates) {
        let (ti, tx) = (unit(t.injury), unit(t.intoxication));
        self.injury = unit(approach(self.injury, ti, dt, tau(self.injury, ti, rates.injury_rise_s, rates.injury_fall_s)));
        self.fatigue = unit(approach(self.fatigue, unit(t.fatigue), dt, rates.fatigue_s));
        self.load = unit(approach(self.load, unit(t.load), dt, rates.load_s));
        self.intoxication = unit(approach(self.intoxication, tx, dt, tau(self.intoxication, tx, rates.intox_rise_s, rates.intox_fall_s)));
        self.mood = signed_unit(approach(self.mood, signed_unit(t.mood), dt, rates.mood_s));
        if ti > 0.0 {
            self.injury_side = t.injury_side;
        }
        self.age = unit(self.age);
    }

    /// Spawn and load: the smoothed fields take their targets, no ramp. With
    /// no target injury there is nothing to fade, so the side is `Center`.
    pub fn snap(&mut self, t: &MotionTargets) {
        self.injury = unit(t.injury);
        self.injury_side = if self.injury > 0.0 { t.injury_side } else { Side::Center };
        self.fatigue = unit(t.fatigue);
        self.load = unit(t.load);
        self.intoxication = unit(t.intoxication);
        self.mood = signed_unit(t.mood);
        self.age = unit(self.age);
    }

    /// Every field in its range (after deserialising): 0-1, `mood` -1..1; NaN is 0.
    #[must_use]
    pub fn clamped(mut self) -> Self {
        self.age = unit(self.age);
        self.injury = unit(self.injury);
        self.fatigue = unit(self.fatigue);
        self.load = unit(self.load);
        self.intoxication = unit(self.intoxication);
        self.mood = signed_unit(self.mood);
        self
    }

    /// The value of `field`.
    pub fn field(&self, field: Field) -> f32 {
        match field {
            Field::Age => self.age,
            Field::Injury => self.injury,
            Field::Fatigue => self.fatigue,
            Field::Load => self.load,
            Field::Intoxication => self.intoxication,
            Field::Mood => self.mood,
        }
    }
}

// ----------------------------------------------------------------- table

/// A profile field a chooser condition reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Field {
    Age,
    Injury,
    Fatigue,
    Load,
    Intoxication,
    Mood,
}

/// Where a class's missing slot goes next.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Fallback {
    /// The base slot.
    #[default]
    None,
    /// That class's slot (and its own fallback).
    Class(ClassName),
    /// The character's identity (`style_id`) class; the caller resolves it.
    Identity,
}

/// Per-class scales of the procedural modifiers, 0-2 (1 = unchanged).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ModifierScales {
    pub hunch: f32,
    pub slump: f32,
    pub chest_up: f32,
    pub load: f32,
    pub sway: f32,
    pub breathing: f32,
    pub limp: f32,
}

impl Default for ModifierScales {
    fn default() -> Self {
        // tunables: const (identity scale: 1 = the modifier unchanged; classes set theirs in data)
        Self { hunch: 1.0, slump: 1.0, chest_up: 1.0, load: 1.0, sway: 1.0, breathing: 1.0, limp: 1.0 }
    }
}

impl ModifierScales {
    /// Every scale in 0-2 (NaN is 1).
    #[must_use]
    pub fn clamped(self) -> Self {
        let c = |x: f32| if x.is_nan() { 1.0 } else { x.clamp(0.0, 2.0) };
        Self {
            hunch: c(self.hunch),
            slump: c(self.slump),
            chest_up: c(self.chest_up),
            load: c(self.load),
            sway: c(self.sway),
            breathing: c(self.breathing),
            limp: c(self.limp),
        }
    }
}

/// Numeric bands a class's clips should fall in (read by the clip grader,
/// never by the runtime).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Bands {
    /// Gait key -> (min, max) m/s, sorted by key.
    pub speed_mps: Vec<(String, (f32, f32))>,
    pub stride_m: Option<(f32, f32)>,
    pub spine_pitch_deg: Option<(f32, f32)>,
    pub asymmetry: Option<(f32, f32)>,
}

/// One style class.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassRow {
    pub name: ClassName,
    /// Replaces the table's `base_prefix`; `None` = the base set.
    pub prefix: Option<String>,
    pub fallback: Fallback,
    pub recipe: Option<String>,
    pub side: Side,
    /// Gait key -> m/s, sorted by key.
    pub speeds: Vec<(String, f32)>,
    pub bands: Bands,
    pub modifiers: ModifierScales,
}

/// One chooser condition: holds at or above `exit`, enters at or above `enter`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cond {
    pub field: Field,
    pub enter: f32,
    /// `None` = `enter - hysteresis`.
    pub exit: Option<f32>,
}

/// One chooser row (first match wins).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChooserRule {
    pub class: ClassName,
    pub when: Vec<Cond>,
    /// Matches `MotionProfile::injury_side` when set.
    pub side: Option<Side>,
}

/// The loaded style class table.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StyleTable {
    pub base_prefix: String,
    pub default_class: ClassName,
    /// Sorted by name: [`ClassId`] is the index.
    pub classes: Vec<ClassRow>,
    pub rules: Vec<ChooserRule>,
    /// Stable hash of the table's canonical bytes (this field 0).
    pub digest: u64,
}

/// Why a style table was refused.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum StyleError {
    #[error("styles: {0}")]
    Ron(String),
    #[error("styles: unknown field `{0}`")]
    UnknownField(String),
    #[error("styles: default_class `{0}` is not a class")]
    UnknownDefaultClass(String),
    #[error("styles: class `{class}` falls back to unknown class `{fallback}`")]
    UnknownFallback { class: String, fallback: String },
    #[error("styles: rule {rule} names unknown class `{class}`")]
    UnknownRuleClass { rule: usize, class: String },
    #[error("styles: fallback cycle through class `{0}`")]
    FallbackCycle(String),
    #[error("styles: rule {rule}: exit {exit} is above enter {enter}")]
    ExitAboveEnter { rule: usize, enter: f32, exit: f32 },
    #[error("styles: {0} classes (at most 65535)")]
    TooManyClasses(usize),
}

#[cfg(feature = "bake")]
mod file {
    //! The RON shape of `animation/styles.ron`.
    use std::collections::BTreeMap;

    use serde::Deserialize;

    use super::{Field, ModifierScales, Side};

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Table {
        pub base_prefix: String,
        pub default_class: String,
        pub classes: BTreeMap<String, Class>,
        #[serde(default)]
        pub rules: Vec<Rule>,
    }

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Class {
        #[serde(default)]
        pub prefix: Option<String>,
        #[serde(default)]
        pub fallback: Fallback,
        #[serde(default)]
        pub recipe: Option<String>,
        #[serde(default)]
        pub side: Side,
        #[serde(default)]
        pub speeds: BTreeMap<String, f32>,
        #[serde(default)]
        pub bands: Bands,
        #[serde(default)]
        pub modifiers: ModifierScales,
    }

    /// `fallback: Some("neutral")`, `Identity`, `None` or omitted.
    #[derive(Deserialize, Default)]
    pub enum Fallback {
        #[default]
        None,
        Some(String),
        Identity,
    }

    #[derive(Deserialize, Default)]
    #[serde(deny_unknown_fields)]
    pub struct Bands {
        #[serde(default)]
        pub speed_mps: BTreeMap<String, (f32, f32)>,
        #[serde(default)]
        pub stride_m: Option<(f32, f32)>,
        #[serde(default)]
        pub spine_pitch_deg: Option<(f32, f32)>,
        #[serde(default)]
        pub asymmetry: Option<(f32, f32)>,
    }

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Rule {
        pub class: String,
        pub when: Vec<Cond>,
        #[serde(default)]
        pub side: Option<Side>,
    }

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Cond {
        pub field: Field,
        pub enter: f32,
        #[serde(default)]
        pub exit: Option<f32>,
    }
}

impl StyleTable {
    /// Parses `animation/styles.ron`. `Option` fields may omit `Some(..)`.
    /// Refuses an unknown class (rule, fallback, `default_class`), a fallback
    /// cycle, a condition with `exit > enter` and unknown fields.
    #[cfg(feature = "bake")]
    pub fn parse(text: &str) -> Result<Self, StyleError> {
        use ron::extensions::Extensions;
        let f: file::Table = ron::Options::default()
            .with_default_extension(Extensions::IMPLICIT_SOME)
            .from_str(text)
            .map_err(|e| match e.code {
                ron::Error::NoSuchStructField { found, .. } => StyleError::UnknownField(found),
                _ => StyleError::Ron(e.to_string()),
            })?;
        if f.classes.len() > usize::from(u16::MAX) {
            return Err(StyleError::TooManyClasses(f.classes.len()));
        }
        // BTreeMap iteration = sorted by name: the ClassId order.
        let classes = f
            .classes
            .into_iter()
            .map(|(name, c)| ClassRow {
                name: ClassName::new(&name),
                prefix: c.prefix,
                fallback: match c.fallback {
                    file::Fallback::None => Fallback::None,
                    file::Fallback::Some(n) => Fallback::Class(ClassName::new(&n)),
                    file::Fallback::Identity => Fallback::Identity,
                },
                recipe: c.recipe,
                side: c.side,
                speeds: c.speeds.into_iter().collect(),
                bands: Bands {
                    speed_mps: c.bands.speed_mps.into_iter().collect(),
                    stride_m: c.bands.stride_m,
                    spine_pitch_deg: c.bands.spine_pitch_deg,
                    asymmetry: c.bands.asymmetry,
                },
                modifiers: c.modifiers.clamped(),
            })
            .collect();
        let rules = f
            .rules
            .into_iter()
            .map(|r| ChooserRule {
                class: ClassName::new(&r.class),
                when: r.when.into_iter().map(|c| Cond { field: c.field, enter: c.enter, exit: c.exit }).collect(),
                side: r.side,
            })
            .collect();
        let mut table = Self { base_prefix: f.base_prefix, default_class: ClassName::new(&f.default_class), classes, rules, digest: 0 };
        table.validate()?;
        table.digest = table.canonical_digest();
        Ok(table)
    }

    #[cfg(feature = "bake")]
    fn validate(&self) -> Result<(), StyleError> {
        if self.class(&self.default_class).is_none() {
            return Err(StyleError::UnknownDefaultClass(self.default_class.to_string()));
        }
        for row in &self.classes {
            if let Fallback::Class(n) = &row.fallback
                && self.class(n).is_none()
            {
                return Err(StyleError::UnknownFallback { class: row.name.to_string(), fallback: n.to_string() });
            }
        }
        for (i, start) in self.classes.iter().enumerate() {
            let mut at = start;
            for _ in 0..=self.classes.len() {
                let Fallback::Class(n) = &at.fallback else { break };
                let Some(next) = self.class(n) else { break };
                if usize::from(next.0) == i {
                    return Err(StyleError::FallbackCycle(start.name.to_string()));
                }
                at = &self.classes[usize::from(next.0)];
            }
        }
        for (i, rule) in self.rules.iter().enumerate() {
            if self.class(&rule.class).is_none() {
                return Err(StyleError::UnknownRuleClass { rule: i, class: rule.class.to_string() });
            }
            for c in &rule.when {
                if let Some(exit) = c.exit
                    && exit > c.enter
                {
                    return Err(StyleError::ExitAboveEnter { rule: i, enter: c.enter, exit });
                }
            }
        }
        Ok(())
    }

    /// Stable hash over postcard of the table with `digest` = 0.
    #[cfg(feature = "bake")]
    fn canonical_digest(&self) -> u64 {
        let mut canon = self.clone();
        canon.digest = 0;
        // A table of strings, floats and enums always serialises.
        let bytes = postcard::to_stdvec(&canon).unwrap_or_default();
        let d = sf_bake::Digest::of(&bytes);
        let mut first = [0u8; 8];
        first.copy_from_slice(&d.0[..8]);
        u64::from_le_bytes(first)
    }

    /// The id of the class named `name`.
    pub fn class(&self, name: &ClassName) -> Option<ClassId> {
        let i = self.classes.binary_search_by(|row| row.name.cmp(name)).ok()?;
        u16::try_from(i).ok().map(ClassId)
    }

    /// The row of `class`.
    pub fn row(&self, class: ClassId) -> Option<&ClassRow> {
        self.classes.get(usize::from(class.0))
    }

    /// Class slot for base slot `slot` (e.g. `loco.walk.cycle`): the class's
    /// prefix, then its `Fallback::Class` chain; the first slot `has` knows
    /// wins. `None` = the base slot: `slot` outside `base_prefix`, or the
    /// chain ends (`Fallback::None`, `Fallback::Identity`, a class with no
    /// prefix).
    pub fn resolve(&self, class: ClassId, slot: &str, has: impl Fn(&str) -> bool) -> Option<String> {
        let rest = slot.strip_prefix(self.base_prefix.as_str())?;
        let mut at = class;
        for _ in 0..=self.classes.len() {
            let row = self.row(at)?;
            let prefix = row.prefix.as_deref()?;
            let candidate = format!("{prefix}{rest}");
            if has(&candidate) {
                return Some(candidate);
            }
            let Fallback::Class(next) = &row.fallback else { return None };
            at = self.class(next)?;
        }
        None
    }

    /// The identity class: the profile's `style_id`, else `default_class`.
    fn identity(&self, p: &MotionProfile) -> ClassId {
        self.class(&p.style_id).or_else(|| self.class(&self.default_class)).unwrap_or_default()
    }

    /// Spawn and load: the first entering row, else the identity; no hold.
    pub fn initial(&self, p: &MotionProfile, tick: u64) -> StyleState {
        let identity = self.identity(p);
        let active = self
            .rules
            .iter()
            .find_map(|r| (side_ok(r, p) && r.when.iter().all(|c| p.field(c.field) >= c.enter)).then(|| self.class(&r.class)).flatten())
            .unwrap_or(identity);
        StyleState { active, identity, since_tick: tick }
    }
}

// --------------------------------------------------------------- chooser

/// The chooser's state per character (replicated).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StyleState {
    pub active: ClassId,
    pub identity: ClassId,
    /// Tick of the last change of `active`.
    pub since_tick: u64,
}

/// Chooser settings (defaults: [`CHOOSE_HOLD_S`] at the caller's tick rate,
/// [`CHOOSE_HYSTERESIS`]).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChooseParams {
    /// Ticks `active` is held before it may change.
    pub hold_ticks: u64,
    /// `exit` of a condition that gives none: `enter - hysteresis`.
    pub hysteresis: f32,
}

fn side_ok(r: &ChooserRule, p: &MotionProfile) -> bool {
    r.side.is_none_or(|s| s == p.injury_side)
}

/// One host tick of the chooser. Rows in order: a row whose class is active
/// holds while every condition stays at or above its exit; a row enters when
/// every condition reaches `enter`; side rows match `injury_side`. The first
/// holding or entering row is wanted (none: the identity); `active` changes
/// only once it has been held `hold_ticks`. Pure and deterministic.
pub fn choose(table: &StyleTable, p: &MotionProfile, prev: &StyleState, tick: u64, c: ChooseParams) -> StyleState {
    let identity = table.identity(p);
    let wanted = table
        .rules
        .iter()
        .find_map(|r| {
            if !side_ok(r, p) {
                return None;
            }
            let id = table.class(&r.class)?;
            let enters = r.when.iter().all(|k| p.field(k.field) >= k.enter);
            let holds = id == prev.active && r.when.iter().all(|k| p.field(k.field) >= k.exit.unwrap_or(k.enter - c.hysteresis));
            (enters || holds).then_some(id)
        })
        .unwrap_or(identity);
    if wanted != prev.active && tick.saturating_sub(prev.since_tick) >= c.hold_ticks {
        StyleState { active: wanted, identity, since_tick: tick }
    } else {
        StyleState { active: prev.active, identity, since_tick: prev.since_tick }
    }
}
