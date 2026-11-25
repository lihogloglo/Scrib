//! The game's result runner (rpg-adapter.md "As built (W3)"): INFO result scripts run in
//! `mw_script`'s native interpreter as the speaker. The results host ([`RunHost`]) only binds
//! the interpreter's calls onto the game API ([`crate::api`]): reads through a [`GameView`] of
//! `sf_rpg` state and the host's [`MwWorld`], the run's own writes seen by its later reads, and
//! [`ResultEffect`]s ([`GameOp`](crate::api::GameOp)s) in call order that the host applies on
//! its tick (net rules 2, 5).

use std::collections::{HashMap, HashSet};
use std::fmt;

use mw_script::ast::{Call, Ident, Target as AstTarget, VarRef, VarType};
use mw_script::functions::Function;
use mw_script::interp::{Host, Interpreter, Locals, MessageArg};
use mw_script::{
    Env, FunctionTable, MemberLookup, Parsed, SourceKind, StatCall, parse_and_resolve,
};
use mw_ui::message::MessageArg as UiArg;
use sf_records::RecordId;
use sf_rpg::{CharacterRpg, Choice, WorldRpg};

use crate::api::{self, GameRun, GameView, MwWorld, Target};
use crate::result::{ResultCall, ResultEffect, ResultError, ResultRunner};

/// What the runner reads: `sf_rpg` world state, the context player's character, the world.
#[derive(Clone, Copy)]
pub struct ResultState<'a> {
    pub world: &'a WorldRpg,
    /// The character of `ctx.player` (the talking player).
    pub player: &'a CharacterRpg,
    pub env: &'a dyn MwWorld,
}

/// Something the host should log. [`MwResults`] reports each one once per session.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResultReport {
    /// A function the interpreter does not run yet: the call read its neutral value.
    Unhandled(String),
    /// A result that did not compile (an unrecoverable diagnostic): it never runs.
    Inert { response: RecordId, reason: String },
    /// A variable nobody declares (`"global x"`, `"speaker.x"`, `"id.x"`): read 0, write dropped.
    UnknownVariable(String),
}

impl fmt::Display for ResultReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unhandled(name) => write!(f, "result scripts: {name} not implemented yet"),
            Self::Inert { response, reason } => {
                write!(f, "result of {response} does not compile ({reason})")
            }
            Self::UnknownVariable(name) => write!(f, "result scripts: unknown variable {name}"),
        }
    }
}

#[derive(Debug, Default)]
struct Reports {
    seen: HashSet<ResultReport>,
    fresh: Vec<ResultReport>,
}

impl Reports {
    fn push(&mut self, report: ResultReport) {
        if self.seen.insert(report.clone()) {
            self.fresh.push(report);
        }
    }
}

/// The game's result runtime: parsed results (cached per response, re-parsed when the
/// text changes) and the report-once log. Lives as long as the session (or the game).
#[derive(Debug)]
pub struct MwResults {
    interp: Interpreter<'static>,
    cache: HashMap<RecordId, (String, Parsed)>,
    reports: Reports,
}

impl Default for MwResults {
    fn default() -> Self {
        Self::new()
    }
}

impl MwResults {
    pub fn new() -> Self {
        Self {
            interp: Interpreter::new(FunctionTable::vanilla()),
            cache: HashMap::new(),
            reports: Reports::default(),
        }
    }

    /// Statements one result may execute (a `While` that never ends fails the run).
    pub fn set_step_limit(&mut self, limit: u32) {
        self.interp.step_limit = limit;
    }

    /// A [`ResultRunner`] over `state` for this step.
    pub fn runner<'a>(&'a mut self, state: ResultState<'a>) -> MwResultRunner<'a> {
        MwResultRunner {
            results: self,
            state,
        }
    }

    /// Reports since the last call, each at most once per [`MwResults`]; the host logs them.
    pub fn drain_reports(&mut self) -> Vec<ResultReport> {
        std::mem::take(&mut self.reports.fresh)
    }
}

/// [`MwResults`] bound to one step's state.
pub struct MwResultRunner<'a> {
    results: &'a mut MwResults,
    state: ResultState<'a>,
}

impl ResultRunner for MwResultRunner<'_> {
    fn run(&mut self, call: ResultCall<'_>) -> Result<Vec<ResultEffect>, ResultError> {
        let MwResults {
            interp,
            cache,
            reports,
        } = &mut *self.results;
        let fresh = cache
            .get(call.response)
            .is_none_or(|(text, _)| text != call.script);
        if fresh {
            let parsed = parse_and_resolve(
                call.script,
                SourceKind::DialogueResult,
                interp.table(),
                &WorldEnv(self.state.env),
            );
            cache.insert(call.response.clone(), (call.script.to_owned(), parsed));
        }
        let parsed = &cache[call.response].1;
        if parsed.is_inert() {
            let reason = parsed
                .diagnostics
                .iter()
                .find(|d| d.severity() == mw_script::Severity::Error)
                .map(ToString::to_string)
                .unwrap_or_default();
            reports.push(ResultReport::Inert {
                response: call.response.clone(),
                reason,
            });
            return Ok(Vec::new());
        }
        let view = GameView {
            ctx: call.ctx,
            world: self.state.world,
            player: self.state.player,
            env: self.state.env,
        };
        let mut host = RunHost {
            run: GameRun::new(view),
            reports,
        };
        let mut locals = Locals::new(&parsed.script);
        interp
            .run(&parsed.script, &mut locals, &mut host)
            .map_err(|e| ResultError::Failed {
                response: call.response.clone(),
                message: e.to_string(),
            })?;
        Ok(host.run.into_ops())
    }
}

/// Name resolution against the world: a bare name is a global if the world has one, else
/// a speaker local (a script local in a script record). Integer globals resolve as `long`
/// (`GlobalNumber` keeps no width). The game parses its script records with it too.
pub struct WorldEnv<'a>(pub &'a dyn MwWorld);

impl Env for WorldEnv<'_> {
    fn global(&self, key: &str) -> Option<VarType> {
        self.0.global(&RecordId::new(key)).map(api::global_type)
    }
    fn object_exists(&self, _: &str) -> Option<bool> {
        None
    }
    fn member(&self, _: &str, _: &str) -> MemberLookup {
        MemberLookup::Unknown
    }
}

/// One run: each interpreter call converts its arguments and calls the game API.
struct RunHost<'a> {
    run: GameRun<'a>,
    reports: &'a mut Reports,
}

impl RunHost<'_> {
    fn unknown(&mut self, owner: Option<&Target>, name: &Ident) {
        self.reports
            .push(ResultReport::UnknownVariable(api::unknown_name(
                owner, name,
            )));
    }
}

impl Host for RunHost<'_> {
    fn get(&mut self, var: &VarRef) -> f64 {
        let Some((owner, name)) = api::owner(var) else {
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
        let Some((owner, name)) = api::owner(var) else {
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

    fn choice(&mut self, choices: Vec<(String, i32)>) {
        self.run.choice(
            choices
                .into_iter()
                .map(|(label, value)| Choice { label, value })
                .collect(),
        );
    }

    fn goodbye(&mut self) {
        self.run.goodbye();
    }

    fn add_item(&mut self, target: &AstTarget, item: &Ident, count: i32) {
        self.run
            .add_item(api::target(target), api::rid(item), count);
    }

    /// Removes up to `count`: fewer when the holder has fewer (verify against the oracle).
    fn remove_item(&mut self, target: &AstTarget, item: &Ident, count: i32) {
        self.run
            .remove_item(api::target(target), api::rid(item), count);
    }

    fn get_item_count(&mut self, target: &AstTarget, item: &Ident) -> i32 {
        let n = self.run.item_count(&api::target(target), &api::rid(item));
        i32::try_from(n.max(0)).unwrap_or(i32::MAX)
    }

    fn set_disposition(&mut self, target: &AstTarget, value: i32) {
        self.run.set_disposition(api::target(target), value);
    }

    fn mod_disposition(&mut self, target: &AstTarget, delta: i32) {
        self.run.mod_disposition(api::target(target), delta);
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
        Some(self.run.stat(&api::target(target), call))
    }

    fn unhandled(&mut self, function: &Function, _call: &Call) -> f64 {
        self.reports
            .push(ResultReport::Unhandled(function.name.clone()));
        f64::from(function.status.neutral())
    }
}
