//! AI settings considerations (`docs/specs/ai-adapter.md` "AI settings (stats + considerations)"):
//! Morrowind's Fight / aggression rating (`MA4d`), Flee rating (`MA4f`) and Hello greeting (`MA4c1`).
//!
//! Ported from `OpenMW` (GPL-3.0, fine in this repo) at commit
//! `5f3aafb010a26b789b6a5e590b4e218d1ddf0cd9`:
//! - [`fight_rating`] is `getFightTerm` (`apps/openmw/mwmechanics/combat.cpp` 556-565): modified Fight
//!   `+ (int)(getFightDistanceBias + getFightDispositionBias)`, the float sum truncated toward zero.
//!   `getFightDispositionBias` (567-575) is `(50 - disposition) * fFightDispMult`, with disposition the
//!   observer's derived disposition for an NPC and 50 for a creature. `getFightDistanceBias` (577-596) is
//!   `iFightDistanceBase - fFightDistanceMultiplier * d`, `d` from `getAggroDistance` (598-603): the
//!   distance in game units, ignoring height when the observer swims or flies ([`aggro_distance_units`]).
//! - [`is_aggressive`] is `isAggressive` (623-647): already in combat with the target wins; an observer
//!   that is not aggression capable (`isAggressionCapable` 605-621: immobile, or under Calm) never is;
//!   else an NPC eyeing an NPC target that is a werewolf (or the player while `PCKnownWerewolf` is set)
//!   adds `iWerewolfFightMod`; aggressive at a rating of 100 or more.
//! - [`engages_on_sight`] is the aggression branch of `Actors::engageCombat`
//!   (`apps/openmw/mwmechanics/actors.cpp` 637-822): skip a dead pair or one already fighting, a
//!   magically hidden or unreachable target (`isTargetMagicallyHidden`, `canFight` in
//!   `aicombataction.cpp` 378); only a target that is the player or on the player's side is rated, never
//!   by the player's own followers; combat starts when the observer sees the target (`getLOS` and
//!   `awarenessCheck`, 815-820). The ally-siding and guard branches of `engageCombat`, and its
//!   actors-processing-range cut, are the caller's.
//!
//! - [`flee_rating`] is `vanillaRateFlee` (`apps/openmw/mwmechanics/aicombataction.cpp` 457-488): a
//!   modified Flee of 100 or more is the rating (no distance bias); else
//!   `(1 - health ratio) * fAIFleeHealthMult + Flee * fAIFleeFleeMult`, replaced by `iWereWolfFleeMod`
//!   for an NPC below `iWereWolfLevelToAttack` facing an NPC werewolf; a non-zero rating adds
//!   `getFightDistanceBias`. The health ratio is `DynamicStat::getRatio(false)` (`stat.cpp` 94-104).
//! - [`wants_to_flee`] is `makeFleeDecision` (`aicombataction.cpp` 490-503): a rating under 100 counts
//!   as 0; flee when it beats the best action's anti-flee rating, or with nothing but fists (anti-flee
//!   0) and a summoned creature out. `prepareNextAction` (162-249) never lets an NPC werewolf flee and
//!   gives potions and enchanted items an infinite anti-flee rating.
//! - [`flee_step`] is the flee half of `AiCombat::execute` and `AiCombat::attack` (`aicombat.cpp`
//!   139-210, 212-280, 309-353): an immobile actor is parked in the flee Idle state, a paralysed or
//!   knocked-down one does nothing; the next action is chosen once the action cooldown has run out
//!   (`ActionFlee::getActionCooldown` 3 s, `aicombataction.hpp` 30); fleeing starts or stops when the
//!   held action's kind differs from the flee state; an unreachable enemy ends combat unless it is on
//!   the player's side and a hit was attempted (then the actor flees); an attack with no path starts
//!   fleeing. While fleeing, [`flee_run_step`] is `updateFleeing` (`aicombat.cpp` 378-482): from Idle,
//!   once the enemy sees the actor within its reach (or has a reach of 1000+), run to a pathgrid point
//!   or blindly for 1 s; a run to a point ends when it is reached or the enemy is past `fFleeDistance`
//!   and out of sight. The random pathgrid point and the run itself are movement (modern, not here).
//! - Intimidation raising Flee (`mechanicsmanagerimp.cpp` 647-668) is already ported with persuasion
//!   (`mw_rpg::persuasion`, `AiNudge`).
//!
//! - [`greeting_step`] is `Actors::updateGreetingState` (`apps/openmw/mwmechanics/actors.cpp` 500-596)
//!   with `turnActorToFacePlayer` (598-619) and `zTurn` (`steering.cpp` 16-43, the turn's epsilon test;
//!   its rotation rate is movement, the caller's); [`GreetingState`] is `greetingstate.hpp` 6-11 and
//!   [`GreetState`] the greeting fields of `Actor` (`actor.hpp` 45-55, 74-80). Every frame, combat,
//!   swimming or a package other than Wander / Travel / none clears the state; an actor greeting keeps
//!   `idle2` playing and finishes a started turn within 5 degrees. On the full update only (the global
//!   0.25 s [`HelloTimer`], `mTimerUpdateHello` and `sUpdateHelloInterval`, 241, 1556-1557, 1674, 1706):
//!   in state None, each update with the player within `Hello * iGreetDistanceMultiplier`, alive, not
//!   magically hidden, seen (`getLOS`) and noticed (`awarenessCheck`) counts one; at 2 the actor says
//!   `hello` (Done when there is nothing to say) and faces the player for `iGreetDuration / 0.25`
//!   updates, then is Done; any state but None returns to None once the player is `fGreetDistanceReset`
//!   or more away. The count never decays out of range (`OpenMW` behaviour, kept).
//!
//! Inputs are plain numbers and flags: the caller reads the sheet and the world. The crime response
//! (`mechanicsmanagerimp.cpp` 1415-1435) reuses both biases; it is not ported here.

use glam::Vec3;
use mw_world::convert::UNIT_METERS_F64;
use sf_records::{RecordDb, RecordId};
use tes3::esp::{GameSetting, GameSettingValue};

use crate::wander::gmst_float;

/// Rating at or above which an actor is aggressive (`isAggressive`'s `fight >= 100`).
pub const AGGRESSIVE_AT: i32 = 100; // tunables: const (OpenMW formula threshold, not a knob)

/// Vanilla `iFightDistanceBase` (Morrowind.esm).
pub const FIGHT_DISTANCE_BASE: i32 = 20; // tunables: const (vanilla GMST fallback; the game reads its records)
/// Vanilla `fFightDistanceMultiplier`, rating per game unit (Morrowind.esm).
pub const FIGHT_DISTANCE_MULTIPLIER: f32 = 0.005; // tunables: const (vanilla GMST fallback; the game reads its records)
/// Vanilla `fFightDispMult`, rating per disposition point (Morrowind.esm).
pub const FIGHT_DISP_MULT: f32 = 0.2; // tunables: const (vanilla GMST fallback; the game reads its records)
/// Vanilla `iWerewolfFightMod` (Bloodmoon.esm).
pub const WEREWOLF_FIGHT_MOD: i32 = 100; // tunables: const (vanilla GMST fallback; the game reads its records)

/// The game settings the Fight rating reads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FightGmst {
    /// `iFightDistanceBase`.
    pub distance_base: i32,
    /// `fFightDistanceMultiplier`, per game unit.
    pub distance_multiplier: f32,
    /// `fFightDispMult`.
    pub disp_mult: f32,
    /// `iWerewolfFightMod`.
    pub werewolf_mod: i32,
}

impl Default for FightGmst {
    /// The vanilla values.
    fn default() -> Self {
        Self {
            distance_base: FIGHT_DISTANCE_BASE,
            distance_multiplier: FIGHT_DISTANCE_MULTIPLIER,
            disp_mult: FIGHT_DISP_MULT,
            werewolf_mod: WEREWOLF_FIGHT_MOD,
        }
    }
}

impl FightGmst {
    /// Reads the settings from the records, each missing one at its vanilla value.
    pub fn from_db(db: &RecordDb) -> Self {
        let d = Self::default();
        Self {
            distance_base: gmst_int(db, "iFightDistanceBase", d.distance_base),
            distance_multiplier: gmst_float(db, "fFightDistanceMultiplier", d.distance_multiplier),
            disp_mult: gmst_float(db, "fFightDispMult", d.disp_mult),
            werewolf_mod: gmst_int(db, "iWerewolfFightMod", d.werewolf_mod),
        }
    }
}

/// An integer game setting, or `fallback` when missing or not a number (a float one truncates, as
/// `ESM::Variant::getInteger` does).
#[allow(clippy::cast_possible_truncation)]
pub fn gmst_int(db: &RecordDb, id: &str, fallback: i32) -> i32 {
    match db.get::<GameSetting>(&RecordId::new(id)).map(|g| &g.value) {
        Some(GameSettingValue::Integer(v)) => *v,
        Some(GameSettingValue::Float(v)) => *v as i32,
        _ => fallback,
    }
}

/// What `getFightTerm` and `isAggressive` read about an observer and its target.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools, reason = "independent facts OpenMW reads about observer and target")]
pub struct FightInput {
    /// The observer's modified Fight (`MwIdx::ai_settings[1]`).
    pub fight: i32,
    /// The observer is an NPC (a creature's disposition counts as 50).
    pub observer_npc: bool,
    /// The observer's derived disposition toward the target (NPC observers only).
    pub disposition: i32,
    /// Observer-to-target distance in game units ([`aggro_distance_units`]).
    pub distance_units: f32,
    /// The observer is already in combat with the target.
    pub in_combat_with_target: bool,
    /// `isAggressionCapable`: mobile and under no Calm of its kind.
    pub aggression_capable: bool,
    /// The target is an NPC.
    pub target_npc: bool,
    /// The target is a werewolf now.
    pub target_werewolf: bool,
    /// The target is the player.
    pub target_is_player: bool,
    /// The `PCKnownWerewolf` global is non-zero.
    pub pc_known_werewolf: bool,
}

/// `getAggroDistance` in game units between two engine-space points (meters, Y up); `ignore_height`
/// when the observer swims or flies (`canActorMoveByZAxis`).
#[allow(clippy::cast_possible_truncation)]
pub fn aggro_distance_units(observer: Vec3, target: Vec3, ignore_height: bool) -> f32 {
    let mut d = target - observer;
    if ignore_height {
        d.y = 0.0;
    }
    (f64::from(d.length()) / UNIT_METERS_F64) as f32
}

/// `getFightDistanceBias`: `iFightDistanceBase - fFightDistanceMultiplier * d`.
#[allow(clippy::cast_precision_loss)]
pub fn fight_distance_bias(distance_units: f32, gmst: &FightGmst) -> f32 {
    gmst.distance_base as f32 - gmst.distance_multiplier * distance_units
}

/// `getFightDispositionBias`: `(50 - disposition) * fFightDispMult`.
#[allow(clippy::cast_precision_loss)]
pub fn fight_disposition_bias(disposition: i32, gmst: &FightGmst) -> f32 {
    (50.0 - disposition as f32) * gmst.disp_mult
}

/// `getFightTerm`: modified Fight plus both biases, their float sum truncated toward zero.
#[allow(clippy::cast_possible_truncation)]
pub fn fight_rating(input: &FightInput, gmst: &FightGmst) -> i32 {
    let disposition = if input.observer_npc { input.disposition } else { 50 };
    let bias = fight_distance_bias(input.distance_units, gmst) + fight_disposition_bias(disposition, gmst);
    input.fight.saturating_add(bias as i32)
}

/// `isAggressive`: whether the observer wants to fight the target.
pub fn is_aggressive(input: &FightInput, gmst: &FightGmst) -> bool {
    if input.in_combat_with_target {
        return true;
    }
    if !input.aggression_capable {
        return false;
    }
    let mut fight = fight_rating(input, gmst);
    if input.observer_npc
        && input.target_npc
        && (input.target_werewolf || (input.target_is_player && input.pc_known_werewolf))
    {
        fight = fight.saturating_add(gmst.werewolf_mod);
    }
    fight >= AGGRESSIVE_AT
}

/// The gates `engageCombat` puts around its aggression check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools, reason = "independent gates of engageCombat")]
pub struct EngageGates {
    /// Observer or target is dead.
    pub either_dead: bool,
    /// The target is magically hidden (`isTargetMagicallyHidden`: Invisibility or Chameleon 75+).
    pub target_hidden: bool,
    /// `canFight`: the observer can reach the target.
    pub can_fight: bool,
    /// The target is the player or on the player's side (a follower or escort of the player).
    pub target_player_side: bool,
    /// The observer is the player's follower or escort.
    pub observer_player_side: bool,
    /// `getLOS(observer, target) && awarenessCheck(target, observer)`.
    pub sees_target: bool,
}

/// The aggression branch of `engageCombat`: does the observer start combat with the target on sight?
/// False when it is already fighting the target (`engageCombat` returns early).
pub fn engages_on_sight(input: &FightInput, gates: &EngageGates, gmst: &FightGmst) -> bool {
    if gates.either_dead || input.in_combat_with_target || gates.target_hidden || !gates.can_fight {
        return false;
    }
    let aggressive =
        gates.target_player_side && !gates.observer_player_side && is_aggressive(input, gmst);
    aggressive && gates.sees_target
}

/// Rating at or above which the Flee rating counts (`makeFleeDecision`'s `fleeRating < 100`), and the
/// modified Flee at which `vanillaRateFlee` returns the stat itself.
pub const FLEE_AT: f32 = 100.0; // tunables: const (OpenMW formula threshold, not a knob)
/// `ActionFlee::getActionCooldown`, seconds before a fleeing actor reconsiders.
pub const FLEE_ACTION_COOLDOWN_S: f32 = 3.0; // tunables: const (OpenMW constant, ported rule)
/// `updateFleeing`'s `blindRunDuration`, seconds of a blind run away from the enemy.
pub const FLEE_BLIND_RUN_S: f32 = 1.0; // tunables: const (OpenMW constant, ported rule)
/// An enemy reach at or above which `updateFleeing` runs at any distance, game units.
pub const FLEE_TRIGGER_ANY_DISTANCE: f32 = 1000.0; // tunables: const (OpenMW constant, ported rule)

/// Vanilla `fAIFleeHealthMult` (Morrowind.esm).
pub const AI_FLEE_HEALTH_MULT: f32 = 7.0; // tunables: const (vanilla GMST fallback; the game reads its records)
/// Vanilla `fAIFleeFleeMult` (Morrowind.esm).
pub const AI_FLEE_FLEE_MULT: f32 = 0.3; // tunables: const (vanilla GMST fallback; the game reads its records)
/// Vanilla `iWereWolfLevelToAttack` (Bloodmoon.esm).
pub const WEREWOLF_LEVEL_TO_ATTACK: i32 = 20; // tunables: const (vanilla GMST fallback; the game reads its records)
/// Vanilla `iWereWolfFleeMod` (Bloodmoon.esm).
pub const WEREWOLF_FLEE_MOD: i32 = 100; // tunables: const (vanilla GMST fallback; the game reads its records)
/// Vanilla `fFleeDistance`, game units (Bloodmoon.esm).
pub const FLEE_DISTANCE: f32 = 3000.0; // tunables: const (vanilla GMST fallback; the game reads its records)

/// The game settings the Flee rating and the flee run read.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FleeGmst {
    /// `fAIFleeHealthMult`.
    pub health_mult: f32,
    /// `fAIFleeFleeMult`.
    pub flee_mult: f32,
    /// `iWereWolfLevelToAttack`.
    pub werewolf_level_to_attack: i32,
    /// `iWereWolfFleeMod`.
    pub werewolf_flee_mod: i32,
    /// `fFleeDistance`, game units.
    pub flee_distance: f32,
    /// The distance bias settings (`iFightDistanceBase`, `fFightDistanceMultiplier`).
    pub fight: FightGmst,
}

impl Default for FleeGmst {
    /// The vanilla values.
    fn default() -> Self {
        Self {
            health_mult: AI_FLEE_HEALTH_MULT,
            flee_mult: AI_FLEE_FLEE_MULT,
            werewolf_level_to_attack: WEREWOLF_LEVEL_TO_ATTACK,
            werewolf_flee_mod: WEREWOLF_FLEE_MOD,
            flee_distance: FLEE_DISTANCE,
            fight: FightGmst::default(),
        }
    }
}

impl FleeGmst {
    /// Reads the settings from the records, each missing one at its vanilla value.
    pub fn from_db(db: &RecordDb) -> Self {
        let d = Self::default();
        Self {
            health_mult: gmst_float(db, "fAIFleeHealthMult", d.health_mult),
            flee_mult: gmst_float(db, "fAIFleeFleeMult", d.flee_mult),
            werewolf_level_to_attack: gmst_int(db, "iWereWolfLevelToAttack", d.werewolf_level_to_attack),
            werewolf_flee_mod: gmst_int(db, "iWereWolfFleeMod", d.werewolf_flee_mod),
            flee_distance: gmst_float(db, "fFleeDistance", d.flee_distance),
            fight: FightGmst::from_db(db),
        }
    }
}

/// What `vanillaRateFlee` reads about an actor and its enemy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FleeInput {
    /// The actor's modified Flee (`MwIdx::ai_settings[2]`).
    pub flee: i32,
    /// The actor's current Health.
    pub health_current: f32,
    /// The actor's modified (maximum) Health.
    pub health_max: f32,
    /// The actor's level.
    pub level: i32,
    /// The actor is an NPC.
    pub observer_npc: bool,
    /// The enemy is an NPC.
    pub enemy_npc: bool,
    /// The enemy is a werewolf now.
    pub enemy_werewolf: bool,
    /// Actor-to-enemy distance in game units ([`aggro_distance_units`]).
    pub distance_units: f32,
}

/// `DynamicStat::getRatio(false)`: current over maximum, 1 when the maximum is 0.
#[allow(clippy::float_cmp, reason = "OpenMW tests the maximum against exactly 0")]
pub fn health_ratio(current: f32, max: f32) -> f32 {
    if max == 0.0 { 1.0 } else { current / max }
}

/// `vanillaRateFlee`: how much the actor wants to run from the enemy.
#[allow(clippy::cast_precision_loss, reason = "AI settings and GMST ints are small")]
#[allow(clippy::float_cmp, reason = "OpenMW adds the bias to any rating that is not exactly 0")]
pub fn flee_rating(input: &FleeInput, gmst: &FleeGmst) -> f32 {
    if input.flee as f32 >= FLEE_AT {
        return input.flee as f32;
    }
    let health = health_ratio(input.health_current, input.health_max);
    let mut rating = (1.0 - health) * gmst.health_mult + input.flee as f32 * gmst.flee_mult;
    if input.observer_npc
        && input.enemy_npc
        && input.enemy_werewolf
        && input.level < gmst.werewolf_level_to_attack
    {
        rating = gmst.werewolf_flee_mod as f32;
    }
    if rating != 0.0 {
        rating += fight_distance_bias(input.distance_units, &gmst.fight);
    }
    rating
}

/// `makeFleeDecision`: flee instead of the best action, whose anti-flee rating is `anti_flee_rating`
/// (0 for fists, the weapon or spell rating, `f32::MAX` for a potion or an enchanted item).
#[allow(clippy::float_cmp, reason = "OpenMW tests for exactly 0 (fists)")]
pub fn wants_to_flee(rating: f32, anti_flee_rating: f32, has_summons: bool) -> bool {
    let rating = if rating < FLEE_AT { 0.0 } else { rating };
    rating > anti_flee_rating || (anti_flee_rating == 0.0 && has_summons)
}

/// `AiCombatStorage::mFleeState`: what a fleeing actor is doing.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum FleeMode {
    /// Not fleeing (`FleeState_None`).
    #[default]
    None,
    /// Fleeing, waiting for the enemy to come close (`FleeState_Idle`).
    Idle,
    /// Running straight away from the enemy for [`FLEE_BLIND_RUN_S`] (`FleeState_RunBlindly`).
    RunBlindly {
        /// Seconds run so far (`mFleeBlindRunTimer`).
        timer: f32,
    },
    /// Running to a pathgrid point (`FleeState_RunToDestination`).
    RunToDestination,
}

impl FleeMode {
    /// `isFleeing`.
    pub fn is_fleeing(self) -> bool {
        self != Self::None
    }
}

/// The kind of combat action the actor holds (`AiCombatStorage::mCurrentAction`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HeldAction {
    /// None chosen yet.
    #[default]
    None,
    /// `ActionFlee`.
    Flee,
    /// A weapon, spell or enchanted item (`isAttackingOrSpell`).
    Attack,
    /// A potion.
    Potion,
}

/// The flee part of `AiCombatStorage`, kept by the caller between ticks.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FleeState {
    /// `mFleeState`.
    pub mode: FleeMode,
    /// `mCurrentAction`'s kind.
    pub action: HeldAction,
    /// `mActionCooldown`, seconds until the next action is chosen.
    pub action_cooldown: f32,
}

/// What `updateFleeing` reads while the actor flees.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools, reason = "independent facts OpenMW reads about the flee")]
pub struct FleeRunInput {
    /// The actor and enemy see each other (`mLOS`, refreshed every 0.5 s by `updateLOS`).
    pub los: bool,
    /// The enemy's reach (`getMaxAttackDistance(target)`), game units.
    pub enemy_reach: f32,
    /// Actor-to-enemy distance minus both half extents (`getDistanceMinusHalfExtents`), game units.
    pub distance_minus_extents: f32,
    /// Actor-to-enemy distance, game units.
    pub distance_units: f32,
    /// A pathgrid point connected to the actor's closest one exists, and the actor is not a pure
    /// water creature (the point is a random pick: movement, the caller's).
    pub has_destination: bool,
    /// The run to the pathgrid point arrived (`pathTo` returned true).
    pub destination_reached: bool,
}

/// `updateFleeing`: the next flee mode. `dt` is the tick in seconds.
pub fn flee_run_step(mode: FleeMode, input: &FleeRunInput, gmst: &FleeGmst, dt: f32) -> FleeMode {
    match mode {
        FleeMode::None => FleeMode::None,
        FleeMode::Idle => {
            let near = input.enemy_reach >= FLEE_TRIGGER_ANY_DISTANCE
                || input.distance_minus_extents <= input.enemy_reach;
            if !(input.los && near) {
                FleeMode::Idle
            } else if input.has_destination {
                FleeMode::RunToDestination
            } else {
                FleeMode::RunBlindly { timer: 0.0 }
            }
        }
        FleeMode::RunBlindly { timer } if timer < FLEE_BLIND_RUN_S => FleeMode::RunBlindly { timer: timer + dt },
        FleeMode::RunBlindly { .. } => FleeMode::Idle,
        FleeMode::RunToDestination => {
            if (input.distance_units > gmst.flee_distance && !input.los) || input.destination_reached {
                FleeMode::Idle
            } else {
                FleeMode::RunToDestination
            }
        }
    }
}

/// What `AiCombat::execute` and `attack` read for the flee decision in one tick.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools, reason = "independent gates of AiCombat::execute")]
pub struct FleeStepInput {
    /// `isMobile`.
    pub mobile: bool,
    /// Paralysed or knocked down.
    pub incapacitated: bool,
    /// The AI reaction timer fired this tick (`mReaction`, a random period: the caller's).
    pub reaction_due: bool,
    /// `canFight`: the actor can reach the enemy.
    pub can_fight: bool,
    /// The enemy is the player or on the player's side, and one of the two attempted a hit on the
    /// other (`hitAttemptMatchesTarget`): an unreachable enemy then makes the actor flee.
    pub force_flee_allowed: bool,
    /// `readyToPrepareAttack`: not mid-hit, weapon at most equipped.
    pub ready_to_prepare: bool,
    /// The actor is an NPC werewolf (`prepareNextAction` picks fists, never flight).
    pub werewolf: bool,
    /// The Flee rating inputs.
    pub rating: FleeInput,
    /// The best non-flee action `prepareNextAction` would pick ([`HeldAction::Attack`] or `Potion`).
    pub best_action: HeldAction,
    /// Its anti-flee rating (0 for fists, `f32::MAX` for a potion or an enchanted item).
    pub anti_flee_rating: f32,
    /// Its cooldown, seconds.
    pub best_action_cooldown: f32,
    /// A summoned creature is out.
    pub has_summons: bool,
    /// The attack found no path to the enemy, not even to a navmesh point within reach
    /// (`aicombat.cpp` 309-353: the caller's nav query).
    pub path_failed: bool,
    /// What `updateFleeing` reads, when fleeing.
    pub run: FleeRunInput,
}

/// The flee outcome of one tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FleeDecision {
    /// Not fleeing, before or after.
    NotFleeing,
    /// Began fleeing this tick (`OpenMW` voices `flee`, except when parking an immobile actor).
    Start,
    /// Still fleeing.
    Keep,
    /// Stopped fleeing: back to fighting.
    Stop,
    /// The enemy is out of reach: combat ends (`attack` returns true).
    EndCombat,
}

/// The flee half of one `AiCombat::execute` tick: updates `state` and says whether the actor starts,
/// keeps or stops fleeing. `dt` is the tick in seconds.
pub fn flee_step(state: &mut FleeState, input: &FleeStepInput, gmst: &FleeGmst, dt: f32) -> FleeDecision {
    let was = state.mode.is_fleeing();
    let outcome = |state: &FleeState| match (was, state.mode.is_fleeing()) {
        (false, false) => FleeDecision::NotFleeing,
        (false, true) => FleeDecision::Start,
        (true, true) => FleeDecision::Keep,
        (true, false) => FleeDecision::Stop,
    };
    if !input.mobile {
        state.mode = FleeMode::Idle;
        return outcome(state);
    }
    if input.incapacitated {
        return outcome(state);
    }
    if was {
        state.mode = flee_run_step(state.mode, &input.run, gmst, dt);
    }
    state.action_cooldown -= dt;
    if !input.reaction_due {
        return outcome(state);
    }
    // `attack`.
    if input.can_fight {
        if state.action_cooldown > 0.0 {
            return outcome(state);
        }
        if input.ready_to_prepare {
            let rating = flee_rating(&input.rating, gmst);
            if !input.werewolf && wants_to_flee(rating, input.anti_flee_rating, input.has_summons) {
                state.action = HeldAction::Flee;
                state.action_cooldown = FLEE_ACTION_COOLDOWN_S;
            } else {
                state.action = input.best_action;
                state.action_cooldown = input.best_action_cooldown;
            }
        }
    } else if input.force_flee_allowed {
        state.action = HeldAction::Flee;
        state.action_cooldown = FLEE_ACTION_COOLDOWN_S;
    } else {
        state.action_cooldown = 0.0;
        return FleeDecision::EndCombat;
    }
    if state.action == HeldAction::None {
        return outcome(state);
    }
    let action_flees = state.action == HeldAction::Flee;
    if state.mode.is_fleeing() != action_flees {
        state.mode = if action_flees { FleeMode::Idle } else { FleeMode::None };
        if action_flees {
            return outcome(state);
        }
    }
    if state.action == HeldAction::Attack && input.path_failed {
        state.action = HeldAction::Flee;
        state.action_cooldown = FLEE_ACTION_COOLDOWN_S;
        state.mode = FleeMode::Idle;
    }
    outcome(state)
}

/// `sUpdateHelloInterval`, seconds between full greeting updates.
pub const GREET_UPDATE_INTERVAL_S: f32 = 0.25; // tunables: const (OpenMW constant, ported rule)
/// `initialDelay`: full updates with the player in reach before the actor says Hello.
pub const GREET_INITIAL_DELAY: i32 = 2; // tunables: const (OpenMW constant, ported rule)
/// The `zTurn` epsilon of a greeting turn, radians (5 degrees).
pub const GREET_TURN_EPSILON_RAD: f32 = 5.0 * std::f32::consts::PI / 180.0; // tunables: const (OpenMW constant, ported rule)
/// The heading error above which a smooth-movement actor starts a greeting turn, radians (60 degrees).
pub const GREET_SMOOTH_TURN_MIN_RAD: f64 = 60.0 * std::f64::consts::PI / 180.0; // tunables: const (OpenMW constant, ported rule)

/// Vanilla `iGreetDistanceMultiplier`, game units per Hello point (Morrowind.esm).
pub const GREET_DISTANCE_MULTIPLIER: i32 = 6; // tunables: const (vanilla GMST fallback; the game reads its records)
/// Vanilla `iGreetDuration`, seconds (Morrowind.esm).
pub const GREET_DURATION: i32 = 4; // tunables: const (vanilla GMST fallback; the game reads its records)
/// Vanilla `fGreetDistanceReset`, game units (Morrowind.esm).
pub const GREET_DISTANCE_RESET: f32 = 512.0; // tunables: const (vanilla GMST fallback; the game reads its records)

/// The game settings the greeting reads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GreetGmst {
    /// `iGreetDistanceMultiplier`, game units per Hello point.
    pub distance_multiplier: i32,
    /// `iGreetDuration`, seconds the actor faces the player after its Hello.
    pub duration: i32,
    /// `fGreetDistanceReset`, game units.
    pub distance_reset: f32,
}

impl Default for GreetGmst {
    /// The vanilla values.
    fn default() -> Self {
        Self {
            distance_multiplier: GREET_DISTANCE_MULTIPLIER,
            duration: GREET_DURATION,
            distance_reset: GREET_DISTANCE_RESET,
        }
    }
}

impl GreetGmst {
    /// Reads the settings from the records, each missing one at its vanilla value.
    pub fn from_db(db: &RecordDb) -> Self {
        let d = Self::default();
        Self {
            distance_multiplier: gmst_int(db, "iGreetDistanceMultiplier", d.distance_multiplier),
            duration: gmst_int(db, "iGreetDuration", d.duration),
            distance_reset: gmst_float(db, "fGreetDistanceReset", d.distance_reset),
        }
    }

    /// `greetDuration`: full updates the actor faces the player, `(int)(iGreetDuration / 0.25)`.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, reason = "OpenMW's int/float cast")]
    pub fn duration_updates(&self) -> i32 {
        (self.duration as f32 / GREET_UPDATE_INTERVAL_S) as i32
    }

    /// The greeting reach, game units: `Hello * iGreetDistanceMultiplier` (an int product).
    #[allow(clippy::cast_precision_loss, reason = "OpenMW casts the int product to float")]
    pub fn hello_distance(&self, hello: i32) -> f32 {
        hello.saturating_mul(self.distance_multiplier) as f32
    }
}

/// `MWMechanics::GreetingState`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GreetingState {
    /// Not greeting: counting full updates with the player in reach.
    #[default]
    None,
    /// Said Hello, facing the player.
    InProgress,
    /// Greeted; waits for the player to walk past `fGreetDistanceReset`.
    Done,
}

/// The greeting part of `MWMechanics::Actor`, kept by the caller per actor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GreetState {
    /// `mGreetingState`.
    pub state: GreetingState,
    /// `mGreetingTimer`, in full updates.
    pub timer: i32,
    /// `mIsTurningToPlayer`.
    pub turning: bool,
    /// `mTargetAngleRadians`, the heading the turn aims at.
    pub angle_to_player: f32,
}

/// `Actors::mTimerUpdateHello`: one for all actors. A frame is a full greeting update when the
/// timer is 0, every [`GREET_UPDATE_INTERVAL_S`]; between them actors only finish their turns.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HelloTimer {
    /// Seconds since the last full update.
    pub elapsed: f32,
}

impl HelloTimer {
    /// Advances one unpaused frame of `dt` seconds and returns `turn_only` for this frame's
    /// [`greeting_step`] calls (`Actors::update` 1556-1557, 1674, 1706).
    pub fn frame(&mut self, dt: f32) -> bool {
        if self.elapsed >= GREET_UPDATE_INTERVAL_S {
            self.elapsed = 0.0;
        }
        let turn_only = self.elapsed > 0.0;
        self.elapsed += dt;
        turn_only
    }
}

/// What `updateGreetingState` reads about an actor and the player in one frame.
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools, reason = "independent facts OpenMW reads about actor and player")]
pub struct GreetInput {
    /// The actor's modified Hello (`MwIdx::ai_settings[0]`).
    pub hello: i32,
    /// Actor-to-player distance in game units, height included ([`greet_geometry`]).
    pub distance_units: f32,
    /// The heading that faces the player, `atan2(dx, dy)` in Morrowind axes ([`greet_geometry`]).
    pub angle_to_player: f32,
    /// The actor's heading (`rot[2]`), same convention.
    pub heading: f32,
    /// The actor's AI sequence is in combat.
    pub in_combat: bool,
    /// The actor swims.
    pub swimming: bool,
    /// The active package is Wander or Travel, or there is none.
    pub package_allows: bool,
    /// The player is dead.
    pub player_dead: bool,
    /// The actor is paralysed.
    pub paralyzed: bool,
    /// The player is magically hidden (`isTargetMagicallyHidden`: Invisibility, or Chameleon 75+).
    pub player_hidden: bool,
    /// `getLOS(player, actor)`.
    pub los: bool,
    /// `awarenessCheck(player, actor)`: the caller rolls it (it is random); read only on a full update
    /// in state None.
    pub aware: bool,
    /// `say(actor, "hello")` would play: the actor has a Hello voice line and is not already talking.
    pub can_say_hello: bool,
    /// The actor has the `ForceJump` or `ForceSneak` movement flag (it never stops to face).
    pub movement_forced: bool,
    /// The game's smooth movement setting (a greeting turn then starts only past 60 degrees).
    pub smooth_movement: bool,
    /// Between full updates ([`HelloTimer::frame`]): only the turn runs.
    pub turn_only: bool,
}

/// What the actor does this frame. Several can hold at once.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GreetAction {
    /// Say the `hello` voice line (only when [`GreetInput::can_say_hello`]).
    pub say_hello: bool,
    /// Keep `idle2` playing (start it when it is not), as Morrowind does while greeting.
    pub play_idle2: bool,
    /// Stop walking this frame (`turnActorToFacePlayer` zeroes the movement).
    pub hold_still: bool,
    /// Turn toward this heading this frame (the rate is the caller's: `smoothTurn`'s limit).
    pub turn_to: Option<f32>,
}

/// `Misc::normalizeAngle`: the angle in `[-pi, pi)`, computed in f64.
pub fn normalize_angle(angle: f64) -> f64 {
    let full_turns = angle / std::f64::consts::TAU + 0.5;
    (full_turns - full_turns.floor() - 0.5) * std::f64::consts::TAU
}

/// The distance in game units and the heading that faces `player` from `actor`, both engine-space
/// points (meters, Y up, -Z north): `dir = player - actor` in Morrowind axes, `atan2(dir.x, dir.y)`.
#[allow(clippy::cast_possible_truncation)]
pub fn greet_geometry(actor: Vec3, player: Vec3) -> (f32, f32) {
    let d = player - actor;
    let distance = (f64::from(d.length()) / UNIT_METERS_F64) as f32;
    (distance, d.x.atan2(-d.z))
}

/// One frame of `updateGreetingState` for an actor (never the player): updates `state` and says what
/// the actor does.
#[allow(clippy::cast_possible_truncation, reason = "OpenMW narrows normalizeAngle's double to float")]
pub fn greeting_step(state: &mut GreetState, input: &GreetInput, gmst: &GreetGmst) -> GreetAction {
    let mut action = GreetAction::default();
    if input.in_combat || input.swimming || !input.package_allows {
        state.turning = false;
        state.timer = 0;
        state.state = GreetingState::None;
        return action;
    }
    let mut greeting = state.state;
    action.play_idle2 = greeting == GreetingState::InProgress;
    if state.turning {
        let diff = normalize_angle(f64::from(state.angle_to_player) - f64::from(input.heading)) as f32;
        if diff.abs() < GREET_TURN_EPSILON_RAD {
            state.turning = false;
        } else {
            action.turn_to = Some(state.angle_to_player);
        }
    }
    if input.turn_only {
        return action;
    }
    let hello_distance = gmst.hello_distance(input.hello);
    let dist_squared = input.distance_units * input.distance_units;
    let mut timer = state.timer;
    if greeting == GreetingState::None {
        if dist_squared <= hello_distance * hello_distance
            && !input.player_dead
            && !input.paralyzed
            && !input.player_hidden
            && input.los
            && input.aware
        {
            timer += 1;
        }
        if timer >= GREET_INITIAL_DELAY {
            greeting = GreetingState::InProgress;
            if input.can_say_hello {
                action.say_hello = true;
            } else {
                greeting = GreetingState::Done;
            }
            timer = 0;
        }
    }
    if greeting == GreetingState::InProgress {
        timer += 1;
        if timer <= gmst.duration_updates() {
            face_player(state, input, &mut action);
        } else {
            greeting = GreetingState::Done;
            timer = 0;
        }
    }
    if greeting != GreetingState::None && dist_squared >= gmst.distance_reset * gmst.distance_reset {
        greeting = GreetingState::None;
        timer = 0;
    }
    state.timer = timer;
    state.state = greeting;
    action
}

/// `turnActorToFacePlayer`: stop, and start a turn unless one runs (with smooth movement, only past
/// 60 degrees).
fn face_player(state: &mut GreetState, input: &GreetInput, action: &mut GreetAction) {
    if input.movement_forced {
        return;
    }
    action.hold_still = true;
    if !state.turning {
        state.angle_to_player = input.angle_to_player;
        let delta = normalize_angle(f64::from(input.angle_to_player) - f64::from(input.heading));
        if !input.smooth_movement || delta.abs() > GREET_SMOOTH_TURN_MIN_RAD {
            state.turning = true;
        }
    }
}
