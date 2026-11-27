//! The player as an interaction user: its focus candidate, the prompt of a target with a def, and
//! its Use presses as `ActivateCommand`s (crate docs, "Player user", "Focus", "Activation").

use bevy_ecs::prelude::*;
use bevy_log::debug;
use mw_game_core::player::{HOST_KEY, PlayerBody};
use mw_interaction::activation::{Availability, DefaultAction, ROWS, RecordClass};
use mw_player_bevy::{Focus, FocusTarget, UsePresses};
use sf_interaction::{TagId, UserKey, VerbTarget};
use sf_interaction_bevy::{ActivateCommand, FocusCandidates, InteractionDefs, InteractionSettings, InteractionUser, PlayerFocus};
use sf_script_bevy::{PLAYER, ScriptPlacements};
use sf_world::PlacementId;
use smallvec::SmallVec;

/// The local player's claim key.
pub const PLAYER_USER: UserKey = UserKey::Player(HOST_KEY);

/// The prompt of furniture verb `verb`: its offered furniture row's (`"Sit"`, `"Lie down"`,
/// `"Lean"`), `None` for a verb no built furniture row has.
#[must_use]
pub fn furniture_prompt(verb: TagId) -> Option<&'static str> {
    ROWS.iter()
        .find(|r| matches!(r.class, RecordClass::Furniture(_)) && r.available == Availability::Offered && TagId::of(r.verb) == verb)
        .map(|r| r.prompt)
}

/// The prompt of light verb `verb` (`snuff`: "Snuff", `relight`: "Relight"; `IN19c`), `None` for any
/// other verb: a light's is verb-only (a fixed flame has no name, `light.cpp:150`).
#[must_use]
pub fn light_prompt(verb: TagId) -> Option<&'static str> {
    ROWS.iter()
        .find(|r| r.class == RecordClass::Light && r.action == DefaultAction::ToggleLit && r.available == Availability::Offered && TagId::of(r.verb) == verb)
        .map(|r| r.prompt)
}

/// The prompts of the offered non-furniture rows with verb `verb`, in row order (`open`: "Open";
/// a swing door's `close`: "Close").
fn class_prompts(verb: TagId) -> impl Iterator<Item = &'static str> {
    ROWS.iter()
        .filter(move |r| !matches!(r.class, RecordClass::Furniture(_)) && r.available == Availability::Offered && TagId::of(r.verb) == verb)
        .map(|r| r.prompt)
}

/// Whether `verb` is a record class's own primary use (it has [`class_prompts`]: `open`, `close`,
/// `use`, ...), the slotless verb the bake keeps on a def'd container or activator
/// (`mw_interaction::class_verb`) and a swing door's def offers (`mw_interaction::door_def`).
fn class_verb(verb: TagId) -> bool {
    class_prompts(verb).next().is_some()
}

/// A focus candidate's score from its distance: nearer is higher (one candidate today; the
/// stabilizer compares scores only between candidates).
fn score(distance: f32) -> f32 {
    1.0 / (1.0 + distance.max(0.0))
}

/// The player body is an [`InteractionUser`] (the seat default seats it), at the live player
/// claim priority.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(crate) fn player_user(
    mut commands: Commands,
    settings: Res<InteractionSettings>,
    fresh: Query<Entity, (With<PlayerBody>, Without<InteractionUser>)>,
    mut users: Query<&mut InteractionUser, With<PlayerBody>>,
) {
    let priority = settings.0.priority_player;
    for body in &fresh {
        commands.entity(body).insert(InteractionUser { user: PLAYER_USER, actor: PLAYER, priority, tags: SmallVec::new() });
    }
    if settings.is_changed() {
        for mut u in users.iter_mut().filter(|u| u.priority != priority) {
            u.priority = priority;
        }
    }
}

/// A focused target with a def prompts its best offered verb (verb only); one offering none (a
/// taken seat) prompts nothing. Def-less targets, and a def'd one whose best verb is its record
/// class's own primary use (a census container's "Open", an activator's "Use"), keep
/// `activatable`'s prompt (name + verb), unless that prompt is not one of the verb's
/// [`class_prompts`]: then its verb becomes the first of them (an open swing door's "Close" over
/// `activatable`'s "Open"), and only then is [`Focus`] written. A light verb (snuff, relight) is
/// always verb-only ([`light_prompt`]).
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(crate) fn prompt(mut focus: ResMut<Focus>, defs: Res<InteractionDefs>, players: Res<PlayerFocus>, placements: Res<ScriptPlacements>) {
    let Some(hit) = focus.hit else { return };
    let target = PlacementId(hit.handle.0);
    if defs.get(target).is_none() {
        return;
    }
    let best = players.get(PLAYER_USER).filter(|s| s.target() == Some(target)).and_then(|s| s.verbs().first());
    let light = best.is_some_and(|c| light_prompt(c.verb).is_some());
    if let Some(c) = best.filter(|c| !light && matches!(c.target, VerbTarget::Slotless { .. }) && class_verb(c.verb)) {
        let stale = focus.target.as_ref().is_some_and(|t| t.handle == hit.handle && !t.verb.is_some_and(|v| class_prompts(c.verb).any(|p| p == v)));
        if stale
            && let Some(first) = class_prompts(c.verb).next()
            && let Some(t) = focus.target.as_mut()
        {
            t.verb = Some(first);
        }
        return;
    }
    let verb = best.map(|c| c.verb);
    let record = focus.target.as_ref().map(|t| t.record.clone()).or_else(|| placements.loaded.get(&hit.handle).map(|p| p.record.clone()));
    focus.target = verb.and_then(|v| furniture_prompt(v).or_else(|| light_prompt(v))).zip(record).map(|(prompt, record)| FocusTarget {
        handle: hit.handle,
        record,
        name: String::new(),
        verb: Some(prompt),
        distance: hit.distance,
    });
}

/// The player's focus candidate this fixed tick: a pending press's target (so the command names
/// what was pressed on), else the ray's raw hit, else none.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub(crate) fn candidates(
    focus: Res<Focus>,
    presses: Res<UsePresses>,
    settings: Res<InteractionSettings>,
    mut out: MessageWriter<FocusCandidates>,
) {
    let hit = focus.hit;
    let candidates = match presses.0.front() {
        Some(latched) => {
            let s = hit.filter(|h| h.handle == *latched).map_or(score(0.0), |h| score(h.distance));
            vec![(PlacementId(latched.0), s)]
        }
        None => hit.map(|h| vec![(PlacementId(h.handle.0), score(h.distance))]).unwrap_or_default(),
    };
    out.write(FocusCandidates {
        player: PLAYER_USER,
        candidates,
        held: false,
        priority: settings.0.priority_player,
        tags: SmallVec::new(),
    });
}

/// Each latched press becomes one [`ActivateCommand`] on the focused target (best verb and slot;
/// a def-less target's primary use), consumed whatever the outcome; a press whose target is no
/// longer the focus is dropped.
pub(crate) fn commands(mut presses: ResMut<UsePresses>, players: Res<PlayerFocus>, mut out: MessageWriter<ActivateCommand>) {
    if presses.0.is_empty() {
        return;
    }
    let state = players.get(PLAYER_USER);
    for latched in presses.0.drain(..) {
        match state.and_then(|s| s.command(PLAYER_USER, PLAYER, None)) {
            Some(cmd) if cmd.target == PlacementId(latched.0) => {
                out.write(cmd);
            }
            other => debug!("interact: press on {:032x} dropped (focus command {other:?})", latched.0),
        }
    }
}
