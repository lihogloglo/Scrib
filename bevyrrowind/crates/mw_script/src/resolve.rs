//! Name resolution (scripting-adapter.md "Pipeline" step 2): bare names
//! become locals, globals, speaker locals or undeclared; `id.var` members and
//! explicit references are checked against the content. Unknown IDs are
//! warnings, never errors: they may come from later plugins at runtime.

use crate::ast::{
    Arg, Call, Expr, Ident, Script, SourceKind, Stmt, StmtKind, Target, VarRef, VarType,
};
use crate::diag::{Diagnostics, Rule};
use crate::functions::FunctionTable;

/// Result of looking up `object.var`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberLookup {
    Found(VarType),
    /// The object (or global script) exists but its script has no such local.
    NoSuchVar,
    NoSuchObject,
    /// The environment can't tell (no content loaded): no diagnostic.
    Unknown,
}

/// What resolution needs to know about the loaded content. All keys are lowercase.
pub trait Env {
    /// A GLOB record's type.
    fn global(&self, key: &str) -> Option<VarType>;
    /// Any record or placed object with this ID (`player` included).
    /// `None`: the environment can't tell.
    fn object_exists(&self, key: &str) -> Option<bool>;
    /// `object.var`: `object` is an object whose base record has a script, or a script ID.
    fn member(&self, object: &str, var: &str) -> MemberLookup;
}

/// An environment that knows nothing: every lookup is `Unknown`, names that
/// are not locals resolve as globals (scripts) or speaker locals (results).
#[derive(Clone, Copy, Debug, Default)]
pub struct NoEnv;

impl Env for NoEnv {
    fn global(&self, _: &str) -> Option<VarType> {
        None
    }
    fn object_exists(&self, _: &str) -> Option<bool> {
        None
    }
    fn member(&self, _: &str, _: &str) -> MemberLookup {
        MemberLookup::Unknown
    }
}

struct Resolver<'a> {
    kind: SourceKind,
    locals: Vec<String>,
    env: &'a dyn Env,
    diags: &'a mut Diagnostics,
    /// `NoEnv`-like environments: unknown names become globals, not warnings.
    blind: bool,
}

/// Resolves every [`VarRef::Name`] in `script` and checks references.
pub fn resolve(script: &mut Script, env: &dyn Env, table: &FunctionTable, diags: &mut Diagnostics) {
    for l in &script.locals {
        if table.get_lower(&l.name.key).is_some() || env.global(&l.name.key).is_some() {
            diags.push(Rule::ShadowedName, l.line, l.name.text.clone());
        }
    }
    let mut r = Resolver {
        kind: script.kind,
        locals: script.locals.iter().map(|l| l.name.key.clone()).collect(),
        env,
        diags,
        blind: env.object_exists("player").is_none(),
    };
    r.stmts(&mut script.body);
}

impl Resolver<'_> {
    fn stmts(&mut self, stmts: &mut [Stmt]) {
        for s in stmts {
            let line = s.line;
            match &mut s.kind {
                StmtKind::Set { target, value } => {
                    self.var(target, line);
                    self.expr(value, line);
                }
                StmtKind::If { arms, otherwise } => {
                    for arm in arms {
                        self.expr(&mut arm.cond, arm.line);
                        self.stmts(&mut arm.body);
                    }
                    self.stmts(otherwise);
                }
                StmtKind::While { cond, body } => {
                    self.expr(cond, line);
                    self.stmts(body);
                }
                StmtKind::Call(call) => self.call(call),
                StmtKind::Return => {}
            }
        }
    }

    fn expr(&mut self, e: &mut Expr, line: u32) {
        match e {
            Expr::Var(v) => self.var(v, line),
            Expr::Call(c) => self.call(c),
            Expr::Neg(x) => self.expr(x, line),
            Expr::Arith(_, a, b) | Expr::Cmp(_, a, b) => {
                self.expr(a, line);
                self.expr(b, line);
            }
            Expr::Int(_) | Expr::Float(_) => {}
        }
    }

    fn call(&mut self, call: &mut Call) {
        if let Target::Explicit(id) = &call.target
            && self.env.object_exists(&id.key) == Some(false)
        {
            self.diags
                .push(Rule::UnknownReference, call.line, id.text.clone());
        }
        for a in &mut call.args {
            if let Arg::Expr(e) = a {
                self.expr(e, call.line);
            }
        }
    }

    fn var(&mut self, v: &mut VarRef, line: u32) {
        match v {
            VarRef::Name(name) => {
                *v = self.name(name.clone(), line);
            }
            VarRef::Member { object, var } => match self.env.member(&object.key, &var.key) {
                MemberLookup::NoSuchObject => {
                    self.diags
                        .push(Rule::UnknownMemberObject, line, object.text.clone());
                }
                MemberLookup::NoSuchVar => {
                    self.diags.push(
                        Rule::UnknownMember,
                        line,
                        format!("{}.{}", object.text, var.text),
                    );
                }
                MemberLookup::Found(_) | MemberLookup::Unknown => {}
            },
            _ => {}
        }
    }

    fn name(&mut self, name: Ident, line: u32) -> VarRef {
        if let Some(slot) = self.locals.iter().position(|l| *l == name.key) {
            return VarRef::Local(slot);
        }
        if self.env.global(&name.key).is_some() {
            return VarRef::Global(name);
        }
        match self.kind {
            SourceKind::DialogueResult => VarRef::Speaker(name),
            SourceKind::Script if self.blind => VarRef::Global(name),
            SourceKind::Script => {
                self.diags
                    .push(Rule::UndeclaredVariable, line, name.text.clone());
                VarRef::Undeclared(name)
            }
        }
    }
}
