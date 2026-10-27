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
    fn first_candidates(slot: &str) -> Option<f32> {
        Some(match slot {
            "attack.sword_1h.light1" => 0.43,
            "attack.sword_1h.light1_rec" => 0.97,
            "attack.sword_1h.light2" => 0.53,
            "attack.sword_1h.light2_rec" => 1.03,
            "attack.sword_1h.light3" => 2.00,
            "dodge.roll.fwd" => 1.47,
            "dodge.roll.left" | "dodge.roll.right" => 1.30,
            s if s.starts_with("react.") => 0.6,
            _ => return None,
        })
    }

    #[test]
    fn the_sword_sheet_compiles_with_every_move_and_reaction() {
        let sheet = sf_combat::load(SWORD_1H, &first_candidates).expect("sword_1h compiles");
        for mov in ["light1", "light2", "light3", "roll", "roll_left", "roll_right"] {
            assert!(sheet.moves.moves.contains_key(&sheet.rule_key(mov)), "move {mov}");
        }
        let slots = sheet_slots(SWORD_1H).unwrap();
        assert_eq!(slots.len(), 14, "{slots:?}");
        assert!(slots.iter().all(|s| first_candidates(s).is_some()), "{slots:?}");
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
