//! Rebindable input actions, Bevy-free (ui.md "Keybinds", U6).
//!
//! A [`Keymap`] holds every rebindable action of the game: the framework's UI
//! actions (context `ui`) and whatever a game registers (`gameplay`, ...).
//! Each action has binding **slots per device** (keyboard/mouse, gamepad),
//! defaults, and the scopes where it is live. The keymap answers what the
//! keybinds screen needs: rows, conflicts (per scope), proposals with swap /
//! unbind / keep-both resolution, resets (action, context, all; per device),
//! glyphs, and the profile table (`[bindings.<context>]`, only what differs
//! from the defaults). The engine adapter turns its input names into real
//! bindings (`sf_ui_bevy`).
//!
//! Model after UE5's `EnhancedInputUserSettings` (player-mappable keys in
//! slots, reset per mapping or profile, a failure reason on map) and Unity's
//! binding overrides (defaults stay, the player's changes are layered on top
//! and saved alone).

use std::collections::BTreeMap;
use std::fmt;

use indexmap::IndexMap;
use sf_mods::Profile;

use crate::input::{Device, Glyph, Input, InputKind, Overlap, PadFamily, glyph, overlap};

/// An action, named by its binding context and action name (`ui` +
/// `confirm`). The context is also the profile table (`[bindings.ui]`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActionRef {
    pub context: String,
    pub action: String,
}

impl ActionRef {
    pub fn new(context: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            context: context.into(),
            action: action.into(),
        }
    }
}

impl fmt::Display for ActionRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.context, self.action)
    }
}

/// What kind of input an action takes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ActionShape {
    /// Keys, mouse buttons (with modifiers), pad buttons, single axes, the
    /// mouse wheel.
    #[default]
    Button,
    /// A 2D value: a whole stick (`stick:Left`) or mouse motion.
    Stick,
}

impl ActionShape {
    pub(crate) fn accepts(self, kind: InputKind) -> bool {
        match self {
            ActionShape::Button => matches!(
                kind,
                InputKind::Key
                    | InputKind::Mouse
                    | InputKind::Pad
                    | InputKind::Axis
                    | InputKind::MouseWheel
            ),
            ActionShape::Stick => matches!(kind, InputKind::Stick | InputKind::MouseMotion),
        }
    }
}

/// A rebindable action.
#[derive(Clone, Debug, PartialEq)]
pub struct ActionDef {
    pub id: ActionRef,
    /// Fluent message id of the row label.
    pub label: String,
    pub shape: ActionShape,
    /// Binding slots per device (keyboard/mouse, gamepad); 0 hides the
    /// action on that device's page.
    pub slots: [usize; 2],
    /// Default input names, any device.
    pub defaults: Vec<String>,
    /// Can be rebound but never left unbound on a device whose defaults bind
    /// it (confirm, back, menu open: the player can't lock themselves out).
    pub required: bool,
    /// Input scopes where the action is live; bindings conflict only between
    /// actions sharing a scope (or linked scopes, [`Keymap::link_scopes`]).
    /// Defaults to the context.
    pub scopes: Vec<String>,
}

impl ActionDef {
    /// A button action with two keyboard/mouse slots and one gamepad slot,
    /// label `keybind-<context>-<action>`.
    pub fn button<'a>(
        context: &str,
        action: &str,
        defaults: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        Self {
            id: ActionRef::new(context, action),
            label: format!("keybind-{}-{action}", context.replace('.', "-")),
            shape: ActionShape::Button,
            slots: [2, 1],
            defaults: defaults.into_iter().map(str::to_owned).collect(),
            required: false,
            scopes: vec![context.to_owned()],
        }
    }

    /// A 2D action (one gamepad slot for a stick, no keyboard slot).
    pub fn stick<'a>(
        context: &str,
        action: &str,
        defaults: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        Self {
            shape: ActionShape::Stick,
            slots: [0, 1],
            ..Self::button(context, action, defaults)
        }
    }

    #[must_use]
    pub fn slots(mut self, kbm: usize, gamepad: usize) -> Self {
        self.slots = [kbm, gamepad];
        self
    }

    #[must_use]
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Adds a scope (the context stays one).
    #[must_use]
    pub fn also_in(mut self, scope: impl Into<String>) -> Self {
        self.scopes.push(scope.into());
        self
    }

    pub fn slot_count(&self, device: Device) -> usize {
        self.slots[device.index()]
    }
}

/// One binding slot of an action.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SlotRef {
    pub action: ActionRef,
    pub device: Device,
    /// 0-based; a slot past the bound inputs appends.
    pub slot: usize,
}

/// Two actions bound to colliding inputs in a shared scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conflict {
    pub action: ActionRef,
    pub input: String,
    pub other: ActionRef,
    pub other_input: String,
    pub overlap: Overlap,
}

/// A binding the player picked, checked against the keymap but not applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub slot: SlotRef,
    /// Canonical input name.
    pub input: String,
    /// What the slot held (a swap gives it to the other action).
    pub previous: Option<String>,
    /// Bindings of other actions it collides with.
    pub conflicts: Vec<Conflict>,
    version: u64,
}

/// How to apply a [`Proposal`] that has conflicts (the screen asks the player).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// The other actions get this slot's previous input instead (or lose
    /// the input when the slot was empty).
    Swap,
    /// The other actions lose the input.
    Unbind,
    /// Bind anyway; the conflict stays reported.
    KeepBoth,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum KeymapError {
    #[error("unknown action `{0}`")]
    UnknownAction(ActionRef),
    #[error("action `{0}` is already registered")]
    Duplicate(ActionRef),
    #[error("`{input}`: {reason}")]
    BadInput { input: String, reason: String },
    #[error("`{input}` is a {got} input, the slot is for {want}")]
    WrongDevice {
        input: String,
        got: Device,
        want: Device,
    },
    #[error("`{input}` doesn't fit `{action}` ({shape:?} action)")]
    WrongShape {
        input: String,
        action: ActionRef,
        shape: ActionShape,
    },
    #[error("`{action}` has {slots} {device} slot(s), not slot {slot}")]
    NoSlot {
        action: ActionRef,
        device: Device,
        slot: usize,
        slots: usize,
    },
    #[error("`{action}` must keep a {device} binding")]
    Required { action: ActionRef, device: Device },
    #[error("the keymap changed since the proposal was made")]
    Stale,
}

/// A profile entry that couldn't be applied as written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeymapIssue {
    pub action: ActionRef,
    pub message: String,
}

impl fmt::Display for KeymapIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[bindings.{}] `{}`: {}",
            self.action.context, self.action.action, self.message
        )
    }
}

/// Engine check of an input name beyond the grammar (does the key exist?).
pub type InputValidator = fn(&str) -> Result<(), String>;

/// A row of the keybinds screen: an action and its slots per device.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeybindRow {
    pub action: ActionRef,
    pub label: String,
    /// Per device ([`Device::Kbm`], [`Device::Gamepad`]), one entry per slot.
    pub slots: [Vec<Option<String>>; 2],
    /// Some binding of this action collides with another action.
    pub conflicted: bool,
    /// Differs from the defaults.
    pub modified: bool,
}

#[derive(Clone, Debug)]
struct Entry {
    def: ActionDef,
    defaults: [Vec<String>; 2],
    bound: [Vec<String>; 2],
}

/// Every rebindable action and its current bindings.
#[derive(Clone, Debug, Default)]
pub struct Keymap {
    entries: IndexMap<ActionRef, Entry>,
    shared: Vec<(ActionRef, ActionRef)>,
    linked: Vec<(String, String)>,
    /// Profile entries of actions not registered (yet): kept for saving and
    /// applied if the action registers later (a disabled mod's actions).
    stored: BTreeMap<String, toml::Table>,
    validator: Option<InputValidator>,
    version: u64,
}

impl Keymap {
    pub fn new() -> Self {
        Self::default()
    }

    /// Engine check for input names (set before registering).
    pub fn set_validator(&mut self, validator: InputValidator) {
        self.validator = Some(validator);
    }

    /// Bumped on every change.
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Registers an action. Defaults must be valid for its shape and fit its
    /// slots. A stored profile entry for it applies now (its issues returned).
    pub fn register(&mut self, def: ActionDef) -> Result<Vec<KeymapIssue>, KeymapError> {
        if self.entries.contains_key(&def.id) {
            return Err(KeymapError::Duplicate(def.id));
        }
        let mut defaults: [Vec<String>; 2] = Default::default();
        for name in &def.defaults {
            let (input, device) = self.check(name, &def)?;
            if !defaults[device.index()].contains(&input) {
                defaults[device.index()].push(input);
            }
        }
        for device in Device::ALL {
            let slots = def.slot_count(device);
            if defaults[device.index()].len() > slots {
                return Err(KeymapError::NoSlot {
                    action: def.id,
                    device,
                    slot: slots,
                    slots,
                });
            }
        }
        let id = def.id.clone();
        self.entries.insert(
            id.clone(),
            Entry {
                def,
                bound: defaults.clone(),
                defaults,
            },
        );
        self.version += 1;
        let stored = self
            .stored
            .get_mut(&id.context)
            .and_then(|t| t.remove(&id.action));
        if self
            .stored
            .get(&id.context)
            .is_some_and(toml::Table::is_empty)
        {
            self.stored.remove(&id.context);
        }
        Ok(stored.map_or_else(Vec::new, |value| self.load_entry(&id, &value)))
    }

    /// Lets two actions share inputs without a conflict (Esc both closes a
    /// menu and opens the main menu).
    pub fn allow_shared(&mut self, a: ActionRef, b: ActionRef) {
        self.shared.push((a, b));
    }

    /// Two scopes that are live at the same time (the HUD's open-menu actions
    /// and gameplay), so their actions' bindings conflict.
    pub fn link_scopes(&mut self, a: impl Into<String>, b: impl Into<String>) {
        self.linked.push((a.into(), b.into()));
    }

    pub fn actions(&self) -> impl Iterator<Item = &ActionDef> {
        self.entries.values().map(|e| &e.def)
    }

    pub fn def(&self, action: &ActionRef) -> Option<&ActionDef> {
        self.entries.get(action).map(|e| &e.def)
    }

    /// Current inputs of an action on a device (empty for unknown actions).
    pub fn bound(&self, action: &ActionRef, device: Device) -> &[String] {
        self.entries
            .get(action)
            .map_or(&[], |e| e.bound[device.index()].as_slice())
    }

    /// Current inputs of an action, keyboard/mouse first.
    pub fn bound_all(&self, action: &ActionRef) -> impl Iterator<Item = &str> {
        Device::ALL
            .into_iter()
            .flat_map(move |d| self.bound(action, d))
            .map(String::as_str)
    }

    pub fn defaults(&self, action: &ActionRef, device: Device) -> &[String] {
        self.entries
            .get(action)
            .map_or(&[], |e| e.defaults[device.index()].as_slice())
    }

    /// Glyph of the first input bound to `action` on `device`.
    pub fn glyph(&self, action: &ActionRef, device: Device, family: PadFamily) -> Option<Glyph> {
        self.bound(action, device)
            .first()
            .and_then(|name| glyph(name, family))
    }

    /// The keybinds screen's rows, in registration order; `context` filters.
    pub fn rows(&self, context: Option<&str>) -> Vec<KeybindRow> {
        let conflicts = self.conflicts();
        self.entries
            .values()
            .filter(|e| context.is_none_or(|c| e.def.id.context == c))
            .map(|e| KeybindRow {
                action: e.def.id.clone(),
                label: e.def.label.clone(),
                slots: Device::ALL.map(|d| {
                    (0..e.def.slot_count(d))
                        .map(|i| e.bound[d.index()].get(i).cloned())
                        .collect()
                }),
                conflicted: conflicts.iter().any(|c| c.action == e.def.id),
                modified: e.bound != e.defaults,
            })
            .collect()
    }

    /// Every conflict, once per direction (each side's row can show it).
    pub fn conflicts(&self) -> Vec<Conflict> {
        let mut out = Vec::new();
        for entry in self.entries.values() {
            for input in entry.bound.iter().flatten() {
                out.extend(self.conflicts_of(&entry.def.id, input));
            }
        }
        out
    }

    /// Bindings of other actions that `input` collides with if `action` binds it.
    pub fn conflicts_of(&self, action: &ActionRef, input: &str) -> Vec<Conflict> {
        let Some(entry) = self.entries.get(action) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for other in self.entries.values() {
            if other.def.id == *action
                || self.is_shared(action, &other.def.id)
                || !self.scopes_meet(&entry.def, &other.def)
            {
                continue;
            }
            for other_input in other.bound.iter().flatten() {
                if let Some(kind) = overlap(input, other_input) {
                    out.push(Conflict {
                        action: action.clone(),
                        input: input.to_owned(),
                        other: other.def.id.clone(),
                        other_input: other_input.clone(),
                        overlap: kind,
                    });
                }
            }
        }
        out
    }

    /// Checks binding `input` to `slot`: validity, then conflicts. Apply it
    /// with [`Keymap::apply`].
    pub fn propose(&self, slot: &SlotRef, input: &str) -> Result<Proposal, KeymapError> {
        let entry = self.entry(&slot.action)?;
        let (input, device) = self.check(input, &entry.def)?;
        if device != slot.device {
            return Err(KeymapError::WrongDevice {
                input,
                got: device,
                want: slot.device,
            });
        }
        Self::check_slot(&entry.def, slot)?;
        Ok(Proposal {
            previous: entry.bound[device.index()].get(slot.slot).cloned(),
            conflicts: self.conflicts_of(&slot.action, &input),
            slot: slot.clone(),
            input,
            version: self.version,
        })
    }

    /// Applies a proposal. Refused when it is stale or would leave a
    /// required action unbound on a device.
    pub fn apply(
        &mut self,
        proposal: &Proposal,
        resolution: Resolution,
    ) -> Result<(), KeymapError> {
        if proposal.version != self.version {
            return Err(KeymapError::Stale);
        }
        let device = proposal.slot.device;
        let d = device.index();
        let mut changed: Vec<(ActionRef, Vec<String>)> = Vec::new();

        let mut list = self.bound(&proposal.slot.action, device).to_vec();
        let at = proposal.slot.slot;
        // Already in another slot of this action: the two slots swap.
        if let Some(i) = list.iter().position(|b| *b == proposal.input)
            && i != at
        {
            match list.get(at).cloned() {
                Some(previous) => list[i] = previous,
                None => {
                    list.remove(i);
                }
            }
        }
        match list.get_mut(at) {
            Some(b) => b.clone_from(&proposal.input),
            None => list.push(proposal.input.clone()),
        }
        changed.push((proposal.slot.action.clone(), list));

        if resolution != Resolution::KeepBoth {
            for conflict in &proposal.conflicts {
                let other = &self.entries[&conflict.other];
                let pos = changed.iter().position(|(a, _)| *a == conflict.other);
                let current = pos.map_or_else(|| other.bound[d].clone(), |i| changed[i].1.clone());
                let swap_in = (resolution == Resolution::Swap)
                    .then(|| proposal.previous.clone())
                    .flatten()
                    .filter(|p| {
                        !current.contains(p)
                            && Input::parse(p).is_ok_and(|i| other.def.shape.accepts(i.kind))
                    });
                let mut next = Vec::with_capacity(current.len());
                for b in current {
                    if b == conflict.other_input {
                        next.extend(swap_in.clone());
                    } else {
                        next.push(b);
                    }
                }
                match pos {
                    Some(i) => changed[i].1 = next,
                    None => changed.push((conflict.other.clone(), next)),
                }
            }
        }
        for (action, list) in &changed {
            self.check_required(action, device, list)?;
        }
        for (action, list) in changed {
            self.entries[&action].bound[d] = list;
        }
        self.version += 1;
        Ok(())
    }

    /// Clears one slot (later slots move up).
    pub fn unbind(&mut self, slot: &SlotRef) -> Result<bool, KeymapError> {
        let entry = self.entry(&slot.action)?;
        Self::check_slot(&entry.def, slot)?;
        let mut list = entry.bound[slot.device.index()].clone();
        if slot.slot >= list.len() {
            return Ok(false);
        }
        list.remove(slot.slot);
        self.check_required(&slot.action, slot.device, &list)?;
        self.entries[&slot.action].bound[slot.device.index()] = list;
        self.version += 1;
        Ok(true)
    }

    /// Replaces an action's inputs on one device (code and scripts; no
    /// conflict resolution). Names are validated and canonicalized.
    pub fn set(
        &mut self,
        action: &ActionRef,
        device: Device,
        inputs: &[&str],
    ) -> Result<(), KeymapError> {
        let entry = self.entry(action)?;
        let mut list = Vec::with_capacity(inputs.len());
        for name in inputs {
            let (input, got) = self.check(name, &entry.def)?;
            if got != device {
                return Err(KeymapError::WrongDevice {
                    input,
                    got,
                    want: device,
                });
            }
            if !list.contains(&input) {
                list.push(input);
            }
        }
        let slots = entry.def.slot_count(device);
        if list.len() > slots {
            return Err(KeymapError::NoSlot {
                action: action.clone(),
                device,
                slot: slots,
                slots,
            });
        }
        self.check_required(action, device, &list)?;
        let bound = &mut self.entries[action].bound[device.index()];
        if *bound != list {
            *bound = list;
            self.version += 1;
        }
        Ok(())
    }

    /// Restores an action's defaults (`None`: every device). Returns whether
    /// anything changed.
    pub fn reset(&mut self, action: &ActionRef, device: Option<Device>) -> bool {
        let Some(entry) = self.entries.get_mut(action) else {
            return false;
        };
        let changed = reset_entry(entry, device);
        if changed {
            self.version += 1;
        }
        changed
    }

    /// Restores the defaults of every action of a context.
    pub fn reset_context(&mut self, context: &str, device: Option<Device>) -> bool {
        self.reset_where(|e| e.def.id.context == context, device)
    }

    /// Restores every default.
    pub fn reset_all(&mut self, device: Option<Device>) -> bool {
        self.reset_where(|_| true, device)
    }

    fn reset_where(&mut self, filter: impl Fn(&Entry) -> bool, device: Option<Device>) -> bool {
        let mut changed = false;
        for entry in self.entries.values_mut().filter(|e| filter(e)) {
            changed |= reset_entry(entry, device);
        }
        if changed {
            self.version += 1;
        }
        changed
    }

    /// Reads `profile.bindings`; see [`Keymap::load_tables`].
    pub fn load_from(&mut self, profile: &Profile) -> Vec<KeymapIssue> {
        self.load_tables(&profile.bindings)
    }

    /// Replaces `profile.bindings` with [`Keymap::to_tables`]. Returns whether
    /// it changed.
    pub fn save_to(&self, profile: &mut Profile) -> bool {
        let tables = self.to_tables();
        let changed = profile.bindings != tables;
        profile.bindings = tables;
        changed
    }

    /// Resets every action, then applies profile tables (context → action →
    /// entry). An entry is either a table of devices
    /// (`confirm = { kbm = ["key:Space"] }`, only the devices listed change)
    /// or a list of names for every device (U1c form). Bad names are dropped
    /// with an issue and the rest kept; a required action left unbound on a
    /// device gets that device's defaults back. Entries of unregistered
    /// actions are kept for saving and for a later [`Keymap::register`].
    pub fn load_tables(&mut self, tables: &BTreeMap<String, toml::Table>) -> Vec<KeymapIssue> {
        for entry in self.entries.values_mut() {
            entry.bound = entry.defaults.clone();
        }
        self.stored.clear();
        self.version += 1;
        let mut issues = Vec::new();
        for (context, table) in tables {
            for (action, value) in table {
                let id = ActionRef::new(context.as_str(), action.as_str());
                if self.entries.contains_key(&id) {
                    issues.extend(self.load_entry(&id, value));
                } else {
                    self.stored
                        .entry(context.clone())
                        .or_default()
                        .insert(action.clone(), value.clone());
                }
            }
        }
        issues
    }

    /// Profile tables: per context, the actions that differ from their
    /// defaults (only the differing devices), plus stored entries.
    pub fn to_tables(&self) -> BTreeMap<String, toml::Table> {
        let mut out = self.stored.clone();
        for entry in self.entries.values() {
            let mut devices = toml::Table::new();
            for device in Device::ALL {
                let d = device.index();
                if entry.bound[d] != entry.defaults[d] {
                    let names = entry.bound[d].iter().cloned().map(toml::Value::String);
                    devices.insert(
                        device.name().to_owned(),
                        toml::Value::Array(names.collect()),
                    );
                }
            }
            if !devices.is_empty() {
                out.entry(entry.def.id.context.clone())
                    .or_default()
                    .insert(entry.def.id.action.clone(), toml::Value::Table(devices));
            }
        }
        out.retain(|_, t| !t.is_empty());
        out
    }

    fn load_entry(&mut self, id: &ActionRef, value: &toml::Value) -> Vec<KeymapIssue> {
        let mut issues = Vec::new();
        let mut issue = |message: String| {
            issues.push(KeymapIssue {
                action: id.clone(),
                message,
            });
        };
        let def = self.entries[id].def.clone();
        // Per device: `Some(names)` replaces that device's bindings.
        let mut lists: [Option<Vec<&toml::Value>>; 2] = [None, None];
        match value {
            toml::Value::Array(items) => {
                lists = [Some(Vec::new()), Some(Vec::new())];
                for item in items {
                    let device = item
                        .as_str()
                        .and_then(|s| Input::parse(s).ok())
                        .map_or(Device::Kbm, |i| i.device());
                    lists[device.index()].get_or_insert_default().push(item);
                }
            }
            toml::Value::Table(devices) => {
                for (name, items) in devices {
                    let Some(device) = Device::from_name(name) else {
                        issue(format!("unknown device `{name}` (kbm, gamepad)"));
                        continue;
                    };
                    let Some(items) = items.as_array() else {
                        issue(format!("`{name}` must be a list of input names"));
                        continue;
                    };
                    lists[device.index()] = Some(items.iter().collect());
                }
            }
            _ => {
                issue("must be a list of input names or a table of devices".to_owned());
                return issues;
            }
        }
        for device in Device::ALL {
            let Some(items) = &lists[device.index()] else {
                continue;
            };
            let mut list: Vec<String> = Vec::new();
            for item in items {
                let Some(name) = item.as_str() else {
                    issue(format!("{item} is not an input name"));
                    continue;
                };
                match self.check(name, &def) {
                    Ok((input, got)) if got == device => {
                        if !list.contains(&input) {
                            list.push(input);
                        }
                    }
                    Ok((input, got)) => {
                        issue(format!("`{input}` is a {got} input, listed under {device}"));
                    }
                    Err(e) => issue(e.to_string()),
                }
            }
            let slots = def.slot_count(device);
            if list.len() > slots {
                issue(format!(
                    "{} {device} inputs for {slots} slot(s); the rest are dropped",
                    list.len()
                ));
                list.truncate(slots);
            }
            let entry = &mut self.entries[id];
            if def.required && list.is_empty() && !entry.defaults[device.index()].is_empty() {
                issue(format!("must keep a {device} binding; defaults restored"));
                list.clone_from(&entry.defaults[device.index()]);
            }
            entry.bound[device.index()] = list;
        }
        self.version += 1;
        issues
    }

    fn entry(&self, action: &ActionRef) -> Result<&Entry, KeymapError> {
        self.entries
            .get(action)
            .ok_or_else(|| KeymapError::UnknownAction(action.clone()))
    }

    /// Grammar, engine validation and shape; the canonical name and device.
    fn check(&self, name: &str, def: &ActionDef) -> Result<(String, Device), KeymapError> {
        let bad = |reason: String| KeymapError::BadInput {
            input: name.to_owned(),
            reason,
        };
        let input = Input::parse(name).map_err(bad)?;
        let canonical = input.canonical();
        if let Some(validate) = self.validator {
            validate(&canonical).map_err(bad)?;
        }
        if !def.shape.accepts(input.kind) {
            return Err(KeymapError::WrongShape {
                input: canonical,
                action: def.id.clone(),
                shape: def.shape,
            });
        }
        Ok((canonical, input.device()))
    }

    fn check_slot(def: &ActionDef, slot: &SlotRef) -> Result<(), KeymapError> {
        let slots = def.slot_count(slot.device);
        if slot.slot >= slots {
            return Err(KeymapError::NoSlot {
                action: def.id.clone(),
                device: slot.device,
                slot: slot.slot,
                slots,
            });
        }
        Ok(())
    }

    fn check_required(
        &self,
        action: &ActionRef,
        device: Device,
        list: &[String],
    ) -> Result<(), KeymapError> {
        let entry = &self.entries[action];
        if entry.def.required && list.is_empty() && !entry.defaults[device.index()].is_empty() {
            return Err(KeymapError::Required {
                action: action.clone(),
                device,
            });
        }
        Ok(())
    }

    fn is_shared(&self, a: &ActionRef, b: &ActionRef) -> bool {
        self.shared
            .iter()
            .any(|(x, y)| (x == a && y == b) || (x == b && y == a))
    }

    fn scopes_meet(&self, a: &ActionDef, b: &ActionDef) -> bool {
        a.scopes.iter().any(|x| {
            b.scopes.iter().any(|y| {
                x == y
                    || self
                        .linked
                        .iter()
                        .any(|(p, q)| (p == x && q == y) || (p == y && q == x))
            })
        })
    }
}

fn reset_entry(entry: &mut Entry, device: Option<Device>) -> bool {
    let mut changed = false;
    for d in Device::ALL {
        if device.is_none_or(|x| x == d) && entry.bound[d.index()] != entry.defaults[d.index()] {
            entry.bound[d.index()].clone_from(&entry.defaults[d.index()]);
            changed = true;
        }
    }
    changed
}
