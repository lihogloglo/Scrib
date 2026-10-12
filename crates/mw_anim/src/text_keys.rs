//! Text keys: animation groups, their sections and their events.
//!
//! A Morrowind animation file holds one timeline. Root text keys
//! (`"<Prefix>: <What>"`, one per line) mark it up:
//! - `<Group>: Start` / `Stop` bound an animation group (one clip each;
//!   weapon groups have no such keys and span their own keys instead);
//!   `Loop Start` / `Loop Stop` inside it bound the looping section.
//! - `<Group>: <Action> ...` keys mark sub-actions of a group: attacks
//!   (`Chop`, `Slash`, `Thrust`, `Shoot`), `Equip` / `Unequip`, spell casts
//!   (`Self`, `Touch`, `Target`), `Block`. Each becomes a [`Section`] plus events.
//! - `SoundGen: <kind>` and `Sound: <id>` keys carry no group: they belong
//!   to every group whose range contains them.
//!
//! The event names produced here are the adapter's vocabulary (see
//! [`events`]); the framework treats them as opaque strings. Group and key
//! matching ignores case, as the original engine does.

use sf_anim::{Event, Looping};

/// Event names this adapter emits (`sf_anim` [`Event::name`]).
pub mod events {
    /// `SoundGen: Left/Right/SwimLeft/SwimRight`; payload `left`, `right`,
    /// `swim_left`, `swim_right`.
    pub const FOOTSTEP: &str = "footstep";
    /// `SoundGen: Land`.
    pub const LAND: &str = "land";
    /// Other `SoundGen` kinds (`Moan`, `Roar`, `Scream`: pain and attack
    /// voices for NPCs and creatures); payload the kind, lowercase.
    pub const SOUND_GEN: &str = "sound_gen";
    /// `Sound: <id>`; payload the sound record id.
    pub const SOUND: &str = "sound";
    /// Window `<a> Min Attack` .. `<a> Max Attack` (wind-up a held attack
    /// can be released in); payload the attack (`chop`, `slash`, `thrust`, `shoot`).
    pub const ATTACK_CHARGE: &str = "attack.charge";
    /// `<a> Min Hit`: where a minimum-strength swing connects; payload the attack.
    pub const ATTACK_MIN_HIT: &str = "attack.min_hit";
    /// `<a> Hit`: the strike lands (damage); payload the attack.
    pub const ATTACK_HIT: &str = "attack.hit";
    /// `Shoot Attach`: the projectile is nocked; payload the attack.
    pub const ATTACK_ATTACH: &str = "attack.attach";
    /// `Shoot Release`: the projectile leaves; payload the attack.
    pub const ATTACK_RELEASE: &str = "attack.release";
    /// Window `<a> [Small|Medium|Large] Follow Start` .. `Stop`: follow-through;
    /// payload `"<attack>"` or `"<attack> <size>"`.
    pub const ATTACK_FOLLOW: &str = "attack.follow";
    /// `Equip Attach`: the item moves to the hand.
    pub const EQUIP_ATTACH: &str = "equip.attach";
    /// `Unequip Detach`: the item leaves the hand.
    pub const UNEQUIP_DETACH: &str = "unequip.detach";
    /// `Block Hit`: the shield block connects (`Shield` group).
    pub const BLOCK_HIT: &str = "block.hit";
    /// `Shoot Follow Attach`: the next projectile is nocked during the follow-through.
    pub const ATTACK_FOLLOW_ATTACH: &str = "attack.follow_attach";
    /// `<Self|Touch|Target> Release`: the spell fires; payload `self`, `touch`, `target`.
    pub const CAST_RELEASE: &str = "cast.release";
    /// Any key this table doesn't know; payload the whole line.
    pub const TEXT_KEY: &str = "text_key";
}

/// Sub-action words that open a [`Section`] inside a group.
const ACTIONS: &[&str] = &["chop", "slash", "thrust", "shoot", "equip", "unequip", "self", "touch", "target", "block"];
const ATTACKS: &[&str] = &["chop", "slash", "thrust", "shoot"];

/// A sub-action of a group (`chop` in `WeaponOneHand`): the range a graph
/// plays for it. Times are seconds from the group start.
#[derive(Clone, Debug, PartialEq)]
pub struct Section {
    /// Lowercase action word: `chop`, `slash`, `thrust`, `shoot`, `equip`,
    /// `unequip`, `self`, `touch`, `target`, `block`.
    pub name: String,
    pub start: f32,
    /// The action's last `... Stop` key (follow-through included).
    pub stop: f32,
}

/// An animation group: one clip of the file's timeline.
#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    /// As written in its `Start` key (`WalkForward`).
    pub name: String,
    /// Absolute key times of `Start` and `Stop` (seconds).
    pub start: f32,
    pub stop: f32,
    /// `Loop Start` .. `Loop Stop`, seconds from `start`.
    pub loop_range: Option<(f32, f32)>,
    pub sections: Vec<Section>,
    /// Sorted by time; seconds from `start`.
    pub events: Vec<Event>,
}

impl Group {
    pub fn duration(&self) -> f32 {
        self.stop - self.start
    }

    /// How the clip loops by default: its `Loop Start`/`Loop Stop` keys, or
    /// the whole clip for idle and movement groups (the original engine
    /// loops those), else once.
    pub fn looping(&self) -> Looping {
        if let Some((a, b)) = self.loop_range {
            return Looping::Range(a, b);
        }
        if loops_whole(&self.name) { Looping::Whole } else { Looping::Once }
    }

    /// The section named `name` (lowercase action word).
    pub fn section(&self, name: &str) -> Option<&Section> {
        self.sections.iter().find(|s| s.name == name)
    }
}

/// Idle and movement groups loop as a whole when they have no loop keys.
fn loops_whole(group: &str) -> bool {
    const LOOPING: &[&str] = &["idle", "walk", "run", "sneak", "turn", "swimwalk", "swimrun", "swimturn"];
    let lower = group.to_ascii_lowercase();
    LOOPING.iter().any(|prefix| lower.starts_with(prefix))
}

/// One parsed text-key line.
#[derive(Clone, Copy, Debug)]
struct Key<'a> {
    time: f32,
    prefix: &'a str,
    what: &'a str,
    line: &'a str,
}

impl Key<'_> {
    /// The event part, lowercase, with the data's spelling variants folded
    /// (`Stop.`, `Start Loop` / `Stop Loop`).
    fn kind(&self) -> String {
        let what = self.what.trim_end_matches('.').trim().to_ascii_lowercase();
        match what.as_str() {
            "start loop" => "loop start".to_owned(),
            "stop loop" => "loop stop".to_owned(),
            _ => what,
        }
    }
}

fn parse(keys: &[(f32, String)]) -> Vec<Key<'_>> {
    keys.iter()
        .map(|(time, line)| {
            let (prefix, what) = line.split_once(':').map_or(("", line.as_str()), |(p, w)| (p.trim(), w.trim()));
            Key { time: *time, prefix, what, line }
        })
        .collect()
}

fn is_sound_prefix(prefix: &str) -> bool {
    prefix.eq_ignore_ascii_case("soundgen") || prefix.eq_ignore_ascii_case("sound")
}

/// Splits text keys into groups, one per key prefix other than `SoundGen`
/// and `Sound`. A group spans its `Start` .. first later `Stop` keys; a
/// group without them (weapon groups hold only sub-action keys) spans its
/// first .. last key. Zero-length groups are dropped. Groups are in
/// timeline order.
pub fn groups(keys: &[(f32, String)]) -> Vec<Group> {
    let keys = parse(keys);
    let mut groups: Vec<Group> = Vec::new();
    for key in &keys {
        if key.prefix.is_empty() || is_sound_prefix(key.prefix) || groups.iter().any(|g| g.name.eq_ignore_ascii_case(key.prefix)) {
            continue;
        }
        let same: Vec<&Key<'_>> = keys.iter().filter(|k| k.prefix.eq_ignore_ascii_case(key.prefix)).collect();
        let named = same.iter().find(|k| k.kind() == "start");
        let start = named.map_or_else(|| same.iter().map(|k| k.time).fold(f32::INFINITY, f32::min), |k| k.time);
        let stop = same
            .iter()
            .find(|k| k.kind() == "stop" && k.time >= start)
            .map_or_else(|| same.iter().map(|k| k.time).fold(f32::NEG_INFINITY, f32::max), |k| k.time);
        if stop - start <= 1e-4 {
            continue;
        }
        let at = |what: &str| same.iter().find(|k| k.kind() == what && (start..=stop).contains(&k.time)).map(|k| k.time - start);
        let loop_range = at("loop start").zip(at("loop stop")).filter(|(a, b)| b > a);
        let name = named.map_or(key.prefix, |k| k.prefix);
        let inside: Vec<Key<'_>> = keys.iter().filter(|k| (start..=stop).contains(&k.time)).copied().collect();
        let (sections, events) = group_contents(key.prefix, start, &inside);
        groups.push(Group { name: name.to_owned(), start, stop, loop_range, sections, events });
    }
    groups.sort_by(|a, b| a.start.total_cmp(&b.start));
    groups
}

/// Sections and events of the group `name` from the keys inside its range.
fn group_contents(name: &str, start: f32, inside: &[Key<'_>]) -> (Vec<Section>, Vec<Event>) {
    let mut events = Vec::new();
    // Per action: (first start, last stop), charge (min, max), follow windows.
    let mut sections: Vec<(String, Option<f32>, Option<f32>)> = Vec::new();
    let mut charges: Vec<(String, Option<f32>, Option<f32>)> = Vec::new();
    let mut follows: Vec<(String, Option<f32>, Option<f32>)> = Vec::new();
    for key in inside {
        let t = key.time - start;
        if is_sound_prefix(key.prefix) {
            events.push(sound_event(t, key));
            continue;
        }
        if !key.prefix.eq_ignore_ascii_case(name) {
            continue; // another group's key (shared boundaries)
        }
        let what = key.kind();
        if matches!(what.as_str(), "start" | "stop" | "loop start" | "loop stop") {
            continue;
        }
        if what == "hit" {
            // Creature attacks (`Attack1: Hit`) name no attack kind.
            events.push(point(t, events::ATTACK_HIT, ""));
            continue;
        }
        let Some((action, verb)) = split_action(&what) else {
            events.push(text_key(t, key));
            continue;
        };
        match verb {
            "start" => {
                slot(&mut sections, action).1.get_or_insert(t);
            }
            "stop" => extend(&mut slot(&mut sections, action).2, t),
            "min attack" if ATTACKS.contains(&action) => slot(&mut charges, action).1 = Some(t),
            "max attack" if ATTACKS.contains(&action) => slot(&mut charges, action).2 = Some(t),
            "min hit" => events.push(point(t, events::ATTACK_MIN_HIT, action)),
            "hit" if action == "block" => events.push(point(t, events::BLOCK_HIT, "")),
            "hit" => events.push(point(t, events::ATTACK_HIT, action)),
            "release" if ATTACKS.contains(&action) => events.push(point(t, events::ATTACK_RELEASE, action)),
            "release" => events.push(point(t, events::CAST_RELEASE, action)),
            "attach" if action == "equip" => events.push(point(t, events::EQUIP_ATTACH, "")),
            "attach" => events.push(point(t, events::ATTACK_ATTACH, action)),
            "detach" if action == "unequip" => events.push(point(t, events::UNEQUIP_DETACH, "")),
            "follow attach" => events.push(point(t, events::ATTACK_FOLLOW_ATTACH, action)),
            _ => {
                if let Some(size) = verb.strip_suffix("follow start") {
                    slot(&mut follows, &follow_payload(action, size.trim())).1 = Some(t);
                } else if let Some(size) = verb.strip_suffix("follow stop") {
                    slot(&mut follows, &follow_payload(action, size.trim())).2 = Some(t);
                    // The follow-through ends its action's section.
                    extend(&mut slot(&mut sections, action).2, t);
                } else {
                    events.push(text_key(t, key));
                }
            }
        }
    }
    for (payload, a, b) in charges {
        match (a, b) {
            (Some(a), Some(b)) if b >= a => events.push(window(a, b, events::ATTACK_CHARGE, &payload)),
            (a, b) => {
                for t in a.into_iter().chain(b) {
                    events.push(point(t, events::ATTACK_CHARGE, &payload));
                }
            }
        }
    }
    for (payload, a, b) in follows {
        if let (Some(a), Some(b)) = (a, b)
            && b >= a
        {
            events.push(window(a, b, events::ATTACK_FOLLOW, &payload));
        }
    }
    events.sort_by(|a, b| a.time.total_cmp(&b.time));
    let sections = sections
        .into_iter()
        .filter_map(|(name, a, b)| {
            let a = a?;
            Some(Section { name, start: a, stop: b.filter(|&b| b >= a).unwrap_or(a) })
        })
        .collect();
    (sections, events)
}

/// `"chop min hit"` → `("chop", "min hit")`; `None` for words outside [`ACTIONS`].
fn split_action(what: &str) -> Option<(&'static str, &str)> {
    let (word, rest) = what.split_once(' ').unwrap_or((what, ""));
    let action = ACTIONS.iter().find(|a| **a == word)?;
    Some((action, rest.trim()))
}

fn follow_payload(action: &str, size: &str) -> String {
    if size.is_empty() { action.to_owned() } else { format!("{action} {size}") }
}

fn slot<'s>(list: &'s mut Vec<(String, Option<f32>, Option<f32>)>, name: &str) -> &'s mut (String, Option<f32>, Option<f32>) {
    let i = list.iter().position(|s| s.0 == name).unwrap_or_else(|| {
        list.push((name.to_owned(), None, None));
        list.len() - 1
    });
    &mut list[i]
}

fn point(time: f32, name: &str, payload: &str) -> Event {
    Event { payload: payload.to_owned(), ..Event::point(time, name) }
}

fn window(start: f32, end: f32, name: &str, payload: &str) -> Event {
    Event { payload: payload.to_owned(), ..Event::window(start, end, name) }
}

fn text_key(time: f32, key: &Key<'_>) -> Event {
    point(time, events::TEXT_KEY, key.line)
}

/// `SoundGen: <kind>` and `Sound: <id>` keys.
fn sound_event(time: f32, key: &Key<'_>) -> Event {
    if key.prefix.eq_ignore_ascii_case("sound") {
        return point(time, events::SOUND, key.what);
    }
    let kind = key.what.to_ascii_lowercase();
    match kind.as_str() {
        "left" | "right" => point(time, events::FOOTSTEP, &kind),
        "swimleft" => point(time, events::FOOTSTEP, "swim_left"),
        "swimright" => point(time, events::FOOTSTEP, "swim_right"),
        "land" => point(time, events::LAND, ""),
        _ => point(time, events::SOUND_GEN, &kind),
    }
}

/// Moves an optional stop time later (keeps the latest).
fn extend(stop: &mut Option<f32>, t: f32) {
    *stop = Some(stop.map_or(t, |s| s.max(t)));
}
