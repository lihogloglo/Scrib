//! The `MWScript` function table: one row per function (`functions.txt`).
//! It drives the parser (`MWScript` calls have no parentheses, so argument
//! count and kinds come from here), resolution, and later the emitter, the
//! `@mw/script` type definitions and the coverage report.

use std::collections::HashMap;
use std::sync::OnceLock;

/// Index of a function in [`FunctionTable::functions`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FnId(pub u16);

/// What an argument position accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArgKind {
    /// A record or object ID: bare word or quoted string.
    Id,
    /// A string literal (a bare word is accepted).
    Text,
    /// A numeric value: literal, variable, `-x`, `(expr)` or a call.
    Num,
    /// `X`, `Y` or `Z`.
    Axis,
    /// A string or a value (`MessageBox` arguments and buttons, `Choice`).
    Any,
}

/// How often an argument position may appear.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Arity {
    Required,
    Optional,
    /// Zero or more (last position only).
    Rest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArgSpec {
    pub kind: ArgKind,
    pub arity: Arity,
}

/// Return kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Returns {
    Void,
    Int,
    Float,
}

/// M5 status of a function (the `status` column; W2 coverage report).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Status {
    /// Implemented in M5 (W3).
    Live,
    /// A logged neutral stand-in returning this value (0 unless `stub=<n>`).
    Stub(i32),
    /// No meaningful M5 stand-in: a logged no-op returning 0.
    Later,
}

impl Status {
    /// `live`, `stub` or `later` (the neutral value dropped).
    pub fn name(self) -> &'static str {
        match self {
            Status::Live => "live",
            Status::Stub(_) => "stub",
            Status::Later => "later",
        }
    }

    /// What the runtime returns while the function is not live.
    pub fn neutral(self) -> i32 {
        match self {
            Status::Stub(n) => n,
            Status::Live | Status::Later => 0,
        }
    }
}

/// One function's signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    /// Canonical spelling.
    pub name: String,
    pub args: Vec<ArgSpec>,
    pub returns: Returns,
    /// Status family (scripting-adapter.md "What M5 needs from M7").
    pub family: String,
    pub status: Status,
    /// Acts on a reference (implicit instance or `id->`).
    pub on_reference: bool,
}

impl Function {
    /// The Luau name: `snake_case` of the canonical name (`GetPCCell` → `get_pc_cell`).
    pub fn luau_name(&self) -> String {
        snake_case(&self.name)
    }

    pub fn required_args(&self) -> usize {
        self.args
            .iter()
            .filter(|a| a.arity == Arity::Required)
            .count()
    }
}

/// Every known function, by lowercase name and alias.
#[derive(Clone, Debug, Default)]
pub struct FunctionTable {
    pub functions: Vec<Function>,
    by_name: HashMap<String, FnId>,
}

/// A malformed `functions.txt` line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableError {
    pub line: usize,
    pub message: String,
}

impl FunctionTable {
    /// The built-in vanilla table (Morrowind, Tribunal, Bloodmoon).
    pub fn vanilla() -> &'static FunctionTable {
        static TABLE: OnceLock<FunctionTable> = OnceLock::new();
        TABLE.get_or_init(|| {
            Self::parse(include_str!("functions.txt")).expect("functions.txt is valid")
        })
    }

    pub fn get(&self, name: &str) -> Option<FnId> {
        self.by_name.get(&name.to_lowercase()).copied()
    }

    /// Every lowercase name and alias (`tcl`, `togglecollision`, ...) with its function.
    pub fn names(&self) -> impl Iterator<Item = (&str, FnId)> + '_ {
        self.by_name.iter().map(|(name, &id)| (name.as_str(), id))
    }

    /// Looks up an already-lowercase name.
    pub fn get_lower(&self, key: &str) -> Option<FnId> {
        self.by_name.get(key).copied()
    }

    pub fn function(&self, id: FnId) -> &Function {
        &self.functions[usize::from(id.0)]
    }

    /// Parses the table format documented at the top of `functions.txt`.
    pub fn parse(text: &str) -> Result<Self, TableError> {
        let mut lists: HashMap<String, Vec<String>> = HashMap::new();
        let mut table = Self::default();
        for (n, raw) in text.lines().enumerate() {
            let err = |message: String| TableError {
                line: n + 1,
                message,
            };
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(def) = line.strip_prefix('$') {
                let (name, items) = def
                    .split_once('=')
                    .ok_or_else(|| err("list without '='".into()))?;
                let items = items.split(',').map(|s| s.trim().to_owned()).collect();
                lists.insert(name.trim().to_owned(), items);
                continue;
            }
            let cols: Vec<&str> = line.split_whitespace().collect();
            let [names, args, returns, family, status, flags] = cols[..] else {
                return Err(err(format!("expected 6 columns, got {}", cols.len())));
            };
            let status = parse_status(status).map_err(&err)?;
            let args = parse_args(args).map_err(&err)?;
            let returns = match returns {
                "void" => Returns::Void,
                "int" => Returns::Int,
                "float" => Returns::Float,
                other => return Err(err(format!("bad return kind {other}"))),
            };
            let on_reference = match flags {
                "r" => true,
                "-" => false,
                other => return Err(err(format!("bad flags {other}"))),
            };
            for name in expand(names, &lists).map_err(&err)? {
                let mut spellings = name.split('|');
                let canonical = spellings.next().unwrap_or_default().to_owned();
                let id = FnId(
                    u16::try_from(table.functions.len())
                        .map_err(|_| err("too many functions".into()))?,
                );
                for spelling in std::iter::once(canonical.as_str()).chain(spellings) {
                    if table.by_name.insert(spelling.to_lowercase(), id).is_some() {
                        return Err(err(format!("duplicate function {spelling}")));
                    }
                }
                table.functions.push(Function {
                    name: canonical,
                    args: args.clone(),
                    returns,
                    family: family.to_owned(),
                    status,
                    on_reference,
                });
            }
        }
        Ok(table)
    }
}

fn parse_status(col: &str) -> Result<Status, String> {
    match col {
        "live" => Ok(Status::Live),
        "stub" => Ok(Status::Stub(0)),
        "later" => Ok(Status::Later),
        _ => col
            .strip_prefix("stub=")
            .and_then(|n| n.parse().ok())
            .map(Status::Stub)
            .ok_or(format!("bad status {col}")),
    }
}

/// `GetPCCell` → `get_pc_cell`, `AIWander` → `ai_wander`, `PCForce1stPerson` → `pc_force1st_person`.
pub fn snake_case(name: &str) -> String {
    let chars: Vec<char> = name.chars().collect();
    let mut out = String::with_capacity(name.len() + 4);
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            let prev = chars[i - 1];
            let next_lower = chars.get(i + 1).is_some_and(char::is_ascii_lowercase);
            if prev.is_ascii_lowercase() || prev.is_ascii_digit() || (prev.is_ascii_uppercase() && next_lower) {
                out.push('_');
            }
        }
        out.push(c.to_ascii_lowercase());
    }
    out
}

fn parse_args(col: &str) -> Result<Vec<ArgSpec>, String> {
    if col == "-" {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for (i, item) in col.split(',').enumerate() {
        let (base, arity) = if let Some(b) = item.strip_suffix('?') {
            (b, Arity::Optional)
        } else if let Some(b) = item.strip_suffix('*') {
            (b, Arity::Rest)
        } else {
            (item, Arity::Required)
        };
        if arity == Arity::Rest && i + 1 != col.split(',').count() {
            return Err(format!("`*` only on the last argument: {col}"));
        }
        let kind = match base {
            "id" => ArgKind::Id,
            "text" => ArgKind::Text,
            "num" => ArgKind::Num,
            "axis" => ArgKind::Axis,
            "any" => ArgKind::Any,
            other => return Err(format!("bad argument kind {other}")),
        };
        out.push(ArgSpec { kind, arity });
    }
    Ok(out)
}

/// Expands one `{...}` group (a `$LIST` or literal items) in a name.
fn expand(names: &str, lists: &HashMap<String, Vec<String>>) -> Result<Vec<String>, String> {
    let Some(open) = names.find('{') else {
        return Ok(vec![names.to_owned()]);
    };
    let close = names[open..].find('}').ok_or("unclosed '{'")? + open;
    let (prefix, inner, suffix) = (&names[..open], &names[open + 1..close], &names[close + 1..]);
    let items: Vec<String> = if let Some(list) = inner.strip_prefix('$') {
        lists
            .get(list)
            .cloned()
            .ok_or(format!("unknown list ${list}"))?
    } else {
        inner.split(',').map(str::to_owned).collect()
    };
    Ok(items
        .into_iter()
        .map(|i| format!("{prefix}{i}{suffix}"))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vanilla_table_parses_and_expands() {
        let t = FunctionTable::vanilla();
        assert!(t.functions.len() > 400, "{}", t.functions.len());
        let f = t.function(t.get("getlongblade").unwrap());
        assert_eq!(
            (f.name.as_str(), f.returns),
            ("GetLongBlade", Returns::Float)
        );
        assert_eq!(t.get("GetLOS"), t.get("GetLineOfSight"));
        let mb = t.function(t.get("MessageBox").unwrap());
        assert_eq!(
            mb.args[1],
            ArgSpec {
                kind: ArgKind::Any,
                arity: Arity::Rest
            }
        );
        assert_eq!(t.function(t.get("AIWander").unwrap()).required_args(), 3);
        assert!(t.get("GetPlayerViewSwitch").is_some());
    }

    #[test]
    fn bad_rows_are_rejected() {
        assert!(FunctionTable::parse("Foo num,bogus void x live -").is_err());
        assert!(FunctionTable::parse("Foo - void x live -\nfoo - void x live -").is_err());
        assert!(FunctionTable::parse("Foo num*,num void x live -").is_err());
        assert!(FunctionTable::parse("Foo - void x maybe -").is_err());
        assert!(FunctionTable::parse("Foo - void x -").is_err());
    }

    #[test]
    fn statuses_and_luau_names() {
        let t = FunctionTable::vanilla();
        let status = |n: &str| t.function(t.get(n).unwrap()).status;
        assert_eq!(status("GetJournalIndex"), Status::Live);
        assert_eq!(status("GetPCRank"), Status::Stub(-1));
        assert_eq!(status("AITravel"), Status::Later);
        let name = |n: &str| t.function(t.get(n).unwrap()).luau_name();
        assert_eq!(name("GetPCCell"), "get_pc_cell");
        assert_eq!(name("AIWander"), "ai_wander");
        assert_eq!(name("PCForce1stPerson"), "pc_force1st_person");
        assert_eq!(name("ModPCFacRep"), "mod_pc_fac_rep");
        // Luau names are unique (the runtime dispatches on them).
        let mut names: Vec<String> = t.functions.iter().map(Function::luau_name).collect();
        names.sort();
        let n = names.len();
        names.dedup();
        assert_eq!(names.len(), n);
    }
}
