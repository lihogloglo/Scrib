//! The global-script host (scripting-adapter.md "The game API", DQ-4b1b): the sibling of the
//! results host (`runner.rs`) for global script instances. It binds `mw_script`'s calls onto the
//! game API ([`crate::api`]) and brackets each run of [`GlobalInstances::tick`] so a faulted
//! run's writes disappear.
//!
//! - One tick is one [`GameRun`] over one [`GameView`]: each script reads the earlier scripts'
//!   writes of the tick (as in the engine, where every call takes effect at once), and the ops of
//!   every committed run accumulate in call order for the caller to apply after the tick.
//! - [`InstanceHost::begin_run`] takes a [`Checkpoint`] and sets the run's self;
//!   [`InstanceHost::end_run`] with `committed == false` (step limit) rolls back to it.
//! - **Self.** A global started with `ref->StartScript` has that reference as its self: the
//!   implicit target lowers to [`Target::Player`] for `player`, else to [`Target::Actor`] of the
//!   target id, the same target `ref->` calls name. It never lowers to [`Target::Speaker`]: ops
//!   of different runs share one op list and one overlay, so each must name its object. A plain
//!   global has no self: a self-targeted call is reported ([`ScriptReport::NoSelf`]) and does
//!   nothing (reads 0). `GameView::ctx` supplies the context player; its speaker is unused.
//! - **Local runs** (DQ-5b): [`InstanceHost::begin_local_run`] of a placement makes that exact
//!   reference the run's self ([`Target::Ref`]), so a self-targeted call never reaches another
//!   reference of the same base; an inventory stack's run has that stack ([`Target::Stack`],
//!   DQ-5b3). A global a local run starts without `ref->` takes the object's base id as its
//!   target (saved targets are record ids: the reference's base, the stack's item), from
//!   [`ScriptHost::with_ref_bases`]; without it, none.
//! - `StartScript` / `StopScript` / `ScriptRunning` never reach this host inside a tick: the
//!   instance set takes them (`mw_script::instances`, its `Scoped` host). Outside a tick they
//!   read as unhandled.
//!
//! [`GlobalInstances::tick`]: mw_script::GlobalInstances::tick

use std::collections::HashSet;
use std::fmt;

use mw_script::ast::{Call, Ident, Target as AstTarget, VarRef};
use mw_script::functions::Function;
use mw_script::interp::{Host, MessageArg, ScriptOp};
use mw_script::{InstanceHost, LocalKey, StatCall};
use mw_ui::message::MessageArg as UiArg;
use sf_records::RecordId;

use crate::api::{self, Checkpoint, GameOp, GameRun, GameView, Target};

/// Something the host should log; [`ScriptReports`] keeps each one once.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScriptReport {
    /// A function the interpreter does not run yet: the call read its neutral value.
    Unhandled(String),
    /// A variable nobody declares (`"global x"`, `"id.x"`): read 0, write dropped.
    UnknownVariable(String),
    /// A self-targeted call in a global started without `ref->`: nothing done.
    NoSelf { script: String, function: String },
    /// A dialogue-session call (`Choice`, `Goodbye`) in a global script: nothing done.
    OutsideDialogue { script: String, function: String },
}

impl fmt::Display for ScriptReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unhandled(name) => write!(f, "global scripts: {name} not implemented yet"),
            Self::UnknownVariable(name) => write!(f, "global scripts: unknown variable {name}"),
            Self::NoSelf { script, function } => {
                write!(f, "global script {script}: {function} needs a target (no ref->StartScript)")
            }
            Self::OutsideDialogue { script, function } => {
                write!(f, "global script {script}: {function} outside dialogue")
            }
        }
    }
}

/// The report-once log; lives as long as the game (or the session) and outlasts the hosts.
#[derive(Debug, Default)]
pub struct ScriptReports {
    seen: HashSet<ScriptReport>,
    fresh: Vec<ScriptReport>,
}

impl ScriptReports {
    fn push(&mut self, report: ScriptReport) {
        if self.seen.insert(report.clone()) {
            self.fresh.push(report);
        }
    }

    /// Reports since the last call, each at most once per log; the host logs them.
    pub fn drain(&mut self) -> Vec<ScriptReport> {
        std::mem::take(&mut self.fresh)
    }
}

/// One tick of global runs over the game API (see the module docs).
pub struct ScriptHost<'a> {
    run: GameRun<'a>,
    reports: &'a mut ScriptReports,
    /// The running script (lower-cased id).
    script: String,
    /// The running instance's self: its `StartScript` target.
    self_target: Option<Target>,
    /// Where the running script's writes start.
    checkpoint: Option<Checkpoint>,
    /// A local instance's base record id (its implicit `StartScript` target).
    ref_bases: Option<&'a dyn Fn(LocalKey) -> Option<String>>,
}

impl<'a> ScriptHost<'a> {
    pub fn new(view: GameView<'a>, reports: &'a mut ScriptReports) -> Self {
        Self {
            run: GameRun::new(view),
            reports,
            script: String::new(),
            self_target: None,
            checkpoint: None,
            ref_bases: None,
        }
    }

    /// Local runs' implicit `StartScript` target: the base record id of a placement, or the
    /// item of an inventory stack (module docs).
    #[must_use]
    pub fn with_ref_bases(mut self, bases: &'a dyn Fn(LocalKey) -> Option<String>) -> Self {
        self.ref_bases = Some(bases);
        self
    }

    /// The committed runs' ops so far, in call order.
    pub fn ops(&self) -> &[GameOp] {
        self.run.ops()
    }

    /// Ends the tick: every committed run's ops, in call order.
    pub fn into_ops(self) -> Vec<GameOp> {
        self.run.into_ops()
    }

    /// The target a call names: the implicit one is the run's self.
    fn bind(&mut self, target: &AstTarget, function: &str) -> Option<Target> {
        match target {
            AstTarget::Explicit(id) => Some(api::object(id)),
            AstTarget::Implicit => {
                if self.self_target.is_none() {
                    self.reports.push(ScriptReport::NoSelf {
                        script: self.script.clone(),
                        function: function.to_owned(),
                    });
                }
                self.self_target.clone()
            }
        }
    }

    /// Where a variable lives, with the speaker's (a bare name nobody resolved) as the self's.
    /// `None`: kept by the interpreter, or reported.
    fn var_owner<'v>(&mut self, var: &'v VarRef) -> Option<(Option<Target>, &'v Ident)> {
        let (owner, name) = api::owner(var)?;
        let owner = match owner {
            Some(Target::Speaker) => {
                let Some(me) = self.self_target.clone() else {
                    self.unknown(Some(&Target::Speaker), name);
                    return None;
                };
                Some(me)
            }
            other => other,
        };
        Some((owner, name))
    }

    fn unknown(&mut self, owner: Option<&Target>, name: &Ident) {
        self.reports
            .push(ScriptReport::UnknownVariable(api::unknown_name(owner, name)));
    }

    fn outside_dialogue(&mut self, function: &str) {
        self.reports.push(ScriptReport::OutsideDialogue {
            script: self.script.clone(),
            function: function.to_owned(),
        });
    }
}

/// A `StartScript` target as an object: `player` is the context player.
fn self_object(target: &str) -> Target {
    if target.eq_ignore_ascii_case("player") {
        Target::Player
    } else {
        Target::Actor(RecordId::new(target))
    }
}

impl InstanceHost for ScriptHost<'_> {
    fn begin_run(&mut self, script: &str, target: Option<&str>) {
        script.clone_into(&mut self.script);
        self.self_target = target.map(self_object);
        self.checkpoint = Some(self.run.checkpoint());
    }

    fn begin_local_run(&mut self, script: &str, key: LocalKey) {
        script.clone_into(&mut self.script);
        self.self_target = match key {
            LocalKey::Placement(p) => Some(Target::Ref(p)),
            LocalKey::Stack { holder, stack } => Some(Target::Stack { holder, stack }),
        };
        self.checkpoint = Some(self.run.checkpoint());
    }

    fn local_self(&mut self, key: LocalKey) -> Option<String> {
        self.ref_bases.and_then(|bases| bases(key))
    }

    fn end_run(&mut self, _script: &str, committed: bool) {
        let checkpoint = self.checkpoint.take();
        if !committed && let Some(checkpoint) = checkpoint {
            self.run.rollback(checkpoint);
        }
        self.self_target = None;
    }
}

impl Host for ScriptHost<'_> {
    fn get(&mut self, var: &VarRef) -> f64 {
        let Some((owner, name)) = self.var_owner(var) else {
            return 0.0;
        };
        let key = api::rid(name);
        let value = match &owner {
            None => self.run.global(&key),
            Some(t) => self.run.var(t, &key),
        };
        value.map_or_else(
            || {
                self.unknown(owner.as_ref(), name);
                0.0
            },
            f64::from,
        )
    }

    fn set(&mut self, var: &VarRef, value: f64) {
        let Some((owner, name)) = self.var_owner(var) else {
            return;
        };
        let key = api::rid(name);
        let known = match &owner {
            None => self.run.set_global(&key, value),
            Some(t) => self.run.set_var(t, &key, value),
        };
        if !known {
            self.unknown(owner.as_ref(), name);
        }
    }

    fn journal(&mut self, quest: &Ident, index: i32) {
        self.run.journal(&api::rid(quest), index);
    }

    fn set_journal_index(&mut self, quest: &Ident, index: i32) {
        self.run.set_journal_index(&api::rid(quest), index);
    }

    fn get_journal_index(&mut self, quest: &Ident) -> i32 {
        self.run.quest_stage(&api::rid(quest))
    }

    fn add_topic(&mut self, topic: &Ident) {
        self.run.add_topic(api::rid(topic));
    }

    fn choice(&mut self, _choices: Vec<(String, i32)>) {
        self.outside_dialogue("Choice");
    }

    fn goodbye(&mut self) {
        self.outside_dialogue("Goodbye");
    }

    fn add_item(&mut self, target: &AstTarget, item: &Ident, count: i32) {
        if let Some(t) = self.bind(target, "AddItem") {
            self.run.add_item(t, api::rid(item), count);
        }
    }

    fn remove_item(&mut self, target: &AstTarget, item: &Ident, count: i32) {
        if let Some(t) = self.bind(target, "RemoveItem") {
            self.run.remove_item(t, api::rid(item), count);
        }
    }

    fn get_item_count(&mut self, target: &AstTarget, item: &Ident) -> i32 {
        let Some(t) = self.bind(target, "GetItemCount") else {
            return 0;
        };
        let n = self.run.item_count(&t, &api::rid(item));
        i32::try_from(n.max(0)).unwrap_or(i32::MAX)
    }

    fn set_disposition(&mut self, target: &AstTarget, value: i32) {
        if let Some(t) = self.bind(target, "SetDisposition") {
            self.run.set_disposition(t, value);
        }
    }

    fn mod_disposition(&mut self, target: &AstTarget, delta: i32) {
        if let Some(t) = self.bind(target, "ModDisposition") {
            self.run.mod_disposition(t, delta);
        }
    }

    fn message_box(&mut self, format: &str, args: &[MessageArg]) {
        let args: Vec<UiArg> = args
            .iter()
            .map(|a| match a {
                MessageArg::Num(v) => UiArg::Num(*v),
                MessageArg::Str(s) => UiArg::Str(s.clone()),
                MessageArg::Word(s) => UiArg::Word(s.clone()),
            })
            .collect();
        self.run.message_box(format, &args);
    }

    fn play_sound(&mut self, sound: &Ident) {
        self.run.play_sound(api::rid(sound));
    }

    fn stat(&mut self, target: &AstTarget, call: &StatCall) -> Option<f64> {
        let name = format!("{:?}", call.function);
        Some(
            self.bind(target, &name)
                .map_or(0.0, |t| self.run.stat(&t, call)),
        )
    }

    /// Script control belongs to the instance set; reached only outside a tick.
    fn script(&mut self, _target: &AstTarget, _op: ScriptOp, _script: &Ident) -> Option<f64> {
        None
    }

    fn unhandled(&mut self, function: &Function, _call: &Call) -> f64 {
        self.reports
            .push(ScriptReport::Unhandled(function.name.clone()));
        f64::from(function.status.neutral())
    }
}
