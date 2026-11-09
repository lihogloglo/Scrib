//! Every SPEL and ALCH record of the installed game files (last version of each ID, load order)
//! maps without dropping an effect of an unknown index (effects-adapter.md "Tests": real
//! install). Prints MGEF gaps, argument notes, and how many entries have no A1 def yet (they
//! are rejected as unknown effects when they land, until their slice). Skips (passes) when no
//! Morrowind install is detected.

use std::collections::BTreeMap;

use mw_effects::{MapNote, MwEffectGmst, MwEffects, effect_name, map_potion, map_spell};
use mw_install::MorrowindInstall;
use tes3::esp::{Alchemy, MagicEffect, ObjectFlags, Plugin, Spell, TES3Object};

#[test]
#[ignore = "real install"]
fn every_spell_and_potion_maps() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let mut mgefs: BTreeMap<i32, MagicEffect> = BTreeMap::new();
    let mut spells: BTreeMap<String, Spell> = BTreeMap::new();
    let mut potions: BTreeMap<String, Alchemy> = BTreeMap::new();
    for file in install.game_files() {
        let bytes = std::fs::read(install.data_files.join(&file)).unwrap();
        let mut plugin = Plugin::new();
        plugin.load_bytes_filtered(&bytes, |tag| matches!(&tag, b"TES3" | b"MGEF" | b"SPEL" | b"ALCH")).unwrap();
        for object in plugin.objects {
            match object {
                TES3Object::MagicEffect(r) => {
                    mgefs.insert(r.effect_id as i32, r);
                }
                TES3Object::Spell(r) => {
                    let id = r.id.to_ascii_lowercase();
                    if r.flags.contains(ObjectFlags::DELETED) {
                        spells.remove(&id);
                    } else {
                        spells.insert(id, r);
                    }
                }
                TES3Object::Alchemy(r) => {
                    let id = r.id.to_ascii_lowercase();
                    if r.flags.contains(ObjectFlags::DELETED) {
                        potions.remove(&id);
                    } else {
                        potions.insert(id, r);
                    }
                }
                _ => {}
            }
        }
    }
    let fx = MwEffects::from_records(mgefs.values());
    let missing: Vec<&str> = fx.missing().filter_map(effect_name).collect();
    eprintln!("MGEF records: {}; indices without one: {missing:?}; bad: {:?}", mgefs.len(), fx.bad_records());
    assert!(fx.bad_records().is_empty());

    let g = MwEffectGmst::vanilla();
    let mut dropped = Vec::new();
    let mut missing_args = Vec::new();
    let mut not_in_a1: BTreeMap<String, u32> = BTreeMap::new();
    let mut entries = 0_u32;
    let mut sources = Vec::new();
    for spell in spells.values() {
        let (src, notes) = map_spell(&fx, &g, spell);
        sources.push((format!("SPEL {}", spell.id), src, notes));
    }
    for potion in potions.values() {
        let (src, notes) = map_potion(&fx, potion);
        sources.push((format!("ALCH {}", potion.id), src, notes));
    }
    for (id, src, notes) in &sources {
        for note in notes {
            match note {
                MapNote::DroppedEffect { index } => dropped.push(format!("{id}: {index}")),
                MapNote::MissingArg { effect, value } => missing_args.push(format!("{id}: {effect} {value}")),
                MapNote::TooManyEffects { dropped: n } => dropped.push(format!("{id}: {n} past the limit")),
            }
        }
        for e in &src.effects {
            entries += 1;
            if fx.table().get(&e.effect).is_none() {
                *not_in_a1.entry(e.effect.to_string()).or_default() += 1;
            }
        }
    }
    eprintln!("SPEL {} ALCH {} entries {entries}", spells.len(), potions.len());
    eprintln!("missing arguments: {} {:?}", missing_args.len(), missing_args.iter().take(30).collect::<Vec<_>>());
    eprintln!("entries without an A1 def: {}", not_in_a1.values().sum::<u32>());
    for (key, n) in &not_in_a1 {
        eprintln!("  {key:<28} {n}");
    }
    assert!(dropped.is_empty(), "dropped effects: {dropped:?}");
}
