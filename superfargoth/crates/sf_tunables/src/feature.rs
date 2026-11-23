//! Feature switches (slice FS-1a). Spec: `docs/specs/feature-switches.md` "The switch".
//!
//! A [`FeatureSwitch`] is metadata on an existing `Bool` tunable (off = `false`) or `Choice`
//! tunable (off = one of its choices, such as `off`): a label, a [`Category`], what turning it
//! off does ([`Effect`]), the timers that measure its cost ([`Cost`]) and the other switches it
//! [`requires`](FeatureSwitch::requires). The switch's name is its tunable's (one namespace);
//! its value, layer, scope and lifetime are the tunable's.
//!
//! [`FeatureSwitches`] is the registry: it checks each switch against the [`Tunables`] store,
//! records the features the app did not build this run ([`FeatureSwitches::mark_not_built`])
//! and reports each switch's [`SwitchState`]: on or off, the layer that decided it, and whether
//! the feature is [`Available`](Availability::Available), not built, or blocked by a required
//! switch that is not running.

use std::collections::HashMap;

use serde::Serialize;
use thiserror::Error;

use crate::{ArgKind, Def, Group, SetBy, TunableId, Tunables, Value};

/// Where a switch is listed: the preset groups, plus features outside them.
///
/// A separate enum rather than new [`Group`] variants: `Group` is a tunable's preset
/// (scalability) group with exactly one `sg.*` driver each ([`Group::driver`]), and there is no
/// `sg.world` or `sg.debug` driver; a tunable tagged with such a group would claim a preset
/// driver that does not exist.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    ViewDistance,
    Shadows,
    Lighting,
    Post,
    Textures,
    Effects,
    Characters,
    Water,
    Sky,
    Foliage,
    /// World features outside the preset groups (streaming, groundcover placement, AI).
    World,
    /// Diagnostics and dev-only features (overlays, probes, timers).
    Debug,
}

impl Category {
    /// Every category, in panel order.
    pub const ALL: [Category; 12] = [
        Category::ViewDistance,
        Category::Shadows,
        Category::Lighting,
        Category::Post,
        Category::Textures,
        Category::Effects,
        Category::Characters,
        Category::Water,
        Category::Sky,
        Category::Foliage,
        Category::World,
        Category::Debug,
    ];

    /// The category of a preset group.
    pub const fn from_group(group: Group) -> Self {
        match group {
            Group::ViewDistance => Category::ViewDistance,
            Group::Shadows => Category::Shadows,
            Group::Lighting => Category::Lighting,
            Group::Post => Category::Post,
            Group::Textures => Category::Textures,
            Group::Effects => Category::Effects,
            Group::Characters => Category::Characters,
            Group::Water => Category::Water,
            Group::Sky => Category::Sky,
            Group::Foliage => Category::Foliage,
        }
    }

    /// The preset group, if it is one.
    pub const fn group(self) -> Option<Group> {
        Some(match self {
            Category::ViewDistance => Group::ViewDistance,
            Category::Shadows => Group::Shadows,
            Category::Lighting => Group::Lighting,
            Category::Post => Group::Post,
            Category::Textures => Group::Textures,
            Category::Effects => Group::Effects,
            Category::Characters => Group::Characters,
            Category::Water => Group::Water,
            Category::Sky => Group::Sky,
            Category::Foliage => Group::Foliage,
            Category::World | Category::Debug => return None,
        })
    }

    /// The snake-case name (`view_distance`), as serde writes it.
    pub const fn name(self) -> &'static str {
        match self {
            Category::ViewDistance => "view_distance",
            Category::Shadows => "shadows",
            Category::Lighting => "lighting",
            Category::Post => "post",
            Category::Textures => "textures",
            Category::Effects => "effects",
            Category::Characters => "characters",
            Category::Water => "water",
            Category::Sky => "sky",
            Category::Foliage => "foliage",
            Category::World => "world",
            Category::Debug => "debug",
        }
    }
}

impl From<Group> for Category {
    fn from(group: Group) -> Self {
        Category::from_group(group)
    }
}

/// What turning the feature off does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    /// The work is skipped: off saves its cost.
    #[default]
    Skips,
    /// Only the result is hidden: the work may still run (off says nothing about cost).
    Hides,
}

/// A timer that measures the feature's cost.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Cost {
    /// A GPU pass timer label.
    Gpu(&'static str),
    /// A CPU system or set span name.
    Cpu(&'static str),
}

/// The tunable value that means "off".
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OffValue {
    Bool(bool),
    /// A choice (lowercase).
    Choice(&'static str),
}

impl OffValue {
    /// Whether `value` is this off value.
    pub fn matches(self, value: &Value) -> bool {
        match (self, value) {
            (OffValue::Bool(off), Value::Bool(v)) => *v == off,
            (OffValue::Choice(off), Value::Str(v)) => v.eq_ignore_ascii_case(off),
            _ => false,
        }
    }

    /// The off value as a tunable value.
    pub fn to_value(self) -> Value {
        match self {
            OffValue::Bool(b) => Value::Bool(b),
            OffValue::Choice(s) => Value::Str(s.to_ascii_lowercase()),
        }
    }
}

/// A feature switch, declared as a `const` next to the tunable it drives.
///
/// ```
/// use sf_tunables::{Cost, Def, Effect, FeatureSwitch, Group, def};
///
/// const FROXELS_ENABLED: Def<bool> = def!(bool "sky.froxels.enabled" = true).group(Group::Sky);
/// const FROXELS: FeatureSwitch = FeatureSwitch::new(&FROXELS_ENABLED)
///     .label("Volumetric fog (froxels)")
///     .effect(Effect::Skips)
///     .cost(&[Cost::Gpu("froxel"), Cost::Cpu("sf_atmos::froxel_inject")])
///     .requires(&["sky.fog.enabled"])
///     .at(file!(), line!());
/// assert_eq!(FROXELS.name(), "sky.froxels.enabled");
/// ```
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FeatureSwitch {
    name: &'static str,
    off: OffValue,
    label: &'static str,
    category: Category,
    effect: Effect,
    cost: &'static [Cost],
    requires: &'static [&'static str],
    location: Option<(&'static str, u32)>,
    startup_only: Option<&'static str>,
}

impl FeatureSwitch {
    /// A switch on a `Bool` tunable: off is `false`. The category starts as the tunable's
    /// preset group ([`Category::World`] without one).
    pub const fn new(def: &Def<bool>) -> Self {
        Self::with_off(def.name(), OffValue::Bool(false), def.meta().group)
    }

    /// A switch on a `Choice` tunable: off is the choice `off` (lowercase, one of its options).
    pub const fn choice(def: &Def<&'static str>, off: &'static str) -> Self {
        Self::with_off(def.name(), OffValue::Choice(off), def.meta().group)
    }

    /// A switch on a tunable registered by name only (no [`Def`]), such as a console variable.
    pub const fn named(name: &'static str, off: OffValue) -> Self {
        Self::with_off(name, off, None)
    }

    const fn with_off(name: &'static str, off: OffValue, group: Option<Group>) -> Self {
        Self {
            name,
            off,
            label: "",
            category: match group {
                Some(g) => Category::from_group(g),
                None => Category::World,
            },
            effect: Effect::Skips,
            cost: &[],
            requires: &[],
            location: None,
            startup_only: None,
        }
    }

    /// The name people read (the panel row); the tunable's name when not set.
    pub const fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    pub const fn category(mut self, category: Category) -> Self {
        self.category = category;
        self
    }

    pub const fn effect(mut self, effect: Effect) -> Self {
        self.effect = effect;
        self
    }

    /// The timers that measure its cost.
    pub const fn cost(mut self, cost: &'static [Cost]) -> Self {
        self.cost = cost;
        self
    }

    /// Other switches (by name) that must be on and running for this feature to run.
    pub const fn requires(mut self, requires: &'static [&'static str]) -> Self {
        self.requires = requires;
        self
    }

    /// The switch only reports a choice made at startup (its tunable says what this run built):
    /// a flip is refused with `hint` (how to choose otherwise, e.g. a launch flag) and changes
    /// nothing.
    pub const fn startup_only(mut self, hint: &'static str) -> Self {
        self.startup_only = Some(hint);
        self
    }

    /// Where it is declared (`file!()`, `line!()`).
    pub const fn at(mut self, file: &'static str, line: u32) -> Self {
        self.location = Some((file, line));
        self
    }

    /// The switch's (and its tunable's) name.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn off_value(&self) -> OffValue {
        self.off
    }

    /// The label, or the name when no label was given.
    pub const fn label_text(&self) -> &'static str {
        if self.label.is_empty() {
            self.name
        } else {
            self.label
        }
    }

    pub const fn category_of(&self) -> Category {
        self.category
    }

    pub const fn effect_of(&self) -> Effect {
        self.effect
    }

    pub const fn costs(&self) -> &'static [Cost] {
        self.cost
    }

    pub const fn required(&self) -> &'static [&'static str] {
        self.requires
    }

    pub const fn location(&self) -> Option<(&'static str, u32)> {
        self.location
    }

    /// The hint of a startup-only switch ([`Self::startup_only`]), `None` for a live one.
    pub const fn startup_hint(&self) -> Option<&'static str> {
        self.startup_only
    }
}

/// Why a switch registration or mark failed.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum SwitchError {
    #[error("switch `{0}` is already registered")]
    Duplicate(String),
    #[error("switch `{0}`: no tunable of that name is registered")]
    UnknownTunable(String),
    #[error("switch `{name}`: the tunable is {kind}, not a bool or a choice with `{off}`")]
    WrongKind {
        name: String,
        kind: String,
        off: String,
    },
    #[error("no switch `{0}` is registered")]
    UnknownSwitch(String),
}

/// Whether the feature can run this session.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Availability<'a> {
    Available,
    /// The app did not build it this run (startup `--off`, missing hardware); the switch can't
    /// bring it back until a restart.
    NotBuilt { reason: &'a str },
    /// A required switch is off, not built, blocked itself, or not registered.
    Blocked { by: &'static str },
}

/// A switch's state now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwitchState<'a> {
    /// The tunable's effective value is not the off value.
    pub on: bool,
    /// The layer the effective value comes from.
    pub layer: SetBy,
    pub availability: Availability<'a>,
}

impl SwitchState<'_> {
    /// On and available: the feature runs.
    pub fn running(&self) -> bool {
        self.on && self.availability == Availability::Available
    }
}

#[derive(Clone, Debug)]
struct Entry {
    switch: &'static FeatureSwitch,
    tunable: TunableId,
    not_built: Option<String>,
}

/// The registry of feature switches, in registration order.
#[derive(Clone, Debug, Default)]
pub struct FeatureSwitches {
    list: Vec<Entry>,
    by_name: HashMap<String, usize>,
}

impl FeatureSwitches {
    /// Registers a switch: its tunable must be registered in `store` (a `Bool`, or a `Choice`
    /// that has the off value) and its name must be new. `requires` names are resolved when the
    /// state is read, so switches may be registered in any order
    /// ([`FeatureSwitches::unresolved_requires`] lists the ones still missing).
    pub fn register(
        &mut self,
        switch: &'static FeatureSwitch,
        store: &Tunables,
    ) -> Result<(), SwitchError> {
        let name = switch.name.to_ascii_lowercase();
        if self.by_name.contains_key(&name) {
            return Err(SwitchError::Duplicate(name));
        }
        let tunable = store
            .id(&name)
            .ok_or_else(|| SwitchError::UnknownTunable(name.clone()))?;
        let kind = store.var(tunable).kind();
        let fits = match (switch.off, kind) {
            (OffValue::Bool(_), ArgKind::Bool) => true,
            (OffValue::Choice(off), ArgKind::Choice(options)) => {
                options.iter().any(|o| o.eq_ignore_ascii_case(off))
            }
            _ => false,
        };
        if !fits {
            return Err(SwitchError::WrongKind {
                name,
                kind: kind.describe(),
                off: switch.off.to_value().to_string(),
            });
        }
        self.by_name.insert(name, self.list.len());
        self.list.push(Entry {
            switch,
            tunable,
            not_built: None,
        });
        Ok(())
    }

    /// The app says this feature was not built this run, and why.
    pub fn mark_not_built(&mut self, name: &str, reason: &str) -> Result<(), SwitchError> {
        let i = self.index(name)?;
        self.list[i].not_built = Some(reason.to_owned());
        Ok(())
    }

    /// The switch registered under `name` (case-insensitive).
    pub fn get(&self, name: &str) -> Option<&'static FeatureSwitch> {
        self.index(name).ok().map(|i| self.list[i].switch)
    }

    /// The tunable a registered switch drives.
    pub fn tunable(&self, name: &str) -> Option<TunableId> {
        self.index(name).ok().map(|i| self.list[i].tunable)
    }

    /// A switch's state; `None` when no switch of that name is registered. `store` must be the
    /// one the switches were registered against.
    pub fn state(&self, store: &Tunables, name: &str) -> Option<SwitchState<'_>> {
        self.index(name).ok().map(|i| self.state_at(store, i))
    }

    /// Whether the feature runs now (on and available); `false` for an unknown name.
    pub fn running(&self, store: &Tunables, name: &str) -> bool {
        self.index(name)
            .is_ok_and(|i| self.running_at(store, i, self.list.len()))
    }

    /// Every switch, in registration order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &'static FeatureSwitch> + '_ {
        self.list.iter().map(|e| e.switch)
    }

    /// The switches of one category, in registration order.
    pub fn by_category(
        &self,
        category: Category,
    ) -> impl Iterator<Item = &'static FeatureSwitch> + '_ {
        self.iter().filter(move |s| s.category == category)
    }

    /// `(switch, required name)` for every `requires` entry that names no registered switch.
    pub fn unresolved_requires(&self) -> Vec<(&'static str, &'static str)> {
        self.list
            .iter()
            .flat_map(|e| e.switch.requires.iter().map(move |r| (e.switch.name, *r)))
            .filter(|(_, r)| self.index(r).is_err())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    fn index(&self, name: &str) -> Result<usize, SwitchError> {
        self.by_name
            .get(&name.to_ascii_lowercase())
            .copied()
            .ok_or_else(|| SwitchError::UnknownSwitch(name.to_owned()))
    }

    fn is_on(&self, store: &Tunables, i: usize) -> (bool, SetBy) {
        let e = &self.list[i];
        let tunable = store.var(e.tunable);
        (!e.switch.off.matches(tunable.value()), tunable.set_by())
    }

    fn state_at(&self, store: &Tunables, i: usize) -> SwitchState<'_> {
        let (on, layer) = self.is_on(store, i);
        let e = &self.list[i];
        let availability = if let Some(reason) = &e.not_built {
            Availability::NotBuilt { reason }
        } else if let Some(by) = self.blocker(store, i, self.list.len()) {
            Availability::Blocked { by }
        } else {
            Availability::Available
        };
        SwitchState {
            on,
            layer,
            availability,
        }
    }

    /// The first required switch that does not run. `depth` bounds the walk (a `requires`
    /// cycle reads as blocked instead of looping).
    fn blocker(&self, store: &Tunables, i: usize, depth: usize) -> Option<&'static str> {
        let switch = self.list[i].switch;
        switch.requires.iter().copied().find(|r| {
            depth == 0
                || self
                    .index(r)
                    .map_or(true, |j| !self.running_at(store, j, depth - 1))
        })
    }

    fn running_at(&self, store: &Tunables, i: usize, depth: usize) -> bool {
        self.is_on(store, i).0
            && self.list[i].not_built.is_none()
            && self.blocker(store, i, depth).is_none()
    }
}
