//! Native `MWScript` interpreter over the resolved AST (rpg-adapter.md "As built (W3)").
//!
//! The vertical slice runs dialogue results here while the Luau runtime is parked (owner
//! 2026-10-18). It walks the same AST the emitter translates and keeps the emitter's numeric
//! semantics (`emit.rs` "Numeric semantics"): expressions evaluate in `f64`, comparisons
//! compare values, and only **assignment** converts through [`store`].
//!
//! The interpreter owns control flow and the running instance's locals; everything that
//! touches game state goes through a [`Host`]: variables outside the instance (globals,
//! `id.var`, speaker locals), the [`Builtin`] functions, and [`Host::unhandled`] for the
//! rest of the function table (neutral value, the host reports it). Runaway loops stop at
//! [`Interpreter::step_limit`] with [`RunError::StepLimit`].
//!
//! `MessageBox` is not formatted here: the interpreter evaluates the number arguments and hands
//! the format and the [`MessageArg`]s, split as the parser kept them, to
//! [`Host::message_box`]; the host formats with `mw_ui::message::message_box_from_args`
//! (`OpenMW`'s formatter and button rule; this crate stays dependency-free).

use crate::ast::{
    Arg, ArithOp, Call, Expr, Ident, Script, Stmt, StmtKind, Target, VarRef, VarType,
};
use crate::functions::{Function, FunctionTable, Returns};
use crate::lexer::CmpOp;
use crate::statfn::{StatCall, StatFn};
use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};

/// `mw.script.step_limit`: statements and loop tests one run may execute before it is stopped
/// ([`RunError::StepLimit`]). Vanilla scripts finish in far fewer; only a `While` that never ends
/// reaches it. [`Interpreter::new`] starts from the default; the host sets
/// [`Interpreter::step_limit`] from the tunable store.
pub const STEP_LIMIT: Def<u32> = def!(count "mw.script.step_limit" = 100_000)
    .range(1_000, 10_000_000)
    .unit(Unit::Count)
    .help("Statements one legacy script run may execute before it is stopped and reported")
    .source(Source::Guess)
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Live);

/// The interpreter's tunables (the catalogue list).
pub const SCRIPT_TUNABLES: &[&dyn AnyDef] = &[&STEP_LIMIT, &crate::locals::ACTIVE_RADIUS];

/// The functions the interpreter dispatches to typed [`Host`] methods.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Builtin {
    Journal,
    SetJournalIndex,
    GetJournalIndex,
    AddTopic,
    Choice,
    Goodbye,
    AddItem,
    RemoveItem,
    GetItemCount,
    SetDisposition,
    ModDisposition,
    MessageBox,
    PlaySound,
    StartScript,
    StopScript,
    ScriptRunning,
}

impl Builtin {
    pub const ALL: [Builtin; 16] = [
        Builtin::Journal,
        Builtin::SetJournalIndex,
        Builtin::GetJournalIndex,
        Builtin::AddTopic,
        Builtin::Choice,
        Builtin::Goodbye,
        Builtin::AddItem,
        Builtin::RemoveItem,
        Builtin::GetItemCount,
        Builtin::SetDisposition,
        Builtin::ModDisposition,
        Builtin::MessageBox,
        Builtin::PlaySound,
        Builtin::StartScript,
        Builtin::StopScript,
        Builtin::ScriptRunning,
    ];

    /// The function table's canonical name.
    pub fn name(self) -> &'static str {
        match self {
            Builtin::Journal => "Journal",
            Builtin::SetJournalIndex => "SetJournalIndex",
            Builtin::GetJournalIndex => "GetJournalIndex",
            Builtin::AddTopic => "AddTopic",
            Builtin::Choice => "Choice",
            Builtin::Goodbye => "Goodbye",
            Builtin::AddItem => "AddItem",
            Builtin::RemoveItem => "RemoveItem",
            Builtin::GetItemCount => "GetItemCount",
            Builtin::SetDisposition => "SetDisposition",
            Builtin::ModDisposition => "ModDisposition",
            Builtin::MessageBox => "MessageBox",
            Builtin::PlaySound => "PlaySound",
            Builtin::StartScript => "StartScript",
            Builtin::StopScript => "StopScript",
            Builtin::ScriptRunning => "ScriptRunning",
        }
    }
}

/// How the interpreter runs a function: the dispatch [`Interpreter::new`] builds and the call
/// path follows (stat functions first, then builtins, then [`Host::unhandled`]). The DQ-2
/// coverage report counts a call site as bound when its function is not [`Binding::Unbound`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Binding {
    /// A stat, faction or crime function, sent to [`Host::stat`] (falls to
    /// [`Host::unhandled`] only when the host answers `None`).
    Stat(StatFn),
    /// A typed [`Host`] method.
    Builtin(Builtin),
    /// No binding: [`Host::unhandled`], the neutral value.
    Unbound,
}

impl Binding {
    /// The binding of a function table name (case-insensitive).
    pub fn of(name: &str) -> Binding {
        if let Some(s) = StatFn::from_name(name) {
            return Binding::Stat(s);
        }
        Builtin::ALL
            .into_iter()
            .find(|b| b.name().eq_ignore_ascii_case(name))
            .map_or(Binding::Unbound, Binding::Builtin)
    }

    pub fn is_bound(self) -> bool {
        self != Binding::Unbound
    }
}

/// One `MessageBox` argument after the format text, as [`Host::message_box`] receives it. It
/// mirrors `mw_ui::message::MessageArg` variant for variant (this crate has no dependencies;
/// the host converts one to the other).
#[derive(Clone, Debug, PartialEq)]
pub enum MessageArg {
    /// A number argument (literal, variable or expression), evaluated.
    Num(f64),
    /// A quoted string ([`Arg::Str`]).
    Str(String),
    /// A bare word as written ([`Arg::Word`]).
    Word(String),
}

/// A global script control call, as [`Host::script`] receives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScriptOp {
    /// `[ref->]StartScript id`.
    Start,
    /// `StopScript id`.
    Stop,
    /// `ScriptRunning id`: 1 or 0.
    Running,
}

/// What the interpreter needs from the game. Ids arrive as written (`Ident::key` is the
/// lowercase lookup form); counts and indices are already truncated to integers.
pub trait Host {
    /// Reads a variable outside the running instance: [`VarRef::Global`],
    /// [`VarRef::Member`] or [`VarRef::Speaker`]. Unknown variables read 0.
    fn get(&mut self, var: &VarRef) -> f64;
    /// Writes such a variable. The host knows its declared type and converts with
    /// [`store`]; writes to unknown variables are dropped.
    fn set(&mut self, var: &VarRef, value: f64);

    fn journal(&mut self, quest: &Ident, index: i32);
    fn set_journal_index(&mut self, quest: &Ident, index: i32);
    fn get_journal_index(&mut self, quest: &Ident) -> i32;
    fn add_topic(&mut self, topic: &Ident);
    /// `Choice "label" value ...`: the pairs in order.
    fn choice(&mut self, choices: Vec<(String, i32)>);
    fn goodbye(&mut self);
    fn add_item(&mut self, target: &Target, item: &Ident, count: i32);
    fn remove_item(&mut self, target: &Target, item: &Ident, count: i32);
    fn get_item_count(&mut self, target: &Target, item: &Ident) -> i32;
    fn set_disposition(&mut self, target: &Target, value: i32);
    fn mod_disposition(&mut self, target: &Target, delta: i32);
    /// `MessageBox`: the format text unformatted and every argument after it in order (values,
    /// then potential buttons). The host formats and picks the buttons; `mw_ui`'s
    /// `message_box_from_args` takes the same argument kinds ([`MessageArg`]).
    fn message_box(&mut self, format: &str, args: &[MessageArg]);
    fn play_sound(&mut self, sound: &Ident);

    /// A stat, faction or crime function ([`StatFn`]) on `target`; the call reads the returned
    /// value (ignored for void functions). `None`: this host doesn't run stats, and the call
    /// goes to [`Host::unhandled`] like any other function. Default: `None`.
    fn stat(&mut self, target: &Target, call: &StatCall) -> Option<f64> {
        let _ = (target, call);
        None
    }

    /// `StartScript`, `StopScript` or `ScriptRunning` on `script` (`target` is the `ref->` of a
    /// `StartScript`). The call reads the returned value (`ScriptRunning` only). `None`: this
    /// host doesn't run global scripts, and the call goes to [`Host::unhandled`]. Default:
    /// `None`. [`crate::instances::GlobalInstances::tick`] answers these itself.
    fn script(&mut self, target: &Target, op: ScriptOp, script: &Ident) -> Option<f64> {
        let _ = (target, op, script);
        None
    }

    /// Any other function. Its numeric arguments were evaluated (calls inside them ran);
    /// the call reads the returned value. Default: the function table's neutral value.
    fn unhandled(&mut self, function: &Function, call: &Call) -> f64 {
        let _ = call;
        f64::from(function.status.neutral())
    }
}

/// Why a run stopped early. What the script did before that already reached the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunError {
    /// More than [`Interpreter::step_limit`] steps (a `While` that never ends).
    StepLimit { line: u32 },
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::StepLimit { line } => write!(f, "L{line}: step limit reached"),
        }
    }
}

impl std::error::Error for RunError {}

/// The running instance's locals by slot ([`Script::locals`]), stored converted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Locals(pub Vec<f64>);

impl Locals {
    /// Every local zero (instances and dialogue results start like this).
    pub fn new(script: &Script) -> Self {
        Self(vec![0.0; script.locals.len()])
    }
}

/// Assignment conversion (`emit.rs` "Numeric semantics", `mw.luau` `short`/`long`/`float`):
/// integers truncate toward zero and wrap in two's complement (non-finite values become 0),
/// floats round through `f32`.
#[allow(clippy::cast_possible_truncation)]
pub fn store(ty: VarType, value: f64) -> f64 {
    let int = |bits: u32| {
        if !value.is_finite() {
            return 0.0;
        }
        let modulus = f64::from(bits).exp2();
        let mut n = value.trunc() % modulus;
        if n < 0.0 {
            n += modulus;
        }
        if n >= modulus / 2.0 {
            n -= modulus;
        }
        n
    };
    match ty {
        VarType::Short => int(16),
        VarType::Long => int(32),
        VarType::Float => f64::from(value as f32),
    }
}

/// A value as an integer argument (count, index, disposition): truncated toward zero,
/// saturating at the `i32` range, NaN 0.
#[allow(clippy::cast_possible_truncation)]
pub fn to_int(value: f64) -> i32 {
    if value.is_nan() {
        0
    } else {
        value.trunc() as i32
    }
}

/// Runs scripts against a [`Host`]. Built once per function table.
#[derive(Clone, Debug)]
pub struct Interpreter<'t> {
    table: &'t FunctionTable,
    builtins: Vec<Option<Builtin>>,
    /// The table's stat, faction and crime functions, by function id.
    stat_fns: Vec<Option<StatFn>>,
    pub step_limit: u32,
}

/// How a block ended.
enum Flow {
    Next,
    Return,
}

struct Run<'r, 't> {
    interp: &'r Interpreter<'t>,
    script: &'r Script,
    locals: &'r mut Locals,
    host: &'r mut dyn Host,
    steps: u32,
}

impl<'t> Interpreter<'t> {
    pub fn new(table: &'t FunctionTable) -> Self {
        let mut builtins = Vec::with_capacity(table.functions.len());
        let mut stat_fns = Vec::with_capacity(table.functions.len());
        for f in &table.functions {
            let binding = Binding::of(&f.name);
            builtins.push(match binding {
                Binding::Builtin(b) => Some(b),
                _ => None,
            });
            stat_fns.push(match binding {
                Binding::Stat(s) => Some(s),
                _ => None,
            });
        }
        Self {
            table,
            builtins,
            stat_fns,
            step_limit: STEP_LIMIT.default_value(),
        }
    }

    pub fn table(&self) -> &'t FunctionTable {
        self.table
    }

    /// Runs `script` once, top to bottom (`Return` ends the run). `locals` must come from
    /// [`Locals::new`] for this script. The caller checks [`crate::Parsed::is_inert`] first:
    /// inert scripts never run.
    pub fn run(
        &self,
        script: &Script,
        locals: &mut Locals,
        host: &mut dyn Host,
    ) -> Result<(), RunError> {
        let mut run = Run {
            interp: self,
            script,
            locals,
            host,
            steps: 0,
        };
        run.block(&script.body).map(|_| ())
    }
}

impl Run<'_, '_> {
    fn step(&mut self, line: u32) -> Result<(), RunError> {
        self.steps += 1;
        if self.steps > self.interp.step_limit {
            return Err(RunError::StepLimit { line });
        }
        Ok(())
    }

    fn block(&mut self, stmts: &[Stmt]) -> Result<Flow, RunError> {
        for s in stmts {
            if let Flow::Return = self.stmt(s)? {
                return Ok(Flow::Return);
            }
        }
        Ok(Flow::Next)
    }

    fn stmt(&mut self, s: &Stmt) -> Result<Flow, RunError> {
        self.step(s.line)?;
        match &s.kind {
            StmtKind::Set { target, value } => {
                let value = self.expr(value);
                self.assign(target, value);
            }
            StmtKind::If { arms, otherwise } => {
                for arm in arms {
                    if self.truth(&arm.cond) {
                        return self.block(&arm.body);
                    }
                }
                return self.block(otherwise);
            }
            StmtKind::While { cond, body } => {
                while self.truth(cond) {
                    if let Flow::Return = self.block(body)? {
                        return Ok(Flow::Return);
                    }
                    self.step(s.line)?;
                }
            }
            StmtKind::Return => return Ok(Flow::Return),
            StmtKind::Call(call) => {
                self.call(call);
            }
        }
        Ok(Flow::Next)
    }

    fn assign(&mut self, target: &VarRef, value: f64) {
        match target {
            VarRef::Local(slot) => {
                let ty = self.script.locals[*slot].ty;
                self.locals.0[*slot] = store(ty, value);
            }
            VarRef::Undeclared(_) => {}
            other => self.host.set(other, value),
        }
    }

    fn read(&mut self, var: &VarRef) -> f64 {
        match var {
            VarRef::Local(slot) => self.locals.0[*slot],
            VarRef::Undeclared(_) => 0.0,
            other => self.host.get(other),
        }
    }

    fn truth(&mut self, cond: &Expr) -> bool {
        self.expr(cond) != 0.0
    }

    fn expr(&mut self, e: &Expr) -> f64 {
        match e {
            #[allow(clippy::cast_precision_loss)]
            Expr::Int(i) => *i as f64,
            Expr::Float(f) => *f,
            Expr::Var(v) => self.read(v),
            Expr::Call(c) => self.call(c),
            Expr::Neg(x) => -self.expr(x),
            Expr::Arith(op, a, b) => {
                let (a, b) = (self.expr(a), self.expr(b));
                match op {
                    ArithOp::Add => a + b,
                    ArithOp::Sub => a - b,
                    ArithOp::Mul => a * b,
                    ArithOp::Div => a / b,
                }
            }
            Expr::Cmp(op, a, b) => {
                let (a, b) = (self.expr(a), self.expr(b));
                // Exact, as the emitter's Luau comparisons (`emit.rs` "Numeric semantics").
                #[allow(clippy::float_cmp)]
                let holds = match op {
                    CmpOp::Eq => a == b,
                    CmpOp::Ne => a != b,
                    CmpOp::Lt => a < b,
                    CmpOp::Le => a <= b,
                    CmpOp::Gt => a > b,
                    CmpOp::Ge => a >= b,
                };
                f64::from(u8::from(holds))
            }
        }
    }

    /// Argument `i` as a number (`Missing` and non-numeric arguments read 0).
    fn num(&mut self, call: &Call, i: usize) -> f64 {
        match call.args.get(i) {
            Some(Arg::Expr(e)) => self.expr(e),
            _ => 0.0,
        }
    }

    fn int(&mut self, call: &Call, i: usize) -> i32 {
        to_int(self.num(call, i))
    }

    /// A stat function's call: the value argument evaluated, the faction id as written.
    fn stat_call(&mut self, function: StatFn, call: &Call) -> f64 {
        let value = function.value_arg().map_or(0.0, |i| self.num(call, i));
        let faction = function.faction_arg().and_then(|i| match call.args.get(i) {
            Some(Arg::Id(id)) if !id.key.is_empty() => Some(id.clone()),
            Some(Arg::Str(s)) if !s.is_empty() => Some(Ident::new(s)),
            _ => None,
        });
        let f = self.interp.table.function(call.function);
        let stat = StatCall { function, value, faction };
        let value = match self.host.stat(&call.target, &stat) {
            Some(v) => v,
            None => self.host.unhandled(f, call),
        };
        if f.returns == Returns::Void { 0.0 } else { value }
    }

    /// Calls the function and returns its value (0 for void functions).
    fn call(&mut self, call: &Call) -> f64 {
        if let Some(function) = self.interp.stat_fns[usize::from(call.function.0)] {
            return self.stat_call(function, call);
        }
        let Some(builtin) = self.interp.builtins[usize::from(call.function.0)] else {
            for a in &call.args {
                if let Arg::Expr(e) = a {
                    self.expr(e);
                }
            }
            let f = self.interp.table.function(call.function);
            let value = self.host.unhandled(f, call);
            return if f.returns == Returns::Void {
                0.0
            } else {
                value
            };
        };
        let missing = Ident::new("");
        let id = |i: usize| match call.args.get(i) {
            Some(Arg::Id(id)) => id.clone(),
            Some(Arg::Str(s)) => Ident::new(s),
            _ => missing.clone(),
        };
        let target = &call.target;
        match builtin {
            Builtin::Journal => {
                let index = self.int(call, 1);
                self.host.journal(&id(0), index);
            }
            Builtin::SetJournalIndex => {
                let index = self.int(call, 1);
                self.host.set_journal_index(&id(0), index);
            }
            Builtin::GetJournalIndex => return f64::from(self.host.get_journal_index(&id(0))),
            Builtin::AddTopic => self.host.add_topic(&id(0)),
            Builtin::Choice => {
                let mut choices = Vec::new();
                let mut label = None;
                for a in &call.args {
                    match a {
                        Arg::Str(s) => label = Some(s.clone()),
                        Arg::Expr(e) => {
                            let value = to_int(self.expr(e));
                            if let Some(label) = label.take() {
                                choices.push((label, value));
                            }
                        }
                        _ => {}
                    }
                }
                self.host.choice(choices);
            }
            Builtin::Goodbye => self.host.goodbye(),
            Builtin::AddItem => {
                let count = self.int(call, 1);
                self.host.add_item(target, &id(0), count);
            }
            Builtin::RemoveItem => {
                let count = self.int(call, 1);
                self.host.remove_item(target, &id(0), count);
            }
            Builtin::GetItemCount => {
                return f64::from(self.host.get_item_count(target, &id(0)));
            }
            Builtin::SetDisposition => {
                let value = self.int(call, 0);
                self.host.set_disposition(target, value);
            }
            Builtin::ModDisposition => {
                let delta = self.int(call, 0);
                self.host.mod_disposition(target, delta);
            }
            Builtin::MessageBox => {
                let format = match call.args.first() {
                    Some(Arg::Str(s) | Arg::Word(s)) => s.as_str(),
                    _ => "",
                };
                let mut args = Vec::with_capacity(call.args.len().saturating_sub(1));
                for a in call.args.iter().skip(1) {
                    args.push(match a {
                        Arg::Expr(e) => MessageArg::Num(self.expr(e)),
                        Arg::Str(s) => MessageArg::Str(s.clone()),
                        Arg::Word(s) => MessageArg::Word(s.clone()),
                        Arg::Id(id) => MessageArg::Word(id.text.clone()),
                        Arg::Axis(_) | Arg::Missing => continue,
                    });
                }
                self.host.message_box(format, &args);
            }
            Builtin::PlaySound => self.host.play_sound(&id(0)),
            Builtin::StartScript => return self.script_call(ScriptOp::Start, call, &id(0)),
            Builtin::StopScript => return self.script_call(ScriptOp::Stop, call, &id(0)),
            Builtin::ScriptRunning => return self.script_call(ScriptOp::Running, call, &id(0)),
        }
        0.0
    }

    /// `StartScript` / `StopScript` / `ScriptRunning` through [`Host::script`], else
    /// [`Host::unhandled`]; only `ScriptRunning` has a value.
    fn script_call(&mut self, op: ScriptOp, call: &Call, script: &Ident) -> f64 {
        let value = match self.host.script(&call.target, op, script) {
            Some(v) => v,
            None => self.host.unhandled(self.interp.table.function(call.function), call),
        };
        if op == ScriptOp::Running { value } else { 0.0 }
    }
}
