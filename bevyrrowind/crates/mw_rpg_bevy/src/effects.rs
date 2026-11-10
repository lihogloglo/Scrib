//! Magic effects on the player, NPCs and creatures (effects.md E4, effects-adapter.md "Crates"):
//! `sf_effects_bevy` with Morrowind's MGEF table (`mw_effects`).
//!
//! - **Setup** ([`add`], called by [`crate::stats::add`]: effects need the sheets): the MGEF
//!   records as `EffectTableRes` (`MwEffects`; A1 defs only, the other indices land as
//!   `Rejected { UnknownEffect }`), the modern rules (`EffectRules::standard`), no sources yet
//!   (the SPEL source table comes with abilities and casting, adapter A3 / A5).
//! - **Attach**: `Effects` sits beside every `Stats` the stats glue attaches (the player
//!   character, NPCs, creatures), same owner; see [`crate::stats`].
//! - **Clock** ([`feed_clock`]): `EffectClock` follows the dialogue's game clock (`DaysPassed`,
//!   `GameHour`, [`crate::dialogue::Rpg::game_time`]) every fixed tick, until ROADMAP 1.17's
//!   single host clock replaces both. Durations count fixed ticks, not this clock.

use std::sync::Arc;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use mw_effects::MwEffects;
use sf_effects_bevy::sf_effects::{EffectRules, GameMinutes};
use sf_effects_bevy::{
    EffectClock, EffectRulesRes, EffectSystems, EffectTableRes, SfEffectsPlugin,
};
use sf_records::RecordDb;

use crate::dialogue::Rpg;

/// The effects layer over `db`'s MGEF records (after the stats setup).
pub fn add(app: &mut App, db: &RecordDb) {
    let fx = MwEffects::from_db(db);
    if !fx.bad_records().is_empty() {
        warn!(
            "effects: MGEF records with indices outside the table ignored: {:?}",
            fx.bad_records()
        );
    }
    let missing = fx.missing().count();
    if missing > 0 {
        info!("effects: {missing} effect indices have no MGEF record (their entries are dropped)");
    }
    if !app.is_plugin_added::<SfEffectsPlugin>() {
        app.add_plugins(SfEffectsPlugin);
    }
    app.insert_resource(EffectTableRes(Arc::new(fx.table().clone())))
        .insert_resource(EffectRulesRes(Arc::new(EffectRules::standard())))
        .add_systems(FixedUpdate, feed_clock.before(EffectSystems::Commands));
}

/// `EffectClock` = the game clock in minutes (see the module docs).
#[allow(clippy::needless_pass_by_value)] // system parameters
pub fn feed_clock(rpg: Option<Res<Rpg>>, mut clock: ResMut<EffectClock>) {
    if let Some(rpg) = rpg {
        let now = GameMinutes(rpg.game_time().total_minutes());
        if clock.0 != now {
            clock.0 = now;
        }
    }
}

#[cfg(test)]
mod tests;

/// Synthetic records for headless tests of the stats and effects glue without an install (this
/// crate's and `mw_saves_bevy`'s): the 27 SKIL records, the MGEF records of Fortify and Drain
/// Attribute, and creatures with Strength `strength`.
#[doc(hidden)]
pub mod test_records {
    use std::sync::Arc;

    use sf_records::{RecordDb, RecordId};
    use tes3::esp::{Creature, EffectId, MagicEffect, Skill, SkillData, SkillId};

    const SKILLS: [SkillId; 27] = [
        SkillId::Block,
        SkillId::Armorer,
        SkillId::MediumArmor,
        SkillId::HeavyArmor,
        SkillId::BluntWeapon,
        SkillId::LongBlade,
        SkillId::Axe,
        SkillId::Spear,
        SkillId::Athletics,
        SkillId::Enchant,
        SkillId::Destruction,
        SkillId::Alteration,
        SkillId::Illusion,
        SkillId::Conjuration,
        SkillId::Mysticism,
        SkillId::Restoration,
        SkillId::Alchemy,
        SkillId::Unarmored,
        SkillId::Security,
        SkillId::Sneak,
        SkillId::Acrobatics,
        SkillId::LightArmor,
        SkillId::ShortBlade,
        SkillId::Marksman,
        SkillId::Mercantile,
        SkillId::Speechcraft,
        SkillId::HandToHand,
    ];

    /// The records, with one creature per `(id, strength)`.
    pub fn db(creatures: &[(&str, u32)]) -> Arc<RecordDb> {
        let mut db = RecordDb::new();
        mw_records::register_kinds(&mut db).expect("kinds register");
        let src = db.add_source("test", &[]);
        for (i, skill_id) in SKILLS.into_iter().enumerate() {
            let skill = Skill {
                skill_id,
                data: SkillData {
                    actions: [1.0; 4],
                    ..SkillData::default()
                },
                ..Skill::default()
            };
            db.define(src, RecordId::new(&format!("skill{i}")), skill)
                .expect("skill");
        }
        for (id, effect_id) in [
            ("fortifyattribute", EffectId::FortifyAttribute),
            ("drainattribute", EffectId::DrainAttribute),
        ] {
            db.define(
                src,
                RecordId::new(id),
                MagicEffect {
                    effect_id,
                    ..MagicEffect::default()
                },
            )
            .expect("mgef");
        }
        for &(id, strength) in creatures {
            let mut c = Creature {
                id: id.into(),
                name: id.into(),
                ..Creature::default()
            };
            c.data.strength = strength;
            c.data.health = 50;
            db.define(src, RecordId::new(id), c).expect("creature");
        }
        Arc::new(db)
    }
}
