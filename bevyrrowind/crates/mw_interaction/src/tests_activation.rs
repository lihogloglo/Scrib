//! The activation table against today's prompts (`mw_player_bevy::activatable`,
//! `bevyrrowind/crates/mw_player_bevy/src/lib.rs:798-832`).

use sf_interaction::TagId;

use crate::activation::{
    Availability, Classic, Condition, DefaultAction, ROWS, RecordClass as R, Switch, offered, primary, rows, verb,
};
use crate::furniture::Kind;

/// Today's `activatable` verbs, class by class (a light: carryable only).
const TODAY: &[(R, Option<&str>)] = &[
    (R::Npc, Some("Talk")),
    (R::Door, Some("Open")),
    (R::Container, Some("Open")),
    (R::Activator, Some("Use")),
    (R::Book, Some("Read")),
    (R::Creature, None),
    (R::Weapon, Some("Take")),
    (R::Armor, Some("Take")),
    (R::Clothing, Some("Take")),
    (R::Ingredient, Some("Take")),
    (R::MiscItem, Some("Take")),
    (R::Alchemy, Some("Take")),
    (R::Apparatus, Some("Take")),
    (R::Lockpick, Some("Take")),
    (R::Probe, Some("Take")),
    (R::RepairItem, Some("Take")),
];

fn prompt(class: R, holds: impl Fn(Condition) -> bool) -> Option<&'static str> {
    primary(class, holds, |_| true).map(|r| r.prompt)
}

#[test]
fn todays_verbs_per_class() {
    for &(class, want) in TODAY {
        assert_eq!(prompt(class, |_| false), want, "{class:?}");
    }
    assert_eq!(prompt(R::Light, |c| c == Condition::Carryable), Some("Take"));
    assert_eq!(prompt(R::Light, |_| false), None);
    // Every item class is covered.
    for class in R::ITEMS {
        assert!(TODAY.iter().any(|(c, _)| *c == class), "{class:?}");
    }
}

/// States today's code ignores don't change the verb while their rows are unbuilt.
#[test]
fn unbuilt_states_keep_todays_verb() {
    let all = |_: Condition| true;
    assert_eq!(prompt(R::Npc, all), Some("Talk"));
    assert_eq!(prompt(R::Creature, all), None);
    assert_eq!(prompt(R::Door, all), Some("Open"));
    assert_eq!(primary(R::Door, all, |_| true).map(|r| r.action), Some(DefaultAction::TravelDoor));
    assert_eq!(prompt(R::Container, all), Some("Open"));
    // A carryable lit light: take only (no snuff until IN19).
    let lit: Vec<_> = offered(R::Light, all, |_| true).map(|r| r.verb).collect();
    assert_eq!(lit, [verb::TAKE]);
    // Hold lists: one verb per class today.
    for &(class, _) in TODAY {
        assert!(offered(class, all, |_| true).count() <= 1, "{class:?}");
    }
}

#[test]
fn verb_tags_match_prompts() {
    for &(class, _) in TODAY {
        if let Some(r) = primary(class, |_| false, |_| true) {
            assert_eq!(r.verb, r.prompt.to_ascii_lowercase(), "{class:?}");
        }
    }
    assert_eq!(TagId::of(verb::SIT), TagId::of("sit"));
}

#[test]
fn furniture_verbs_follow_the_sit_switch() {
    let on = |_: Switch| true;
    let off = |s: Switch| s != Switch::Sit;
    let verbs = |kind, sw: &dyn Fn(Switch) -> bool| -> Vec<&str> {
        offered(R::Furniture(kind), |_| false, sw).map(|r| r.verb).collect()
    };
    assert_eq!(verbs(Kind::Seat, &on), [verb::SIT]);
    assert_eq!(verbs(Kind::Bed, &on), [verb::LIE, verb::SIT]);
    assert_eq!(verbs(Kind::Table, &on), [verb::LEAN]);
    assert_eq!(verbs(Kind::Counter, &on), [verb::LEAN]);
    assert!(verbs(Kind::Shelf, &on).is_empty());
    for kind in Kind::ALL {
        assert!(verbs(kind, &off).is_empty(), "{kind:?}");
    }
    assert_eq!(prompt(R::Furniture(Kind::Seat), |_| false), Some("Sit"));
    for r in rows(R::Furniture(Kind::Seat)) {
        assert_eq!((r.classic, r.switch), (Classic::Off, Some(Switch::Sit)));
    }
}

/// No verb without a handler: actions not built yet are never offered, with their slice named.
#[test]
fn unbuilt_actions_are_not_offered() {
    let built = [
        DefaultAction::Dialogue,
        DefaultAction::OpenContainer,
        DefaultAction::TravelDoor,
        DefaultAction::Take,
        DefaultAction::Script,
        DefaultAction::Pose,
    ];
    for r in ROWS {
        match r.available {
            Availability::Offered => assert!(built.contains(&r.action), "{r:?}"),
            Availability::NotYet(slice) => assert!(!slice.is_empty(), "{r:?}"),
        }
    }
    let not_yet = |class, v: &str| {
        rows(class).filter(|r| r.verb == v).all(|r| matches!(r.available, Availability::NotYet(_)))
    };
    assert!(not_yet(R::Light, verb::SNUFF));
    assert!(not_yet(R::Light, verb::RELIGHT));
    assert!(not_yet(R::Door, verb::CLOSE));
    assert!(not_yet(R::Door, verb::BASH));
    assert!(not_yet(R::Container, verb::BASH));
    assert!(not_yet(R::Furniture(Kind::Bed), verb::SLEEP));
    assert!(not_yet(R::Npc, verb::SEARCH));
    assert!(not_yet(R::Npc, verb::PICKPOCKET));
    assert!(not_yet(R::Creature, verb::TALK));
    let slice = |class, v: &str| rows(class).find(|r| r.verb == v).map(|r| r.available);
    assert_eq!(slice(R::Light, verb::SNUFF), Some(Availability::NotYet("IN19")));
    assert_eq!(slice(R::Furniture(Kind::Bed), verb::SLEEP), Some(Availability::NotYet("IN3")));
    assert_eq!(slice(R::Container, verb::BASH), Some(Availability::NotYet("IN37")));
}
