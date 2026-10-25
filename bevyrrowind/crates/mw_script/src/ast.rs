//! The `MWScript` AST, shaped for the Luau emitter (W2).
//!
//! State never lives in Luau globals (scripting-adapter.md "Emit Luau"): a
//! sandboxed Luau state gives every chunk one shared writable environment,
//! so the AST names every variable by *where its state lives*, and the
//! emitter maps each [`VarRef`] onto a module table:
//!
//! - [`VarRef::Local`]: a slot in this instance's variable table (`v.<name>`),
//! - [`VarRef::Global`]: the GLOB table (`mw.globals`),
//! - [`VarRef::Member`]: another object's or global script's variable table,
//! - [`VarRef::Speaker`]: the speaker's variable table (dialogue results).
//!
//! Names are case-insensitive: [`Ident::key`] is the lowercase form used for
//! lookups and as the Luau table key, [`Ident::text`] the source spelling.

use crate::functions::FnId;
use crate::lexer::CmpOp;

/// A case-insensitive name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ident {
    /// Lowercase: the lookup and table key.
    pub key: String,
    /// As written in the source.
    pub text: String,
}

impl Ident {
    pub fn new(text: &str) -> Self {
        Self {
            key: text.to_lowercase(),
            text: text.to_owned(),
        }
    }
}

/// Storage type of a variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VarType {
    /// 16-bit signed integer.
    Short,
    /// 32-bit signed integer.
    Long,
    /// 32-bit float.
    Float,
}

/// Where the text came from: decides the top-level structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SourceKind {
    /// An SCPT record: `Begin name` ... `End`, locals allowed.
    Script,
    /// An INFO result (`BNAM`): bare statements run once as the speaker.
    DialogueResult,
}

/// A local variable declaration. Its index in [`Script::locals`] is its slot.
#[derive(Clone, Debug, PartialEq)]
pub struct Local {
    pub name: Ident,
    pub ty: VarType,
    pub line: u32,
}

/// A parsed (and, after [`crate::resolve`], resolved) script.
#[derive(Clone, Debug, PartialEq)]
pub struct Script {
    pub kind: SourceKind,
    /// The `Begin` name (scripts only; not checked against the record ID).
    pub name: Option<Ident>,
    /// Script-wide locals in declaration order (slot = index).
    pub locals: Vec<Local>,
    pub body: Vec<Stmt>,
}

impl Script {
    /// The slot of a local by lowercase name.
    pub fn local(&self, key: &str) -> Option<usize> {
        self.locals.iter().position(|l| l.name.key == key)
    }

    /// Visits every statement, depth first, in source order.
    pub fn walk_stmts(&self, f: &mut impl FnMut(&Stmt)) {
        fn walk(stmts: &[Stmt], f: &mut impl FnMut(&Stmt)) {
            for s in stmts {
                f(s);
                match &s.kind {
                    StmtKind::If { arms, otherwise } => {
                        for arm in arms {
                            walk(&arm.body, f);
                        }
                        walk(otherwise, f);
                    }
                    StmtKind::While { body, .. } => walk(body, f),
                    _ => {}
                }
            }
        }
        walk(&self.body, f);
    }

    /// Visits every call (statements, conditions, arguments), in source order.
    pub fn walk_calls(&self, f: &mut impl FnMut(&Call)) {
        self.walk_stmts(&mut |s| match &s.kind {
            StmtKind::Set { value, .. } => value.walk_calls(f),
            StmtKind::If { arms, .. } => arms.iter().for_each(|a| a.cond.walk_calls(f)),
            StmtKind::While { cond, .. } => cond.walk_calls(f),
            StmtKind::Call(call) => call.walk_calls(f),
            StmtKind::Return => {}
        });
    }
}

/// A statement and its 1-based source line (for `-- L<n>` source maps).
#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub line: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    /// `Set target To value`.
    Set {
        target: VarRef,
        value: Expr,
    },
    /// `If`/`ElseIf` arms in order, then the `Else` body (may be empty).
    If {
        arms: Vec<CondArm>,
        otherwise: Vec<Stmt>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    /// Ends this frame's run of the script.
    Return,
    /// A function call whose result (if any) is discarded.
    Call(Call),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CondArm {
    pub cond: Expr,
    pub line: u32,
    pub body: Vec<Stmt>,
}

/// The object a call acts on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    /// The instance running the script (the speaker, for dialogue results).
    Implicit,
    /// `id->`: an object by record ID (`player` included; resolution
    /// classifies it, the runtime looks it up).
    Explicit(Ident),
}

/// A function call.
#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub target: Target,
    /// Index into the [`crate::FunctionTable`].
    pub function: FnId,
    pub args: Vec<Arg>,
    pub line: u32,
}

impl Call {
    fn walk_calls(&self, f: &mut impl FnMut(&Call)) {
        f(self);
        for a in &self.args {
            if let Arg::Expr(e) = a {
                e.walk_calls(f);
            }
        }
    }
}

/// A function argument, typed by the function's signature.
#[derive(Clone, Debug, PartialEq)]
pub enum Arg {
    /// A record or object ID (bare word or quoted string).
    Id(Ident),
    /// Text (message box text, button labels, sound files, animation groups).
    Str(String),
    /// A numeric value.
    Expr(Expr),
    /// `X`, `Y` or `Z`.
    Axis(Axis),
    /// A required argument the source left out (`MissingArgument`): reads 0 / "".
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// A numeric expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Var(VarRef),
    Call(Box<Call>),
    Neg(Box<Expr>),
    Arith(ArithOp, Box<Expr>, Box<Expr>),
    /// 1 or 0.
    Cmp(CmpOp, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn walk_calls(&self, f: &mut impl FnMut(&Call)) {
        match self {
            Expr::Call(c) => c.walk_calls(f),
            Expr::Neg(e) => e.walk_calls(f),
            Expr::Arith(_, a, b) | Expr::Cmp(_, a, b) => {
                a.walk_calls(f);
                b.walk_calls(f);
            }
            Expr::Int(_) | Expr::Float(_) | Expr::Var(_) => {}
        }
    }
}

/// A variable, named by where its state lives (see the module docs).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VarRef {
    /// Unresolved bare name (parser output; [`crate::resolve`] replaces it).
    Name(Ident),
    /// A local of this script: slot in [`Script::locals`].
    Local(usize),
    /// A global variable (GLOB record).
    Global(Ident),
    /// `object.var`: a local of another object's script, or of a global script by script ID.
    Member { object: Ident, var: Ident },
    /// A dialogue result's bare name that is no global: the speaker's local.
    Speaker(Ident),
    /// A name nothing declares (`UndeclaredVariable`): reads 0, writes are dropped.
    Undeclared(Ident),
}
