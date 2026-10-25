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

use std::fmt::Write as _;

use crate::ast::{
    Arg, ArithOp, Call, Expr, Ident, Script, Stmt, StmtKind, Target, VarRef, VarType,
};
use crate::functions::{Function, FunctionTable, Returns};
use crate::lexer::CmpOp;

/// Statements and loop tests one run may execute before it is stopped.
pub const DEFAULT_STEP_LIMIT: u32 = 100_000;

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
}

impl Builtin {
    pub const ALL: [Builtin; 13] = [
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
        }
    }
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
    /// The formatted text ([`format_message`]) and the button labels (none: a notification).
    fn message_box(&mut self, text: String, buttons: Vec<String>);
    fn play_sound(&mut self, sound: &Ident);

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

/// `MessageBox` text substitution (`printf`-style, scripting-adapter.md semantics 5):
/// `%f`/`%F` (6 decimals), `%.Nf`, `%g`/`%G` (6 significant digits, trailing zeros
/// dropped), `%d`/`%D` (truncated), `%%`. Values are used in order; missing ones read 0.
/// Anything else after `%` stays as written.
pub fn format_message(text: &str, values: &[f64]) -> String {
    let mut out = String::with_capacity(text.len());
    let mut values = values.iter().copied();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        let mut spec = String::from("%");
        let mut precision = None;
        if chars.peek() == Some(&'.') {
            spec.push('.');
            chars.next();
            let mut digits = String::new();
            while let Some(d) = chars.peek().copied().filter(char::is_ascii_digit) {
                digits.push(d);
                chars.next();
            }
            spec.push_str(&digits);
            precision = Some(digits.parse::<usize>().unwrap_or(0));
        }
        match chars.peek().copied() {
            Some('%') if precision.is_none() => {
                chars.next();
                out.push('%');
            }
            Some('f' | 'F') => {
                chars.next();
                let v = values.next().unwrap_or(0.0);
                let _ = write!(out, "{v:.*}", precision.unwrap_or(6));
            }
            Some('g' | 'G') if precision.is_none() => {
                chars.next();
                out.push_str(&format_g(values.next().unwrap_or(0.0)));
            }
            Some('d' | 'D') if precision.is_none() => {
                chars.next();
                let _ = write!(out, "{}", to_int(values.next().unwrap_or(0.0)));
            }
            _ => out.push_str(&spec),
        }
    }
    out
}

/// C's `%g` with precision 6: fixed notation for exponents -4..6, else scientific;
/// trailing zeros dropped.
#[allow(clippy::cast_possible_truncation)]
fn format_g(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    if !v.is_finite() {
        return format!("{v}").to_lowercase();
    }
    let sci = format!("{v:.5e}");
    let (mantissa, exp) = sci.split_once('e').unwrap_or((&sci, "0"));
    let exp: i32 = exp.parse().unwrap_or(0);
    let trim = |s: &str| {
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_owned()
        } else {
            s.to_owned()
        }
    };
    if (-4..6).contains(&exp) {
        let decimals = usize::try_from(5 - exp).unwrap_or(0);
        trim(&format!("{v:.decimals$}"))
    } else {
        let sign = if exp < 0 { '-' } else { '+' };
        format!("{}e{sign}{:02}", trim(mantissa), exp.abs())
    }
}

/// Runs scripts against a [`Host`]. Built once per function table.
#[derive(Clone, Debug)]
pub struct Interpreter<'t> {
    table: &'t FunctionTable,
    builtins: Vec<Option<Builtin>>,
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
        let mut builtins = vec![None; table.functions.len()];
        for b in Builtin::ALL {
            if let Some(id) = table.get(b.name()) {
                builtins[usize::from(id.0)] = Some(b);
            }
        }
        Self {
            table,
            builtins,
            step_limit: DEFAULT_STEP_LIMIT,
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

    /// Calls the function and returns its value (0 for void functions).
    fn call(&mut self, call: &Call) -> f64 {
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
                let mut text = String::new();
                let mut values = Vec::new();
                let mut buttons = Vec::new();
                for (i, a) in call.args.iter().enumerate() {
                    match a {
                        Arg::Str(s) if i == 0 => text.clone_from(s),
                        Arg::Str(s) => buttons.push(s.clone()),
                        Arg::Expr(e) => values.push(self.expr(e)),
                        _ => {}
                    }
                }
                self.host
                    .message_box(format_message(&text, &values), buttons);
            }
            Builtin::PlaySound => self.host.play_sound(&id(0)),
        }
        0.0
    }
}
