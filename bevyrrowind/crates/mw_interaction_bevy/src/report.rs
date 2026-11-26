//! The game's log of the shell's seat reports and refused uses (`sf_interaction_bevy::SeatReport`,
//! `UseRefused`): the framework writes the trail, the game logs it, one line per message.
//!
//! - Normal path (`info!`): `seat: <user> asks | holds | offered | entered | exited <placement> slot <n>`.
//! - A step that ends a use without sitting (`warn!`): `seat: use dropped: <reason>, <user> on
//!   <placement> slot <n>`; an approval ignored while the user is in a use: `seat: use ignored: ...`.
//! - A gate's refusal (`warn!`): `interact: use refused: target <handle> verb <verb> reason <reason>`.
//!
//! Names: the player prints as `player`, the seat rules as their ids (`interact.sit.enter`), the
//! shell's door verbs as `open` / `close`; every other id prints as `{:?}` (the `activate:` line
//! above it in the log names the record).

use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use sf_interaction::{TagId, UserKey, VERB_CLOSE, VERB_OPEN, fnv1a32};
use sf_interaction_bevy::{SIT_APPROACH, SIT_ENTER, SIT_EXIT, SIT_LOOP, SeatReport, SeatStep, UseRefused};

use crate::input::PLAYER_USER;

/// The seat rules' keys: `sf_action::RuleKey::of(id)` is this same 32-bit FNV-1a of the id.
const SIT_RULES: [(u32, &str); 4] = [
    (fnv1a32(SIT_APPROACH.as_bytes()), SIT_APPROACH),
    (fnv1a32(SIT_ENTER.as_bytes()), SIT_ENTER),
    (fnv1a32(SIT_LOOP.as_bytes()), SIT_LOOP),
    (fnv1a32(SIT_EXIT.as_bytes()), SIT_EXIT),
];

fn user_name(user: UserKey) -> String {
    if user == PLAYER_USER { "player".to_owned() } else { format!("{user:?}") }
}

/// A rule key's id when it is one of the seat rules, else its `Debug`.
fn rule_name(key: u32, debug: String) -> String {
    SIT_RULES.iter().find(|(k, _)| *k == key).map_or(debug, |(_, id)| (*id).to_owned())
}

fn verb_name(verb: Option<TagId>) -> String {
    match verb {
        None => "primary".to_owned(),
        Some(v) if v == VERB_OPEN => "open".to_owned(),
        Some(v) if v == VERB_CLOSE => "close".to_owned(),
        Some(v) => format!("{v:?}"),
    }
}

/// One seat report as a log line; `true` = a warning (the use ends or is ignored without sitting).
pub(crate) fn seat_line(r: &SeatReport) -> (bool, String) {
    let user = user_name(r.user);
    let seat = format!("{:?} slot {}", r.slot.placement, r.slot.slot);
    let dropped = |why: String| (true, format!("seat: use dropped: {why}, {user} on {seat}"));
    match &r.step {
        SeatStep::Asked => (false, format!("seat: {user} asks {seat}")),
        SeatStep::Held => (false, format!("seat: {user} holds {seat} already: granted")),
        SeatStep::Busy(phase) => (true, format!("seat: use ignored: already in a seat use ({phase:?}) on {seat}, {user}")),
        SeatStep::ClaimRefused(refusal) => dropped(format!("claim refused ({refusal:?})")),
        SeatStep::UserGone => dropped("user gone".to_owned()),
        SeatStep::Lost(loss) => dropped(format!("lost {loss:?}")),
        SeatStep::NoEntrance => dropped("no free entrance".to_owned()),
        SeatStep::Offered { entry, feet_to_seat_m, yaw_off } => {
            let entry = entry.map_or_else(|| "any (no entrances)".to_owned(), |s| format!("{s:?}"));
            let yaw = yaw_off.to_degrees();
            (false, format!("seat: {user} offered {seat}: entry {entry}, feet {feet_to_seat_m:.2} m from the seat, yaw off {yaw:.0} deg"))
        }
        SeatStep::Approaching => (false, format!("seat: {user} walks to {seat}")),
        SeatStep::Entered => (false, format!("seat: {user} entered {seat}")),
        SeatStep::Exited => (false, format!("seat: {user} exited {seat}")),
        SeatStep::EnterRefused(refusals) => {
            let list: Vec<String> =
                refusals.iter().map(|f| format!("{}: {:?}", rule_name(f.rule.0, format!("{:?}", f.rule)), f.reason)).collect();
            let list = if list.is_empty() { "no refusal named".to_owned() } else { list.join("; ") };
            dropped(format!("enter refused by the action layer ({list})"))
        }
    }
}

/// One refused use as a log line.
fn refused_line(r: &UseRefused) -> String {
    format!("interact: use refused: target {:?} verb {} reason {:?}", r.activation.target, verb_name(r.verb), r.reason)
}

/// `Update`: one log line per [`SeatReport`] and per [`UseRefused`] (module docs).
pub(crate) fn log_reports(mut seats: MessageReader<SeatReport>, mut refused: MessageReader<UseRefused>) {
    for r in seats.read() {
        match seat_line(r) {
            (true, line) => warn!("{line}"),
            (false, line) => info!("{line}"),
        }
    }
    for r in refused.read() {
        warn!("{}", refused_line(r));
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::{App, Update};
    use bevy_ecs::prelude::*;
    use sf_interaction::{Refusal, Side, SlotRef, TagId, VERB_OPEN};
    use sf_interaction_bevy::{SeatLoss, SeatPhase, SeatReport, SeatStep, UseRefused};
    use sf_script_bevy::sf_script::Handle;
    use sf_script_bevy::{Activated, PLAYER};
    use sf_world::PlacementId;
    use smallvec::smallvec;

    use super::{SIT_APPROACH, SIT_ENTER, fnv1a32, log_reports, refused_line, seat_line};
    use crate::input::PLAYER_USER;

    const SLOT: SlotRef = SlotRef { placement: PlacementId(7), slot: 1 };

    fn report(step: SeatStep) -> SeatReport {
        SeatReport { user: PLAYER_USER, entity: Entity::PLACEHOLDER, slot: SLOT, step }
    }

    /// The action layer's enter refusal of `interact.sit.enter` for `MaxCorrection`. This crate has
    /// no `sf_action` dependency, so the value is decoded from its postcard form: the rule key
    /// (a `u32` newtype) and the `Refused` variant index (9 = `MaxCorrection`).
    fn max_correction_step() -> SeatStep {
        let bytes = postcard::to_stdvec(&(fnv1a32(SIT_ENTER.as_bytes()), 9_u32)).expect("encode");
        let enter = postcard::from_bytes(&bytes).expect("decode");
        let other = postcard::from_bytes(&postcard::to_stdvec(&(5_u32, 6_u32)).expect("encode")).expect("decode");
        SeatStep::EnterRefused(smallvec![enter, other])
    }

    /// The approach rule's refusal for `ApproachCancelled` (variant index 15), decoded the same way.
    fn approach_cancelled_step() -> SeatStep {
        let bytes = postcard::to_stdvec(&(fnv1a32(SIT_APPROACH.as_bytes()), 15_u32)).expect("encode");
        SeatStep::EnterRefused(smallvec![postcard::from_bytes(&bytes).expect("decode")])
    }

    #[test]
    fn every_step_has_its_line() {
        let on = "player on PlacementId(7) slot 1";
        let cases: Vec<(SeatStep, bool, String)> = vec![
            (SeatStep::Asked, false, "seat: player asks PlacementId(7) slot 1".into()),
            (SeatStep::Held, false, "seat: player holds PlacementId(7) slot 1 already: granted".into()),
            (
                SeatStep::Busy(SeatPhase::Seated),
                true,
                "seat: use ignored: already in a seat use (Seated) on PlacementId(7) slot 1, player".into(),
            ),
            (SeatStep::ClaimRefused(Refusal::Taken), true, format!("seat: use dropped: claim refused (Taken), {on}")),
            (SeatStep::UserGone, true, format!("seat: use dropped: user gone, {on}")),
            (SeatStep::Lost(SeatLoss::Frame), true, format!("seat: use dropped: lost Frame, {on}")),
            (SeatStep::NoEntrance, true, format!("seat: use dropped: no free entrance, {on}")),
            (
                SeatStep::Offered { entry: Some(Side::Front), feet_to_seat_m: 0.4249, yaw_off: -std::f32::consts::FRAC_PI_4 },
                false,
                "seat: player offered PlacementId(7) slot 1: entry Front, feet 0.42 m from the seat, yaw off -45 deg".into(),
            ),
            (
                SeatStep::Offered { entry: None, feet_to_seat_m: 1.0, yaw_off: 0.0 },
                false,
                "seat: player offered PlacementId(7) slot 1: entry any (no entrances), feet 1.00 m from the seat, yaw off 0 deg"
                    .into(),
            ),
            (SeatStep::Approaching, false, "seat: player walks to PlacementId(7) slot 1".into()),
            (SeatStep::Entered, false, "seat: player entered PlacementId(7) slot 1".into()),
            (SeatStep::Exited, false, "seat: player exited PlacementId(7) slot 1".into()),
            (
                max_correction_step(),
                true,
                format!(
                    "seat: use dropped: enter refused by the action layer (interact.sit.enter: MaxCorrection; RuleKey(5): Probe), {on}"
                ),
            ),
            (
                approach_cancelled_step(),
                true,
                format!("seat: use dropped: enter refused by the action layer (interact.sit.approach: ApproachCancelled), {on}"),
            ),
            (
                SeatStep::EnterRefused(smallvec![]),
                true,
                format!("seat: use dropped: enter refused by the action layer (no refusal named), {on}"),
            ),
        ];
        // `PlacementId`'s `Debug` is its 32 hex digits.
        let placement = format!("{:?}", SLOT.placement);
        assert_eq!(placement, "00000000000000000000000000000007");
        for (step, warning, text) in cases {
            assert_eq!(seat_line(&report(step.clone())), (warning, text.replace("PlacementId(7)", &placement)), "{step:?}");
        }
    }

    #[test]
    fn refused_use_line() {
        let r = UseRefused {
            activation: Activated { target: Handle(12), actor: PLAYER, verb: Some(VERB_OPEN.0) },
            verb: Some(VERB_OPEN),
            reason: TagId(42),
        };
        assert_eq!(refused_line(&r), "interact: use refused: target Handle(12) verb open reason TagId(42)");
        let primary = UseRefused { verb: None, ..r };
        assert_eq!(refused_line(&primary), "interact: use refused: target Handle(12) verb primary reason TagId(42)");
    }

    #[test]
    fn log_reports_reads_both_messages() {
        let mut app = App::new();
        app.add_message::<SeatReport>().add_message::<UseRefused>().add_systems(Update, log_reports);
        app.world_mut().write_message(report(SeatStep::NoEntrance));
        app.world_mut().write_message(UseRefused {
            activation: Activated { target: Handle(12), actor: PLAYER, verb: None },
            verb: None,
            reason: TagId(42),
        });
        app.update();
        app.update();
    }
}
