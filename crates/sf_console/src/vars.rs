//! Named, typed console variables with change tracking.

use std::fmt::Write as _;

use crate::value::{ArgKind, Value};

/// Who set a variable's current value, lowest priority first (UE `ECVF_SetBy*`). A set
/// from a lower source than the current one is ignored, so a config file read after the
/// command line can't undo `--off`, and a value typed in the console wins over everything.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SetBy {
    /// The registered default.
    #[default]
    Default,
    /// A config file read at start.
    Config,
    /// The app's command line (`--off ssao`, `+set name value`).
    CommandLine,
    /// Game or engine code at run time (a settings menu, a script).
    Code,
    /// A line typed in the console.
    Console,
}

impl SetBy {
    pub fn describe(self) -> &'static str {
        match self {
            SetBy::Default => "default",
            SetBy::Config => "config",
            SetBy::CommandLine => "command line",
            SetBy::Code => "code",
            SetBy::Console => "console",
        }
    }
}

/// What a set did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetOutcome {
    /// The value changed.
    Changed,
    /// Same value as before; the source is still recorded.
    Unchanged,
    /// Ignored: the current value came from a higher-priority source.
    Outranked(SetBy),
}

impl SetOutcome {
    pub fn changed(self) -> bool {
        self == SetOutcome::Changed
    }
}

/// Handle to a registered variable: index-based, cheap to read every frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VarId(u32);

/// What to register: name, type, default, help.
#[must_use]
#[derive(Clone, Debug, PartialEq)]
pub struct VarSpec {
    pub(crate) name: String,
    pub(crate) help: String,
    pub(crate) kind: ArgKind,
    pub(crate) default: Value,
}

impl VarSpec {
    fn new(name: &str, kind: ArgKind, default: Value) -> Self {
        Self {
            name: name.to_owned(),
            help: String::new(),
            kind,
            default,
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

    /// Limits an integer variable to `min..=max`.
    pub fn int_range(mut self, min: i64, max: i64) -> Self {
        self.kind = ArgKind::Int { min, max };
        self
    }

    /// Limits a float variable to `min..=max`.
    pub fn float_range(mut self, min: f64, max: f64) -> Self {
        self.kind = ArgKind::Float { min, max };
        self
    }

    pub fn help(mut self, text: &str) -> Self {
        text.clone_into(&mut self.help);
        self
    }
}

/// A registered variable.
#[derive(Clone, Debug, PartialEq)]
pub struct Var {
    spec: VarSpec,
    value: Value,
    set_by: SetBy,
    generation: u64,
}

impl Var {
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

    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Who set the current value.
    pub fn set_by(&self) -> SetBy {
        self.set_by
    }

    /// How many times the value changed.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// `name = value (default d) [kind] (set by s): help`.
    pub fn describe(&self) -> String {
        let mut s = format!("{} = {}", self.name(), self.value);
        if self.value != self.spec.default {
            let _ = write!(s, " (default {})", self.spec.default);
        }
        let _ = write!(s, " [{}]", self.spec.kind.describe());
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
}

/// Every registered variable. Values change only through [`Vars::set`] and friends, which
/// validate against the variable's kind and record the change for [`Vars::take_changes`].
#[derive(Clone, Debug, Default)]
pub struct Vars {
    list: Vec<Var>,
    /// Changed since the last `take_changes`, in order of first change.
    changed: Vec<VarId>,
    pending: Vec<bool>,
    revision: u64,
}

impl Vars {
    /// The console checks the name; this checks the default.
    pub(crate) fn register(&mut self, spec: VarSpec) -> Result<VarId, String> {
        if !spec.kind.accepts(&spec.default) {
            return Err(format!(
                "default {} is not {}",
                spec.default,
                spec.kind.describe()
            ));
        }
        let id =
            VarId(u32::try_from(self.list.len()).map_err(|_| "too many variables".to_owned())?);
        self.list.push(Var {
            value: spec.default.clone(),
            spec,
            set_by: SetBy::Default,
            generation: 0,
        });
        self.pending.push(false);
        Ok(id)
    }

    /// Finds a variable by name (case-insensitive).
    pub fn id(&self, name: &str) -> Option<VarId> {
        self.iter()
            .find(|(_, v)| v.name().eq_ignore_ascii_case(name))
            .map(|(id, _)| id)
    }

    pub fn var(&self, id: VarId) -> &Var {
        &self.list[id.0 as usize]
    }

    pub fn get(&self, id: VarId) -> &Value {
        &self.var(id).value
    }

    /// A boolean variable's value (`false` for any other kind).
    pub fn flag(&self, id: VarId) -> bool {
        self.get(id).as_bool().unwrap_or(false)
    }

    /// A float variable's value (integers widen; 0 for any other kind).
    pub fn float(&self, id: VarId) -> f64 {
        self.get(id).as_float().unwrap_or(0.0)
    }

    /// An integer variable's value (0 for any other kind).
    pub fn int(&self, id: VarId) -> i64 {
        self.get(id).as_int().unwrap_or(0)
    }

    /// A text or choice variable's value (`""` for any other kind).
    pub fn text(&self, id: VarId) -> &str {
        self.get(id).as_str().unwrap_or_default()
    }

    /// Sets a value from game code ([`SetBy::Code`]). Integers widen to float for float
    /// variables. Returns whether the value changed.
    pub fn set(&mut self, id: VarId, value: Value) -> Result<bool, String> {
        self.set_by(id, value, SetBy::Code).map(SetOutcome::changed)
    }

    /// Sets a value on behalf of `source`; ignored if the current value came from a
    /// higher-priority source. Integers widen to float for float variables.
    pub fn set_by(&mut self, id: VarId, value: Value, source: SetBy) -> Result<SetOutcome, String> {
        let var = &self.list[id.0 as usize];
        let value = match (&var.spec.kind, value) {
            (ArgKind::Float { .. }, v @ Value::Int(_)) => {
                Value::Float(v.as_float().expect("int widens"))
            }
            (_, v) => v,
        };
        if !var.spec.kind.accepts(&value) {
            return Err(format!(
                "expected {}, got {value}",
                var.spec.kind.describe()
            ));
        }
        Ok(self.store(id, value, source))
    }

    /// Parses and sets a value typed in the console ([`SetBy::Console`]).
    /// Returns whether the value changed.
    pub fn set_text(&mut self, id: VarId, text: &str) -> Result<bool, String> {
        self.set_text_by(id, text, SetBy::Console)
            .map(SetOutcome::changed)
    }

    /// Parses and sets a value on behalf of `source` (a config or command-line line).
    pub fn set_text_by(
        &mut self,
        id: VarId,
        text: &str,
        source: SetBy,
    ) -> Result<SetOutcome, String> {
        let value = self.var(id).spec.kind.parse(text)?;
        Ok(self.store(id, value, source))
    }

    /// Back to the default, which any source may then set again. Returns whether the
    /// value changed.
    pub fn reset(&mut self, id: VarId) -> bool {
        let default = self.var(id).spec.default.clone();
        let changed = self.store(id, default, SetBy::Console).changed();
        self.list[id.0 as usize].set_by = SetBy::Default;
        changed
    }

    fn store(&mut self, id: VarId, value: Value, source: SetBy) -> SetOutcome {
        let i = id.0 as usize;
        let var = &mut self.list[i];
        if source < var.set_by {
            return SetOutcome::Outranked(var.set_by);
        }
        var.set_by = source;
        if var.value == value {
            return SetOutcome::Unchanged;
        }
        var.value = value;
        var.generation += 1;
        self.revision += 1;
        if !self.pending[i] {
            self.pending[i] = true;
            self.changed.push(id);
        }
        SetOutcome::Changed
    }

    /// Total number of changes over all variables (cheap "anything changed?" check).
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Variables changed since the last call, each once, in order of first change.
    pub fn take_changes(&mut self) -> Vec<VarId> {
        for id in &self.changed {
            self.pending[id.0 as usize] = false;
        }
        std::mem::take(&mut self.changed)
    }

    /// In registration order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (VarId, &Var)> {
        self.list
            .iter()
            .enumerate()
            .map(|(i, v)| (VarId(u32::try_from(i).expect("checked at register")), v))
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}
