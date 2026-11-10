//! Spellcasting on the host (effects-adapter.md "Casting rules", slice A5a): a player's cast
//! request becomes the framework's `Cast` op on the player's effects, and touch casts land on
//! their contact. No input or spell menu here (A5b: `mw_player_bevy`).
//!
//! - **Request, then host** (effects.md "Co-op": clients send intents, the host makes ops): a
//!   [`CastRequest`] (player, spell id, the contact the caller found) becomes
//!   `EffectCommand(Player(HOST_KEY), [Cast { source, target, rolls }])` in
//!   [`apply_cast_requests`] (`Update`); the step checks the book, a power's recharge on
//!   `EffectClock`, then the game's cast model ([`crate::effects::add`]: modern,
//!   `mw_effects::MwModernEffectRules`), spends the cost from the magicka and lands the self
//!   entries on the caster at once, on the next fixed tick.
//! - **Delivery** ([`deliver_casts`], `EffectSystems::Requests`): touch entries come back as
//!   `EffectRequest::Touch { target, delivery }` and land on that target with the caster's seed
//!   (`Delivery::inflict`, one fixed tick later); target entries come back as
//!   `EffectRequest::Launch` and are logged and dropped until projectiles own them (adapter
//!   "Later: bolts and area").
//! - **Outcome** ([`report_casts`]): the player's refused and failed casts are logged; the
//!   message boxes (`sMagicSkillFail`, `sMagicInsufficientSP`, `sPowerAlreadyUsed`) are A5b's HUD.
//! - **Dev** ([`console_cast`]): the console's `cast <spell>` (`mw_devtools_bevy`) sends a
//!   request from the player at the click-selected actor, learning the spell first if the book
//!   lacks it (a dev shortcut; vanilla's console has `AddSpell` for that).
//!
//! Not yet: the cast animation and its release frame (the action layer, A5b: the request is the
//! release), the skill use on a success (`Spellcast_Success`, stats S9, `mw_effects::spell_school`
//! names the school), breaking Invisibility on a cast (A7), NPC casts (AI).

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use mw_game_core::player::HOST_KEY;
use sf_effects_bevy::sf_effects::{EffectEvent, EffectOp, EffectRequest, RollSeed};
use sf_effects_bevy::{
    EffectCommand, EffectIndex, EffectMessage, EffectRequestMessage, EffectSystems, Effects,
    SourceTableRes,
};
use sf_records::RecordId;
use sf_rpg::PlayerId;
use sf_stats_bevy::StatTick;
use sf_stats_bevy::sf_stats::StatOwner;

/// Salt bit of a cast's seed, so a cast and a drink on the same tick roll apart.
const CAST_SEED_SALT: u64 = 1 << 31;

/// A player casts a spell or power of their book at the release moment. `target` is the actor
/// the caller found in contact (touch entries land on it); `None` drops the touch entries.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct CastRequest {
    pub player: PlayerId,
    pub source: RecordId,
    pub target: Option<StatOwner>,
}

/// The request, delivery and reports (on the effects layer: [`crate::effects::add`] calls it).
pub fn add(app: &mut App) {
    app.add_message::<CastRequest>()
        .add_systems(Update, (apply_cast_requests, report_casts))
        .add_systems(FixedUpdate, deliver_casts.in_set(EffectSystems::Requests));
}

/// Host side of [`CastRequest`] (see the module docs). Only the host player casts here.
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn apply_cast_requests(
    mut requests: MessageReader<CastRequest>,
    tick: Option<Res<StatTick>>,
    mut commands: MessageWriter<EffectCommand>,
) {
    let tick = tick.map_or(0, |t| t.0);
    for (n, request) in requests.read().enumerate() {
        if request.player != PlayerId::HOST {
            warn!("cast: player {:?} is not on this host", request.player);
            continue;
        }
        let rolls = RollSeed((u64::from(tick) << 32) | CAST_SEED_SALT | n as u64);
        let op = EffectOp::Cast {
            source: request.source.clone(),
            target: request.target,
            rolls,
        };
        commands.write(EffectCommand::new(StatOwner::Player(HOST_KEY), vec![op]));
    }
}

/// Touch entries land on their contact; target entries have no projectile yet.
pub fn deliver_casts(
    mut requests: MessageReader<EffectRequestMessage>,
    mut commands: MessageWriter<EffectCommand>,
) {
    for EffectRequestMessage(request) in requests.read() {
        match request {
            EffectRequest::Touch { target, delivery } => {
                commands.write(EffectCommand::new(*target, vec![delivery.inflict()]));
            }
            EffectRequest::Launch(d) => info!(
                "cast: {:?} launched {} ({} target entries): no projectile owner yet, dropped",
                d.caster,
                d.source.record,
                d.entries.len()
            ),
            EffectRequest::Effect { .. } => {}
        }
    }
}

/// The player's cast outcomes, logged until the HUD shows them (A5b).
pub fn report_casts(mut events: MessageReader<EffectMessage>) {
    let player = StatOwner::Player(HOST_KEY);
    for EffectMessage(event) in events.read() {
        match event {
            EffectEvent::CastFailed {
                caster,
                source,
                reason,
                ..
            } if *caster == player => info!("cast: {source} failed: {reason:?}"),
            EffectEvent::CastSucceeded { caster, source, .. } if *caster == player => {
                info!("cast: {source}");
            }
            _ => {}
        }
    }
}

/// The dev console's `cast <spell>`: a [`CastRequest`] from the host player at `selected` (the
/// console's click selection, when it has effects and is not the player). Learns the spell
/// first when the book lacks it. Returns the line to print.
pub fn console_cast(
    world: &mut World,
    spell: &str,
    selected: Option<Entity>,
) -> Result<String, String> {
    let id = RecordId::new(spell);
    let sources = world
        .get_resource::<SourceTableRes>()
        .ok_or("no effects in this run")?;
    if sources.0.get(&id).is_none() {
        return Err(format!("no spell {spell:?}"));
    }
    let player = StatOwner::Player(HOST_KEY);
    let entity = world
        .get_resource::<EffectIndex>()
        .and_then(|i| i.get(player))
        .ok_or("the player has no effects")?;
    let knows = world
        .get::<Effects>(entity)
        .is_some_and(|fx| fx.book.knows(&id));
    let target = selected
        .and_then(|e| world.get::<Effects>(e))
        .map(|fx| fx.owner)
        .filter(|o| *o != player);
    if !knows {
        world.write_message(EffectCommand::new(
            player,
            vec![EffectOp::Learn { source: id.clone() }],
        ));
    }
    world.write_message(CastRequest {
        player: PlayerId::HOST,
        source: id,
        target,
    });
    let at = target.map_or_else(|| "no touch target".to_owned(), |t| format!("at {t:?}"));
    let learned = if knows { "" } else { " (learned first)" };
    Ok(format!(
        "cast {spell}{learned}, {at}; the outcome is in the log"
    ))
}

#[cfg(test)]
mod tests;
