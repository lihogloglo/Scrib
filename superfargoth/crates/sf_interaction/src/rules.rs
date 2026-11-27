//! The rules layer (interaction-world-rules.md "Rules layer", IN17b): one table of
//! [`InteractionRule`] records (source tags x target material + states -> state changes, damage,
//! stage, effect, world change, noise), evaluated host-side by the pure [`step_rules`].
//!
//! - **Causes** ([`Cause`]): the tick's hits, effect areas, heat, rain and water contact the shell
//!   gathered, each one target placement, the target's material and the cause's source tags. They wait
//!   in a [`RuleQueue`] in a canonical order: the tick they were queued, then placement, material and
//!   (sorted) source tags, so the arrival order of a tick's causes never changes the result.
//! - **Matching:** a rule matches when its `source` filter passes the cause's tags and its `target`
//!   filter passes the target's material plus its current state tags. The first match in record order
//!   applies (mods reorder by patching); a cause matches at most one rule.
//! - **Results** apply in a fixed order: `remove`, then `add` (water on `Burning`: remove `Burning`, add
//!   `Wet`), then the damage, stage, effect, world change and noise outputs. State events report only
//!   real changes (a refresh is silent).
//! - **Budget:** at most `interact.rules.evals_per_tick` causes are evaluated per tick (a cause that
//!   matches nothing still costs its scan); the rest wait, oldest first.
//! - TTLs expire at the start of each step ([`ObjectStates::expire`]), before the tick's causes, so a
//!   rule sees the states that are current that tick.
//!
//! [`MaterialDef`] and [`Flammable`] are the material records fire reads (`IN17c`); `step_rules` reads
//! only the cause's material tag.

use sf_records::RecordId;
use sf_world::PlacementId;
use smallvec::SmallVec;

use crate::claims::Tick;
use crate::def::{TagFilter, TagId};
use crate::params::InteractParams;
use crate::states::{ObjectStates, Ttl};

/// One rule record (`sf.interact.rule`, mod-patchable): which causes on which targets give what.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InteractionRule {
    pub id: RecordId,
    /// The cause's tags: element, tool, projectile, impact, spell.
    #[serde(default)]
    pub source: TagFilter,
    /// The target's material plus its required (`all`/`any`) and forbidden (`none`) states.
    #[serde(default)]
    pub target: TagFilter,
    #[serde(default)]
    pub result: RuleResult,
}

/// What a matched rule does to its target.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RuleResult {
    /// States added (or refreshed) with their time to live.
    pub add: SmallVec<[(TagId, Ttl); 2]>,
    /// States removed, before `add`.
    pub remove: SmallVec<[TagId; 2]>,
    /// Damage kind and amount per match, for the host's destruction rules.
    pub damage: Option<(TagId, f32)>,
    /// A destruction stage forced on the target (`Charred`).
    pub stage: Option<u8>,
    /// An `sf_effects` id, seeded per client.
    pub effect: Option<TagId>,
    /// A world change kind (world-reactions.md, `WorldChange` vocabulary).
    pub change: Option<TagId>,
    /// A noise kind (interaction-world-rules.md "Noise").
    pub noise: Option<TagId>,
}

/// A material record (`sf.interact.material`): wood, cloth, straw, glass, stone, grass.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaterialDef {
    pub id: TagId,
    #[serde(default)]
    pub flammable: Option<Flammable>,
    /// Noise kind of an impact on this material.
    pub noise: TagId,
    /// Noise reach of an impact at `interact.noise.ref_speed_mps`, m.
    pub impact_reach_m: f32,
}

/// How a material burns (fire, `IN17c`).
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Flammable {
    /// Heat at which it ignites, ratio.
    pub ignite_heat: f32,
    /// How long it burns before it is spent, s.
    pub fuel_s: f32,
    /// The destruction stage it ends in once spent.
    #[serde(default)]
    pub burnt_stage: Option<u8>,
    /// Whether it passes fire on to its neighbours.
    pub spread: bool,
}

/// The rules table, in record order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Rules {
    pub rules: Vec<InteractionRule>,
}

impl Rules {
    #[must_use]
    pub fn new(rules: Vec<InteractionRule>) -> Self {
        Self { rules }
    }

    /// The first rule (record order) matching `source` tags on a target with `target` tags.
    #[must_use]
    pub fn first_match(&self, source: &[TagId], target: &[TagId]) -> Option<&InteractionRule> {
        self.rules.iter().find(|r| r.source.matches(source) && r.target.matches(target))
    }
}

/// One cause on one target this tick.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cause {
    pub placement: PlacementId,
    /// The target's material (its def's), matched with its states by a rule's `target`.
    pub material: Option<TagId>,
    /// The cause's tags, sorted and deduplicated by [`RuleQueue::push`].
    pub source: SmallVec<[TagId; 4]>,
}

/// Causes waiting for evaluation, oldest first.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuleQueue {
    /// `(queued tick, cause)`; sorted at each step.
    pending: Vec<(Tick, Cause)>,
}

impl RuleQueue {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues `cause`, gathered at tick `now`.
    pub fn push(&mut self, now: Tick, mut cause: Cause) {
        cause.source.sort_unstable();
        cause.source.dedup();
        self.pending.push((now, cause));
    }

    /// Causes still waiting.
    #[must_use]
    pub fn len(&self) -> usize {
        self.pending.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

/// What a step did to one placement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RuleOutput {
    /// A state the placement did not have.
    Added(TagId),
    /// A state a rule removed.
    Removed(TagId),
    /// A timed state that ran out.
    Expired(TagId),
    /// Damage for the host's destruction rules.
    Damage { kind: TagId, amount: f32 },
    /// A forced destruction stage.
    Stage(u8),
    /// An effect to play (seeded per client).
    Effect(TagId),
    /// A world change kind.
    Change(TagId),
    /// A noise kind.
    Noise(TagId),
}

/// One output of [`step_rules`]: the placement, the rule that caused it (`None` for an expiry) and
/// what happened.
#[derive(Clone, Debug, PartialEq)]
pub struct RuleEvent {
    pub placement: PlacementId,
    pub rule: Option<RecordId>,
    pub output: RuleOutput,
}

/// One rules tick at `now`: expires the states whose time has come, then evaluates up to
/// `interact.rules.evals_per_tick` queued causes, oldest first, applying each one's first matching
/// rule. Returns the tick's events in order (expiries first, then per cause in queue order).
pub fn step_rules(
    states: &mut ObjectStates,
    queue: &mut RuleQueue,
    rules: &Rules,
    now: Tick,
    params: &InteractParams,
) -> Vec<RuleEvent> {
    let mut out: Vec<RuleEvent> = states
        .expire(now)
        .into_iter()
        .map(|(placement, tag)| RuleEvent { placement, rule: None, output: RuleOutput::Expired(tag) })
        .collect();

    // Stable sort: equal keys are identical causes, so their order cannot matter.
    queue.pending.sort();
    let n = queue.pending.len().min(usize::from(params.rules_evals_per_tick));
    for (_, cause) in queue.pending.drain(..n) {
        let mut target = states.tags(cause.placement);
        if let Some(m) = cause.material {
            target.push(m);
        }
        let Some(rule) = rules.first_match(&cause.source, &target) else {
            continue;
        };
        let p = cause.placement;
        let mut emit = |output| out.push(RuleEvent { placement: p, rule: Some(rule.id.clone()), output });
        let r = &rule.result;
        for &tag in &r.remove {
            if states.remove(p, tag) {
                emit(RuleOutput::Removed(tag));
            }
        }
        for &(tag, ttl) in &r.add {
            if states.add(p, tag, ttl, now, params) {
                emit(RuleOutput::Added(tag));
            }
        }
        if let Some((kind, amount)) = r.damage {
            emit(RuleOutput::Damage { kind, amount });
        }
        if let Some(s) = r.stage {
            emit(RuleOutput::Stage(s));
        }
        if let Some(e) = r.effect {
            emit(RuleOutput::Effect(e));
        }
        if let Some(c) = r.change {
            emit(RuleOutput::Change(c));
        }
        if let Some(n) = r.noise {
            emit(RuleOutput::Noise(n));
        }
    }
    out
}
