//! The activation table against today's prompts (`mw_player_bevy::activatable`,
//! `bevyrrowind/crates/mw_player_bevy/src/lib.rs:798-832`).

use sf_interaction::TagId;

use crate::activation::{
    Availability, Classic, Condition, DefaultAction, ROWS, RecordClass as R, Switch, offered, primary, rows, runs_default,
    verb,
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
    // A door's swing rows are built (IN33): with every state holding, the swing's "Open" leads.
    assert_eq!(prompt(R::Door, all), Some("Open"));
    assert_eq!(primary(R::Door, all, |_| true).map(|r| r.action), Some(DefaultAction::SwingDoor));
    assert_eq!(prompt(R::Container, all), Some("Open"));
    // A carryable light (IN19c): take, then snuff and relight; the switch off keeps take only.
    let lit: Vec<_> = offered(R::Light, all, |_| true).map(|r| r.verb).collect();
    assert_eq!(lit, [verb::TAKE, verb::SNUFF, verb::RELIGHT]);
    let classic: Vec<_> = offered(R::Light, all, |s| s != Switch::Snuff).map(|r| r.verb).collect();
    assert_eq!(classic, [verb::TAKE]);
    // A fixed lit flame prompts Snuff, an unlit one Relight.
    assert_eq!(prompt(R::Light, |c| c == Condition::Lit), Some("Snuff"));
    assert_eq!(prompt(R::Light, |c| c == Condition::Unlit), Some("Relight"));
    // Hold lists: one verb per class today, but a door (its swing rows: `swing_door_rows_hold_...`)
    // and a light.
    for &(class, _) in TODAY.iter().filter(|(c, _)| !matches!(c, R::Door | R::Light)) {
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
    assert_eq!(verbs(Kind::Bed, &on), [verb::SLEEP, verb::LIE, verb::SIT]);
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
    // IN3a: a bed's press is Sleep (the rest hook); Classic records the vanilla rest menu.
    assert_eq!(prompt(R::Furniture(Kind::Bed), |_| false), Some("Sleep"));
    let sleep = rows(R::Furniture(Kind::Bed)).find(|r| r.verb == verb::SLEEP).expect("the sleep row");
    assert_eq!((sleep.action, sleep.classic, sleep.switch), (DefaultAction::Rest, Classic::RestMenu, Some(Switch::Sit)));
    assert_eq!(sleep.available, Availability::Offered);
}

/// No verb without a handler: actions not built yet are never offered, with their slice named.
#[test]
fn unbuilt_actions_are_not_offered() {
    let built = [
        DefaultAction::Dialogue,
        DefaultAction::OpenContainer,
        DefaultAction::TravelDoor,
        DefaultAction::SwingDoor,
        DefaultAction::Take,
        DefaultAction::Script,
        DefaultAction::Pose,
        DefaultAction::Rest,
        DefaultAction::ToggleLit,
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
    assert!(!not_yet(R::Light, verb::SNUFF));
    assert!(!not_yet(R::Light, verb::RELIGHT));
    assert!(not_yet(R::Door, verb::BASH));
    assert!(not_yet(R::Container, verb::BASH));
    assert!(not_yet(R::Npc, verb::SEARCH));
    assert!(not_yet(R::Npc, verb::PICKPOCKET));
    assert!(not_yet(R::Creature, verb::TALK));
    let slice = |class, v: &str| rows(class).find(|r| r.verb == v).map(|r| r.available);
    assert_eq!(slice(R::Light, verb::SNUFF), Some(Availability::Offered));
    assert_eq!(slice(R::Furniture(Kind::Bed), verb::SLEEP), Some(Availability::Offered));
    assert_eq!(slice(R::Container, verb::BASH), Some(Availability::NotYet("IN37")));
}

/// Every verb tag of the table.
const VERBS: [&str; 16] = [
    verb::TALK,
    verb::OPEN,
    verb::CLOSE,
    verb::USE,
    verb::READ,
    verb::TAKE,
    verb::SEARCH,
    verb::PICKPOCKET,
    verb::SNUFF,
    verb::RELIGHT,
    verb::SIT,
    verb::LIE,
    verb::LEAN,
    verb::SLEEP,
    verb::BASH,
    verb::BROWSE,
];

/// The verbs `action`'s default runs on, besides the primary use (`None`).
fn runs_on(action: DefaultAction) -> Vec<&'static str> {
    VERBS.into_iter().filter(|v| runs_default(action, Some(TagId::of(v)))).collect()
}

/// `IN32c2`: each class's primary default runs on the primary use and on its own verb only; a pose
/// verb on a def'd container or activator never opens or uses it.
#[test]
fn default_handlers_run_on_their_own_verbs_only() {
    // Class by class: the primary row's action and the verbs it runs on, exactly.
    let table: &[(R, DefaultAction, &[&str])] = &[
        (R::Npc, DefaultAction::Dialogue, &[verb::TALK]),
        (R::Door, DefaultAction::TravelDoor, &[verb::OPEN]),
        (R::Container, DefaultAction::OpenContainer, &[verb::OPEN]),
        (R::Activator, DefaultAction::Script, &[verb::USE]),
        // A book is taken until the reader exists: read and take both run it.
        (R::Book, DefaultAction::Take, &[verb::READ, verb::TAKE]),
        (R::Weapon, DefaultAction::Take, &[verb::READ, verb::TAKE]),
        (R::MiscItem, DefaultAction::Take, &[verb::READ, verb::TAKE]),
    ];
    for &(class, action, verbs) in table {
        let row = primary(class, |_| false, |_| true).expect("an offered primary row");
        assert_eq!(row.action, action, "{class:?}");
        assert!(runs_default(action, None), "{class:?}: the primary use");
        assert!(runs_default(action, Some(TagId::of(row.verb))), "{class:?}: its own verb");
        assert_eq!(runs_on(action), verbs, "{class:?}");
    }
    for pose in [verb::SIT, verb::LIE, verb::LEAN] {
        for action in [DefaultAction::OpenContainer, DefaultAction::Script, DefaultAction::TravelDoor, DefaultAction::Take, DefaultAction::Dialogue] {
            assert!(!runs_default(action, Some(TagId::of(pose))), "{action:?} on {pose}");
        }
    }
    // The pose verbs are the seat default's; unbuilt actions run on nothing.
    assert_eq!(runs_on(DefaultAction::Pose), [verb::SIT, verb::LIE, verb::LEAN]);
    // The rest hook runs on `sleep` only (IN3a).
    assert_eq!(runs_on(DefaultAction::Rest), [verb::SLEEP]);
    // The light default runs on snuff and relight (IN19c; the shell's `light_default`).
    assert_eq!(runs_on(DefaultAction::ToggleLit), [verb::SNUFF, verb::RELIGHT]);
    for action in [DefaultAction::Bash, DefaultAction::Loot] {
        assert!(runs_on(action).is_empty(), "{action:?}");
    }
}

/// `IN33c1`: a door without a destination offers the swing's `open` and `close` ahead of the travel
/// row; a caller that never evaluates `NoDestination` still gets the travel row.
#[test]
fn swing_door_rows_hold_without_a_destination() {
    let no_dest = |c: Condition| c == Condition::NoDestination;
    let rows: Vec<_> = offered(R::Door, no_dest, |_| true).map(|r| (r.verb, r.action)).collect();
    assert_eq!(
        rows,
        [
            (verb::OPEN, DefaultAction::SwingDoor),
            (verb::CLOSE, DefaultAction::SwingDoor),
            (verb::OPEN, DefaultAction::TravelDoor),
        ]
    );
    assert_eq!(primary(R::Door, |_| false, |_| true).map(|r| r.action), Some(DefaultAction::TravelDoor));
    assert_eq!(offered(R::Door, |_| false, |_| true).count(), 1);
}

/// `IN33c1`: the swing default runs on the primary use, `open` and `close`; the travel default never
/// on `close`.
#[test]
fn swing_default_runs_on_open_and_close() {
    assert_eq!(runs_on(DefaultAction::SwingDoor), [verb::OPEN, verb::CLOSE]);
    assert!(runs_default(DefaultAction::SwingDoor, None));
    assert!(runs_default(DefaultAction::SwingDoor, Some(TagId::of(verb::OPEN))));
    assert!(runs_default(DefaultAction::SwingDoor, Some(TagId::of(verb::CLOSE))));
    assert!(!runs_default(DefaultAction::TravelDoor, Some(TagId::of(verb::CLOSE))));
    assert!(!runs_default(DefaultAction::SwingDoor, Some(TagId::of(verb::PASS))));
}
