//! Diagnostics. Every leniency the front end applies is a named [`Rule`],
//! so the corpus test can count how often each one fires and the bake
//! report can list them per script.

use std::fmt;

/// How bad a diagnostic is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// Syntax the original compiler accepted silently; recorded for counts.
    Accepted,
    /// Malformed text repaired with a defined meaning (the script runs).
    Repaired,
    /// Resolution warning: the name may exist at runtime (later plugins).
    Warning,
    /// Unrecoverable: the script is inert (never runs, never half-runs).
    Error,
}

macro_rules! rules {
    ($($(#[doc = $doc:literal])* $name:ident = $sev:ident,)*) => {
        /// A named leniency or error rule.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Rule { $($(#[doc = $doc])* $name,)* }

        impl Rule {
            /// Every rule, in declaration order.
            pub const ALL: &'static [Rule] = &[$(Rule::$name,)*];

            pub const fn severity(self) -> Severity {
                match self { $(Rule::$name => Severity::$sev,)* }
            }

            pub const fn name(self) -> &'static str {
                match self { $(Rule::$name => stringify!($name),)* }
            }
        }
    };
}

rules! {
    // Lexical.
    /// `,` between arguments (optional separator in `MWScript`).
    CommaBetweenArgs = Accepted,
    /// A comparison operator split by a space (`> =`, `= =`).
    SpacedOperator = Repaired,
    /// `=<`, `=>`, `=` used as a comparison, `<>` as "not equal".
    NonstandardOperator = Repaired,
    /// A string literal missing its closing quote (runs to the line end).
    UnterminatedString = Repaired,
    /// A character with no meaning in `MWScript`, skipped.
    StrayCharacter = Repaired,

    // Structure.
    /// Values after a call's last argument, ignored (`GetDisabled "id" == 0`, `Disable "id"`).
    ExtraArgument = Repaired,
    /// Tokens after a complete statement, ignored (as the original compiler did).
    TrailingTokens = Repaired,
    /// `End` followed by a name or other text (the name is not checked).
    EndWithName = Accepted,
    /// Text after the script's `End`, ignored.
    TextAfterEnd = Repaired,
    /// No `End` before the end of the text.
    MissingEnd = Repaired,
    /// A second `Begin`, ignored.
    DuplicateBegin = Repaired,
    /// No `Begin` line (the whole text is the body).
    MissingBegin = Repaired,
    /// Statements before `Begin`, kept in order.
    StatementsBeforeBegin = Repaired,
    /// An `If`/`While` still open at `End` (closed there).
    MissingEndIf = Repaired,
    /// `EndIf`/`EndWhile`/`Else`/`ElseIf` without an open block, ignored.
    UnmatchedBlockEnd = Repaired,
    /// `EndIf` closing a `While` or `EndWhile` closing an `If`.
    MismatchedBlockEnd = Repaired,
    /// `ElseIf` after `Else` in one `If`, treated as part of the `Else` branch's chain.
    ElseIfAfterElse = Repaired,
    /// `Else If (...)` written as two words, read as `ElseIf`.
    ElseIfAsTwoWords = Repaired,
    /// Unbalanced parentheses in a condition or expression.
    UnbalancedParens = Repaired,
    /// `If`/`While` with no condition: never true.
    MissingCondition = Repaired,
    /// `Set x x`, `Set x = y` or `Set x to = y`: the `to` keyword missing or doubled.
    SetWithoutTo = Repaired,
    /// A declaration whose type keyword is followed by no name, ignored.
    EmptyDeclaration = Repaired,
    /// A local declared twice (the first declaration wins).
    DuplicateLocal = Accepted,
    /// A local declared after Begin's first statement (all locals are script-wide).
    LateDeclaration = Accepted,
    /// A function called with fewer required arguments than its signature (missing read as 0 / "").
    MissingArgument = Repaired,
    /// `ref->Unknown` inside an expression: reads 0 (the rest of the line is ignored).
    UnknownFunctionInExpression = Repaired,
    /// An ID with a hyphen written without spaces (`Ald-ruhn`), read as one ID.
    HyphenatedId = Accepted,
    /// `ElseIf`/`Else` right after an `If` block closed: its `EndIf` was extra and is ignored.
    ExtraEndIf = Repaired,
    /// `x->( Fn args ) == 1`: a call in parentheses after `->` (the reference applies to the calls inside).
    ParenthesizedCall = Repaired,
    /// `x->name` where `name` is a declared local: reads the local (the reference is ignored).
    ReferenceOnVariable = Repaired,
    /// An explicit reference (`x->`) before `Set`, `If` or a keyword, ignored.
    ReferenceOnKeyword = Repaired,
    /// A line that is not a statement (stray expression, garbage), ignored.
    UnknownStatement = Repaired,
    /// A statement line beginning with an unknown function name: the line is ignored.
    UnknownFunction = Error,
    /// An expression could not be parsed.
    BadExpression = Error,
    /// `Set` with no variable name.
    BadSetTarget = Error,

    // Resolution.
    /// A name used as a variable that is no local, global or speaker local.
    UndeclaredVariable = Warning,
    /// `id.var` where `id` is no known object or script.
    UnknownMemberObject = Warning,
    /// `id.var` where `id`'s script has no such local.
    UnknownMember = Warning,
    /// An explicit reference to an ID that is no known record.
    UnknownReference = Warning,
    /// A local named like a function or a global (the local wins).
    ShadowedName = Accepted,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// One diagnostic at a source position (1-based line).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub rule: Rule,
    pub line: u32,
    /// Free detail (a name, a token); never the source line itself.
    pub detail: String,
}

impl Diagnostic {
    pub fn severity(&self) -> Severity {
        self.rule.severity()
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L{}: {}", self.line, self.rule)?;
        if !self.detail.is_empty() {
            write!(f, " ({})", self.detail)?;
        }
        Ok(())
    }
}

/// Collected diagnostics for one script.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Diagnostics(pub Vec<Diagnostic>);

impl Diagnostics {
    pub fn push(&mut self, rule: Rule, line: u32, detail: impl Into<String>) {
        self.0.push(Diagnostic {
            rule,
            line,
            detail: detail.into(),
        });
    }

    /// Any [`Severity::Error`]: the script is inert.
    pub fn is_unrecoverable(&self) -> bool {
        self.0.iter().any(|d| d.severity() == Severity::Error)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.0.iter()
    }
}
