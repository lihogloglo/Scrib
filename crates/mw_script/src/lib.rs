//! `MWScript` front end for the `MWScript` → Luau translator
//! (`docs/specs/scripting-adapter.md`, M5 W1).
//!
//! Pipeline: [`lexer::lex`] → lenient line parser → [`resolve`]. Every
//! leniency is a named [`Rule`] with a [`Severity`]; a script with any
//! [`Severity::Error`] is **inert** ([`Parsed::is_inert`]): it never runs and
//! never half-runs. The AST ([`ast`]) names every variable by where its
//! state lives, so the emitter (W2) keeps all state in module tables, never
//! in Luau globals.
//!
//! No existing parser was reused: no `MWScript` crate exists on crates.io
//! (searched 2026-10-13), `OpenMW`'s compiler is C++ and tied to its
//! interpreter, and the language is line-based with call arity taken from a
//! function table, which parser generators don't model well; a hand-written
//! recursive-descent parser with per-line recovery is the standard choice
//! for lenient legacy front ends.

pub mod ast;
pub mod diag;
pub mod functions;
pub mod lexer;
mod parser;
pub mod resolve;

pub use ast::{Script, SourceKind};
pub use diag::{Diagnostic, Diagnostics, Rule, Severity};
pub use functions::{FnId, FunctionTable};
pub use resolve::{Env, MemberLookup, NoEnv};

/// A parsed script and everything reported on the way.
#[derive(Clone, Debug)]
pub struct Parsed {
    pub script: Script,
    pub diagnostics: Diagnostics,
}

impl Parsed {
    /// Unrecoverable: the translator emits an inert script and reports it.
    pub fn is_inert(&self) -> bool {
        self.diagnostics.is_unrecoverable()
    }
}

/// Lexes and parses `text` (no name resolution).
pub fn parse(text: &str, kind: SourceKind, table: &FunctionTable) -> Parsed {
    let mut diagnostics = Diagnostics::default();
    let tokens = lexer::lex(text, &mut diagnostics);
    let script = parser::Parser::new(tokens, table, &mut diagnostics).parse(kind);
    Parsed {
        script,
        diagnostics,
    }
}

/// [`parse`] then [`resolve::resolve`] against `env`.
pub fn parse_and_resolve(
    text: &str,
    kind: SourceKind,
    table: &FunctionTable,
    env: &dyn Env,
) -> Parsed {
    let mut parsed = parse(text, kind, table);
    resolve::resolve(&mut parsed.script, env, table, &mut parsed.diagnostics);
    parsed
}

#[cfg(test)]
mod tests;
