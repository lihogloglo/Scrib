//! The coverage report (scripting-adapter.md "Coverage report", M5 W2): call sites per
//! function in the translated corpus, with each function's M5 [`Status`], summed by family
//! and by status. Names and counts only, never script text.

use std::collections::BTreeMap;
use std::fmt::{self, Write as _};

use crate::Parsed;
use crate::ast::{Arg, ArithOp, Expr, StmtKind, VarRef, VarType};
use crate::functions::{FunctionTable, Returns, Status};
use crate::resolve::{Env, MemberLookup};

/// Where a call site is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    Script,
    DialogueResult,
}

/// Call sites by function, split by origin.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CallSites {
    /// Canonical function name → (in scripts, in dialogue results).
    pub by_function: BTreeMap<String, (u32, u32)>,
}

impl CallSites {
    /// Counts every call of a parsed text (statements, conditions, arguments).
    pub fn add_parsed(&mut self, parsed: &Parsed, origin: Origin, table: &FunctionTable) {
        parsed.script.walk_calls(&mut |c| {
            self.add(&table.function(c.function).name, origin, 1);
        });
    }

    pub fn add(&mut self, function: &str, origin: Origin, n: u32) {
        let e = self.by_function.entry(function.to_owned()).or_default();
        match origin {
            Origin::Script => e.0 += n,
            Origin::DialogueResult => e.1 += n,
        }
    }

    pub fn merge(&mut self, other: &CallSites) {
        for (f, (s, r)) in &other.by_function {
            self.add(f, Origin::Script, *s);
            self.add(f, Origin::DialogueResult, *r);
        }
    }

    /// The report against `table` (statuses and families).
    pub fn report(&self, table: &FunctionTable) -> Coverage {
        let mut rows = Vec::new();
        for (name, &(scripts, results)) in &self.by_function {
            let (family, status) = table.get(name).map_or(("unknown".to_owned(), Status::Later), |id| {
                let f = table.function(id);
                (f.family.clone(), f.status)
            });
            rows.push(Row {
                function: name.clone(),
                family,
                status,
                scripts,
                results,
            });
        }
        rows.sort_by(|a, b| {
            (b.scripts + b.results)
                .cmp(&(a.scripts + a.results))
                .then_with(|| a.function.cmp(&b.function))
        });
        Coverage { rows }
    }
}

/// One function's line in the report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub function: String,
    pub family: String,
    pub status: Status,
    pub scripts: u32,
    pub results: u32,
}

impl Row {
    pub fn total(&self) -> u32 {
        self.scripts + self.results
    }
}

/// Call sites live / stub / later.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Split {
    pub live: u32,
    pub stub: u32,
    pub later: u32,
}

impl Split {
    fn add(&mut self, status: Status, n: u32) {
        match status {
            Status::Live => self.live += n,
            Status::Stub(_) => self.stub += n,
            Status::Later => self.later += n,
        }
    }

    pub fn total(&self) -> u32 {
        self.live + self.stub + self.later
    }

    /// Percentage of call sites that are live.
    #[allow(clippy::cast_precision_loss)]
    pub fn live_percent(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            100.0 * f64::from(self.live) / f64::from(self.total())
        }
    }
}

impl fmt::Display for Split {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} call sites: {} live ({:.1} %), {} stub, {} later",
            self.total(),
            self.live,
            self.live_percent(),
            self.stub,
            self.later
        )
    }
}

/// The coverage report: one row per called function, busiest first.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Coverage {
    pub rows: Vec<Row>,
}

impl Coverage {
    /// All call sites by status.
    pub fn split(&self) -> Split {
        let mut s = Split::default();
        for r in &self.rows {
            s.add(r.status, r.total());
        }
        s
    }

    /// Call sites by family (busiest first), each split by status.
    pub fn families(&self) -> Vec<(String, Split)> {
        let mut map: BTreeMap<&str, Split> = BTreeMap::new();
        for r in &self.rows {
            map.entry(&r.family).or_default().add(r.status, r.total());
        }
        let mut v: Vec<(String, Split)> = map.into_iter().map(|(k, s)| (k.to_owned(), s)).collect();
        v.sort_by(|a, b| b.1.total().cmp(&a.1.total()).then_with(|| a.0.cmp(&b.0)));
        v
    }
}

impl fmt::Display for Coverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MWScript coverage (M5 status by call sites, scripts + dialogue results)")?;
        writeln!(f, "total: {}", self.split())?;
        writeln!(f, "\nby family:")?;
        for (family, s) in self.families() {
            writeln!(
                f,
                "  {family:<12} {:>6}  live {:>6}  stub {:>6}  later {:>6}",
                s.total(),
                s.live,
                s.stub,
                s.later
            )?;
        }
        writeln!(f, "\nby function (scripts / results):")?;
        let mut line = String::new();
        for r in &self.rows {
            line.clear();
            let status = match r.status {
                Status::Stub(n) if n != 0 => format!("stub={n}"),
                s => s.name().to_owned(),
            };
            let _ = write!(
                line,
                "  {:<24} {:<12} {status:<8} {:>6} {:>6}",
                r.function, r.family, r.scripts, r.results
            );
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

/// Numeric spots whose semantics wait for the `OpenMW` black-box oracle (spec open question 1).
/// The emitter follows the original's float evaluation (emit.rs "Numeric semantics"); these
/// counts say how much content a different oracle answer would touch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OracleFlags {
    /// `/` with both operands integer-typed (`short`/`long` variables, integer literals or
    /// integer-returning functions): float division today, truncated only on assignment.
    pub int_div: u32,
    /// Comparisons between an integer-typed operand (not a literal) and a `float`-typed one.
    pub mixed_cmp: u32,
}

impl OracleFlags {
    pub fn add(&mut self, other: OracleFlags) {
        self.int_div += other.int_div;
        self.mixed_cmp += other.mixed_cmp;
    }
}

impl fmt::Display for OracleFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} integer divisions, {} short/float comparisons (pending the OpenMW oracle)",
            self.int_div, self.mixed_cmp
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Num {
    Int,
    Float,
}

/// Counts the oracle-pending numeric spots of a parsed, resolved text.
pub fn oracle_flags(parsed: &Parsed, table: &FunctionTable, env: &dyn Env) -> OracleFlags {
    let mut cx = Flagger {
        parsed,
        table,
        env,
        flags: OracleFlags::default(),
    };
    parsed.script.walk_stmts(&mut |s| match &s.kind {
        StmtKind::Set { value, .. } => {
            cx.expr(value);
        }
        StmtKind::If { arms, .. } => {
            for a in arms {
                cx.expr(&a.cond);
            }
        }
        StmtKind::While { cond, .. } => {
            cx.expr(cond);
        }
        StmtKind::Call(call) => cx.args(&call.args),
        StmtKind::Return => {}
    });
    cx.flags
}

struct Flagger<'a> {
    parsed: &'a Parsed,
    table: &'a FunctionTable,
    env: &'a dyn Env,
    flags: OracleFlags,
}

impl Flagger<'_> {
    fn args(&mut self, args: &[Arg]) {
        for a in args {
            if let Arg::Expr(e) = a {
                self.expr(e);
            }
        }
    }

    /// Visits `e` (counting flags) and returns its static numeric kind, if known.
    fn expr(&mut self, e: &Expr) -> Option<Num> {
        let of = |t: VarType| if t == VarType::Float { Num::Float } else { Num::Int };
        match e {
            Expr::Int(_) => Some(Num::Int),
            Expr::Float(_) => Some(Num::Float),
            Expr::Var(v) => match v {
                VarRef::Local(slot) => Some(of(self.parsed.script.locals[*slot].ty)),
                VarRef::Global(name) => self.env.global(&name.key).map(of),
                VarRef::Member { object, var } => match self.env.member(&object.key, &var.key) {
                    MemberLookup::Found(t) => Some(of(t)),
                    _ => None,
                },
                VarRef::Undeclared(_) => Some(Num::Int),
                VarRef::Name(_) | VarRef::Speaker(_) => None,
            },
            Expr::Call(c) => {
                self.args(&c.args);
                match self.table.function(c.function).returns {
                    Returns::Float => Some(Num::Float),
                    Returns::Int | Returns::Void => Some(Num::Int),
                }
            }
            Expr::Neg(x) => self.expr(x),
            Expr::Arith(op, a, b) => {
                let (a, b) = (self.expr(a), self.expr(b));
                match (op, a, b) {
                    (ArithOp::Div, Some(Num::Int), Some(Num::Int)) => {
                        self.flags.int_div += 1;
                        None
                    }
                    (ArithOp::Div, ..) | (_, Some(Num::Float), _) | (_, _, Some(Num::Float)) => {
                        Some(Num::Float)
                    }
                    (_, Some(Num::Int), Some(Num::Int)) => Some(Num::Int),
                    _ => None,
                }
            }
            Expr::Cmp(_, a, b) => {
                // An integer literal converts to float exactly: only typed operands count.
                let literal = |e: &Expr| matches!(e, Expr::Int(_)) || matches!(e, Expr::Neg(x) if matches!(**x, Expr::Int(_)));
                let (lit_a, lit_b) = (literal(a), literal(b));
                let (a, b) = (self.expr(a), self.expr(b));
                if matches!(
                    (a, b),
                    (Some(Num::Int), Some(Num::Float)) if !lit_a
                ) || matches!((a, b), (Some(Num::Float), Some(Num::Int)) if !lit_b)
                {
                    self.flags.mixed_cmp += 1;
                }
                Some(Num::Int)
            }
        }
    }
}
