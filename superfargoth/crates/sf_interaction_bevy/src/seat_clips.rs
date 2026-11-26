//! The seat clip binder (`AN26c`, `docs/specs/action.md` "Per-character slot aliases"): the seat
//! rules name fixed slots (`interact.sit.enter`, `.loop`, `.exit`); the clips a use plays depend on
//! the seat ([`SeatClips`]: family, band, sides). [`bind_seat_clips`] writes them into the user's
//! [`ActionAliases`] before the step.
//!
//! - **Sets.** [`SeatClipSets`] indexes the host's clip slot names once by the naming contract
//!   `<family>.b<band cm>.{enter.<side>, loop, exit.<side>}` (sides lower-case; the family is
//!   everything before the `.b<digits>.` segment, a style included: `seat.chair.old`). Other names
//!   are ignored (`loop.fidget` too, for now).
//! - **Resolve.** A use takes its family's set with the nearest band (tie: the lower band), else the
//!   fallback family's nearest (host data), else nothing. A side lookup takes the asked side, else
//!   `front`, else nothing.
//! - **Bind** (`FixedUpdate`, [`SeatClipSystems`], after `SeatSystems::Offer`, which inserts
//!   [`SeatClips`] through commands: the ordering puts a sync point between them, so the offer's
//!   tick already binds; before `ActionSystems::Step`). With clips and a set: enter -> the set's
//!   enter of `clips.enter`, loop -> its loop, exit -> its exit of `clips.exit`; a clip the set
//!   lacks unbinds that slot (the move is then refused as without aliases). Without clips or a set:
//!   the three slots are unbound (only those: other binders own other slots). **Latch:** an alias
//!   whose rule slot is the one playing (`ActionCharacter::output.slot`) is left as it is, rebind or
//!   unbind, until the slot ends (the stand-up keeps the exit side it started with). The component
//!   is written only when a binding changes.

use std::collections::{BTreeMap, HashMap};

use bevy_ecs::prelude::*;
use sf_action::{SlotAliases, SlotKey};
use sf_action_bevy::{ActionAliases, ActionCharacter};
use sf_interaction::Side;

use crate::SeatClips;

/// The seat rules' enter slot (`sf_action` `seat_rules.ron`).
pub const SIT_ENTER: &str = "interact.sit.enter";
/// The seat rules' approach walk (to the enter clip's start pose, when the enter cannot start).
pub const SIT_APPROACH: &str = "interact.sit.approach";
/// The seat rules' seated loop slot.
pub const SIT_LOOP: &str = "interact.sit.loop";
/// The seat rules' stand-up slot.
pub const SIT_EXIT: &str = "interact.sit.exit";

const SIDES: [Side; 6] = [Side::Front, Side::Left, Side::Right, Side::Back, Side::Bottom, Side::Top];

const fn side_index(side: Side) -> usize {
    match side {
        Side::Front => 0,
        Side::Left => 1,
        Side::Right => 2,
        Side::Back => 3,
        Side::Bottom => 4,
        Side::Top => 5,
    }
}

fn parse_side(s: &str) -> Option<Side> {
    SIDES.into_iter().find(|side| side_name(*side) == s)
}

const fn side_name(side: Side) -> &'static str {
    match side {
        Side::Front => "front",
        Side::Left => "left",
        Side::Right => "right",
        Side::Back => "back",
        Side::Bottom => "bottom",
        Side::Top => "top",
    }
}

/// One family's clips at one band: enter and exit clips by side, and the seated loop.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SeatSet {
    enter: [Option<SlotKey>; 6],
    exit: [Option<SlotKey>; 6],
    seated: Option<SlotKey>,
}

impl SeatSet {
    /// The enter clip from `side`, else from the front.
    #[must_use]
    pub fn enter(&self, side: Side) -> Option<SlotKey> {
        self.enter[side_index(side)].or(self.enter[side_index(Side::Front)])
    }

    /// The seated loop clip.
    #[must_use]
    pub fn seated(&self) -> Option<SlotKey> {
        self.seated
    }

    /// The exit clip toward `side`, else toward the front.
    #[must_use]
    pub fn exit(&self, side: Side) -> Option<SlotKey> {
        self.exit[side_index(side)].or(self.exit[side_index(Side::Front)])
    }
}

/// What a name says under the naming contract.
enum Part {
    Enter(Side),
    Loop,
    Exit(Side),
}

/// `<family>.b<cm>.<part>` -> (family, cm, part); `None` for any other name.
fn parse(name: &str) -> Option<(&str, u16, Part)> {
    let mut start = 0;
    for (i, seg) in name.split('.').enumerate() {
        let end = start + seg.len();
        if i > 0
            && let Some(digits) = seg.strip_prefix('b')
            && !digits.is_empty()
            && digits.bytes().all(|b| b.is_ascii_digit())
        {
            let band = digits.parse().ok()?;
            let family = &name[..start - 1];
            let rest: Vec<&str> = name.get(end + 1..)?.split('.').collect();
            let part = match rest.as_slice() {
                ["enter", side] => Part::Enter(parse_side(side)?),
                ["loop"] => Part::Loop,
                ["exit", side] => Part::Exit(parse_side(side)?),
                _ => return None,
            };
            return Some((family, band, part));
        }
        start = end + 1;
    }
    None
}

/// The host's seat clip sets by family and band (module docs), built once from its slot names.
#[derive(Resource, Clone, Debug, Default)]
pub struct SeatClipSets {
    families: HashMap<String, BTreeMap<u16, SeatSet>>,
    fallback: String,
}

impl SeatClipSets {
    /// Indexes `names` by the naming contract; `fallback` is the family used when a seat's own has
    /// no set (data: the host names it).
    pub fn from_names(names: impl IntoIterator<Item = impl AsRef<str>>, fallback: &str) -> Self {
        let mut families: HashMap<String, BTreeMap<u16, SeatSet>> = HashMap::new();
        for name in names {
            let name = name.as_ref();
            let Some((family, band, part)) = parse(name) else { continue };
            let set = families.entry(family.to_owned()).or_default().entry(band).or_default();
            let key = SlotKey::of(name);
            match part {
                Part::Enter(side) => set.enter[side_index(side)] = Some(key),
                Part::Loop => set.seated = Some(key),
                Part::Exit(side) => set.exit[side_index(side)] = Some(key),
            }
        }
        Self { families, fallback: fallback.to_owned() }
    }

    /// `family`'s set nearest `band_cm` (tie: the lower band), else the fallback family's, else
    /// `None`.
    #[must_use]
    pub fn resolve(&self, family: &str, band_cm: u16) -> Option<&SeatSet> {
        self.families
            .get(family)
            .and_then(|bands| nearest(bands, band_cm))
            .or_else(|| self.families.get(&self.fallback).and_then(|bands| nearest(bands, band_cm)))
    }
}

/// The set nearest `band_cm`; ascending bands and a strict `<`: the lower band wins a tie.
fn nearest(bands: &BTreeMap<u16, SeatSet>, band_cm: u16) -> Option<&SeatSet> {
    let mut best: Option<(u16, &SeatSet)> = None;
    for (&band, set) in bands {
        let d = band.abs_diff(band_cm);
        if best.is_none_or(|(b, _)| d < b) {
            best = Some((d, set));
        }
    }
    best.map(|(_, set)| set)
}

/// Where [`bind_seat_clips`] runs (module docs).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SeatClipSystems;

/// Binds or unbinds `rule` toward `want`, unless `rule` is playing and already aliased (the latch);
/// whether the map changed.
fn apply(aliases: &mut SlotAliases, playing: Option<SlotKey>, rule: SlotKey, want: Option<SlotKey>) -> bool {
    if playing == Some(rule) && aliases.get(rule).is_some() {
        return false;
    }
    match want {
        Some(clip) => aliases.bind(rule, clip),
        None => aliases.unbind(rule),
    }
}

/// Writes every slot user's seat aliases from its [`SeatClips`] (module docs).
pub fn bind_seat_clips(
    sets: Option<Res<SeatClipSets>>,
    mut users: Query<(&ActionCharacter, &mut ActionAliases, Option<&SeatClips>)>,
) {
    let (enter, seated, exit) = (SlotKey::of(SIT_ENTER), SlotKey::of(SIT_LOOP), SlotKey::of(SIT_EXIT));
    for (character, mut aliases, clips) in &mut users {
        let set = clips.and_then(|c| sets.as_deref()?.resolve(&c.family.0, c.band_cm).map(|s| (c, s)));
        let (want_enter, want_loop, want_exit) =
            set.map_or((None, None, None), |(c, s)| (s.enter(c.enter), s.seated(), s.exit(c.exit)));
        let playing = character.output.slot;
        let map = &mut aliases.bypass_change_detection().0;
        let mut changed = apply(map, playing, enter, want_enter);
        changed |= apply(map, playing, seated, want_loop);
        changed |= apply(map, playing, exit, want_exit);
        if changed {
            aliases.set_changed();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names() -> Vec<&'static str> {
        vec![
            "seat.chair.b48.enter.front",
            "seat.chair.b48.enter.left",
            "seat.chair.b48.loop",
            "seat.chair.b48.loop.fidget",
            "seat.chair.b48.exit.front",
            "seat.chair.b48.exit.right",
            "seat.chair.b54.enter.front",
            "seat.chair.b54.loop",
            "seat.chair.b54.exit.front",
            "seat.stool.b64.enter.front",
            "seat.stool.b64.loop",
            "seat.stool.b64.exit.front",
            "seat.stool.b70.loop",
            "seat.chair.old.b48.loop",
            "seat.chair.enter.front.seat48.00",
            "traverse.mantle.waist",
            "seat.bench.b40.enter.sideways",
            "seat.bench.b40.wave",
        ]
    }

    #[test]
    fn the_nearest_band_wins() {
        let sets = SeatClipSets::from_names(names(), "seat.chair");
        let loop_of = |family: &str, band| sets.resolve(family, band).and_then(SeatSet::seated);
        assert_eq!(loop_of("seat.chair", 45), Some(SlotKey::of("seat.chair.b48.loop")));
        assert_eq!(loop_of("seat.chair", 53), Some(SlotKey::of("seat.chair.b54.loop")));
        assert_eq!(loop_of("seat.chair", 90), Some(SlotKey::of("seat.chair.b54.loop")));
        assert_eq!(loop_of("seat.stool", 68), Some(SlotKey::of("seat.stool.b70.loop")));
    }

    #[test]
    fn a_tie_takes_the_lower_band() {
        let sets = SeatClipSets::from_names(names(), "seat.chair");
        assert_eq!(sets.resolve("seat.chair", 51).and_then(SeatSet::seated), Some(SlotKey::of("seat.chair.b48.loop")));
        assert_eq!(sets.resolve("seat.stool", 67).and_then(SeatSet::seated), Some(SlotKey::of("seat.stool.b64.loop")));
    }

    #[test]
    fn a_family_with_no_set_takes_the_fallback_family() {
        let sets = SeatClipSets::from_names(names(), "seat.chair");
        let set = sets.resolve("seat.throne", 60).expect("fallback");
        assert_eq!(set.seated(), Some(SlotKey::of("seat.chair.b54.loop")));
        // A style family is its own family.
        let old = sets.resolve("seat.chair.old", 48).expect("style family");
        assert_eq!(old.seated(), Some(SlotKey::of("seat.chair.old.b48.loop")));
        assert_eq!(old.enter(Side::Front), None);
    }

    #[test]
    fn an_unknown_family_without_a_fallback_set_resolves_nothing() {
        let sets = SeatClipSets::from_names(names(), "seat.missing");
        assert!(sets.resolve("seat.throne", 48).is_none());
        assert!(SeatClipSets::from_names(Vec::<String>::new(), "seat.chair").resolve("seat.chair", 48).is_none());
    }

    #[test]
    fn a_side_falls_back_to_front() {
        let sets = SeatClipSets::from_names(names(), "seat.chair");
        let set = sets.resolve("seat.chair", 48).unwrap();
        assert_eq!(set.enter(Side::Left), Some(SlotKey::of("seat.chair.b48.enter.left")));
        assert_eq!(set.enter(Side::Right), Some(SlotKey::of("seat.chair.b48.enter.front")));
        assert_eq!(set.exit(Side::Right), Some(SlotKey::of("seat.chair.b48.exit.right")));
        assert_eq!(set.exit(Side::Back), Some(SlotKey::of("seat.chair.b48.exit.front")));
        let stool70 = sets.resolve("seat.stool", 70).unwrap();
        assert_eq!(stool70.enter(Side::Left), None, "no front either");
    }

    #[test]
    fn names_off_the_contract_are_ignored() {
        let sets = SeatClipSets::from_names(names(), "seat.chair");
        // `seat.bench.b40.*` has only off-contract parts: no set at all, so the fallback answers.
        let bench = sets.resolve("seat.bench", 40).unwrap();
        assert_eq!(bench.seated(), Some(SlotKey::of("seat.chair.b48.loop")));
        // `loop.fidget` does not replace the loop; the old pattern makes no `seat.chair.enter` family.
        assert_eq!(sets.resolve("seat.chair", 48).unwrap().seated(), Some(SlotKey::of("seat.chair.b48.loop")));
        assert!(!sets.families.contains_key("seat.chair.enter.front"));
        assert!(!sets.families.contains_key("traverse.mantle"));
        assert_eq!(sets.families.len(), 3, "{:?}", sets.families.keys().collect::<Vec<_>>());
    }
}
