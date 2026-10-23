//! The game's result runner (rpg-adapter.md "As built (W3)"): INFO result scripts run in
//! `mw_script`'s native interpreter as the speaker, reading `sf_rpg` state (quests, the
//! context player's inventory) and, through [`MwWorld`], what `mw_rpg` cannot own (globals,
//! script variables, items held by world actors). A run is side-effect free: it returns
//! [`ResultEffect`]s in call order and the host applies them on its tick (net rules 2, 5).
//! Inside one run, reads see the run's own earlier writes (an overlay), as in the original
//! engine where every call takes effect at once.

use std::collections::{HashMap, HashSet};
use std::fmt;

use mw_script::ast::{Call, Ident, Target as AstTarget, VarRef, VarType};
use mw_script::functions::Function;
use mw_script::interp::{Host, Interpreter, Locals, store};
use mw_script::{Env, FunctionTable, MemberLookup, Parsed, SourceKind, parse_and_resolve};
use sf_records::RecordId;
use sf_rpg::{CharacterRpg, Choice, ConversationCtx, WorldRpg};

use crate::result::{ResultCall, ResultEffect, ResultError, ResultRunner, Target};
use crate::text::GlobalNumber;

/// A script variable and its declared type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScriptVar {
    pub ty: VarType,
    pub value: f32,
}

/// World-side reads the runner needs and `mw_rpg` does not own. The host implements it
/// over its world (ECS, placement overlay); tests use doubles.
pub trait MwWorld {
    /// A global (GLOB); `None`: no such global (reads 0, writes are dropped).
    fn global(&self, name: &RecordId) -> Option<GlobalNumber>;
    /// A script variable of `target` (the speaker's locals, `id.var`, `id->` objects);
    /// `None`: no script or no such variable (reads 0, writes are dropped).
    fn var(&self, ctx: &ConversationCtx, target: &Target, name: &RecordId) -> Option<ScriptVar>;
    /// Items held by `target` when it is not the context player (the speaker, `id->`
    /// actors and containers); the player's come from its `CharacterRpg`.
    fn item_count(&self, ctx: &ConversationCtx, target: &Target, item: &RecordId) -> u64;
}

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
        let mut host = RunHost {
            ctx: call.ctx,
            state: self.state,
            reports,
            effects: Vec::new(),
            stages: HashMap::new(),
            items: HashMap::new(),
            vars: HashMap::new(),
            globals: HashMap::new(),
        };
        let mut locals = Locals::new(&parsed.script);
        interp
            .run(&parsed.script, &mut locals, &mut host)
            .map_err(|e| ResultError::Failed {
                response: call.response.clone(),
                message: e.to_string(),
            })?;
        Ok(host.effects)
    }
}

/// Name resolution against the world: a bare name is a global if the world has one, else
/// a speaker local. Integer globals resolve as `long` (`GlobalNumber` keeps no width).
struct WorldEnv<'a>(&'a dyn MwWorld);

impl Env for WorldEnv<'_> {
    fn global(&self, key: &str) -> Option<VarType> {
        self.0.global(&RecordId::new(key)).map(|g| match g {
            GlobalNumber::Int(_) => VarType::Long,
            GlobalNumber::Float(_) => VarType::Float,
        })
    }
    fn object_exists(&self, _: &str) -> Option<bool> {
        None
    }
    fn member(&self, _: &str, _: &str) -> MemberLookup {
        MemberLookup::Unknown
    }
}

fn rid(id: &Ident) -> RecordId {
    RecordId::new(&id.key)
}

fn target(t: &AstTarget) -> Target {
    match t {
        AstTarget::Implicit => Target::Speaker,
        AstTarget::Explicit(id) => object(id),
    }
}

fn object(id: &Ident) -> Target {
    if id.key == "player" {
        Target::Player
    } else {
        Target::Actor(rid(id))
    }
}

#[allow(clippy::cast_possible_truncation)]
fn to_f32(v: f64) -> f32 {
    v as f32
}

/// One run: reads with the overlay of this run's writes, effects in call order.
struct RunHost<'a> {
    ctx: &'a ConversationCtx,
    state: ResultState<'a>,
    reports: &'a mut Reports,
    effects: Vec<ResultEffect>,
    stages: HashMap<RecordId, i32>,
    items: HashMap<(Target, RecordId), i64>,
    vars: HashMap<(Target, RecordId), f32>,
    globals: HashMap<RecordId, f32>,
}

impl RunHost<'_> {
    /// The quest stage this run sees.
    fn get_stage(&self, quest: &RecordId) -> i32 {
        self.stages
            .get(quest)
            .copied()
            .unwrap_or_else(|| self.state.world.quests.stage(quest))
    }

    /// Where a variable lives: `(None, name)` a global, `(Some(target), name)` a script
    /// variable of `target`; `None` for what the interpreter keeps itself.
    fn owner(var: &VarRef) -> Option<(Option<Target>, &Ident)> {
        match var {
            VarRef::Global(name) => Some((None, name)),
            VarRef::Speaker(name) | VarRef::Name(name) => Some((Some(Target::Speaker), name)),
            VarRef::Member { object: o, var } => Some((Some(object(o)), var)),
            VarRef::Local(_) | VarRef::Undeclared(_) => None,
        }
    }

    fn unknown(&mut self, owner: Option<&Target>, name: &Ident) {
        let what = match owner {
            None => format!("global {}", name.key),
            Some(Target::Speaker) => format!("speaker.{}", name.key),
            Some(Target::Player) => format!("player.{}", name.key),
            Some(Target::Actor(a)) => format!("{a}.{}", name.key),
        };
        self.reports.push(ResultReport::UnknownVariable(what));
    }

    fn count(&self, target: &Target, item: &RecordId) -> i64 {
        let base = match target {
            Target::Player => self.state.player.loadout.count(item),
            other => self.state.env.item_count(self.ctx, other, item),
        };
        let delta = self
            .items
            .get(&(target.clone(), item.clone()))
            .copied()
            .unwrap_or(0);
        i64::try_from(base)
            .unwrap_or(i64::MAX)
            .saturating_add(delta)
    }
}

impl Host for RunHost<'_> {
    fn get(&mut self, var: &VarRef) -> f64 {
        let Some((owner, name)) = Self::owner(var) else {
            return 0.0;
        };
        let key = rid(name);
        let value = match &owner {
            None => self
                .globals
                .get(&key)
                .copied()
                .or_else(|| self.state.env.global(&key).map(GlobalNumber::as_f32)),
            Some(t) => self
                .vars
                .get(&(t.clone(), key.clone()))
                .copied()
                .or_else(|| self.state.env.var(self.ctx, t, &key).map(|v| v.value)),
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
        let Some((owner, name)) = Self::owner(var) else {
            return;
        };
        let key = rid(name);
        let ty = match &owner {
            None => self.state.env.global(&key).map(|g| match g {
                GlobalNumber::Int(_) => VarType::Long,
                GlobalNumber::Float(_) => VarType::Float,
            }),
            Some(t) => self.state.env.var(self.ctx, t, &key).map(|v| v.ty),
        };
        let Some(ty) = ty else {
            self.unknown(owner.as_ref(), name);
            return;
        };
        let value = to_f32(store(ty, value));
        match owner {
            None => {
                self.globals.insert(key.clone(), value);
                self.effects
                    .push(ResultEffect::SetGlobal { name: key, value });
            }
            Some(target) => {
                self.vars.insert((target.clone(), key.clone()), value);
                self.effects.push(ResultEffect::SetLocal {
                    target,
                    name: key,
                    value,
                });
            }
        }
    }

    fn journal(&mut self, quest: &Ident, index: i32) {
        let quest = rid(quest);
        let current = self.get_stage(&quest);
        self.stages.insert(quest.clone(), current.max(index));
        self.effects.push(ResultEffect::Journal { quest, index });
    }

    fn set_journal_index(&mut self, quest: &Ident, index: i32) {
        let quest = rid(quest);
        self.stages.insert(quest.clone(), index);
        self.effects
            .push(ResultEffect::SetJournalIndex { quest, index });
    }

    fn get_journal_index(&mut self, quest: &Ident) -> i32 {
        self.get_stage(&rid(quest))
    }

    fn add_topic(&mut self, topic: &Ident) {
        self.effects.push(ResultEffect::AddTopic(rid(topic)));
    }

    fn choice(&mut self, choices: Vec<(String, i32)>) {
        self.effects.push(ResultEffect::Choices(
            choices
                .into_iter()
                .map(|(label, value)| Choice { label, value })
                .collect(),
        ));
    }

    fn goodbye(&mut self) {
        self.effects.push(ResultEffect::Goodbye);
    }

    fn add_item(&mut self, target: &AstTarget, item: &Ident, count: i32) {
        let Ok(count @ 1..) = u32::try_from(count) else {
            return;
        };
        if item.key.is_empty() {
            return;
        }
        let (target, item) = (self::target(target), rid(item));
        *self
            .items
            .entry((target.clone(), item.clone()))
            .or_default() += i64::from(count);
        self.effects.push(ResultEffect::AddItem {
            target,
            item,
            count,
        });
    }

    /// Removes up to `count`: fewer when the holder has fewer (verify against the oracle).
    fn remove_item(&mut self, target: &AstTarget, item: &Ident, count: i32) {
        let (target, item) = (self::target(target), rid(item));
        let held = self.count(&target, &item);
        let Ok(count @ 1..) = u32::try_from(i64::from(count).min(held)) else {
            return;
        };
        *self
            .items
            .entry((target.clone(), item.clone()))
            .or_default() -= i64::from(count);
        self.effects.push(ResultEffect::RemoveItem {
            target,
            item,
            count,
        });
    }

    fn get_item_count(&mut self, target: &AstTarget, item: &Ident) -> i32 {
        let n = self.count(&self::target(target), &rid(item));
        i32::try_from(n.max(0)).unwrap_or(i32::MAX)
    }

    fn set_disposition(&mut self, target: &AstTarget, value: i32) {
        self.effects.push(ResultEffect::SetDisposition {
            target: self::target(target),
            value,
        });
    }

    fn mod_disposition(&mut self, target: &AstTarget, delta: i32) {
        self.effects.push(ResultEffect::ModDisposition {
            target: self::target(target),
            delta,
        });
    }

    fn message_box(&mut self, text: String, buttons: Vec<String>) {
        self.effects
            .push(ResultEffect::MessageBox { text, buttons });
    }

    fn play_sound(&mut self, sound: &Ident) {
        self.effects.push(ResultEffect::PlaySound(rid(sound)));
    }

    fn unhandled(&mut self, function: &Function, _call: &Call) -> f64 {
        self.reports
            .push(ResultReport::Unhandled(function.name.clone()));
        f64::from(function.status.neutral())
    }
}
