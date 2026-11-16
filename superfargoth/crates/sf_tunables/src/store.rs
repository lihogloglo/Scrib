//! The registry: named, typed tunables with one optional value per layer, the effective value
//! being the strongest layer set, and change tracking. Spec: `docs/specs/tunables.md`
//! "Values: layers". `sf_console` re-exports it as its variable store (`Vars`, `Var`, `VarId`,
//! `VarSpec`).

use std::collections::HashMap;
use std::fmt::Write as _;

use thiserror::Error;

use crate::meta::{Group, Lifetime, Meta, Net, Scope, Source, Unit};
use crate::value::{ArgKind, Value};

/// Layers a value can be set on, weakest first (UE `ECVF_SetBy*`). Each tunable holds one
/// optional value per layer and the strongest one set is in effect, so a config file read after
/// the command line can't undo `--off`, a value typed in the console wins over everything, and
/// clearing a layer uncovers the one below.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SetBy {
    /// The registered default.
    #[default]
    Default,
    /// A mod's record (load order: the later mod wins).
    Mod,
    /// The level a preset driver wrote.
    Preset,
    /// The user's config: options menu, machine file, profile (was `Config`).
    User,
    /// The app's command line (`--off ssao`, `--set name value`, env aliases, perf arms).
    CommandLine,
    /// Game or engine code at run time (a system decides, a script).
    Code,
    /// A line typed in the console, or BRP.
    Console,
}

impl SetBy {
    /// Every layer, weakest first.
    pub const ALL: [SetBy; 7] = [
        SetBy::Default,
        SetBy::Mod,
        SetBy::Preset,
        SetBy::User,
        SetBy::CommandLine,
        SetBy::Code,
        SetBy::Console,
    ];

    /// The old name of [`SetBy::User`] (a config file read at start).
    #[allow(non_upper_case_globals)]
    pub const Config: SetBy = SetBy::User;

    pub fn describe(self) -> &'static str {
        match self {
            SetBy::Default => "default",
            SetBy::Mod => "mod",
            SetBy::Preset => "preset",
            SetBy::User => "config",
            SetBy::CommandLine => "command line",
            SetBy::Code => "code",
            SetBy::Console => "console",
        }
    }

    const fn index(self) -> usize {
        self as usize
    }
}

/// What a set did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetOutcome {
    /// The effective value changed.
    Changed,
    /// Same effective value as before; the layer still holds the value.
    Unchanged,
    /// Stored on its layer, but a stronger layer is in effect.
    Outranked(SetBy),
}

impl SetOutcome {
    pub fn changed(self) -> bool {
        self == SetOutcome::Changed
    }
}

/// Why a registration failed.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum RegisterError {
    #[error("invalid name {0:?}: use a lowercase letter, then letters, digits, `_`, `.` or `-`")]
    InvalidName(String),
    #[error("`{0}` is already registered")]
    Duplicate(String),
    #[error("default {default} is not {kind}")]
    BadDefault { default: String, kind: String },
    #[error("`{0}` is a number and needs a range")]
    NoRange(String),
    #[error("`{name}` is registered as {registered}, not {wanted}")]
    WrongKind {
        name: String,
        registered: String,
        wanted: String,
    },
    #[error("`{0}` is not registered")]
    Unknown(String),
    #[error("too many tunables")]
    Full,
}

/// Handle to a registered tunable: an index, cheap to read every frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TunableId(pub(crate) u32);

/// What to register: name, kind, default, help, and the [`Meta`] (unit, source, lifetime...).
/// [`crate::Def`] builds one from `const` data; console variables build one directly.
#[must_use]
#[derive(Clone, Debug, PartialEq)]
pub struct Spec {
    pub(crate) name: String,
    pub(crate) help: String,
    pub(crate) kind: ArgKind,
    pub(crate) default: Value,
    pub(crate) meta: Meta,
}

impl Spec {
    /// Any kind; [`Tunables::register`] checks that the default fits.
    pub fn new(name: &str, kind: ArgKind, default: Value) -> Self {
        Self {
            name: name.to_owned(),
            help: String::new(),
            kind,
            default,
            meta: Meta::default(),
        }
    }

    /// An on/off switch.
    pub fn bool(name: &str, default: bool) -> Self {
        Self::new(name, ArgKind::Bool, Value::Bool(default))
    }

    pub fn int(name: &str, default: i64) -> Self {
        Self::new(name, ArgKind::int(), Value::Int(default))
    }

    pub fn float(name: &str, default: f64) -> Self {
        Self::new(name, ArgKind::float(), Value::Float(default))
    }

    /// A fixed pair (`a,b`).
    pub fn float2(name: &str, default: [f64; 2]) -> Self {
        let kind = ArgKind::Float2 {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        };
        Self::new(name, kind, Value::Float2(default))
    }

    /// A fixed triple (`a,b,c`).
    pub fn float3(name: &str, default: [f64; 3]) -> Self {
        let kind = ArgKind::Float3 {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        };
        Self::new(name, kind, Value::Float3(default))
    }

    pub fn text(name: &str, default: &str) -> Self {
        Self::new(name, ArgKind::Str, Value::Str(default.to_owned()))
    }

    /// One of `options` (case-insensitive, stored lowercase).
    pub fn choice(name: &str, options: &[&str], default: &str) -> Self {
        Self::new(
            name,
            ArgKind::choice(options),
            Value::Str(default.to_ascii_lowercase()),
        )
    }

    /// Limits an integer to `min..=max`.
    pub fn int_range(mut self, min: i64, max: i64) -> Self {
        self.kind = ArgKind::Int { min, max };
        self
    }

    /// Limits a float (or each component of a pair or triple) to `min..=max`.
    pub fn float_range(mut self, min: f64, max: f64) -> Self {
        self.kind = match self.kind {
            ArgKind::Float2 { .. } => ArgKind::Float2 { min, max },
            ArgKind::Float3 { .. } => ArgKind::Float3 { min, max },
            _ => ArgKind::Float { min, max },
        };
        self
    }

    pub fn help(mut self, text: &str) -> Self {
        text.clone_into(&mut self.help);
        self
    }

    pub fn unit(mut self, unit: Unit) -> Self {
        self.meta.unit = unit;
        self
    }

    pub fn source(mut self, source: Source) -> Self {
        self.meta.source = source;
        self
    }

    pub fn group(mut self, group: Group) -> Self {
        self.meta.group = Some(group);
        self
    }

    pub fn scope(mut self, scope: Scope) -> Self {
        self.meta.scope = scope;
        self
    }

    pub fn net(mut self, net: Net) -> Self {
        self.meta.net = net;
        self
    }

    pub fn lifetime(mut self, lifetime: Lifetime) -> Self {
        self.meta.lifetime = lifetime;
        self
    }

    /// All the metadata at once.
    pub fn meta(mut self, meta: Meta) -> Self {
        self.meta = meta;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// The same spec under another name (the console lowercases and checks names first).
    pub fn renamed(mut self, name: &str) -> Self {
        name.clone_into(&mut self.name);
        self
    }

    pub fn kind(&self) -> &ArgKind {
        &self.kind
    }

    pub fn default_value(&self) -> &Value {
        &self.default
    }
}

/// One layer's value and an optional note saying who set it and why (a mod's name and its
/// own source, a preset level, a file and line).
#[derive(Clone, Debug, PartialEq)]
pub struct Layer {
    pub value: Value,
    pub note: Option<String>,
}

/// A registered tunable.
#[derive(Clone, Debug, PartialEq)]
pub struct Tunable {
    spec: Spec,
    /// Indexed by [`SetBy`]; `Default` is never stored here (it is `spec.default`).
    layers: [Option<Layer>; 7],
    value: Value,
    set_by: SetBy,
    generation: u64,
}

impl Tunable {
    pub fn name(&self) -> &str {
        &self.spec.name
    }

    pub fn help(&self) -> &str {
        &self.spec.help
    }

    pub fn kind(&self) -> &ArgKind {
        &self.spec.kind
    }

    pub fn default(&self) -> &Value {
        &self.spec.default
    }

    pub fn meta(&self) -> &Meta {
        &self.spec.meta
    }

    /// The effective value.
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// The layer the effective value comes from.
    pub fn set_by(&self) -> SetBy {
        self.set_by
    }

    /// How many times the effective value changed.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// What `layer` holds (`Default` always holds the default).
    pub fn layer(&self, layer: SetBy) -> Option<&Value> {
        match layer {
            SetBy::Default => Some(&self.spec.default),
            l => self.layers[l.index()].as_ref().map(|l| &l.value),
        }
    }

    /// The note on `layer`'s value, if any.
    pub fn note(&self, layer: SetBy) -> Option<&str> {
        self.layers[layer.index()]
            .as_ref()
            .and_then(|l| l.note.as_deref())
    }

    /// The layers that hold a value, weakest first, `Default` included.
    pub fn layers(&self) -> impl Iterator<Item = (SetBy, &Value)> {
        SetBy::ALL
            .into_iter()
            .filter_map(|l| self.layer(l).map(|v| (l, v)))
    }

    /// `name = value (default d) [kind unit] (set by s): help`.
    pub fn describe(&self) -> String {
        let mut s = format!("{} = {}", self.name(), self.value);
        if self.value != self.spec.default {
            let _ = write!(s, " (default {})", self.spec.default);
        }
        let _ = write!(s, " [{}", self.spec.kind.describe());
        if self.spec.meta.unit != Unit::None {
            let _ = write!(s, " {}", self.spec.meta.unit.symbol());
        }
        s.push(']');
        // The console is the usual setter: only name the others.
        if !matches!(self.set_by, SetBy::Default | SetBy::Console) {
            let _ = write!(s, " (set by {})", self.set_by.describe());
        }
        if !self.spec.help.is_empty() {
            s += ": ";
            s += &self.spec.help;
        }
        s
    }

    /// Recomputes the effective value from the layers; true if it changed.
    fn settle(&mut self) -> bool {
        let (by, value) = SetBy::ALL[1..]
            .iter()
            .rev()
            .find_map(|&l| self.layers[l.index()].as_ref().map(|v| (l, &v.value)))
            .unwrap_or((SetBy::Default, &self.spec.default));
        self.set_by = by;
        if *value == self.value {
            return false;
        }
        self.value = value.clone();
        self.generation += 1;
        true
    }
}

/// Every registered tunable. Values change only through [`Tunables::set`] and friends, which
/// validate against the kind and record effective changes for [`Tunables::take_changes`].
#[derive(Clone, Debug, Default)]
pub struct Tunables {
    list: Vec<Tunable>,
    by_name: HashMap<String, TunableId>,
    /// Changed since the last `take_changes`, in order of first change.
    changed: Vec<TunableId>,
    pending: Vec<bool>,
    revision: u64,
}

/// Lowercases `name` and checks it: a lowercase letter, then letters, digits, `_`, `.`, `-`
/// (the console's rules).
pub fn check_name(name: &str) -> Result<String, RegisterError> {
    let lower = name.to_ascii_lowercase();
    let mut chars = lower.chars();
    let valid = chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '_' | '.' | '-'));
    if valid {
        Ok(lower)
    } else {
        Err(RegisterError::InvalidName(name.to_owned()))
    }
}

impl Tunables {
    /// Registers a tunable: checks the name (lowercased), that it is new, and that the
    /// default fits the kind.
    pub fn register(&mut self, mut spec: Spec) -> Result<TunableId, RegisterError> {
        spec.name = check_name(&spec.name)?;
        if self.by_name.contains_key(&spec.name) {
            return Err(RegisterError::Duplicate(spec.name));
        }
        if !spec.kind.accepts(&spec.default) {
            return Err(RegisterError::BadDefault {
                default: spec.default.to_string(),
                kind: spec.kind.describe(),
            });
        }
        let id = TunableId(u32::try_from(self.list.len()).map_err(|_| RegisterError::Full)?);
        self.by_name.insert(spec.name.clone(), id);
        self.list.push(Tunable {
            value: spec.default.clone(),
            spec,
            layers: Default::default(),
            set_by: SetBy::Default,
            generation: 0,
        });
        self.pending.push(false);
        Ok(id)
    }

    /// Finds a tunable by name (case-insensitive).
    pub fn id(&self, name: &str) -> Option<TunableId> {
        self.by_name.get(&name.to_ascii_lowercase()).copied()
    }

    pub fn var(&self, id: TunableId) -> &Tunable {
        &self.list[id.0 as usize]
    }

    /// The effective value.
    pub fn get(&self, id: TunableId) -> &Value {
        &self.var(id).value
    }

    /// A boolean's value (`false` for any other kind).
    pub fn flag(&self, id: TunableId) -> bool {
        self.get(id).as_bool().unwrap_or(false)
    }

    /// A float's value (integers widen; 0 for any other kind).
    pub fn float(&self, id: TunableId) -> f64 {
        self.get(id).as_float().unwrap_or(0.0)
    }

    /// An integer's value (0 for any other kind).
    pub fn int(&self, id: TunableId) -> i64 {
        self.get(id).as_int().unwrap_or(0)
    }

    /// A text or choice value (`""` for any other kind).
    pub fn text(&self, id: TunableId) -> &str {
        self.get(id).as_str().unwrap_or_default()
    }

    /// Sets a value from game code ([`SetBy::Code`]). Integers widen to float for floats.
    /// Returns whether the effective value changed.
    pub fn set(&mut self, id: TunableId, value: Value) -> Result<bool, String> {
        self.set_by(id, value, SetBy::Code).map(SetOutcome::changed)
    }

    /// Sets `layer`'s value. Integers widen to float for floats; a value the kind refuses
    /// (wrong type, out of range) is an error and changes nothing.
    pub fn set_by(
        &mut self,
        id: TunableId,
        value: Value,
        layer: SetBy,
    ) -> Result<SetOutcome, String> {
        self.set_noted(id, value, layer, None)
    }

    /// [`Tunables::set_by`] with a note on who set it and why (shown by the dump).
    pub fn set_noted(
        &mut self,
        id: TunableId,
        value: Value,
        layer: SetBy,
        note: Option<String>,
    ) -> Result<SetOutcome, String> {
        no_default(layer)?;
        let kind = &self.var(id).spec.kind;
        let value = match (kind, value) {
            (ArgKind::Float { .. }, v @ Value::Int(_)) => {
                Value::Float(v.as_float().expect("int widens"))
            }
            (_, v) => v,
        };
        if !kind.accepts(&value) {
            return Err(format!("expected {}, got {value}", kind.describe()));
        }
        Ok(self.store(id, Layer { value, note }, layer))
    }

    /// Parses and sets a value typed in the console ([`SetBy::Console`]).
    /// Returns whether the effective value changed.
    pub fn set_text(&mut self, id: TunableId, text: &str) -> Result<bool, String> {
        self.set_text_by(id, text, SetBy::Console)
            .map(SetOutcome::changed)
    }

    /// Parses and sets `layer`'s value (a config or command-line line).
    pub fn set_text_by(
        &mut self,
        id: TunableId,
        text: &str,
        layer: SetBy,
    ) -> Result<SetOutcome, String> {
        no_default(layer)?;
        let value = self.var(id).spec.kind.parse(text)?;
        Ok(self.store(id, Layer { value, note: None }, layer))
    }

    /// Clears one layer (peels it): the next one down takes effect. Returns whether the
    /// effective value changed. Clearing `Default` does nothing.
    pub fn clear(&mut self, id: TunableId, layer: SetBy) -> bool {
        if layer == SetBy::Default {
            return false;
        }
        self.list[id.0 as usize].layers[layer.index()] = None;
        self.settle(id)
    }

    /// Clears `layer` on every tunable (a preset driver rewriting its level, a mod unloaded).
    /// Returns how many effective values changed.
    pub fn clear_layer(&mut self, layer: SetBy) -> usize {
        (0..self.list.len())
            .filter(|&i| {
                self.clear(
                    TunableId(u32::try_from(i).expect("checked at register")),
                    layer,
                )
            })
            .count()
    }

    /// Clears every layer but `Default`: back to the default, which any layer may then set
    /// again. Returns whether the effective value changed.
    pub fn reset(&mut self, id: TunableId) -> bool {
        self.list[id.0 as usize].layers = Default::default();
        self.settle(id)
    }

    fn store(&mut self, id: TunableId, layer_value: Layer, layer: SetBy) -> SetOutcome {
        let var = &mut self.list[id.0 as usize];
        var.layers[layer.index()] = Some(layer_value);
        if self.settle(id) {
            SetOutcome::Changed
        } else {
            let top = self.var(id).set_by;
            if top > layer {
                SetOutcome::Outranked(top)
            } else {
                SetOutcome::Unchanged
            }
        }
    }

    fn settle(&mut self, id: TunableId) -> bool {
        let i = id.0 as usize;
        if !self.list[i].settle() {
            return false;
        }
        self.revision += 1;
        if !self.pending[i] {
            self.pending[i] = true;
            self.changed.push(id);
        }
        true
    }

    /// Total number of effective changes over all tunables (cheap "anything changed?" check).
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Tunables whose effective value changed since the last call, each once, in order of
    /// first change.
    pub fn take_changes(&mut self) -> Vec<TunableId> {
        for id in &self.changed {
            self.pending[id.0 as usize] = false;
        }
        std::mem::take(&mut self.changed)
    }

    /// In registration order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (TunableId, &Tunable)> {
        self.list
            .iter()
            .enumerate()
            .map(|(i, v)| (TunableId(u32::try_from(i).expect("checked at register")), v))
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

/// The default layer is the def's own: setting it is a mistake, not a no-op.
fn no_default(layer: SetBy) -> Result<(), String> {
    if layer == SetBy::Default {
        Err("the default layer can't be set; set a layer above it".into())
    } else {
        Ok(())
    }
}
