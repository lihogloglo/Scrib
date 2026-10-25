//! Dev console core, Bevy-free. Spec: `docs/specs/console.md`.
//!
//! - [`Console`]: one namespace of commands and variables (case-insensitive,
//!   dotted names like `render.shadows`), typed argument binding, built-in
//!   commands (`help`, `set`, `toggle`, `reset`, `vars`/`list`, `history`,
//!   `echo`, `clear`), "did you mean" on unknown names.
//! - Variables ([`Vars`]): typed values with defaults, validated on every set,
//!   read by [`VarId`] every frame, with change tracking for the engine side and
//!   a "set by" priority ([`SetBy`]: default < config < command line < code < console).
//! - Line editing support: [`Prompt`] (the input line, its history browsing and
//!   completion popup), [`Console::complete`], [`History`], and the output [`Log`]
//!   a front end draws from.
//!
//! Commands get a caller-chosen context `C` (the Bevy plugin passes its `World`).
//! Line syntax: words split on whitespace, `"quoted text"` with `\"` and `\\`,
//! `;` between commands. A bare variable name prints it; `name value` sets it.

use std::collections::HashMap;
use std::fmt::Write as _;

mod history;
mod lex;
mod prompt;
mod value;
mod vars;

#[cfg(test)]
mod tests;

pub use history::{History, Level, Line, Log};
pub use lex::{Segment, Token, split};
pub use prompt::{Edit, Prompt};
pub use value::{ArgKind, NameFilter, Value, parse_bool};
pub use vars::{SetBy, SetOutcome, Var, VarId, VarSpec, Vars};

/// Default number of history entries kept.
pub const HISTORY_CAPACITY: usize = 200;
/// Default number of log lines kept.
pub const LOG_CAPACITY: usize = 2000;

/// One parameter of a command.
#[must_use]
#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    name: String,
    kind: ArgKind,
    optional: bool,
    rest: bool,
}

impl Param {
    pub fn new(name: &str, kind: ArgKind) -> Self {
        Self {
            name: name.to_owned(),
            kind,
            optional: false,
            rest: false,
        }
    }

    /// May be left out; only trailing parameters can be optional.
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// The rest of the command as raw text, quotes and all (e.g. code for a script
    /// command). Must be the last parameter. Ends at the next unquoted `;`.
    pub fn rest(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            kind: ArgKind::Str,
            optional: false,
            rest: true,
        }
    }

    fn usage(&self) -> String {
        let inner = if self.rest {
            format!("{}...", self.name)
        } else {
            format!("{}:{}", self.name, self.kind.describe())
        };
        if self.optional {
            format!("[{inner}]")
        } else {
            format!("<{inner}>")
        }
    }
}

/// What to register for a command: name, aliases, help, parameters.
#[must_use]
#[derive(Clone, Debug, PartialEq)]
pub struct CommandSpec {
    name: String,
    aliases: Vec<String>,
    help: String,
    params: Vec<Param>,
}

impl CommandSpec {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            aliases: Vec::new(),
            help: String::new(),
            params: Vec::new(),
        }
    }

    /// Another name for the command (e.g. a short form).
    pub fn alias(mut self, alias: &str) -> Self {
        self.aliases.push(alias.to_owned());
        self
    }

    pub fn help(mut self, text: &str) -> Self {
        text.clone_into(&mut self.help);
        self
    }

    pub fn param(mut self, param: Param) -> Self {
        self.params.push(param);
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// `name <a:int> [b:on|off]`.
    pub fn usage(&self) -> String {
        let mut s = self.name.clone();
        for p in &self.params {
            s.push(' ');
            s += &p.usage();
        }
        s
    }
}

/// Bound arguments of one call, by parameter name. Optional parameters left out are absent.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Args {
    values: Vec<(String, Value)>,
}

impl Args {
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.iter().find(|(n, _)| n == name).map(|(_, v)| v)
    }

    pub fn flag(&self, name: &str) -> Option<bool> {
        self.get(name).and_then(Value::as_bool)
    }

    pub fn int(&self, name: &str) -> Option<i64> {
        self.get(name).and_then(Value::as_int)
    }

    pub fn float(&self, name: &str) -> Option<f64> {
        self.get(name).and_then(Value::as_float)
    }

    /// Text, choice, name and rest parameters.
    pub fn str(&self, name: &str) -> Option<&str> {
        self.get(name).and_then(Value::as_str)
    }
}

/// What a command handler gets.
pub struct Invocation<'a, C> {
    /// The caller's context (the Bevy plugin passes its `World`).
    pub ctx: &'a mut C,
    pub args: &'a Args,
    pub vars: &'a mut Vars,
    /// Output; a returned `Err` is logged as an error line for you.
    pub log: &'a mut Log,
}

/// A command's code. `Err(message)` fails the call (and stops a `;` chain).
pub type Handler<C> = Box<dyn FnMut(Invocation<'_, C>) -> Result<(), String> + Send + Sync>;

/// What the fallback gets: a command whose first word no command or variable claims
/// ([`Console::set_fallback`]; a game's script language, as Bethesda consoles run script lines).
pub struct Unclaimed<'a, C> {
    pub ctx: &'a mut C,
    /// The command as typed: its first word to its `;` or the line end, quotes kept.
    pub text: &'a str,
    pub vars: &'a mut Vars,
    /// Output; a returned `Err` is logged as an error line, prefixed with the fallback's label.
    pub log: &'a mut Log,
}

/// The fallback's code.
pub type FallbackHandler<C> = Box<dyn FnMut(Unclaimed<'_, C>) -> Result<(), String> + Send + Sync>;
/// Completions the fallback adds for a line's first word (given in lowercase).
pub type FallbackCompleter = Box<dyn Fn(&str) -> Vec<String> + Send + Sync>;

struct Fallback<C> {
    label: String,
    run: FallbackHandler<C>,
    complete: Option<FallbackCompleter>,
}

/// Why a line didn't run. Also written to the log as an error line.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum ExecError {
    #[error("unterminated quote")]
    UnterminatedQuote,
    #[error("unknown {what} {name:?}{}", did_you_mean(.suggestions))]
    Unknown {
        what: String,
        name: String,
        suggestions: Vec<String>,
    },
    #[error("{command}: missing <{param}>; usage: {usage}")]
    MissingArg {
        command: String,
        param: String,
        usage: String,
    },
    #[error("{command}: too many arguments; usage: {usage}")]
    TooManyArgs { command: String, usage: String },
    #[error("{command}: <{param}> {message}")]
    BadArg {
        command: String,
        param: String,
        message: String,
    },
    #[error("{command}: {message}")]
    Failed { command: String, message: String },
}

fn did_you_mean(suggestions: &[String]) -> String {
    if suggestions.is_empty() {
        String::new()
    } else {
        format!("; did you mean {}?", suggestions.join(", "))
    }
}

/// Why a registration was refused (a programming error in the registering plugin).
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum RegisterError {
    #[error("invalid console name {0:?}: use a letter, then letters, digits, '_', '.' or '-'")]
    InvalidName(String),
    #[error("console name {0:?} is already taken")]
    Duplicate(String),
    #[error("variable {name:?}: {message}")]
    BadDefault { name: String, message: String },
    #[error("command {name:?}: {message}")]
    BadParams { name: String, message: String },
}

/// Completion of the word at the end of a line.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Completion {
    /// Byte offset where the completed word starts.
    pub start: usize,
    /// Matches, sorted.
    pub candidates: Vec<String>,
}

impl Completion {
    /// Longest prefix shared by every candidate.
    pub fn common_prefix(&self) -> &str {
        let Some(first) = self.candidates.first() else {
            return "";
        };
        let mut len = first.len();
        for c in &self.candidates[1..] {
            len = len.min(
                first
                    .bytes()
                    .zip(c.bytes())
                    .take_while(|(a, b)| a == b)
                    .count(),
            );
        }
        while !first.is_char_boundary(len) {
            len -= 1;
        }
        &first[..len]
    }

    /// The line after pressing Tab: a single match is inserted with a trailing space,
    /// several extend the word to their common prefix (unchanged if that adds nothing).
    pub fn apply(&self, line: &str) -> String {
        let head = &line[..self.start.min(line.len())];
        match self.candidates.as_slice() {
            [] => line.to_owned(),
            [only] => format!("{head}{} ", Value::Str(only.clone())),
            _ => {
                let common = self.common_prefix();
                let typed = &line[head.len()..];
                if common.len() > typed.len() {
                    format!("{head}{common}")
                } else {
                    line.to_owned()
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Target {
    Command(usize),
    Var(VarId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Builtin {
    Help,
    Set,
    Toggle,
    Reset,
    Vars,
    History,
    Echo,
    Clear,
}

enum Action<C> {
    Builtin(Builtin),
    User(Handler<C>),
}

struct Command<C> {
    spec: CommandSpec,
    action: Action<C>,
}

/// The console: registry, variables, history and log.
pub struct Console<C = ()> {
    commands: Vec<Command<C>>,
    /// Lowercase command names, aliases and variable names.
    names: HashMap<String, Target>,
    vars: Vars,
    history: History,
    log: Log,
    /// Who the line being run speaks for (variable priority).
    source: SetBy,
    /// Runs commands nobody claims (`None`: they are "unknown command or variable").
    fallback: Option<Fallback<C>>,
}

impl<C> Default for Console<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> Console<C> {
    /// A console with the built-in commands and default capacities.
    pub fn new() -> Self {
        Self::with_capacity(HISTORY_CAPACITY, LOG_CAPACITY)
    }

    pub fn with_capacity(history: usize, log: usize) -> Self {
        let mut console = Self {
            commands: Vec::new(),
            names: HashMap::new(),
            vars: Vars::default(),
            history: History::new(history),
            log: Log::new(log),
            source: SetBy::Console,
            fallback: None,
        };
        let var = |n| Param::new(n, ArgKind::Name(NameFilter::Vars));
        let builtins = [
            (
                CommandSpec::new("help")
                    .alias("?")
                    .help(
                        "List commands, describe one command or variable, or search names and help",
                    )
                    .param(Param::new("name", ArgKind::Name(NameFilter::Search)).optional()),
                Builtin::Help,
            ),
            (
                CommandSpec::new("set")
                    .help("Set a variable (same as `<variable> <value>`)")
                    .param(var("variable"))
                    .param(Param::new("value", ArgKind::Str)),
                Builtin::Set,
            ),
            (
                CommandSpec::new("toggle")
                    .help("Flip an on/off variable")
                    .param(Param::new("variable", ArgKind::Name(NameFilter::BoolVars))),
                Builtin::Toggle,
            ),
            (
                CommandSpec::new("reset")
                    .help("Reset a variable to its default, or every variable")
                    .param(var("variable").optional()),
                Builtin::Reset,
            ),
            (
                CommandSpec::new("vars")
                    .alias("list")
                    .help(
                        "List variables (changed ones marked *), optionally those containing text",
                    )
                    .param(Param::new("filter", ArgKind::Str).optional()),
                Builtin::Vars,
            ),
            (
                CommandSpec::new("history").help("List submitted lines"),
                Builtin::History,
            ),
            (
                CommandSpec::new("echo")
                    .help("Print text")
                    .param(Param::rest("text")),
                Builtin::Echo,
            ),
            (
                CommandSpec::new("clear")
                    .alias("cls")
                    .help("Clear the output"),
                Builtin::Clear,
            ),
        ];
        for (spec, b) in builtins {
            console
                .insert(spec, Action::Builtin(b))
                .expect("built-in names are valid and unique");
        }
        console
    }

    /// Registers a variable.
    pub fn add_var(&mut self, mut spec: VarSpec) -> Result<VarId, RegisterError> {
        spec.name = self.check_name(&spec.name)?;
        let name = spec.name.clone();
        let id = self
            .vars
            .register(spec)
            .map_err(|message| RegisterError::BadDefault {
                name: name.clone(),
                message,
            })?;
        self.names.insert(name, Target::Var(id));
        Ok(id)
    }

    /// Registers a command.
    pub fn add_command(
        &mut self,
        spec: CommandSpec,
        handler: impl FnMut(Invocation<'_, C>) -> Result<(), String> + Send + Sync + 'static,
    ) -> Result<(), RegisterError> {
        self.insert(spec, Action::User(Box::new(handler)))
    }

    fn insert(&mut self, mut spec: CommandSpec, action: Action<C>) -> Result<(), RegisterError> {
        let bad = |message: &str| RegisterError::BadParams {
            name: spec.name.clone(),
            message: message.to_owned(),
        };
        for (i, p) in spec.params.iter().enumerate() {
            if p.rest && i + 1 != spec.params.len() {
                return Err(bad("a rest parameter must be last"));
            }
            if !p.optional && spec.params[..i].iter().any(|q| q.optional) {
                return Err(bad("a required parameter follows an optional one"));
            }
            if spec.params[..i].iter().any(|q| q.name == p.name) {
                return Err(bad("two parameters share a name"));
            }
        }
        spec.name = self.check_name(&spec.name)?;
        for a in &mut spec.aliases {
            // `?` is the one non-word alias (built-in help).
            *a = if a == "?" {
                a.clone()
            } else {
                self.check_name(a)?
            };
        }
        let mut all: Vec<&String> = spec.aliases.iter().collect();
        all.push(&spec.name);
        if let Some(dup) = all
            .iter()
            .enumerate()
            .find(|(i, n)| all[..*i].contains(n))
            .map(|(_, n)| (*n).clone())
        {
            return Err(RegisterError::Duplicate(dup));
        }
        let i = self.commands.len();
        for n in all {
            self.names.insert(n.clone(), Target::Command(i));
        }
        self.commands.push(Command { spec, action });
        Ok(())
    }

    fn check_name(&self, name: &str) -> Result<String, RegisterError> {
        let lower = name.to_ascii_lowercase();
        let mut chars = lower.chars();
        let valid = chars.next().is_some_and(|c| c.is_ascii_lowercase())
            && chars.all(|c| {
                c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '_' | '.' | '-')
            });
        if !valid {
            return Err(RegisterError::InvalidName(name.to_owned()));
        }
        if self.names.contains_key(&lower) {
            return Err(RegisterError::Duplicate(lower));
        }
        Ok(lower)
    }

    pub fn vars(&self) -> &Vars {
        &self.vars
    }

    pub fn vars_mut(&mut self) -> &mut Vars {
        &mut self.vars
    }

    /// A variable's id by name (case-insensitive).
    pub fn var_id(&self, name: &str) -> Option<VarId> {
        match self.names.get(&name.to_ascii_lowercase()) {
            Some(Target::Var(id)) => Some(*id),
            _ => None,
        }
    }

    pub fn log(&self) -> &Log {
        &self.log
    }

    pub fn log_mut(&mut self) -> &mut Log {
        &mut self.log
    }

    pub fn history(&self) -> &History {
        &self.history
    }

    pub fn history_mut(&mut self) -> &mut History {
        &mut self.history
    }

    /// Registered commands (built-ins included), in registration order.
    pub fn commands(&self) -> impl Iterator<Item = &CommandSpec> {
        self.commands.iter().map(|c| &c.spec)
    }

    /// Runs every command whose first word is no command or variable (registered names always
    /// win), instead of failing it as unknown. `label` names it in errors (`label: message`).
    pub fn set_fallback(
        &mut self,
        label: &str,
        run: impl FnMut(Unclaimed<'_, C>) -> Result<(), String> + Send + Sync + 'static,
    ) {
        let complete = self.fallback.take().and_then(|f| f.complete);
        self.fallback = Some(Fallback {
            label: label.to_owned(),
            run: Box::new(run),
            complete,
        });
    }

    /// Extra first-word completions from the fallback (needs [`Console::set_fallback`] first).
    pub fn set_fallback_completion(
        &mut self,
        complete: impl Fn(&str) -> Vec<String> + Send + Sync + 'static,
    ) {
        if let Some(f) = self.fallback.as_mut() {
            f.complete = Some(Box::new(complete));
        }
    }

    /// A line typed by the user: recorded in the history, echoed, then run
    /// with [`SetBy::Console`] priority.
    pub fn submit(&mut self, ctx: &mut C, line: &str) -> Result<(), ExecError> {
        self.history.push(line);
        self.log.push(Level::Echo, line);
        self.exec(ctx, line)
    }

    /// Runs a line (from a user or a script) without recording it, with
    /// [`SetBy::Console`] priority. Commands run left to right; the first error is
    /// logged, stops the rest of the line, and is returned.
    pub fn exec(&mut self, ctx: &mut C, line: &str) -> Result<(), ExecError> {
        self.exec_as(ctx, line, SetBy::Console)
    }

    /// Runs a line on behalf of `source`: variables it sets keep a value from a
    /// higher-priority source (a config file line can't undo a command-line switch).
    pub fn exec_as(&mut self, ctx: &mut C, line: &str, source: SetBy) -> Result<(), ExecError> {
        self.source = source;
        let mut out = Ok(());
        for seg in split(line) {
            if let Err(e) = self.run(ctx, line, &seg) {
                self.log.error(&e.to_string());
                out = Err(e);
                break;
            }
        }
        self.source = SetBy::Console;
        out
    }

    fn run(&mut self, ctx: &mut C, line: &str, seg: &Segment) -> Result<(), ExecError> {
        let Some(first) = seg.tokens.first() else {
            return Ok(());
        };
        if seg.tokens.iter().any(|t| t.unterminated) {
            return Err(ExecError::UnterminatedQuote);
        }
        let name = first.text.to_ascii_lowercase();
        let target = match self.lookup(&name, NameFilter::Any) {
            Ok(t) => t,
            Err(unknown) => {
                let Some(fallback) = self.fallback.as_mut() else {
                    return Err(unknown);
                };
                let text = line[first.start..seg.end].trim_end();
                let unclaimed = Unclaimed {
                    ctx,
                    text,
                    vars: &mut self.vars,
                    log: &mut self.log,
                };
                return (fallback.run)(unclaimed).map_err(|message| ExecError::Failed {
                    command: fallback.label.clone(),
                    message,
                });
            }
        };
        match target {
            Target::Var(id) => match &seg.tokens[1..] {
                [] => {
                    self.log.info(&self.vars.var(id).describe());
                    Ok(())
                }
                [value] => self.set_var(&name, id, &value.text),
                _ => Err(ExecError::TooManyArgs {
                    command: name.clone(),
                    usage: format!("{name} [value:{}]", self.vars.var(id).kind().describe()),
                }),
            },
            Target::Command(i) => {
                let args = self.bind(i, line, seg)?;
                let command = &mut self.commands[i];
                match &mut command.action {
                    Action::User(handler) => {
                        let out = handler(Invocation {
                            ctx,
                            args: &args,
                            vars: &mut self.vars,
                            log: &mut self.log,
                        });
                        out.map_err(|message| ExecError::Failed {
                            command: command.spec.name.clone(),
                            message,
                        })
                    }
                    Action::Builtin(b) => {
                        let b = *b;
                        self.builtin(b, &args)
                    }
                }
            }
        }
    }

    /// Resolves a name of the given filter, with suggestions when it doesn't exist.
    fn lookup(&self, name: &str, filter: NameFilter) -> Result<Target, ExecError> {
        match self.names.get(name) {
            Some(&t) if self.passes(t, filter) => Ok(t),
            _ => Err(ExecError::Unknown {
                what: match filter {
                    NameFilter::Any | NameFilter::Search => "command or variable",
                    NameFilter::Vars => "variable",
                    NameFilter::BoolVars => "on/off variable",
                }
                .to_owned(),
                name: name.to_owned(),
                suggestions: self.suggest(name, filter),
            }),
        }
    }

    fn passes(&self, target: Target, filter: NameFilter) -> bool {
        match (filter, target) {
            (NameFilter::Any | NameFilter::Search, _) => true,
            (NameFilter::Vars, t) => matches!(t, Target::Var(_)),
            (NameFilter::BoolVars, Target::Var(id)) => self.vars.var(id).kind() == &ArgKind::Bool,
            (NameFilter::BoolVars, Target::Command(_)) => false,
        }
    }

    /// Up to three known names close to `name`: small edit distance (to the whole name or
    /// its last dotted part), or containing it.
    pub fn suggestions(&self, name: &str) -> Vec<String> {
        self.suggest(&name.to_ascii_lowercase(), NameFilter::Any)
    }

    fn suggest(&self, name: &str, filter: NameFilter) -> Vec<String> {
        let limit = if name.chars().count() <= 4 { 1 } else { 2 };
        let mut scored: Vec<(usize, &String)> = self
            .names
            .iter()
            .filter(|&(n, &t)| n != "?" && self.passes(t, filter))
            .filter_map(|(n, _)| {
                let last = n.rsplit('.').next().unwrap_or(n);
                let d = edit_distance(name, n).min(edit_distance(name, last));
                let contains = name.len() >= 3 && n.contains(name);
                (d <= limit || contains).then_some((if contains { d.min(limit) } else { d }, n))
            })
            .collect();
        scored.sort();
        scored.into_iter().take(3).map(|(_, n)| n.clone()).collect()
    }

    fn set_var(&mut self, name: &str, id: VarId, text: &str) -> Result<(), ExecError> {
        let outcome = self
            .vars
            .set_text_by(id, text, self.source)
            .map_err(|message| ExecError::BadArg {
                command: name.to_owned(),
                param: "value".into(),
                message,
            })?;
        if let SetOutcome::Outranked(by) = outcome {
            self.log.info(&format!(
                "{name} kept: set by {}, which outranks {}",
                by.describe(),
                self.source.describe()
            ));
        } else {
            self.log.info(&self.vars.var(id).describe());
        }
        Ok(())
    }

    fn bind(&self, i: usize, line: &str, seg: &Segment) -> Result<Args, ExecError> {
        let spec = &self.commands[i].spec;
        let given = &seg.tokens[1..];
        let mut args = Args::default();
        for (k, p) in spec.params.iter().enumerate() {
            let Some(token) = given.get(k) else {
                if p.optional {
                    break;
                }
                return Err(ExecError::MissingArg {
                    command: spec.name.clone(),
                    param: p.name.clone(),
                    usage: spec.usage(),
                });
            };
            if p.rest {
                let raw = line[token.start..seg.end].trim_end();
                args.values
                    .push((p.name.clone(), Value::Str(raw.to_owned())));
                return Ok(args);
            }
            let value = p
                .kind
                .parse(&token.text)
                .map_err(|message| ExecError::BadArg {
                    command: spec.name.clone(),
                    param: p.name.clone(),
                    message,
                })?;
            if let (ArgKind::Name(filter), Value::Str(n)) = (&p.kind, &value)
                && *filter != NameFilter::Search
            {
                self.lookup(n, *filter)?;
            }
            args.values.push((p.name.clone(), value));
        }
        if given.len() > spec.params.len() {
            return Err(ExecError::TooManyArgs {
                command: spec.name.clone(),
                usage: spec.usage(),
            });
        }
        Ok(args)
    }

    fn builtin(&mut self, b: Builtin, args: &Args) -> Result<(), ExecError> {
        // Names in `args` were checked by `bind`.
        let var = |this: &Self, p: &str| args.str(p).and_then(|n| this.var_id(n));
        match b {
            Builtin::Help => self.help(args.str("name")),
            Builtin::Set => {
                let name = args.str("variable").expect("required");
                let id = var(self, "variable").expect("checked");
                return self.set_var(name, id, args.str("value").expect("required"));
            }
            Builtin::Toggle => {
                let id = var(self, "variable").expect("checked");
                let on = !self.vars.flag(id);
                let name = self.vars.var(id).name().to_owned();
                return self.set_var(&name, id, if on { "on" } else { "off" });
            }
            Builtin::Reset => {
                if let Some(id) = var(self, "variable") {
                    self.vars.reset(id);
                    self.log.info(&self.vars.var(id).describe());
                } else {
                    let ids: Vec<VarId> = self.vars.iter().map(|(id, _)| id).collect();
                    let n = ids.into_iter().filter(|&id| self.vars.reset(id)).count();
                    self.log.info(&format!("reset {n} variable(s)"));
                }
            }
            Builtin::Vars => {
                let filter = args.str("filter").map(str::to_ascii_lowercase);
                let mut lines: Vec<String> = self
                    .vars
                    .iter()
                    .filter(|(_, v)| {
                        filter
                            .as_ref()
                            .is_none_or(|f| v.name().contains(f.as_str()))
                    })
                    .map(|(_, v)| {
                        let mark = if v.value() == v.default() { ' ' } else { '*' };
                        let by = match v.set_by() {
                            SetBy::Default => String::new(),
                            by => format!("  ({})", by.describe()),
                        };
                        format!("{mark} {} = {}{by}", v.name(), v.value())
                    })
                    .collect();
                lines.sort_by(|a, b| a[2..].cmp(&b[2..]));
                self.log.info(&format!("{} variable(s)", lines.len()));
                for l in lines {
                    self.log.info(&l);
                }
            }
            Builtin::History => {
                let lines: Vec<String> = self
                    .history
                    .entries()
                    .enumerate()
                    .map(|(i, e)| format!("{:>4}  {e}", i + 1))
                    .collect();
                for l in lines {
                    self.log.info(&l);
                }
            }
            Builtin::Echo => self.log.info(args.str("text").unwrap_or_default()),
            Builtin::Clear => self.log.clear(),
        }
        Ok(())
    }

    fn help(&mut self, name: Option<&str>) {
        let Some(name) = name else {
            let mut lines: Vec<String> = self
                .commands
                .iter()
                .map(|c| {
                    let aliases = if c.spec.aliases.is_empty() {
                        String::new()
                    } else {
                        format!(" ({})", c.spec.aliases.join(", "))
                    };
                    format!("{}{aliases}: {}", c.spec.name, c.spec.help)
                })
                .collect();
            lines.sort();
            for l in lines {
                self.log.info(&l);
            }
            self.log.info(&format!(
                "{} variable(s): `vars` lists them, `help <name>` describes one",
                self.vars.len()
            ));
            return;
        };
        let Some(&target) = self.names.get(name) else {
            self.search(name);
            return;
        };
        let text = match target {
            Target::Var(id) => self.vars.var(id).describe(),
            Target::Command(i) => {
                let spec = &self.commands[i].spec;
                let mut s = format!("usage: {}", spec.usage());
                if !spec.help.is_empty() {
                    s += "\n";
                    s += &spec.help;
                }
                if !spec.aliases.is_empty() {
                    let _ = write!(s, "\naliases: {}", spec.aliases.join(", "));
                }
                s
            }
        };
        self.log.info(&text);
    }

    /// `help <text>` for a text that isn't a name: commands and variables whose name or
    /// help contains it (Creation Kit console `help <text>`, Source `find`).
    fn search(&mut self, text: &str) {
        let text = text.to_ascii_lowercase();
        let mut hits: Vec<String> = self
            .commands
            .iter()
            .filter(|c| {
                c.spec.name.contains(&text) || c.spec.help.to_ascii_lowercase().contains(&text)
            })
            .map(|c| format!("{}: {}", c.spec.usage(), c.spec.help))
            .chain(
                self.vars
                    .iter()
                    .filter(|(_, v)| {
                        v.name().contains(&text) || v.help().to_ascii_lowercase().contains(&text)
                    })
                    .map(|(_, v)| v.describe()),
            )
            .collect();
        hits.sort();
        if hits.is_empty() {
            let suggestions = self.suggest(&text, NameFilter::Any);
            self.log.info(&format!(
                "nothing matches {text:?}{}",
                did_you_mean(&suggestions)
            ));
        }
        for h in hits {
            self.log.info(&h);
        }
    }

    /// Completes the word at the end of `line` (the cursor is at the end): command and
    /// variable names first, then each parameter by its kind (names, on/off, choices; a
    /// variable's value after `set <variable>` or a bare variable name).
    pub fn complete(&self, line: &str) -> Completion {
        let seg = split(line)
            .pop()
            .expect("split never returns an empty list");
        let tokens = &seg.tokens;
        let at_new_word = match tokens.last() {
            None => true,
            Some(t) => !t.unterminated && t.end < line.len(),
        };
        let (index, prefix, start) = if at_new_word {
            (tokens.len(), String::new(), line.len())
        } else {
            let t = tokens.last().expect("not at a new word");
            (tokens.len() - 1, t.text.to_ascii_lowercase(), t.start)
        };
        let names = |filter: NameFilter| -> Vec<String> {
            let mut out: Vec<String> = self
                .names
                .iter()
                .filter(|&(n, &t)| n != "?" && self.passes(t, filter))
                .filter(|&(n, &t)| match t {
                    // Complete primary names only; aliases still run.
                    Target::Command(i) => &self.commands[i].spec.name == n,
                    Target::Var(_) => true,
                })
                .map(|(n, _)| n.clone())
                .collect();
            out.sort();
            out
        };
        let kind_candidates = |kind: &ArgKind| match kind {
            ArgKind::Name(filter) => names(*filter),
            k => k.value_candidates(),
        };
        let all = if index == 0 {
            let mut all = names(NameFilter::Any);
            if let Some(complete) = self.fallback.as_ref().and_then(|f| f.complete.as_ref()) {
                all.extend(complete(&prefix));
            }
            all
        } else {
            let head = tokens[0].text.to_ascii_lowercase();
            match self.names.get(&head) {
                Some(&Target::Var(id)) if index == 1 => self.vars.var(id).kind().value_candidates(),
                Some(&Target::Command(i)) => {
                    let command = &self.commands[i];
                    let set_value =
                        matches!(command.action, Action::Builtin(Builtin::Set)) && index == 2;
                    if set_value {
                        self.var_id(&tokens[1].text)
                            .map(|id| self.vars.var(id).kind().value_candidates())
                            .unwrap_or_default()
                    } else {
                        // A rest parameter (always last) is free text.
                        match command.spec.params.get(index - 1) {
                            Some(p) if !p.rest => kind_candidates(&p.kind),
                            _ => Vec::new(),
                        }
                    }
                }
                _ => Vec::new(),
            }
        };
        let candidates = all.into_iter().filter(|c| c.starts_with(&prefix)).collect();
        Completion { start, candidates }
    }
}

/// Levenshtein distance over chars.
fn edit_distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut diag = row[0];
        row[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let next = (diag + usize::from(ca != cb))
                .min(row[j] + 1)
                .min(row[j + 1] + 1);
            diag = row[j + 1];
            row[j + 1] = next;
        }
    }
    row[b.len()]
}
