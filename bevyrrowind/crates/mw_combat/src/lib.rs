//! Morrowind combat content for the framework's `sf_combat` (`docs/specs/combat-adapter.md`):
//! the builtin moveset sheets, the slots they name (the host loads their clips from the baked
//! humanoid graph) and record stats to [`Fighter`] numbers. Bevy-free; the game's Bevy glue lives
//! in `mw_player_bevy` (player, training dummy).
//!
//! C1b: one sheet, [`SWORD_1H`]; the weapon-record mapping and Morrowind text-key windows come
//! with C3 (combat-adapter.md "Weapon records to moveset sheets").

use sf_action::CharacterId;
use sf_combat::{CompiledSheet, Defense, Fighter, HurtShape, Offense, Pool, SheetDef, SheetError, Tier, Vitals};

/// The one-handed sword sheet (combat.md "C1b" first sheet values).
pub const SWORD_1H: &str = include_str!("../data/movesets/sword_1h.ron");

/// Modern stamina = record fatigue x this (Morrowind.esm's player: 200 fatigue -> 100 stamina).
pub const STAMINA_PER_FATIGUE: f32 = 0.5;

/// The skeleton joint whose track carries the weapon (Morrowind's `Weapon Bone`), and the joint
/// used when a rig has none.
pub const WEAPON_BONES: [&str; 2] = ["Weapon Bone", "Bip01 R Hand"];

/// Attack value until weapon records feed `Offense` (C3): a mid iron sword's slash.
pub const DEFAULT_ATTACK: f32 = 10.0;

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
    let mut out: Vec<String> = def.moves.values().flat_map(|m| m.slots.iter().cloned()).chain(def.reactions.into_values()).collect();
    out.sort();
    out.dedup();
    Ok(out)
}

/// A fighter with `sheet`'s weapon: full `health` and stamina from `fatigue`, a hurt capsule of
/// the body's `radius` and `height`, unguarded (no guard in C1), every hit staggers (`Small`).
#[must_use]
pub fn fighter(id: CharacterId, sheet: &CompiledSheet, health: f32, fatigue: f32, (radius, height): (f32, f32), hostile: bool) -> Fighter {
    Fighter {
        id,
        moveset: sheet.key,
        vitals: Vitals { health: Pool::full(health.max(1.0)), stamina: Pool::full(stamina_from_fatigue(fatigue)), poise: None },
        weapon: sheet.weapon,
        hurt: HurtShape::from_capsule(radius, height),
        offense: Offense { attack: DEFAULT_ATTACK },
        defense: Defense { absorb: 0.0, guard_absorb: 0.0, guard_stability: 0.0 },
        hostile,
        min_tier: Tier::Small,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// First-candidate clip lengths (combat.md "C1b" table), clip seconds.
    const CLIP_LENGTHS: [(&str, f32); 17] = [
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
        ("dodge.roll.left", 1.30),
        ("dodge.roll.right", 1.30),
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
        for mov in ["light1", "light2", "light3", "light4", "heavy1", "heavy2", "heavy3", "heavy4", "roll", "roll_left", "roll_right"] {
            assert!(sheet.moves.moves.contains_key(&sheet.rule_key(mov)), "move {mov}");
        }
        let slots = sheet_slots(SWORD_1H).unwrap();
        assert_eq!(slots.len(), 23, "{slots:?}");
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

    #[test]
    fn stamina_is_half_the_record_fatigue() {
        let sheet = sf_combat::load(SWORD_1H, &first_candidates).unwrap();
        let f = fighter(CharacterId(1), &sheet, 50.0, 200.0, (0.3, 1.8), true);
        assert!((f.vitals.stamina.max - 100.0).abs() < 1e-6 && (f.vitals.health.cur - 50.0).abs() < 1e-6);
    }
}
