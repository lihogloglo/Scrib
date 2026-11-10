//! Morrowind combat content for the framework's `sf_combat` (`docs/specs/combat-adapter.md`):
//! the builtin moveset sheets, the slots they name (the host loads their clips from the baked
//! humanoid graph) and record stats to [`Fighter`] numbers. Bevy-free; the game's Bevy glue lives
//! in `mw_player_bevy` (player, training and sparring dummies; [`SPARRING`] is the sparring
//! dummy's strike schedule).
//!
//! C1b: one sheet, [`SWORD_1H`]; the weapon-record mapping and Morrowind text-key windows come
//! with C3 (combat-adapter.md "Weapon records to moveset sheets").

use sf_action::CharacterId;
use sf_combat::{CompiledSheet, Defense, Fighter, HurtShape, Offense, Pool, SheetDef, SheetError, SparSchedule, Tier, Vitals};

/// The one-handed sword sheet (combat.md "C1b" first sheet values).
pub const SWORD_1H: &str = include_str!("../data/movesets/sword_1h.ron");

/// The sparring dummy's strike schedule (combat.md "C2a"; it fights with [`SWORD_1H`]).
pub const SPARRING: &str = include_str!("../data/sparring.ron");

/// [`SPARRING`] parsed and checked.
///
/// # Errors
/// The RON doesn't parse or a number is out of range ([`SparSchedule::valid`]).
pub fn sparring(ron: &str) -> Result<SparSchedule, String> {
    let s: SparSchedule = ron::from_str(ron).map_err(|e| e.to_string())?;
    if s.valid() { Ok(s) } else { Err(format!("sparring schedule out of range: {s:?}")) }
}

/// The sword sheet's draw clip: `base_anim`'s `WeaponOneHand` equip section, a slot clip of the
/// humanoid graph (its `upper` slot plays it over locomotion). Its `Equip Attach` text key
/// (`mw_anim::events::EQUIP_ATTACH`) is where the weapon reaches the hand.
pub const SWORD_1H_DRAW: &str = "WeaponOneHand@equip";
/// The sword sheet's sheathe clip (`WeaponOneHand` unequip section); its `Unequip Detach` key
/// (`mw_anim::events::UNEQUIP_DETACH`) is where the weapon leaves the hand.
pub const SWORD_1H_SHEATHE: &str = "WeaponOneHand@unequip";

/// Modern stamina = record fatigue x this (Morrowind.esm's player: 200 fatigue -> 100 stamina).
pub const STAMINA_PER_FATIGUE: f32 = 0.5;

/// The skeleton joint whose track carries the weapon (Morrowind's `Weapon Bone`), and the joint
/// used when a rig has none.
pub const WEAPON_BONES: [&str; 2] = ["Weapon Bone", "Bip01 R Hand"];

/// Attack value until weapon records feed `Offense` (C3): a mid iron sword's slash.
pub const DEFAULT_ATTACK: f32 = 10.0;

/// Share of a guarded hit's damage the sword guard absorbs (C2b; weapon records feed it at C3).
/// Elden Ring's straight swords negate roughly 60-70 % of guarded physical damage, shields 100 %.
pub const GUARD_ABSORB: f32 = 0.65;
/// The sword guard's stability (`StandardRules`: guarded stamina damage x 0.55 / this). 1 is the
/// weapon baseline; a shield record raises it (C3).
pub const GUARD_STABILITY: f32 = 1.0;

/// Modern stamina for a record's fatigue.
#[must_use]
pub fn stamina_from_fatigue(fatigue: f32) -> f32 {
    fatigue.max(0.0) * STAMINA_PER_FATIGUE
}

/// Every slot `sheet` names (moves and reactions), sorted and deduplicated: the host looks up a
/// clip for each.
///
/// # Errors
/// The sheet's RON doesn't parse.
pub fn sheet_slots(sheet: &str) -> Result<Vec<String>, SheetError> {
    let ron = ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
    let def: SheetDef = ron.from_str(sheet).map_err(|e| SheetError { sheet: "?".into(), mov: None, message: e.to_string() })?;
    let guard = def.guard_hit.into_iter().chain(def.guard_break);
    let mut out: Vec<String> = def.moves.values().flat_map(|m| m.slots.iter().cloned()).chain(def.reactions.into_values()).chain(guard).collect();
    out.sort();
    out.dedup();
    Ok(out)
}

/// A fighter with `sheet`'s weapon: full `health` and stamina from `fatigue`, a hurt capsule of
/// the body's `radius` and `height`, the sword's guard numbers ([`GUARD_ABSORB`],
/// [`GUARD_STABILITY`]; they count only while a guard move is raised), every unguarded hit
/// staggers (`Small`).
#[must_use]
pub fn fighter(id: CharacterId, sheet: &CompiledSheet, health: f32, fatigue: f32, (radius, height): (f32, f32), hostile: bool) -> Fighter {
    Fighter {
        id,
        moveset: sheet.key,
        vitals: Vitals { health: Pool::full(health.max(1.0)), stamina: Pool::full(stamina_from_fatigue(fatigue)), poise: None },
        weapon: sheet.weapon,
        hurt: HurtShape::from_capsule(radius, height),
        offense: Offense { attack: DEFAULT_ATTACK },
        defense: Defense { absorb: 0.0, guard_absorb: GUARD_ABSORB, guard_stability: GUARD_STABILITY },
        hostile,
        min_tier: Tier::Small,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// First-candidate clip lengths (combat.md "C1b" table), clip seconds.
    const CLIP_LENGTHS: [(&str, f32); 18] = [
        ("attack.sword_1h.light1", 0.37),
        ("attack.sword_1h.light1_rec", 0.50),
        ("attack.sword_1h.light2", 0.43),
        ("attack.sword_1h.light2_rec", 0.57),
        ("attack.sword_1h.light3", 0.87),
        ("attack.sword_1h.light3_rec", 0.70),
        ("attack.sword_1h.light4", 1.67),
        ("attack.sword_1h.heavy1", 0.73),
        ("attack.sword_1h.heavy1_rec", 1.00),
        ("attack.sword_1h.heavy2", 0.50),
        ("attack.sword_1h.heavy2_rec", 0.77),
        ("attack.sword_1h.heavy3", 0.70),
        ("attack.sword_1h.heavy3_rec", 0.57),
        ("attack.sword_1h.heavy4", 2.33),
        ("dodge.roll.fwd", 1.47),
        ("guard.sword_1h.raise", 0.21),
        ("guard.sword_1h.idle", 0.30),
        ("guard.sword_1h.break", 1.07),
    ];

    fn first_candidates(slot: &str) -> Option<f32> {
        if slot.starts_with("react.") {
            return Some(0.6);
        }
        CLIP_LENGTHS.iter().find(|(n, _)| *n == slot).map(|(_, l)| *l)
    }

    #[test]
    fn the_sword_sheet_compiles_with_every_move_and_reaction() {
        let sheet = sf_combat::load(SWORD_1H, &first_candidates).expect("sword_1h compiles");
        for mov in ["light1", "light2", "light3", "light4", "heavy1", "heavy2", "heavy3", "heavy4", "roll", "guard"] {
            assert!(sheet.moves.moves.contains_key(&sheet.rule_key(mov)), "move {mov}");
        }
        // Every attack limits the first-person look while committed (C4b), the guard its pitch
        // (C2b: free yaw); the roll looks free.
        for r in &sheet.rules {
            let limited = matches!(r.category, sf_action::Category::Attack | sf_action::Category::Guard);
            assert_eq!(r.look.is_some(), limited, "{}: look {:?}", r.id, r.look);
        }
        let guard = &sheet.moves.moves[&sheet.rule_key("guard")];
        let g = guard.guard.expect("the guard move guards");
        assert_eq!(g.raise, sf_action::ClockTime::from_ticks(4));
        assert!((g.arc_cos - 80f32.to_radians().cos()).abs() < 1e-6);
        assert_eq!(sheet.reactions.guard_break, Some(sheet.rule_key("guard.break")));
        assert_eq!(sheet.reactions.guard_hit, None, "no guard-hit clip: the guard holds");
        let slots = sheet_slots(SWORD_1H).unwrap();
        assert_eq!(slots.len(), 24, "{slots:?}");
        assert!(slots.iter().all(|s| first_candidates(s).is_some()), "{slots:?}");
    }

    /// Horizontal root travel of each attack slot's first candidate, m: the retargeted UAL2 clips
    /// in the bake, largest over the humanoid skeletons (measured 2026-10-27). The light
    /// recoveries and `Sword_Heavy_C_Rec` stand on their back foot: no root travel.
    const ROOT_TRAVEL: [(&str, f32); 14] = [
        ("attack.sword_1h.light1", 0.37),
        ("attack.sword_1h.light2", 0.43),
        ("attack.sword_1h.light3", 1.69),
        ("attack.sword_1h.light4", 2.03),
        ("attack.sword_1h.light1_rec", 0.0),
        ("attack.sword_1h.light2_rec", 0.0),
        ("attack.sword_1h.light3_rec", 0.0),
        ("attack.sword_1h.heavy1", 1.65),
        ("attack.sword_1h.heavy1_rec", 0.41),
        ("attack.sword_1h.heavy2", 0.89),
        ("attack.sword_1h.heavy2_rec", 0.14),
        ("attack.sword_1h.heavy3", 2.03),
        ("attack.sword_1h.heavy3_rec", 0.0),
        ("attack.sword_1h.heavy4", 1.68),
    ];

    fn root_travel(slot: &str) -> f32 {
        ROOT_TRAVEL.iter().find(|(n, _)| *n == slot).unwrap_or_else(|| panic!("no travel for {slot}")).1
    }

    /// CB-1: a light attack played out (swing + recovery) steps a short way, not a stride.
    #[test]
    fn light_attacks_step_less_than_a_stride() {
        let ron = ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
        let def: SheetDef = ron.from_str(SWORD_1H).unwrap();
        for name in ["light1", "light2", "light3", "light4"] {
            let m = &def.moves[name];
            let travel = m.slots.iter().map(|s| root_travel(s)).sum::<f32>() * m.root_scale;
            eprintln!("{name}: root_scale {} -> {travel:.2} m", m.root_scale);
            assert!((0.2..=0.6).contains(&travel), "{name} steps {travel:.2} m");
        }
    }

    /// A heavy attack played out steps further than a light one, still short of a lunge (sanity
    /// range; the per-move numbers are printed).
    #[test]
    fn heavy_attacks_step_further_than_light_ones() {
        let ron = ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
        let def: SheetDef = ron.from_str(SWORD_1H).unwrap();
        for name in ["heavy1", "heavy2", "heavy3", "heavy4"] {
            let m = &def.moves[name];
            let travel = m.slots.iter().map(|s| root_travel(s)).sum::<f32>() * m.root_scale;
            eprintln!("{name}: root_scale {} -> {travel:.2} m", m.root_scale);
            assert!((0.4..=0.9).contains(&travel), "{name} steps {travel:.2} m");
        }
    }

    /// Heavies chain among themselves; a light or a heavy cancels into the other class (light1 /
    /// heavy1 open from any state), and each heavy hits harder and costs more than any light (its
    /// stagger tier and hit-stop at least as big).
    #[test]
    fn heavies_chain_and_outweigh_lights() {
        use sf_action::ActionClass;
        use sf_combat::WindowDef;
        let ron = ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
        let def: SheetDef = ron.from_str(SWORD_1H).unwrap();
        let hit = |n: &str| def.moves[n].hits[0].clone();
        let lights = ["light1", "light2", "light3", "light4"].map(hit);
        for (name, next) in [("heavy1", "heavy2"), ("heavy2", "heavy3"), ("heavy3", "heavy4"), ("heavy4", "heavy1")] {
            assert_eq!(def.moves[name].next.get(&ActionClass::Heavy).map(String::as_str), Some(next));
            let h = hit(name);
            assert!(lights.iter().all(|l| h.mv > l.mv && h.stamina > l.stamina && h.tier >= l.tier && h.stop >= l.stop), "{name}");
        }
        assert_eq!((def.moves["heavy1"].from.clone(), def.moves["heavy1"].rank), (def.moves["light1"].from.clone(), def.moves["light1"].rank));
        // Every attack cancels into both classes.
        for (name, m) in def.moves.iter().filter(|(n, _)| n.starts_with("light") || n.starts_with("heavy")) {
            let both = m.windows.iter().any(|w| matches!(w, WindowDef::Cancel(c, _, _) if c.contains(&ActionClass::Light) && c.contains(&ActionClass::Heavy)));
            assert!(both, "{name}");
        }
    }

    #[test]
    fn a_sheet_without_roll_clips_still_loads_its_attacks() {
        let sheet = sf_combat::load(SWORD_1H, &|s| if s.starts_with("dodge.") { None } else { first_candidates(s) }).expect("loads");
        assert!(sheet.moves.moves.contains_key(&sheet.rule_key("light1")));
        assert!(!sheet.moves.moves.contains_key(&sheet.rule_key("roll")));
    }

    /// Reaction pick by tier x side on the sword sheet (combat.md "Reactions"): Small has all four
    /// sides; Medium and Large only Front (a missing side falls back to Front); Knockdown and Death
    /// have none (a missing tier falls back to the next lower: Large); Additive is no move.
    #[test]
    fn reactions_by_tier_and_side_fall_back_to_front_then_the_next_lower_tier() {
        use sf_combat::Side;
        let sheet = sf_combat::load(SWORD_1H, &first_candidates).unwrap();
        let key = |slot: &str| sheet.rule_key(slot);
        for side in [Side::Front, Side::Back, Side::Left, Side::Right] {
            let small = format!("react.small.{side:?}").to_lowercase();
            let want = [
                (Tier::Additive, None),
                (Tier::Small, Some(key(&small))),
                (Tier::Medium, Some(key("react.medium.front"))),
                (Tier::Large, Some(key("react.large.front"))),
                (Tier::Knockdown, Some(key("react.large.front"))),
                (Tier::Death, Some(key("react.large.front"))),
            ];
            for (tier, mov) in want {
                assert_eq!(sheet.reactions.lookup(tier, side), mov, "{tier:?} x {side:?}");
            }
        }
    }

    #[test]
    fn the_sparring_schedule_parses_and_strikes_light() {
        let s = sparring(SPARRING).expect("sparring.ron");
        assert_eq!(s, SparSchedule { class: sf_action::ActionClass::Light, first: 1.5, period: 2.5, reach: 2.0 });
        assert!(sparring("(class: Light, first: 0.0, period: 0.0, reach: 2.0)").is_err());
        assert!(sparring("(class: Light, first: 0.0, period: 1.0)").is_err(), "reach is required");
    }

    /// Guard numbers (C2b): a guarded light costs 24 x 0.55 = 13.2 of the player's 100 stamina, so
    /// the eighth light (or fifth heavy) on a full pool breaks the guard without regen; guarded
    /// damage keeps 35 %; and every attack rises into a held guard at its Dodge cancel.
    #[test]
    #[allow(clippy::default_trait_access)] // the contact's glam vectors, without a glam dependency
    fn the_sword_guard_breaks_after_eight_lights_or_five_heavies() {
        use sf_action::ActionClass;
        use sf_combat::{CombatRules, Contact, DefenderState, StandardRules, WindowDef};
        let sheet = sf_combat::load(SWORD_1H, &first_candidates).unwrap();
        let rules = StandardRules::default();
        let blocks = |mov: &str| {
            let row = sheet.moves.moves[&sheet.rule_key(mov)].hits[0];
            let mut d = fighter(CharacterId(2), &sheet, 100.0, 200.0, (0.3, 1.8), false);
            let a = fighter(CharacterId(1), &sheet, 100.0, 200.0, (0.3, 1.8), true);
            let c = Contact {
                attacker: a.id,
                defender: d.id,
                move_key: sheet.rule_key(mov),
                hit: 0,
                strength: 1.0,
                kind: row.kind,
                mv: row.mv,
                flags: row.flags,
                guard_damage: row.guard_damage,
                point: Default::default(),
                direction: Default::default(),
                side: sf_combat::Side::Front,
                defender_state: DefenderState { guarded: true, ..DefenderState::default() },
            };
            for n in 1..=20 {
                let o = rules.resolve_contact(&c, &a, &d);
                assert!(o.guarded);
                let want = a.offense.attack * row.mv * (1.0 - GUARD_ABSORB);
                assert!((o.defender.health + want).abs() < 1e-5, "{mov}: {o:?}");
                if o.guard_break {
                    return n;
                }
                d.vitals.apply(o.defender);
            }
            panic!("{mov}: never broke");
        };
        assert_eq!(blocks("light1"), 8);
        assert_eq!(blocks("heavy1"), 5);
        let def: SheetDef = ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME).from_str(SWORD_1H).unwrap();
        for (name, m) in def.moves.iter().filter(|(n, _)| n.starts_with("light") || n.starts_with("heavy")) {
            let accepts = m.windows.iter().any(|w| matches!(w, WindowDef::Accept(c, _, _) if c.contains(&ActionClass::Guard)));
            let rises = m.windows.iter().any(|w| matches!(w, WindowDef::Cancel(c, _, _) if c.contains(&ActionClass::Guard) && c.contains(&ActionClass::Dodge)));
            assert!(accepts && rises, "{name}");
        }
    }

    #[test]
    fn stamina_is_half_the_record_fatigue() {
        let sheet = sf_combat::load(SWORD_1H, &first_candidates).unwrap();
        let f = fighter(CharacterId(1), &sheet, 50.0, 200.0, (0.3, 1.8), true);
        assert!((f.vitals.stamina.max - 100.0).abs() < 1e-6 && (f.vitals.health.cur - 50.0).abs() < 1e-6);
    }
}
